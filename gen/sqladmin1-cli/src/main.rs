// DO NOT EDIT !
// This file was generated automatically from 'src/generator/templates/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;

use std::io::Write;

use clap::{App, Arg, SubCommand};

use google_sqladmin1::{api, yup_oauth2, Error};

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
    hub: api::SQLAdmin<C>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}

impl<'n, C> Engine<'n, C>
where
    C: apis_common::Connector,
{
    async fn _backups_create_backup(
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
                    "backup-interval.end-time" => Some(("backupInterval.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "backup-interval.start-time" => Some(("backupInterval.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "backup-kind" => Some(("backupKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "backup-run" => Some(("backupRun", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "database-version" => Some(("databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error.code" => Some(("error.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error.kind" => Some(("error.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error.message" => Some(("error.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiry-time" => Some(("expiryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance" => Some(("instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-deletion-time" => Some(("instanceDeletionTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.available-maintenance-versions" => Some(("instanceSettings.availableMaintenanceVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.backend-type" => Some(("instanceSettings.backendType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.connection-name" => Some(("instanceSettings.connectionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.create-time" => Some(("instanceSettings.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.current-disk-size" => Some(("instanceSettings.currentDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.database-installed-version" => Some(("instanceSettings.databaseInstalledVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.database-version" => Some(("instanceSettings.databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-configuration.kind" => Some(("instanceSettings.diskEncryptionConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-configuration.kms-key-name" => Some(("instanceSettings.diskEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-status.kind" => Some(("instanceSettings.diskEncryptionStatus.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-status.kms-key-version-name" => Some(("instanceSettings.diskEncryptionStatus.kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.dns-name" => Some(("instanceSettings.dnsName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.etag" => Some(("instanceSettings.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.failover-replica.available" => Some(("instanceSettings.failoverReplica.available", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.failover-replica.name" => Some(("instanceSettings.failoverReplica.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.gce-zone" => Some(("instanceSettings.gceZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.active-query-enabled" => Some(("instanceSettings.geminiConfig.activeQueryEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.entitled" => Some(("instanceSettings.geminiConfig.entitled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.flag-recommender-enabled" => Some(("instanceSettings.geminiConfig.flagRecommenderEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.google-vacuum-mgmt-enabled" => Some(("instanceSettings.geminiConfig.googleVacuumMgmtEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.index-advisor-enabled" => Some(("instanceSettings.geminiConfig.indexAdvisorEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.oom-session-cancel-enabled" => Some(("instanceSettings.geminiConfig.oomSessionCancelEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.include-replicas-for-major-version-upgrade" => Some(("instanceSettings.includeReplicasForMajorVersionUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.instance-type" => Some(("instanceSettings.instanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.ipv6-address" => Some(("instanceSettings.ipv6Address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.kind" => Some(("instanceSettings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.maintenance-version" => Some(("instanceSettings.maintenanceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.master-instance-name" => Some(("instanceSettings.masterInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.max-disk-size" => Some(("instanceSettings.maxDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.name" => Some(("instanceSettings.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.node-count" => Some(("instanceSettings.nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.ca-certificate" => Some(("instanceSettings.onPremisesConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.client-certificate" => Some(("instanceSettings.onPremisesConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.client-key" => Some(("instanceSettings.onPremisesConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.dump-file-path" => Some(("instanceSettings.onPremisesConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.host-port" => Some(("instanceSettings.onPremisesConfiguration.hostPort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.kind" => Some(("instanceSettings.onPremisesConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.password" => Some(("instanceSettings.onPremisesConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.source-instance.name" => Some(("instanceSettings.onPremisesConfiguration.sourceInstance.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.source-instance.project" => Some(("instanceSettings.onPremisesConfiguration.sourceInstance.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.source-instance.region" => Some(("instanceSettings.onPremisesConfiguration.sourceInstance.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.ssl-option" => Some(("instanceSettings.onPremisesConfiguration.sslOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.username" => Some(("instanceSettings.onPremisesConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.out-of-disk-report.sql-min-recommended-increase-size-gb" => Some(("instanceSettings.outOfDiskReport.sqlMinRecommendedIncreaseSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.out-of-disk-report.sql-out-of-disk-state" => Some(("instanceSettings.outOfDiskReport.sqlOutOfDiskState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.primary-dns-name" => Some(("instanceSettings.primaryDnsName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.project" => Some(("instanceSettings.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.psc-service-attachment-link" => Some(("instanceSettings.pscServiceAttachmentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.region" => Some(("instanceSettings.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.cascadable-replica" => Some(("instanceSettings.replicaConfiguration.cascadableReplica", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.failover-target" => Some(("instanceSettings.replicaConfiguration.failoverTarget", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.kind" => Some(("instanceSettings.replicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.ca-certificate" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.client-certificate" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.client-key" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.connect-retry-interval" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.dump-file-path" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.kind" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.master-heartbeat-period" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.password" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.ssl-cipher" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.sslCipher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.username" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.verify-server-certificate" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replica-names" => Some(("instanceSettings.replicaNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.replication-cluster.dr-replica" => Some(("instanceSettings.replicationCluster.drReplica", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replication-cluster.failover-dr-replica-name" => Some(("instanceSettings.replicationCluster.failoverDrReplicaName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replication-cluster.psa-write-endpoint" => Some(("instanceSettings.replicationCluster.psaWriteEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.root-password" => Some(("instanceSettings.rootPassword", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.satisfies-pzi" => Some(("instanceSettings.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.satisfies-pzs" => Some(("instanceSettings.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.can-defer" => Some(("instanceSettings.scheduledMaintenance.canDefer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.can-reschedule" => Some(("instanceSettings.scheduledMaintenance.canReschedule", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.schedule-deadline-time" => Some(("instanceSettings.scheduledMaintenance.scheduleDeadlineTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.start-time" => Some(("instanceSettings.scheduledMaintenance.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.secondary-gce-zone" => Some(("instanceSettings.secondaryGceZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.self-link" => Some(("instanceSettings.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.cert" => Some(("instanceSettings.serverCaCert.cert", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.cert-serial-number" => Some(("instanceSettings.serverCaCert.certSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.common-name" => Some(("instanceSettings.serverCaCert.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.create-time" => Some(("instanceSettings.serverCaCert.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.expiration-time" => Some(("instanceSettings.serverCaCert.expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.instance" => Some(("instanceSettings.serverCaCert.instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.kind" => Some(("instanceSettings.serverCaCert.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.self-link" => Some(("instanceSettings.serverCaCert.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.sha1-fingerprint" => Some(("instanceSettings.serverCaCert.sha1Fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.service-account-email-address" => Some(("instanceSettings.serviceAccountEmailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.activation-policy" => Some(("instanceSettings.settings.activationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.admin-credential-secret-name" => Some(("instanceSettings.settings.activeDirectoryConfig.adminCredentialSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.dns-servers" => Some(("instanceSettings.settings.activeDirectoryConfig.dnsServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.active-directory-config.domain" => Some(("instanceSettings.settings.activeDirectoryConfig.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.kind" => Some(("instanceSettings.settings.activeDirectoryConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.mode" => Some(("instanceSettings.settings.activeDirectoryConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.organizational-unit" => Some(("instanceSettings.settings.activeDirectoryConfig.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.advanced-machine-features.threads-per-core" => Some(("instanceSettings.settings.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.authorized-gae-applications" => Some(("instanceSettings.settings.authorizedGaeApplications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.auto-upgrade-enabled" => Some(("instanceSettings.settings.autoUpgradeEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.availability-type" => Some(("instanceSettings.settings.availabilityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.backup-retention-settings.retained-backups" => Some(("instanceSettings.settings.backupConfiguration.backupRetentionSettings.retainedBackups", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.backup-retention-settings.retention-unit" => Some(("instanceSettings.settings.backupConfiguration.backupRetentionSettings.retentionUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.backup-tier" => Some(("instanceSettings.settings.backupConfiguration.backupTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.binary-log-enabled" => Some(("instanceSettings.settings.backupConfiguration.binaryLogEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.enabled" => Some(("instanceSettings.settings.backupConfiguration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.kind" => Some(("instanceSettings.settings.backupConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.location" => Some(("instanceSettings.settings.backupConfiguration.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.point-in-time-recovery-enabled" => Some(("instanceSettings.settings.backupConfiguration.pointInTimeRecoveryEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.replication-log-archiving-enabled" => Some(("instanceSettings.settings.backupConfiguration.replicationLogArchivingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.start-time" => Some(("instanceSettings.settings.backupConfiguration.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.transaction-log-retention-days" => Some(("instanceSettings.settings.backupConfiguration.transactionLogRetentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.transactional-log-storage-state" => Some(("instanceSettings.settings.backupConfiguration.transactionalLogStorageState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.collation" => Some(("instanceSettings.settings.collation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.connection-pool-config.connection-pooling-enabled" => Some(("instanceSettings.settings.connectionPoolConfig.connectionPoolingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.connection-pool-config.pooler-count" => Some(("instanceSettings.settings.connectionPoolConfig.poolerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.connector-enforcement" => Some(("instanceSettings.settings.connectorEnforcement", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.crash-safe-replication-enabled" => Some(("instanceSettings.settings.crashSafeReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-api-access" => Some(("instanceSettings.settings.dataApiAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-cache-config.data-cache-enabled" => Some(("instanceSettings.settings.dataCacheConfig.dataCacheEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-provisioned-iops" => Some(("instanceSettings.settings.dataDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-provisioned-throughput" => Some(("instanceSettings.settings.dataDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-size-gb" => Some(("instanceSettings.settings.dataDiskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-type" => Some(("instanceSettings.settings.dataDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.database-replication-enabled" => Some(("instanceSettings.settings.databaseReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.deletion-protection-enabled" => Some(("instanceSettings.settings.deletionProtectionEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.edition" => Some(("instanceSettings.settings.edition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.enable-dataplex-integration" => Some(("instanceSettings.settings.enableDataplexIntegration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.enable-google-ml-integration" => Some(("instanceSettings.settings.enableGoogleMlIntegration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.entraid-config.application-id" => Some(("instanceSettings.settings.entraidConfig.applicationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.entraid-config.kind" => Some(("instanceSettings.settings.entraidConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.entraid-config.tenant-id" => Some(("instanceSettings.settings.entraidConfig.tenantId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.final-backup-config.enabled" => Some(("instanceSettings.settings.finalBackupConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.final-backup-config.retention-days" => Some(("instanceSettings.settings.finalBackupConfig.retentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.query-insights-enabled" => Some(("instanceSettings.settings.insightsConfig.queryInsightsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.query-plans-per-minute" => Some(("instanceSettings.settings.insightsConfig.queryPlansPerMinute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.query-string-length" => Some(("instanceSettings.settings.insightsConfig.queryStringLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.record-application-tags" => Some(("instanceSettings.settings.insightsConfig.recordApplicationTags", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.record-client-address" => Some(("instanceSettings.settings.insightsConfig.recordClientAddress", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.allocated-ip-range" => Some(("instanceSettings.settings.ipConfiguration.allocatedIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.custom-subject-alternative-names" => Some(("instanceSettings.settings.ipConfiguration.customSubjectAlternativeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.ip-configuration.enable-private-path-for-google-cloud-services" => Some(("instanceSettings.settings.ipConfiguration.enablePrivatePathForGoogleCloudServices", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.ipv4-enabled" => Some(("instanceSettings.settings.ipConfiguration.ipv4Enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.private-network" => Some(("instanceSettings.settings.ipConfiguration.privateNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.psc-config.allowed-consumer-projects" => Some(("instanceSettings.settings.ipConfiguration.pscConfig.allowedConsumerProjects", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.ip-configuration.psc-config.network-attachment-uri" => Some(("instanceSettings.settings.ipConfiguration.pscConfig.networkAttachmentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.psc-config.psc-enabled" => Some(("instanceSettings.settings.ipConfiguration.pscConfig.pscEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.require-ssl" => Some(("instanceSettings.settings.ipConfiguration.requireSsl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.server-ca-mode" => Some(("instanceSettings.settings.ipConfiguration.serverCaMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.server-ca-pool" => Some(("instanceSettings.settings.ipConfiguration.serverCaPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.server-certificate-rotation-mode" => Some(("instanceSettings.settings.ipConfiguration.serverCertificateRotationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.ssl-mode" => Some(("instanceSettings.settings.ipConfiguration.sslMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.kind" => Some(("instanceSettings.settings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.follow-gae-application" => Some(("instanceSettings.settings.locationPreference.followGaeApplication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.kind" => Some(("instanceSettings.settings.locationPreference.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.secondary-zone" => Some(("instanceSettings.settings.locationPreference.secondaryZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.zone" => Some(("instanceSettings.settings.locationPreference.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.day" => Some(("instanceSettings.settings.maintenanceWindow.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.hour" => Some(("instanceSettings.settings.maintenanceWindow.hour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.kind" => Some(("instanceSettings.settings.maintenanceWindow.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.update-track" => Some(("instanceSettings.settings.maintenanceWindow.updateTrack", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.complexity" => Some(("instanceSettings.settings.passwordValidationPolicy.complexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.disallow-compromised-credentials" => Some(("instanceSettings.settings.passwordValidationPolicy.disallowCompromisedCredentials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.disallow-username-substring" => Some(("instanceSettings.settings.passwordValidationPolicy.disallowUsernameSubstring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.enable-password-policy" => Some(("instanceSettings.settings.passwordValidationPolicy.enablePasswordPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.min-length" => Some(("instanceSettings.settings.passwordValidationPolicy.minLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.password-change-interval" => Some(("instanceSettings.settings.passwordValidationPolicy.passwordChangeInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.reuse-interval" => Some(("instanceSettings.settings.passwordValidationPolicy.reuseInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.enabled" => Some(("instanceSettings.settings.performanceCaptureConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.probe-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.probeThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.probing-interval-seconds" => Some(("instanceSettings.settings.performanceCaptureConfig.probingIntervalSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.running-threads-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.runningThreadsThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.seconds-behind-source-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.secondsBehindSourceThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.transaction-duration-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.transactionDurationThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.pricing-plan" => Some(("instanceSettings.settings.pricingPlan", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.disable-scale-in" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.disableScaleIn", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.enabled" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.max-node-count" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.min-node-count" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.scale-in-cooldown-seconds" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.scaleInCooldownSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.scale-out-cooldown-seconds" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.scaleOutCooldownSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.replication-lag-max-seconds" => Some(("instanceSettings.settings.replicationLagMaxSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.replication-type" => Some(("instanceSettings.settings.replicationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.retain-backups-on-delete" => Some(("instanceSettings.settings.retainBackupsOnDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.settings-version" => Some(("instanceSettings.settings.settingsVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.bucket" => Some(("instanceSettings.settings.sqlServerAuditConfig.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.kind" => Some(("instanceSettings.settings.sqlServerAuditConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.retention-interval" => Some(("instanceSettings.settings.sqlServerAuditConfig.retentionInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.upload-interval" => Some(("instanceSettings.settings.sqlServerAuditConfig.uploadInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.storage-auto-resize" => Some(("instanceSettings.settings.storageAutoResize", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.storage-auto-resize-limit" => Some(("instanceSettings.settings.storageAutoResizeLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.tier" => Some(("instanceSettings.settings.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.time-zone" => Some(("instanceSettings.settings.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.user-labels" => Some(("instanceSettings.settings.userLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "instance-settings.sql-network-architecture" => Some(("instanceSettings.sqlNetworkArchitecture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.state" => Some(("instanceSettings.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.suspension-reason" => Some(("instanceSettings.suspensionReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.switch-transaction-logs-to-cloud-storage-enabled" => Some(("instanceSettings.switchTransactionLogsToCloudStorageEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.tags" => Some(("instanceSettings.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "instance-settings.write-endpoint" => Some(("instanceSettings.writeEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key" => Some(("kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-version" => Some(("kmsKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-chargeable-bytes" => Some(("maxChargeableBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ttl-days" => Some(("ttlDays", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "active-directory-config", "active-query-enabled", "admin-credential-secret-name", "advanced-machine-features", "allocated-ip-range", "allowed-consumer-projects", "application-id", "authorized-gae-applications", "auto-upgrade-enabled", "availability-type", "available", "available-maintenance-versions", "backend-type", "backup-configuration", "backup-interval", "backup-kind", "backup-retention-settings", "backup-run", "backup-tier", "binary-log-enabled", "bucket", "ca-certificate", "can-defer", "can-reschedule", "cascadable-replica", "cert", "cert-serial-number", "client-certificate", "client-key", "code", "collation", "common-name", "complexity", "connect-retry-interval", "connection-name", "connection-pool-config", "connection-pooling-enabled", "connector-enforcement", "crash-safe-replication-enabled", "create-time", "current-disk-size", "custom-subject-alternative-names", "data-api-access", "data-cache-config", "data-cache-enabled", "data-disk-provisioned-iops", "data-disk-provisioned-throughput", "data-disk-size-gb", "data-disk-type", "database-installed-version", "database-replication-enabled", "database-version", "day", "deletion-protection-enabled", "description", "disable-scale-in", "disallow-compromised-credentials", "disallow-username-substring", "disk-encryption-configuration", "disk-encryption-status", "dns-name", "dns-servers", "domain", "dr-replica", "dump-file-path", "edition", "enable-dataplex-integration", "enable-google-ml-integration", "enable-password-policy", "enable-private-path-for-google-cloud-services", "enabled", "end-time", "entitled", "entraid-config", "error", "etag", "expiration-time", "expiry-time", "failover-dr-replica-name", "failover-replica", "failover-target", "final-backup-config", "flag-recommender-enabled", "follow-gae-application", "gce-zone", "gemini-config", "google-vacuum-mgmt-enabled", "host-port", "hour", "include-replicas-for-major-version-upgrade", "index-advisor-enabled", "insights-config", "instance", "instance-deletion-time", "instance-settings", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "kms-key", "kms-key-name", "kms-key-version", "kms-key-version-name", "location", "location-preference", "maintenance-version", "maintenance-window", "master-heartbeat-period", "master-instance-name", "max-chargeable-bytes", "max-disk-size", "max-node-count", "message", "min-length", "min-node-count", "mode", "mysql-replica-configuration", "name", "network-attachment-uri", "node-count", "on-premises-configuration", "oom-session-cancel-enabled", "organizational-unit", "out-of-disk-report", "password", "password-change-interval", "password-validation-policy", "performance-capture-config", "point-in-time-recovery-enabled", "pooler-count", "pricing-plan", "primary-dns-name", "private-network", "probe-threshold", "probing-interval-seconds", "project", "psa-write-endpoint", "psc-config", "psc-enabled", "psc-service-attachment-link", "query-insights-enabled", "query-plans-per-minute", "query-string-length", "read-pool-auto-scale-config", "record-application-tags", "record-client-address", "region", "replica-configuration", "replica-names", "replication-cluster", "replication-lag-max-seconds", "replication-log-archiving-enabled", "replication-type", "require-ssl", "retain-backups-on-delete", "retained-backups", "retention-days", "retention-interval", "retention-unit", "reuse-interval", "root-password", "running-threads-threshold", "satisfies-pzi", "satisfies-pzs", "scale-in-cooldown-seconds", "scale-out-cooldown-seconds", "schedule-deadline-time", "scheduled-maintenance", "secondary-gce-zone", "secondary-zone", "seconds-behind-source-threshold", "self-link", "server-ca-cert", "server-ca-mode", "server-ca-pool", "server-certificate-rotation-mode", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "source-instance", "sql-min-recommended-increase-size-gb", "sql-network-architecture", "sql-out-of-disk-state", "sql-server-audit-config", "ssl-cipher", "ssl-mode", "ssl-option", "start-time", "state", "storage-auto-resize", "storage-auto-resize-limit", "suspension-reason", "switch-transaction-logs-to-cloud-storage-enabled", "tags", "tenant-id", "threads-per-core", "tier", "time-zone", "transaction-duration-threshold", "transaction-log-retention-days", "transactional-log-storage-state", "ttl-days", "type", "update-track", "upload-interval", "user-labels", "username", "verify-server-certificate", "write-endpoint", "zone"]);
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
        let mut request: api::Backup = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .backups()
            .create_backup(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _backups_delete_backup(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .backups()
            .delete_backup(opt.value_of("name").unwrap_or(""));
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

    async fn _backups_get_backup(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .backups()
            .get_backup(opt.value_of("name").unwrap_or(""));
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

    async fn _backups_list_backups(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .backups()
            .list_backups(opt.value_of("parent").unwrap_or(""));
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

    async fn _backups_update_backup(
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
                    "backup-interval.end-time" => Some(("backupInterval.endTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "backup-interval.start-time" => Some(("backupInterval.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "backup-kind" => Some(("backupKind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "backup-run" => Some(("backupRun", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "database-version" => Some(("databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "description" => Some(("description", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error.code" => Some(("error.code", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error.kind" => Some(("error.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "error.message" => Some(("error.message", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "expiry-time" => Some(("expiryTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance" => Some(("instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-deletion-time" => Some(("instanceDeletionTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.available-maintenance-versions" => Some(("instanceSettings.availableMaintenanceVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.backend-type" => Some(("instanceSettings.backendType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.connection-name" => Some(("instanceSettings.connectionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.create-time" => Some(("instanceSettings.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.current-disk-size" => Some(("instanceSettings.currentDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.database-installed-version" => Some(("instanceSettings.databaseInstalledVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.database-version" => Some(("instanceSettings.databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-configuration.kind" => Some(("instanceSettings.diskEncryptionConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-configuration.kms-key-name" => Some(("instanceSettings.diskEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-status.kind" => Some(("instanceSettings.diskEncryptionStatus.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.disk-encryption-status.kms-key-version-name" => Some(("instanceSettings.diskEncryptionStatus.kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.dns-name" => Some(("instanceSettings.dnsName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.etag" => Some(("instanceSettings.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.failover-replica.available" => Some(("instanceSettings.failoverReplica.available", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.failover-replica.name" => Some(("instanceSettings.failoverReplica.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.gce-zone" => Some(("instanceSettings.gceZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.active-query-enabled" => Some(("instanceSettings.geminiConfig.activeQueryEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.entitled" => Some(("instanceSettings.geminiConfig.entitled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.flag-recommender-enabled" => Some(("instanceSettings.geminiConfig.flagRecommenderEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.google-vacuum-mgmt-enabled" => Some(("instanceSettings.geminiConfig.googleVacuumMgmtEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.index-advisor-enabled" => Some(("instanceSettings.geminiConfig.indexAdvisorEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.gemini-config.oom-session-cancel-enabled" => Some(("instanceSettings.geminiConfig.oomSessionCancelEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.include-replicas-for-major-version-upgrade" => Some(("instanceSettings.includeReplicasForMajorVersionUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.instance-type" => Some(("instanceSettings.instanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.ipv6-address" => Some(("instanceSettings.ipv6Address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.kind" => Some(("instanceSettings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.maintenance-version" => Some(("instanceSettings.maintenanceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.master-instance-name" => Some(("instanceSettings.masterInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.max-disk-size" => Some(("instanceSettings.maxDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.name" => Some(("instanceSettings.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.node-count" => Some(("instanceSettings.nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.ca-certificate" => Some(("instanceSettings.onPremisesConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.client-certificate" => Some(("instanceSettings.onPremisesConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.client-key" => Some(("instanceSettings.onPremisesConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.dump-file-path" => Some(("instanceSettings.onPremisesConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.host-port" => Some(("instanceSettings.onPremisesConfiguration.hostPort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.kind" => Some(("instanceSettings.onPremisesConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.password" => Some(("instanceSettings.onPremisesConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.source-instance.name" => Some(("instanceSettings.onPremisesConfiguration.sourceInstance.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.source-instance.project" => Some(("instanceSettings.onPremisesConfiguration.sourceInstance.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.source-instance.region" => Some(("instanceSettings.onPremisesConfiguration.sourceInstance.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.ssl-option" => Some(("instanceSettings.onPremisesConfiguration.sslOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.on-premises-configuration.username" => Some(("instanceSettings.onPremisesConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.out-of-disk-report.sql-min-recommended-increase-size-gb" => Some(("instanceSettings.outOfDiskReport.sqlMinRecommendedIncreaseSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.out-of-disk-report.sql-out-of-disk-state" => Some(("instanceSettings.outOfDiskReport.sqlOutOfDiskState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.primary-dns-name" => Some(("instanceSettings.primaryDnsName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.project" => Some(("instanceSettings.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.psc-service-attachment-link" => Some(("instanceSettings.pscServiceAttachmentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.region" => Some(("instanceSettings.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.cascadable-replica" => Some(("instanceSettings.replicaConfiguration.cascadableReplica", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.failover-target" => Some(("instanceSettings.replicaConfiguration.failoverTarget", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.kind" => Some(("instanceSettings.replicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.ca-certificate" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.client-certificate" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.client-key" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.connect-retry-interval" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.dump-file-path" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.kind" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.master-heartbeat-period" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.password" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.ssl-cipher" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.sslCipher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.username" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replica-configuration.mysql-replica-configuration.verify-server-certificate" => Some(("instanceSettings.replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replica-names" => Some(("instanceSettings.replicaNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.replication-cluster.dr-replica" => Some(("instanceSettings.replicationCluster.drReplica", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.replication-cluster.failover-dr-replica-name" => Some(("instanceSettings.replicationCluster.failoverDrReplicaName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.replication-cluster.psa-write-endpoint" => Some(("instanceSettings.replicationCluster.psaWriteEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.root-password" => Some(("instanceSettings.rootPassword", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.satisfies-pzi" => Some(("instanceSettings.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.satisfies-pzs" => Some(("instanceSettings.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.can-defer" => Some(("instanceSettings.scheduledMaintenance.canDefer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.can-reschedule" => Some(("instanceSettings.scheduledMaintenance.canReschedule", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.schedule-deadline-time" => Some(("instanceSettings.scheduledMaintenance.scheduleDeadlineTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.scheduled-maintenance.start-time" => Some(("instanceSettings.scheduledMaintenance.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.secondary-gce-zone" => Some(("instanceSettings.secondaryGceZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.self-link" => Some(("instanceSettings.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.cert" => Some(("instanceSettings.serverCaCert.cert", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.cert-serial-number" => Some(("instanceSettings.serverCaCert.certSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.common-name" => Some(("instanceSettings.serverCaCert.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.create-time" => Some(("instanceSettings.serverCaCert.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.expiration-time" => Some(("instanceSettings.serverCaCert.expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.instance" => Some(("instanceSettings.serverCaCert.instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.kind" => Some(("instanceSettings.serverCaCert.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.self-link" => Some(("instanceSettings.serverCaCert.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.server-ca-cert.sha1-fingerprint" => Some(("instanceSettings.serverCaCert.sha1Fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.service-account-email-address" => Some(("instanceSettings.serviceAccountEmailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.activation-policy" => Some(("instanceSettings.settings.activationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.admin-credential-secret-name" => Some(("instanceSettings.settings.activeDirectoryConfig.adminCredentialSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.dns-servers" => Some(("instanceSettings.settings.activeDirectoryConfig.dnsServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.active-directory-config.domain" => Some(("instanceSettings.settings.activeDirectoryConfig.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.kind" => Some(("instanceSettings.settings.activeDirectoryConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.mode" => Some(("instanceSettings.settings.activeDirectoryConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.active-directory-config.organizational-unit" => Some(("instanceSettings.settings.activeDirectoryConfig.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.advanced-machine-features.threads-per-core" => Some(("instanceSettings.settings.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.authorized-gae-applications" => Some(("instanceSettings.settings.authorizedGaeApplications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.auto-upgrade-enabled" => Some(("instanceSettings.settings.autoUpgradeEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.availability-type" => Some(("instanceSettings.settings.availabilityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.backup-retention-settings.retained-backups" => Some(("instanceSettings.settings.backupConfiguration.backupRetentionSettings.retainedBackups", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.backup-retention-settings.retention-unit" => Some(("instanceSettings.settings.backupConfiguration.backupRetentionSettings.retentionUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.backup-tier" => Some(("instanceSettings.settings.backupConfiguration.backupTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.binary-log-enabled" => Some(("instanceSettings.settings.backupConfiguration.binaryLogEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.enabled" => Some(("instanceSettings.settings.backupConfiguration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.kind" => Some(("instanceSettings.settings.backupConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.location" => Some(("instanceSettings.settings.backupConfiguration.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.point-in-time-recovery-enabled" => Some(("instanceSettings.settings.backupConfiguration.pointInTimeRecoveryEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.replication-log-archiving-enabled" => Some(("instanceSettings.settings.backupConfiguration.replicationLogArchivingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.start-time" => Some(("instanceSettings.settings.backupConfiguration.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.transaction-log-retention-days" => Some(("instanceSettings.settings.backupConfiguration.transactionLogRetentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.backup-configuration.transactional-log-storage-state" => Some(("instanceSettings.settings.backupConfiguration.transactionalLogStorageState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.collation" => Some(("instanceSettings.settings.collation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.connection-pool-config.connection-pooling-enabled" => Some(("instanceSettings.settings.connectionPoolConfig.connectionPoolingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.connection-pool-config.pooler-count" => Some(("instanceSettings.settings.connectionPoolConfig.poolerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.connector-enforcement" => Some(("instanceSettings.settings.connectorEnforcement", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.crash-safe-replication-enabled" => Some(("instanceSettings.settings.crashSafeReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-api-access" => Some(("instanceSettings.settings.dataApiAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-cache-config.data-cache-enabled" => Some(("instanceSettings.settings.dataCacheConfig.dataCacheEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-provisioned-iops" => Some(("instanceSettings.settings.dataDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-provisioned-throughput" => Some(("instanceSettings.settings.dataDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-size-gb" => Some(("instanceSettings.settings.dataDiskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.data-disk-type" => Some(("instanceSettings.settings.dataDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.database-replication-enabled" => Some(("instanceSettings.settings.databaseReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.deletion-protection-enabled" => Some(("instanceSettings.settings.deletionProtectionEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.edition" => Some(("instanceSettings.settings.edition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.enable-dataplex-integration" => Some(("instanceSettings.settings.enableDataplexIntegration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.enable-google-ml-integration" => Some(("instanceSettings.settings.enableGoogleMlIntegration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.entraid-config.application-id" => Some(("instanceSettings.settings.entraidConfig.applicationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.entraid-config.kind" => Some(("instanceSettings.settings.entraidConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.entraid-config.tenant-id" => Some(("instanceSettings.settings.entraidConfig.tenantId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.final-backup-config.enabled" => Some(("instanceSettings.settings.finalBackupConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.final-backup-config.retention-days" => Some(("instanceSettings.settings.finalBackupConfig.retentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.query-insights-enabled" => Some(("instanceSettings.settings.insightsConfig.queryInsightsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.query-plans-per-minute" => Some(("instanceSettings.settings.insightsConfig.queryPlansPerMinute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.query-string-length" => Some(("instanceSettings.settings.insightsConfig.queryStringLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.record-application-tags" => Some(("instanceSettings.settings.insightsConfig.recordApplicationTags", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.insights-config.record-client-address" => Some(("instanceSettings.settings.insightsConfig.recordClientAddress", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.allocated-ip-range" => Some(("instanceSettings.settings.ipConfiguration.allocatedIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.custom-subject-alternative-names" => Some(("instanceSettings.settings.ipConfiguration.customSubjectAlternativeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.ip-configuration.enable-private-path-for-google-cloud-services" => Some(("instanceSettings.settings.ipConfiguration.enablePrivatePathForGoogleCloudServices", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.ipv4-enabled" => Some(("instanceSettings.settings.ipConfiguration.ipv4Enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.private-network" => Some(("instanceSettings.settings.ipConfiguration.privateNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.psc-config.allowed-consumer-projects" => Some(("instanceSettings.settings.ipConfiguration.pscConfig.allowedConsumerProjects", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.settings.ip-configuration.psc-config.network-attachment-uri" => Some(("instanceSettings.settings.ipConfiguration.pscConfig.networkAttachmentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.psc-config.psc-enabled" => Some(("instanceSettings.settings.ipConfiguration.pscConfig.pscEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.require-ssl" => Some(("instanceSettings.settings.ipConfiguration.requireSsl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.server-ca-mode" => Some(("instanceSettings.settings.ipConfiguration.serverCaMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.server-ca-pool" => Some(("instanceSettings.settings.ipConfiguration.serverCaPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.server-certificate-rotation-mode" => Some(("instanceSettings.settings.ipConfiguration.serverCertificateRotationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.ip-configuration.ssl-mode" => Some(("instanceSettings.settings.ipConfiguration.sslMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.kind" => Some(("instanceSettings.settings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.follow-gae-application" => Some(("instanceSettings.settings.locationPreference.followGaeApplication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.kind" => Some(("instanceSettings.settings.locationPreference.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.secondary-zone" => Some(("instanceSettings.settings.locationPreference.secondaryZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.location-preference.zone" => Some(("instanceSettings.settings.locationPreference.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.day" => Some(("instanceSettings.settings.maintenanceWindow.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.hour" => Some(("instanceSettings.settings.maintenanceWindow.hour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.kind" => Some(("instanceSettings.settings.maintenanceWindow.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.maintenance-window.update-track" => Some(("instanceSettings.settings.maintenanceWindow.updateTrack", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.complexity" => Some(("instanceSettings.settings.passwordValidationPolicy.complexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.disallow-compromised-credentials" => Some(("instanceSettings.settings.passwordValidationPolicy.disallowCompromisedCredentials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.disallow-username-substring" => Some(("instanceSettings.settings.passwordValidationPolicy.disallowUsernameSubstring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.enable-password-policy" => Some(("instanceSettings.settings.passwordValidationPolicy.enablePasswordPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.min-length" => Some(("instanceSettings.settings.passwordValidationPolicy.minLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.password-change-interval" => Some(("instanceSettings.settings.passwordValidationPolicy.passwordChangeInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.password-validation-policy.reuse-interval" => Some(("instanceSettings.settings.passwordValidationPolicy.reuseInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.enabled" => Some(("instanceSettings.settings.performanceCaptureConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.probe-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.probeThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.probing-interval-seconds" => Some(("instanceSettings.settings.performanceCaptureConfig.probingIntervalSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.running-threads-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.runningThreadsThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.seconds-behind-source-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.secondsBehindSourceThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.performance-capture-config.transaction-duration-threshold" => Some(("instanceSettings.settings.performanceCaptureConfig.transactionDurationThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.pricing-plan" => Some(("instanceSettings.settings.pricingPlan", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.disable-scale-in" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.disableScaleIn", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.enabled" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.max-node-count" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.min-node-count" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.scale-in-cooldown-seconds" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.scaleInCooldownSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.read-pool-auto-scale-config.scale-out-cooldown-seconds" => Some(("instanceSettings.settings.readPoolAutoScaleConfig.scaleOutCooldownSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.replication-lag-max-seconds" => Some(("instanceSettings.settings.replicationLagMaxSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "instance-settings.settings.replication-type" => Some(("instanceSettings.settings.replicationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.retain-backups-on-delete" => Some(("instanceSettings.settings.retainBackupsOnDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.settings-version" => Some(("instanceSettings.settings.settingsVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.bucket" => Some(("instanceSettings.settings.sqlServerAuditConfig.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.kind" => Some(("instanceSettings.settings.sqlServerAuditConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.retention-interval" => Some(("instanceSettings.settings.sqlServerAuditConfig.retentionInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.sql-server-audit-config.upload-interval" => Some(("instanceSettings.settings.sqlServerAuditConfig.uploadInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.storage-auto-resize" => Some(("instanceSettings.settings.storageAutoResize", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.settings.storage-auto-resize-limit" => Some(("instanceSettings.settings.storageAutoResizeLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.tier" => Some(("instanceSettings.settings.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.time-zone" => Some(("instanceSettings.settings.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.settings.user-labels" => Some(("instanceSettings.settings.userLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "instance-settings.sql-network-architecture" => Some(("instanceSettings.sqlNetworkArchitecture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.state" => Some(("instanceSettings.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "instance-settings.suspension-reason" => Some(("instanceSettings.suspensionReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "instance-settings.switch-transaction-logs-to-cloud-storage-enabled" => Some(("instanceSettings.switchTransactionLogsToCloudStorageEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-settings.tags" => Some(("instanceSettings.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "instance-settings.write-endpoint" => Some(("instanceSettings.writeEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kind" => Some(("kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key" => Some(("kmsKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "kms-key-version" => Some(("kmsKeyVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location" => Some(("location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "max-chargeable-bytes" => Some(("maxChargeableBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "satisfies-pzi" => Some(("satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "satisfies-pzs" => Some(("satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "self-link" => Some(("selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "state" => Some(("state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "time-zone" => Some(("timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "ttl-days" => Some(("ttlDays", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "type" => Some(("type", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "active-directory-config", "active-query-enabled", "admin-credential-secret-name", "advanced-machine-features", "allocated-ip-range", "allowed-consumer-projects", "application-id", "authorized-gae-applications", "auto-upgrade-enabled", "availability-type", "available", "available-maintenance-versions", "backend-type", "backup-configuration", "backup-interval", "backup-kind", "backup-retention-settings", "backup-run", "backup-tier", "binary-log-enabled", "bucket", "ca-certificate", "can-defer", "can-reschedule", "cascadable-replica", "cert", "cert-serial-number", "client-certificate", "client-key", "code", "collation", "common-name", "complexity", "connect-retry-interval", "connection-name", "connection-pool-config", "connection-pooling-enabled", "connector-enforcement", "crash-safe-replication-enabled", "create-time", "current-disk-size", "custom-subject-alternative-names", "data-api-access", "data-cache-config", "data-cache-enabled", "data-disk-provisioned-iops", "data-disk-provisioned-throughput", "data-disk-size-gb", "data-disk-type", "database-installed-version", "database-replication-enabled", "database-version", "day", "deletion-protection-enabled", "description", "disable-scale-in", "disallow-compromised-credentials", "disallow-username-substring", "disk-encryption-configuration", "disk-encryption-status", "dns-name", "dns-servers", "domain", "dr-replica", "dump-file-path", "edition", "enable-dataplex-integration", "enable-google-ml-integration", "enable-password-policy", "enable-private-path-for-google-cloud-services", "enabled", "end-time", "entitled", "entraid-config", "error", "etag", "expiration-time", "expiry-time", "failover-dr-replica-name", "failover-replica", "failover-target", "final-backup-config", "flag-recommender-enabled", "follow-gae-application", "gce-zone", "gemini-config", "google-vacuum-mgmt-enabled", "host-port", "hour", "include-replicas-for-major-version-upgrade", "index-advisor-enabled", "insights-config", "instance", "instance-deletion-time", "instance-settings", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "kms-key", "kms-key-name", "kms-key-version", "kms-key-version-name", "location", "location-preference", "maintenance-version", "maintenance-window", "master-heartbeat-period", "master-instance-name", "max-chargeable-bytes", "max-disk-size", "max-node-count", "message", "min-length", "min-node-count", "mode", "mysql-replica-configuration", "name", "network-attachment-uri", "node-count", "on-premises-configuration", "oom-session-cancel-enabled", "organizational-unit", "out-of-disk-report", "password", "password-change-interval", "password-validation-policy", "performance-capture-config", "point-in-time-recovery-enabled", "pooler-count", "pricing-plan", "primary-dns-name", "private-network", "probe-threshold", "probing-interval-seconds", "project", "psa-write-endpoint", "psc-config", "psc-enabled", "psc-service-attachment-link", "query-insights-enabled", "query-plans-per-minute", "query-string-length", "read-pool-auto-scale-config", "record-application-tags", "record-client-address", "region", "replica-configuration", "replica-names", "replication-cluster", "replication-lag-max-seconds", "replication-log-archiving-enabled", "replication-type", "require-ssl", "retain-backups-on-delete", "retained-backups", "retention-days", "retention-interval", "retention-unit", "reuse-interval", "root-password", "running-threads-threshold", "satisfies-pzi", "satisfies-pzs", "scale-in-cooldown-seconds", "scale-out-cooldown-seconds", "schedule-deadline-time", "scheduled-maintenance", "secondary-gce-zone", "secondary-zone", "seconds-behind-source-threshold", "self-link", "server-ca-cert", "server-ca-mode", "server-ca-pool", "server-certificate-rotation-mode", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "source-instance", "sql-min-recommended-increase-size-gb", "sql-network-architecture", "sql-out-of-disk-state", "sql-server-audit-config", "ssl-cipher", "ssl-mode", "ssl-option", "start-time", "state", "storage-auto-resize", "storage-auto-resize-limit", "suspension-reason", "switch-transaction-logs-to-cloud-storage-enabled", "tags", "tenant-id", "threads-per-core", "tier", "time-zone", "transaction-duration-threshold", "transaction-log-retention-days", "transactional-log-storage-state", "ttl-days", "type", "update-track", "upload-interval", "user-labels", "username", "verify-server-certificate", "write-endpoint", "zone"]);
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
        let mut request: api::Backup = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .backups()
            .update_backup(request, opt.value_of("name").unwrap_or(""));
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(
                        value
                            .map(|v| arg_from_str(v, err, "update-mask", "google-fieldmask"))
                            .unwrap_or(apis_common::FieldMask::default()),
                    );
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
                                v.extend(["update-mask"].iter().map(|v| *v));
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

    async fn _backup_runs_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let id: i64 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "int64");
        let mut call = self.hub.backup_runs().delete(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            id,
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

    async fn _backup_runs_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let id: i64 = arg_from_str(&opt.value_of("id").unwrap_or(""), err, "<id>", "int64");
        let mut call = self.hub.backup_runs().get(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            id,
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

    async fn _backup_runs_insert(
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
                "backup-kind" => Some((
                    "backupKind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database-version" => Some((
                    "databaseVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "description" => Some((
                    "description",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kind" => Some((
                    "diskEncryptionConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kms-key-name" => Some((
                    "diskEncryptionConfiguration.kmsKeyName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kind" => Some((
                    "diskEncryptionStatus.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kms-key-version-name" => Some((
                    "diskEncryptionStatus.kmsKeyVersionName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "end-time" => Some((
                    "endTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "enqueued-time" => Some((
                    "enqueuedTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "error.code" => Some((
                    "error.code",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "error.kind" => Some((
                    "error.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "error.message" => Some((
                    "error.message",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "id" => Some((
                    "id",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance" => Some((
                    "instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "location" => Some((
                    "location",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "max-chargeable-bytes" => Some((
                    "maxChargeableBytes",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "self-link" => Some((
                    "selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "start-time" => Some((
                    "startTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "status" => Some((
                    "status",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "time-zone" => Some((
                    "timeZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "type" => Some((
                    "type",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "window-start-time" => Some((
                    "windowStartTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "backup-kind",
                            "code",
                            "database-version",
                            "description",
                            "disk-encryption-configuration",
                            "disk-encryption-status",
                            "end-time",
                            "enqueued-time",
                            "error",
                            "id",
                            "instance",
                            "kind",
                            "kms-key-name",
                            "kms-key-version-name",
                            "location",
                            "max-chargeable-bytes",
                            "message",
                            "self-link",
                            "start-time",
                            "status",
                            "time-zone",
                            "type",
                            "window-start-time",
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
        let mut request: api::BackupRun = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.backup_runs().insert(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _backup_runs_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.backup_runs().list(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
        );
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
                "max-results" => {
                    call = call.max_results(
                        value
                            .map(|v| arg_from_str(v, err, "max-results", "int32"))
                            .unwrap_or(-0),
                    );
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
                                v.extend(["max-results", "page-token"].iter().map(|v| *v));
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

    async fn _connect_generate_ephemeral(
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
                "access-token" => Some((
                    "access_token",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "public-key" => Some((
                    "public_key",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "read-time" => Some((
                    "readTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "valid-duration" => Some((
                    "validDuration",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["access-token", "public-key", "read-time", "valid-duration"],
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
        let mut request: api::GenerateEphemeralCertRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.connect().generate_ephemeral(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _connect_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.connect().get(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "read-time" => {
                    call = call.read_time(
                        value
                            .map(|v| arg_from_str(v, err, "read-time", "google-datetime"))
                            .unwrap_or(chrono::Utc::now()),
                    );
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
                                v.extend(["read-time"].iter().map(|v| *v));
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

    async fn _databases_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.databases().delete(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            opt.value_of("database").unwrap_or(""),
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

    async fn _databases_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.databases().get(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            opt.value_of("database").unwrap_or(""),
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

    async fn _databases_insert(
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
                "charset" => Some((
                    "charset",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "collation" => Some((
                    "collation",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance" => Some((
                    "instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
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
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "self-link" => Some((
                    "selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-database-details.compatibility-level" => Some((
                    "sqlserverDatabaseDetails.compatibilityLevel",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-database-details.recovery-model" => Some((
                    "sqlserverDatabaseDetails.recoveryModel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "charset",
                            "collation",
                            "compatibility-level",
                            "etag",
                            "instance",
                            "kind",
                            "name",
                            "project",
                            "recovery-model",
                            "self-link",
                            "sqlserver-database-details",
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
        let mut request: api::Database = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.databases().insert(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _databases_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.databases().list(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _databases_patch(
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
                "charset" => Some((
                    "charset",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "collation" => Some((
                    "collation",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance" => Some((
                    "instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
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
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "self-link" => Some((
                    "selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-database-details.compatibility-level" => Some((
                    "sqlserverDatabaseDetails.compatibilityLevel",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-database-details.recovery-model" => Some((
                    "sqlserverDatabaseDetails.recoveryModel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "charset",
                            "collation",
                            "compatibility-level",
                            "etag",
                            "instance",
                            "kind",
                            "name",
                            "project",
                            "recovery-model",
                            "self-link",
                            "sqlserver-database-details",
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
        let mut request: api::Database = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.databases().patch(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            opt.value_of("database").unwrap_or(""),
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

    async fn _databases_update(
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
                "charset" => Some((
                    "charset",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "collation" => Some((
                    "collation",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance" => Some((
                    "instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
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
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "self-link" => Some((
                    "selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-database-details.compatibility-level" => Some((
                    "sqlserverDatabaseDetails.compatibilityLevel",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-database-details.recovery-model" => Some((
                    "sqlserverDatabaseDetails.recoveryModel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "charset",
                            "collation",
                            "compatibility-level",
                            "etag",
                            "instance",
                            "kind",
                            "name",
                            "project",
                            "recovery-model",
                            "self-link",
                            "sqlserver-database-details",
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
        let mut request: api::Database = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.databases().update(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            opt.value_of("database").unwrap_or(""),
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

    async fn _flags_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.flags().list();
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "flag-scope" => {
                    call = call.flag_scope(value.unwrap_or(""));
                }
                "database-version" => {
                    call = call.database_version(value.unwrap_or(""));
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
                                v.extend(["database-version", "flag-scope"].iter().map(|v| *v));
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

    async fn _instances_list_entra_id_certificates(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().list_entra_id_certificates(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_list_server_certificates(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().list_server_certificates(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_rotate_entra_id_certificate(
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
                "rotate-entra-id-certificate-context.kind" => Some((
                    "rotateEntraIdCertificateContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "rotate-entra-id-certificate-context.next-version" => Some((
                    "rotateEntraIdCertificateContext.nextVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "kind",
                            "next-version",
                            "rotate-entra-id-certificate-context",
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
        let mut request: api::InstancesRotateEntraIdCertificateRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().rotate_entra_id_certificate(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_rotate_server_certificate(
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
                "rotate-server-certificate-context.kind" => Some((
                    "rotateServerCertificateContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "rotate-server-certificate-context.next-version" => Some((
                    "rotateServerCertificateContext.nextVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["kind", "next-version", "rotate-server-certificate-context"],
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
        let mut request: api::InstancesRotateServerCertificateRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().rotate_server_certificate(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_acquire_ssrs_lease(
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
                "acquire-ssrs-lease-context.duration" => Some((
                    "acquireSsrsLeaseContext.duration",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "acquire-ssrs-lease-context.report-database" => Some((
                    "acquireSsrsLeaseContext.reportDatabase",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "acquire-ssrs-lease-context.service-login" => Some((
                    "acquireSsrsLeaseContext.serviceLogin",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "acquire-ssrs-lease-context.setup-login" => Some((
                    "acquireSsrsLeaseContext.setupLogin",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "acquire-ssrs-lease-context",
                            "duration",
                            "report-database",
                            "service-login",
                            "setup-login",
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
        let mut request: api::InstancesAcquireSsrsLeaseRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().acquire_ssrs_lease(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_add_entra_id_certificate(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().add_entra_id_certificate(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_add_server_ca(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().add_server_ca(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_add_server_certificate(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().add_server_certificate(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_clone(
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
                "clone-context.allocated-ip-range" => Some((
                    "cloneContext.allocatedIpRange",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.bin-log-coordinates.bin-log-file-name" => Some((
                    "cloneContext.binLogCoordinates.binLogFileName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.bin-log-coordinates.bin-log-position" => Some((
                    "cloneContext.binLogCoordinates.binLogPosition",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.bin-log-coordinates.kind" => Some((
                    "cloneContext.binLogCoordinates.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.database-names" => Some((
                    "cloneContext.databaseNames",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "clone-context.destination-instance-name" => Some((
                    "cloneContext.destinationInstanceName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.kind" => Some((
                    "cloneContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.pitr-timestamp-ms" => Some((
                    "cloneContext.pitrTimestampMs",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.point-in-time" => Some((
                    "cloneContext.pointInTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.preferred-secondary-zone" => Some((
                    "cloneContext.preferredSecondaryZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.preferred-zone" => Some((
                    "cloneContext.preferredZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "clone-context.source-instance-deletion-time" => Some((
                    "cloneContext.sourceInstanceDeletionTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "allocated-ip-range",
                            "bin-log-coordinates",
                            "bin-log-file-name",
                            "bin-log-position",
                            "clone-context",
                            "database-names",
                            "destination-instance-name",
                            "kind",
                            "pitr-timestamp-ms",
                            "point-in-time",
                            "preferred-secondary-zone",
                            "preferred-zone",
                            "source-instance-deletion-time",
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
        let mut request: api::InstancesCloneRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().clone(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().delete(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "final-backup-ttl-days" => {
                    call = call.final_backup_ttl_days(
                        value
                            .map(|v| arg_from_str(v, err, "final-backup-ttl-days", "int64"))
                            .unwrap_or(-0),
                    );
                }
                "final-backup-expiry-time" => {
                    call = call.final_backup_expiry_time(
                        value
                            .map(|v| {
                                arg_from_str(v, err, "final-backup-expiry-time", "google-datetime")
                            })
                            .unwrap_or(chrono::Utc::now()),
                    );
                }
                "final-backup-description" => {
                    call = call.final_backup_description(value.unwrap_or(""));
                }
                "enable-final-backup" => {
                    call = call.enable_final_backup(
                        value
                            .map(|v| arg_from_str(v, err, "enable-final-backup", "boolean"))
                            .unwrap_or(false),
                    );
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
                                    [
                                        "enable-final-backup",
                                        "final-backup-description",
                                        "final-backup-expiry-time",
                                        "final-backup-ttl-days",
                                    ]
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

    async fn _instances_demote(
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
                "demote-context.kind" => Some((
                    "demoteContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "demote-context.source-representative-instance-name" => Some((
                    "demoteContext.sourceRepresentativeInstanceName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "demote-context",
                            "kind",
                            "source-representative-instance-name",
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
        let mut request: api::InstancesDemoteRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().demote(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_demote_master(
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
                    "demote-master-context.kind" => Some(("demoteMasterContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.master-instance-name" => Some(("demoteMasterContext.masterInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.replica-configuration.kind" => Some(("demoteMasterContext.replicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.replica-configuration.mysql-replica-configuration.ca-certificate" => Some(("demoteMasterContext.replicaConfiguration.mysqlReplicaConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.replica-configuration.mysql-replica-configuration.client-certificate" => Some(("demoteMasterContext.replicaConfiguration.mysqlReplicaConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.replica-configuration.mysql-replica-configuration.client-key" => Some(("demoteMasterContext.replicaConfiguration.mysqlReplicaConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.replica-configuration.mysql-replica-configuration.kind" => Some(("demoteMasterContext.replicaConfiguration.mysqlReplicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.replica-configuration.mysql-replica-configuration.password" => Some(("demoteMasterContext.replicaConfiguration.mysqlReplicaConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.replica-configuration.mysql-replica-configuration.username" => Some(("demoteMasterContext.replicaConfiguration.mysqlReplicaConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "demote-master-context.skip-replication-setup" => Some(("demoteMasterContext.skipReplicationSetup", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "demote-master-context.verify-gtid-consistency" => Some(("demoteMasterContext.verifyGtidConsistency", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ca-certificate", "client-certificate", "client-key", "demote-master-context", "kind", "master-instance-name", "mysql-replica-configuration", "password", "replica-configuration", "skip-replication-setup", "username", "verify-gtid-consistency"]);
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
        let mut request: api::InstancesDemoteMasterRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().demote_master(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_execute_sql(
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
                "auto-iam-authn" => Some((
                    "autoIamAuthn",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database" => Some((
                    "database",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "partial-result-mode" => Some((
                    "partialResultMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "row-limit" => Some((
                    "rowLimit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sql-statement" => Some((
                    "sqlStatement",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "user" => Some((
                    "user",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "auto-iam-authn",
                            "database",
                            "partial-result-mode",
                            "row-limit",
                            "sql-statement",
                            "user",
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
        let mut request: api::ExecuteSqlPayload = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().execute_sql(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_export(
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
                "export-context.bak-export-options.bak-type" => Some((
                    "exportContext.bakExportOptions.bakType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.bak-export-options.copy-only" => Some((
                    "exportContext.bakExportOptions.copyOnly",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.bak-export-options.differential-base" => Some((
                    "exportContext.bakExportOptions.differentialBase",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.bak-export-options.export-log-end-time" => Some((
                    "exportContext.bakExportOptions.exportLogEndTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.bak-export-options.export-log-start-time" => Some((
                    "exportContext.bakExportOptions.exportLogStartTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.bak-export-options.stripe-count" => Some((
                    "exportContext.bakExportOptions.stripeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.bak-export-options.striped" => Some((
                    "exportContext.bakExportOptions.striped",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.csv-export-options.escape-character" => Some((
                    "exportContext.csvExportOptions.escapeCharacter",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.csv-export-options.fields-terminated-by" => Some((
                    "exportContext.csvExportOptions.fieldsTerminatedBy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.csv-export-options.lines-terminated-by" => Some((
                    "exportContext.csvExportOptions.linesTerminatedBy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.csv-export-options.quote-character" => Some((
                    "exportContext.csvExportOptions.quoteCharacter",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.csv-export-options.select-query" => Some((
                    "exportContext.csvExportOptions.selectQuery",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.databases" => Some((
                    "exportContext.databases",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "export-context.file-type" => Some((
                    "exportContext.fileType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.kind" => Some((
                    "exportContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.offload" => Some((
                    "exportContext.offload",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.sql-export-options.mysql-export-options.master-data" => Some((
                    "exportContext.sqlExportOptions.mysqlExportOptions.masterData",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.sql-export-options.parallel" => Some((
                    "exportContext.sqlExportOptions.parallel",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.sql-export-options.postgres-export-options.clean" => Some((
                    "exportContext.sqlExportOptions.postgresExportOptions.clean",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.sql-export-options.postgres-export-options.if-exists" => Some((
                    "exportContext.sqlExportOptions.postgresExportOptions.ifExists",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.sql-export-options.schema-only" => Some((
                    "exportContext.sqlExportOptions.schemaOnly",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.sql-export-options.tables" => Some((
                    "exportContext.sqlExportOptions.tables",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "export-context.sql-export-options.threads" => Some((
                    "exportContext.sqlExportOptions.threads",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.tde-export-options.certificate-path" => Some((
                    "exportContext.tdeExportOptions.certificatePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.tde-export-options.name" => Some((
                    "exportContext.tdeExportOptions.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.tde-export-options.private-key-password" => Some((
                    "exportContext.tdeExportOptions.privateKeyPassword",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.tde-export-options.private-key-path" => Some((
                    "exportContext.tdeExportOptions.privateKeyPath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "export-context.uri" => Some((
                    "exportContext.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "bak-export-options",
                            "bak-type",
                            "certificate-path",
                            "clean",
                            "copy-only",
                            "csv-export-options",
                            "databases",
                            "differential-base",
                            "escape-character",
                            "export-context",
                            "export-log-end-time",
                            "export-log-start-time",
                            "fields-terminated-by",
                            "file-type",
                            "if-exists",
                            "kind",
                            "lines-terminated-by",
                            "master-data",
                            "mysql-export-options",
                            "name",
                            "offload",
                            "parallel",
                            "postgres-export-options",
                            "private-key-password",
                            "private-key-path",
                            "quote-character",
                            "schema-only",
                            "select-query",
                            "sql-export-options",
                            "stripe-count",
                            "striped",
                            "tables",
                            "tde-export-options",
                            "threads",
                            "uri",
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
        let mut request: api::InstancesExportRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().export(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_failover(
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
                "failover-context.kind" => Some((
                    "failoverContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "failover-context.settings-version" => Some((
                    "failoverContext.settingsVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["failover-context", "kind", "settings-version"],
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
        let mut request: api::InstancesFailoverRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().failover(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().get(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_import(
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
                "import-context.bak-import-options.bak-type" => Some((
                    "importContext.bakImportOptions.bakType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.encryption-options.cert-path" => Some((
                    "importContext.bakImportOptions.encryptionOptions.certPath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.encryption-options.keep-encrypted" => Some((
                    "importContext.bakImportOptions.encryptionOptions.keepEncrypted",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.encryption-options.pvk-password" => Some((
                    "importContext.bakImportOptions.encryptionOptions.pvkPassword",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.encryption-options.pvk-path" => Some((
                    "importContext.bakImportOptions.encryptionOptions.pvkPath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.no-recovery" => Some((
                    "importContext.bakImportOptions.noRecovery",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.recovery-only" => Some((
                    "importContext.bakImportOptions.recoveryOnly",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.stop-at" => Some((
                    "importContext.bakImportOptions.stopAt",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.stop-at-mark" => Some((
                    "importContext.bakImportOptions.stopAtMark",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.bak-import-options.striped" => Some((
                    "importContext.bakImportOptions.striped",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.csv-import-options.columns" => Some((
                    "importContext.csvImportOptions.columns",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "import-context.csv-import-options.escape-character" => Some((
                    "importContext.csvImportOptions.escapeCharacter",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.csv-import-options.fields-terminated-by" => Some((
                    "importContext.csvImportOptions.fieldsTerminatedBy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.csv-import-options.lines-terminated-by" => Some((
                    "importContext.csvImportOptions.linesTerminatedBy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.csv-import-options.quote-character" => Some((
                    "importContext.csvImportOptions.quoteCharacter",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.csv-import-options.table" => Some((
                    "importContext.csvImportOptions.table",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.database" => Some((
                    "importContext.database",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.file-type" => Some((
                    "importContext.fileType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.import-user" => Some((
                    "importContext.importUser",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.kind" => Some((
                    "importContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.sql-import-options.parallel" => Some((
                    "importContext.sqlImportOptions.parallel",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.sql-import-options.postgres-import-options.clean" => Some((
                    "importContext.sqlImportOptions.postgresImportOptions.clean",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.sql-import-options.postgres-import-options.if-exists" => Some((
                    "importContext.sqlImportOptions.postgresImportOptions.ifExists",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.sql-import-options.threads" => Some((
                    "importContext.sqlImportOptions.threads",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.tde-import-options.certificate-path" => Some((
                    "importContext.tdeImportOptions.certificatePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.tde-import-options.name" => Some((
                    "importContext.tdeImportOptions.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.tde-import-options.private-key-password" => Some((
                    "importContext.tdeImportOptions.privateKeyPassword",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.tde-import-options.private-key-path" => Some((
                    "importContext.tdeImportOptions.privateKeyPath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "import-context.uri" => Some((
                    "importContext.uri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "bak-import-options",
                            "bak-type",
                            "cert-path",
                            "certificate-path",
                            "clean",
                            "columns",
                            "csv-import-options",
                            "database",
                            "encryption-options",
                            "escape-character",
                            "fields-terminated-by",
                            "file-type",
                            "if-exists",
                            "import-context",
                            "import-user",
                            "keep-encrypted",
                            "kind",
                            "lines-terminated-by",
                            "name",
                            "no-recovery",
                            "parallel",
                            "postgres-import-options",
                            "private-key-password",
                            "private-key-path",
                            "pvk-password",
                            "pvk-path",
                            "quote-character",
                            "recovery-only",
                            "sql-import-options",
                            "stop-at",
                            "stop-at-mark",
                            "striped",
                            "table",
                            "tde-import-options",
                            "threads",
                            "uri",
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
        let mut request: api::InstancesImportRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().import(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_insert(
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
                "available-maintenance-versions" => Some((
                    "availableMaintenanceVersions",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "backend-type" => Some((
                    "backendType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "connection-name" => Some((
                    "connectionName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "current-disk-size" => Some((
                    "currentDiskSize",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database-installed-version" => Some((
                    "databaseInstalledVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database-version" => Some((
                    "databaseVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kind" => Some((
                    "diskEncryptionConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kms-key-name" => Some((
                    "diskEncryptionConfiguration.kmsKeyName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kind" => Some((
                    "diskEncryptionStatus.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kms-key-version-name" => Some((
                    "diskEncryptionStatus.kmsKeyVersionName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "dns-name" => Some((
                    "dnsName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "failover-replica.available" => Some((
                    "failoverReplica.available",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "failover-replica.name" => Some((
                    "failoverReplica.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gce-zone" => Some((
                    "gceZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.active-query-enabled" => Some((
                    "geminiConfig.activeQueryEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.entitled" => Some((
                    "geminiConfig.entitled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.flag-recommender-enabled" => Some((
                    "geminiConfig.flagRecommenderEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.google-vacuum-mgmt-enabled" => Some((
                    "geminiConfig.googleVacuumMgmtEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.index-advisor-enabled" => Some((
                    "geminiConfig.indexAdvisorEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.oom-session-cancel-enabled" => Some((
                    "geminiConfig.oomSessionCancelEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "include-replicas-for-major-version-upgrade" => Some((
                    "includeReplicasForMajorVersionUpgrade",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance-type" => Some((
                    "instanceType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "ipv6-address" => Some((
                    "ipv6Address",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "maintenance-version" => Some((
                    "maintenanceVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "master-instance-name" => Some((
                    "masterInstanceName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "max-disk-size" => Some((
                    "maxDiskSize",
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
                "on-premises-configuration.ca-certificate" => Some((
                    "onPremisesConfiguration.caCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.client-certificate" => Some((
                    "onPremisesConfiguration.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.client-key" => Some((
                    "onPremisesConfiguration.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.dump-file-path" => Some((
                    "onPremisesConfiguration.dumpFilePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.host-port" => Some((
                    "onPremisesConfiguration.hostPort",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.kind" => Some((
                    "onPremisesConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.password" => Some((
                    "onPremisesConfiguration.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.name" => Some((
                    "onPremisesConfiguration.sourceInstance.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.project" => Some((
                    "onPremisesConfiguration.sourceInstance.project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.region" => Some((
                    "onPremisesConfiguration.sourceInstance.region",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.ssl-option" => Some((
                    "onPremisesConfiguration.sslOption",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.username" => Some((
                    "onPremisesConfiguration.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "out-of-disk-report.sql-min-recommended-increase-size-gb" => Some((
                    "outOfDiskReport.sqlMinRecommendedIncreaseSizeGb",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "out-of-disk-report.sql-out-of-disk-state" => Some((
                    "outOfDiskReport.sqlOutOfDiskState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "primary-dns-name" => Some((
                    "primaryDnsName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "psc-service-attachment-link" => Some((
                    "pscServiceAttachmentLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "region" => Some((
                    "region",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.cascadable-replica" => Some((
                    "replicaConfiguration.cascadableReplica",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.failover-target" => Some((
                    "replicaConfiguration.failoverTarget",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.kind" => Some((
                    "replicaConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.ca-certificate" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.caCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.client-certificate" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.client-key" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval",
                        JsonTypeInfo {
                            jtype: JsonType::Int,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-configuration.mysql-replica-configuration.dump-file-path" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.kind" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod",
                        JsonTypeInfo {
                            jtype: JsonType::String,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-configuration.mysql-replica-configuration.password" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.sslCipher",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.username" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-names" => Some((
                    "replicaNames",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "replication-cluster.dr-replica" => Some((
                    "replicationCluster.drReplica",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replication-cluster.failover-dr-replica-name" => Some((
                    "replicationCluster.failoverDrReplicaName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replication-cluster.psa-write-endpoint" => Some((
                    "replicationCluster.psaWriteEndpoint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "root-password" => Some((
                    "rootPassword",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "satisfies-pzi" => Some((
                    "satisfiesPzi",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "satisfies-pzs" => Some((
                    "satisfiesPzs",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.can-defer" => Some((
                    "scheduledMaintenance.canDefer",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.can-reschedule" => Some((
                    "scheduledMaintenance.canReschedule",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.schedule-deadline-time" => Some((
                    "scheduledMaintenance.scheduleDeadlineTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.start-time" => Some((
                    "scheduledMaintenance.startTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "secondary-gce-zone" => Some((
                    "secondaryGceZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "self-link" => Some((
                    "selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.cert" => Some((
                    "serverCaCert.cert",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.cert-serial-number" => Some((
                    "serverCaCert.certSerialNumber",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.common-name" => Some((
                    "serverCaCert.commonName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.create-time" => Some((
                    "serverCaCert.createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.expiration-time" => Some((
                    "serverCaCert.expirationTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.instance" => Some((
                    "serverCaCert.instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.kind" => Some((
                    "serverCaCert.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.self-link" => Some((
                    "serverCaCert.selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.sha1-fingerprint" => Some((
                    "serverCaCert.sha1Fingerprint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "service-account-email-address" => Some((
                    "serviceAccountEmailAddress",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.activation-policy" => Some((
                    "settings.activationPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.admin-credential-secret-name" => Some((
                    "settings.activeDirectoryConfig.adminCredentialSecretName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.dns-servers" => Some((
                    "settings.activeDirectoryConfig.dnsServers",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.active-directory-config.domain" => Some((
                    "settings.activeDirectoryConfig.domain",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.kind" => Some((
                    "settings.activeDirectoryConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.mode" => Some((
                    "settings.activeDirectoryConfig.mode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.organizational-unit" => Some((
                    "settings.activeDirectoryConfig.organizationalUnit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.advanced-machine-features.threads-per-core" => Some((
                    "settings.advancedMachineFeatures.threadsPerCore",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.authorized-gae-applications" => Some((
                    "settings.authorizedGaeApplications",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.auto-upgrade-enabled" => Some((
                    "settings.autoUpgradeEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.availability-type" => Some((
                    "settings.availabilityType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.backup-retention-settings.retained-backups" => {
                    Some((
                        "settings.backupConfiguration.backupRetentionSettings.retainedBackups",
                        JsonTypeInfo {
                            jtype: JsonType::Int,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "settings.backup-configuration.backup-retention-settings.retention-unit" => Some((
                    "settings.backupConfiguration.backupRetentionSettings.retentionUnit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.backup-tier" => Some((
                    "settings.backupConfiguration.backupTier",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.binary-log-enabled" => Some((
                    "settings.backupConfiguration.binaryLogEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.enabled" => Some((
                    "settings.backupConfiguration.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.kind" => Some((
                    "settings.backupConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.location" => Some((
                    "settings.backupConfiguration.location",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.point-in-time-recovery-enabled" => Some((
                    "settings.backupConfiguration.pointInTimeRecoveryEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.replication-log-archiving-enabled" => Some((
                    "settings.backupConfiguration.replicationLogArchivingEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.start-time" => Some((
                    "settings.backupConfiguration.startTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.transaction-log-retention-days" => Some((
                    "settings.backupConfiguration.transactionLogRetentionDays",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.transactional-log-storage-state" => Some((
                    "settings.backupConfiguration.transactionalLogStorageState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.collation" => Some((
                    "settings.collation",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connection-pool-config.connection-pooling-enabled" => Some((
                    "settings.connectionPoolConfig.connectionPoolingEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connection-pool-config.pooler-count" => Some((
                    "settings.connectionPoolConfig.poolerCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connector-enforcement" => Some((
                    "settings.connectorEnforcement",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.crash-safe-replication-enabled" => Some((
                    "settings.crashSafeReplicationEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-api-access" => Some((
                    "settings.dataApiAccess",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-cache-config.data-cache-enabled" => Some((
                    "settings.dataCacheConfig.dataCacheEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-provisioned-iops" => Some((
                    "settings.dataDiskProvisionedIops",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-provisioned-throughput" => Some((
                    "settings.dataDiskProvisionedThroughput",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-size-gb" => Some((
                    "settings.dataDiskSizeGb",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-type" => Some((
                    "settings.dataDiskType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.database-replication-enabled" => Some((
                    "settings.databaseReplicationEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.deletion-protection-enabled" => Some((
                    "settings.deletionProtectionEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.edition" => Some((
                    "settings.edition",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.enable-dataplex-integration" => Some((
                    "settings.enableDataplexIntegration",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.enable-google-ml-integration" => Some((
                    "settings.enableGoogleMlIntegration",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.application-id" => Some((
                    "settings.entraidConfig.applicationId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.kind" => Some((
                    "settings.entraidConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.tenant-id" => Some((
                    "settings.entraidConfig.tenantId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.final-backup-config.enabled" => Some((
                    "settings.finalBackupConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.final-backup-config.retention-days" => Some((
                    "settings.finalBackupConfig.retentionDays",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-insights-enabled" => Some((
                    "settings.insightsConfig.queryInsightsEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-plans-per-minute" => Some((
                    "settings.insightsConfig.queryPlansPerMinute",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-string-length" => Some((
                    "settings.insightsConfig.queryStringLength",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.record-application-tags" => Some((
                    "settings.insightsConfig.recordApplicationTags",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.record-client-address" => Some((
                    "settings.insightsConfig.recordClientAddress",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.allocated-ip-range" => Some((
                    "settings.ipConfiguration.allocatedIpRange",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.custom-subject-alternative-names" => Some((
                    "settings.ipConfiguration.customSubjectAlternativeNames",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.ip-configuration.enable-private-path-for-google-cloud-services" => {
                    Some((
                        "settings.ipConfiguration.enablePrivatePathForGoogleCloudServices",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "settings.ip-configuration.ipv4-enabled" => Some((
                    "settings.ipConfiguration.ipv4Enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.private-network" => Some((
                    "settings.ipConfiguration.privateNetwork",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.psc-config.allowed-consumer-projects" => Some((
                    "settings.ipConfiguration.pscConfig.allowedConsumerProjects",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.ip-configuration.psc-config.network-attachment-uri" => Some((
                    "settings.ipConfiguration.pscConfig.networkAttachmentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.psc-config.psc-enabled" => Some((
                    "settings.ipConfiguration.pscConfig.pscEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.require-ssl" => Some((
                    "settings.ipConfiguration.requireSsl",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-ca-mode" => Some((
                    "settings.ipConfiguration.serverCaMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-ca-pool" => Some((
                    "settings.ipConfiguration.serverCaPool",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-certificate-rotation-mode" => Some((
                    "settings.ipConfiguration.serverCertificateRotationMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.ssl-mode" => Some((
                    "settings.ipConfiguration.sslMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.kind" => Some((
                    "settings.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.follow-gae-application" => Some((
                    "settings.locationPreference.followGaeApplication",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.kind" => Some((
                    "settings.locationPreference.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.secondary-zone" => Some((
                    "settings.locationPreference.secondaryZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.zone" => Some((
                    "settings.locationPreference.zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.day" => Some((
                    "settings.maintenanceWindow.day",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.hour" => Some((
                    "settings.maintenanceWindow.hour",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.kind" => Some((
                    "settings.maintenanceWindow.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.update-track" => Some((
                    "settings.maintenanceWindow.updateTrack",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.complexity" => Some((
                    "settings.passwordValidationPolicy.complexity",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.disallow-compromised-credentials" => Some((
                    "settings.passwordValidationPolicy.disallowCompromisedCredentials",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.disallow-username-substring" => Some((
                    "settings.passwordValidationPolicy.disallowUsernameSubstring",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.enable-password-policy" => Some((
                    "settings.passwordValidationPolicy.enablePasswordPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.min-length" => Some((
                    "settings.passwordValidationPolicy.minLength",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.password-change-interval" => Some((
                    "settings.passwordValidationPolicy.passwordChangeInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.reuse-interval" => Some((
                    "settings.passwordValidationPolicy.reuseInterval",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.enabled" => Some((
                    "settings.performanceCaptureConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.probe-threshold" => Some((
                    "settings.performanceCaptureConfig.probeThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.probing-interval-seconds" => Some((
                    "settings.performanceCaptureConfig.probingIntervalSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.running-threads-threshold" => Some((
                    "settings.performanceCaptureConfig.runningThreadsThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.seconds-behind-source-threshold" => Some((
                    "settings.performanceCaptureConfig.secondsBehindSourceThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.transaction-duration-threshold" => Some((
                    "settings.performanceCaptureConfig.transactionDurationThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.pricing-plan" => Some((
                    "settings.pricingPlan",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.disable-scale-in" => Some((
                    "settings.readPoolAutoScaleConfig.disableScaleIn",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.enabled" => Some((
                    "settings.readPoolAutoScaleConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.max-node-count" => Some((
                    "settings.readPoolAutoScaleConfig.maxNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.min-node-count" => Some((
                    "settings.readPoolAutoScaleConfig.minNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.scale-in-cooldown-seconds" => Some((
                    "settings.readPoolAutoScaleConfig.scaleInCooldownSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.scale-out-cooldown-seconds" => Some((
                    "settings.readPoolAutoScaleConfig.scaleOutCooldownSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.replication-lag-max-seconds" => Some((
                    "settings.replicationLagMaxSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.replication-type" => Some((
                    "settings.replicationType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.retain-backups-on-delete" => Some((
                    "settings.retainBackupsOnDelete",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.settings-version" => Some((
                    "settings.settingsVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.bucket" => Some((
                    "settings.sqlServerAuditConfig.bucket",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.kind" => Some((
                    "settings.sqlServerAuditConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.retention-interval" => Some((
                    "settings.sqlServerAuditConfig.retentionInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.upload-interval" => Some((
                    "settings.sqlServerAuditConfig.uploadInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.storage-auto-resize" => Some((
                    "settings.storageAutoResize",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.storage-auto-resize-limit" => Some((
                    "settings.storageAutoResizeLimit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.tier" => Some((
                    "settings.tier",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.time-zone" => Some((
                    "settings.timeZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.user-labels" => Some((
                    "settings.userLabels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "sql-network-architecture" => Some((
                    "sqlNetworkArchitecture",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "state" => Some((
                    "state",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "suspension-reason" => Some((
                    "suspensionReason",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "switch-transaction-logs-to-cloud-storage-enabled" => Some((
                    "switchTransactionLogsToCloudStorageEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "tags" => Some((
                    "tags",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "write-endpoint" => Some((
                    "writeEndpoint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "activation-policy",
                            "active-directory-config",
                            "active-query-enabled",
                            "admin-credential-secret-name",
                            "advanced-machine-features",
                            "allocated-ip-range",
                            "allowed-consumer-projects",
                            "application-id",
                            "authorized-gae-applications",
                            "auto-upgrade-enabled",
                            "availability-type",
                            "available",
                            "available-maintenance-versions",
                            "backend-type",
                            "backup-configuration",
                            "backup-retention-settings",
                            "backup-tier",
                            "binary-log-enabled",
                            "bucket",
                            "ca-certificate",
                            "can-defer",
                            "can-reschedule",
                            "cascadable-replica",
                            "cert",
                            "cert-serial-number",
                            "client-certificate",
                            "client-key",
                            "collation",
                            "common-name",
                            "complexity",
                            "connect-retry-interval",
                            "connection-name",
                            "connection-pool-config",
                            "connection-pooling-enabled",
                            "connector-enforcement",
                            "crash-safe-replication-enabled",
                            "create-time",
                            "current-disk-size",
                            "custom-subject-alternative-names",
                            "data-api-access",
                            "data-cache-config",
                            "data-cache-enabled",
                            "data-disk-provisioned-iops",
                            "data-disk-provisioned-throughput",
                            "data-disk-size-gb",
                            "data-disk-type",
                            "database-installed-version",
                            "database-replication-enabled",
                            "database-version",
                            "day",
                            "deletion-protection-enabled",
                            "disable-scale-in",
                            "disallow-compromised-credentials",
                            "disallow-username-substring",
                            "disk-encryption-configuration",
                            "disk-encryption-status",
                            "dns-name",
                            "dns-servers",
                            "domain",
                            "dr-replica",
                            "dump-file-path",
                            "edition",
                            "enable-dataplex-integration",
                            "enable-google-ml-integration",
                            "enable-password-policy",
                            "enable-private-path-for-google-cloud-services",
                            "enabled",
                            "entitled",
                            "entraid-config",
                            "etag",
                            "expiration-time",
                            "failover-dr-replica-name",
                            "failover-replica",
                            "failover-target",
                            "final-backup-config",
                            "flag-recommender-enabled",
                            "follow-gae-application",
                            "gce-zone",
                            "gemini-config",
                            "google-vacuum-mgmt-enabled",
                            "host-port",
                            "hour",
                            "include-replicas-for-major-version-upgrade",
                            "index-advisor-enabled",
                            "insights-config",
                            "instance",
                            "instance-type",
                            "ip-configuration",
                            "ipv4-enabled",
                            "ipv6-address",
                            "kind",
                            "kms-key-name",
                            "kms-key-version-name",
                            "location",
                            "location-preference",
                            "maintenance-version",
                            "maintenance-window",
                            "master-heartbeat-period",
                            "master-instance-name",
                            "max-disk-size",
                            "max-node-count",
                            "min-length",
                            "min-node-count",
                            "mode",
                            "mysql-replica-configuration",
                            "name",
                            "network-attachment-uri",
                            "node-count",
                            "on-premises-configuration",
                            "oom-session-cancel-enabled",
                            "organizational-unit",
                            "out-of-disk-report",
                            "password",
                            "password-change-interval",
                            "password-validation-policy",
                            "performance-capture-config",
                            "point-in-time-recovery-enabled",
                            "pooler-count",
                            "pricing-plan",
                            "primary-dns-name",
                            "private-network",
                            "probe-threshold",
                            "probing-interval-seconds",
                            "project",
                            "psa-write-endpoint",
                            "psc-config",
                            "psc-enabled",
                            "psc-service-attachment-link",
                            "query-insights-enabled",
                            "query-plans-per-minute",
                            "query-string-length",
                            "read-pool-auto-scale-config",
                            "record-application-tags",
                            "record-client-address",
                            "region",
                            "replica-configuration",
                            "replica-names",
                            "replication-cluster",
                            "replication-lag-max-seconds",
                            "replication-log-archiving-enabled",
                            "replication-type",
                            "require-ssl",
                            "retain-backups-on-delete",
                            "retained-backups",
                            "retention-days",
                            "retention-interval",
                            "retention-unit",
                            "reuse-interval",
                            "root-password",
                            "running-threads-threshold",
                            "satisfies-pzi",
                            "satisfies-pzs",
                            "scale-in-cooldown-seconds",
                            "scale-out-cooldown-seconds",
                            "schedule-deadline-time",
                            "scheduled-maintenance",
                            "secondary-gce-zone",
                            "secondary-zone",
                            "seconds-behind-source-threshold",
                            "self-link",
                            "server-ca-cert",
                            "server-ca-mode",
                            "server-ca-pool",
                            "server-certificate-rotation-mode",
                            "service-account-email-address",
                            "settings",
                            "settings-version",
                            "sha1-fingerprint",
                            "source-instance",
                            "sql-min-recommended-increase-size-gb",
                            "sql-network-architecture",
                            "sql-out-of-disk-state",
                            "sql-server-audit-config",
                            "ssl-cipher",
                            "ssl-mode",
                            "ssl-option",
                            "start-time",
                            "state",
                            "storage-auto-resize",
                            "storage-auto-resize-limit",
                            "suspension-reason",
                            "switch-transaction-logs-to-cloud-storage-enabled",
                            "tags",
                            "tenant-id",
                            "threads-per-core",
                            "tier",
                            "time-zone",
                            "transaction-duration-threshold",
                            "transaction-log-retention-days",
                            "transactional-log-storage-state",
                            "update-track",
                            "upload-interval",
                            "user-labels",
                            "username",
                            "verify-server-certificate",
                            "write-endpoint",
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
        let mut request: api::DatabaseInstance = serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .instances()
            .insert(request, opt.value_of("project").unwrap_or(""));
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

    async fn _instances_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .instances()
            .list(opt.value_of("project").unwrap_or(""));
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
                "max-results" => {
                    call = call.max_results(
                        value
                            .map(|v| arg_from_str(v, err, "max-results", "uint32"))
                            .unwrap_or(0),
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
                                v.extend(
                                    ["filter", "max-results", "page-token"].iter().map(|v| *v),
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

    async fn _instances_list_server_cas(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().list_server_cas(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_patch(
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
                "available-maintenance-versions" => Some((
                    "availableMaintenanceVersions",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "backend-type" => Some((
                    "backendType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "connection-name" => Some((
                    "connectionName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "current-disk-size" => Some((
                    "currentDiskSize",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database-installed-version" => Some((
                    "databaseInstalledVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database-version" => Some((
                    "databaseVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kind" => Some((
                    "diskEncryptionConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kms-key-name" => Some((
                    "diskEncryptionConfiguration.kmsKeyName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kind" => Some((
                    "diskEncryptionStatus.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kms-key-version-name" => Some((
                    "diskEncryptionStatus.kmsKeyVersionName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "dns-name" => Some((
                    "dnsName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "failover-replica.available" => Some((
                    "failoverReplica.available",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "failover-replica.name" => Some((
                    "failoverReplica.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gce-zone" => Some((
                    "gceZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.active-query-enabled" => Some((
                    "geminiConfig.activeQueryEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.entitled" => Some((
                    "geminiConfig.entitled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.flag-recommender-enabled" => Some((
                    "geminiConfig.flagRecommenderEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.google-vacuum-mgmt-enabled" => Some((
                    "geminiConfig.googleVacuumMgmtEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.index-advisor-enabled" => Some((
                    "geminiConfig.indexAdvisorEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.oom-session-cancel-enabled" => Some((
                    "geminiConfig.oomSessionCancelEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "include-replicas-for-major-version-upgrade" => Some((
                    "includeReplicasForMajorVersionUpgrade",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance-type" => Some((
                    "instanceType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "ipv6-address" => Some((
                    "ipv6Address",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "maintenance-version" => Some((
                    "maintenanceVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "master-instance-name" => Some((
                    "masterInstanceName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "max-disk-size" => Some((
                    "maxDiskSize",
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
                "on-premises-configuration.ca-certificate" => Some((
                    "onPremisesConfiguration.caCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.client-certificate" => Some((
                    "onPremisesConfiguration.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.client-key" => Some((
                    "onPremisesConfiguration.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.dump-file-path" => Some((
                    "onPremisesConfiguration.dumpFilePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.host-port" => Some((
                    "onPremisesConfiguration.hostPort",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.kind" => Some((
                    "onPremisesConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.password" => Some((
                    "onPremisesConfiguration.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.name" => Some((
                    "onPremisesConfiguration.sourceInstance.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.project" => Some((
                    "onPremisesConfiguration.sourceInstance.project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.region" => Some((
                    "onPremisesConfiguration.sourceInstance.region",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.ssl-option" => Some((
                    "onPremisesConfiguration.sslOption",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.username" => Some((
                    "onPremisesConfiguration.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "out-of-disk-report.sql-min-recommended-increase-size-gb" => Some((
                    "outOfDiskReport.sqlMinRecommendedIncreaseSizeGb",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "out-of-disk-report.sql-out-of-disk-state" => Some((
                    "outOfDiskReport.sqlOutOfDiskState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "primary-dns-name" => Some((
                    "primaryDnsName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "psc-service-attachment-link" => Some((
                    "pscServiceAttachmentLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "region" => Some((
                    "region",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.cascadable-replica" => Some((
                    "replicaConfiguration.cascadableReplica",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.failover-target" => Some((
                    "replicaConfiguration.failoverTarget",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.kind" => Some((
                    "replicaConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.ca-certificate" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.caCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.client-certificate" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.client-key" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval",
                        JsonTypeInfo {
                            jtype: JsonType::Int,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-configuration.mysql-replica-configuration.dump-file-path" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.kind" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod",
                        JsonTypeInfo {
                            jtype: JsonType::String,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-configuration.mysql-replica-configuration.password" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.sslCipher",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.username" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-names" => Some((
                    "replicaNames",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "replication-cluster.dr-replica" => Some((
                    "replicationCluster.drReplica",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replication-cluster.failover-dr-replica-name" => Some((
                    "replicationCluster.failoverDrReplicaName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replication-cluster.psa-write-endpoint" => Some((
                    "replicationCluster.psaWriteEndpoint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "root-password" => Some((
                    "rootPassword",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "satisfies-pzi" => Some((
                    "satisfiesPzi",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "satisfies-pzs" => Some((
                    "satisfiesPzs",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.can-defer" => Some((
                    "scheduledMaintenance.canDefer",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.can-reschedule" => Some((
                    "scheduledMaintenance.canReschedule",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.schedule-deadline-time" => Some((
                    "scheduledMaintenance.scheduleDeadlineTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.start-time" => Some((
                    "scheduledMaintenance.startTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "secondary-gce-zone" => Some((
                    "secondaryGceZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "self-link" => Some((
                    "selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.cert" => Some((
                    "serverCaCert.cert",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.cert-serial-number" => Some((
                    "serverCaCert.certSerialNumber",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.common-name" => Some((
                    "serverCaCert.commonName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.create-time" => Some((
                    "serverCaCert.createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.expiration-time" => Some((
                    "serverCaCert.expirationTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.instance" => Some((
                    "serverCaCert.instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.kind" => Some((
                    "serverCaCert.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.self-link" => Some((
                    "serverCaCert.selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.sha1-fingerprint" => Some((
                    "serverCaCert.sha1Fingerprint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "service-account-email-address" => Some((
                    "serviceAccountEmailAddress",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.activation-policy" => Some((
                    "settings.activationPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.admin-credential-secret-name" => Some((
                    "settings.activeDirectoryConfig.adminCredentialSecretName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.dns-servers" => Some((
                    "settings.activeDirectoryConfig.dnsServers",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.active-directory-config.domain" => Some((
                    "settings.activeDirectoryConfig.domain",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.kind" => Some((
                    "settings.activeDirectoryConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.mode" => Some((
                    "settings.activeDirectoryConfig.mode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.organizational-unit" => Some((
                    "settings.activeDirectoryConfig.organizationalUnit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.advanced-machine-features.threads-per-core" => Some((
                    "settings.advancedMachineFeatures.threadsPerCore",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.authorized-gae-applications" => Some((
                    "settings.authorizedGaeApplications",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.auto-upgrade-enabled" => Some((
                    "settings.autoUpgradeEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.availability-type" => Some((
                    "settings.availabilityType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.backup-retention-settings.retained-backups" => {
                    Some((
                        "settings.backupConfiguration.backupRetentionSettings.retainedBackups",
                        JsonTypeInfo {
                            jtype: JsonType::Int,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "settings.backup-configuration.backup-retention-settings.retention-unit" => Some((
                    "settings.backupConfiguration.backupRetentionSettings.retentionUnit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.backup-tier" => Some((
                    "settings.backupConfiguration.backupTier",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.binary-log-enabled" => Some((
                    "settings.backupConfiguration.binaryLogEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.enabled" => Some((
                    "settings.backupConfiguration.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.kind" => Some((
                    "settings.backupConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.location" => Some((
                    "settings.backupConfiguration.location",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.point-in-time-recovery-enabled" => Some((
                    "settings.backupConfiguration.pointInTimeRecoveryEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.replication-log-archiving-enabled" => Some((
                    "settings.backupConfiguration.replicationLogArchivingEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.start-time" => Some((
                    "settings.backupConfiguration.startTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.transaction-log-retention-days" => Some((
                    "settings.backupConfiguration.transactionLogRetentionDays",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.transactional-log-storage-state" => Some((
                    "settings.backupConfiguration.transactionalLogStorageState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.collation" => Some((
                    "settings.collation",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connection-pool-config.connection-pooling-enabled" => Some((
                    "settings.connectionPoolConfig.connectionPoolingEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connection-pool-config.pooler-count" => Some((
                    "settings.connectionPoolConfig.poolerCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connector-enforcement" => Some((
                    "settings.connectorEnforcement",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.crash-safe-replication-enabled" => Some((
                    "settings.crashSafeReplicationEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-api-access" => Some((
                    "settings.dataApiAccess",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-cache-config.data-cache-enabled" => Some((
                    "settings.dataCacheConfig.dataCacheEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-provisioned-iops" => Some((
                    "settings.dataDiskProvisionedIops",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-provisioned-throughput" => Some((
                    "settings.dataDiskProvisionedThroughput",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-size-gb" => Some((
                    "settings.dataDiskSizeGb",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-type" => Some((
                    "settings.dataDiskType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.database-replication-enabled" => Some((
                    "settings.databaseReplicationEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.deletion-protection-enabled" => Some((
                    "settings.deletionProtectionEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.edition" => Some((
                    "settings.edition",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.enable-dataplex-integration" => Some((
                    "settings.enableDataplexIntegration",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.enable-google-ml-integration" => Some((
                    "settings.enableGoogleMlIntegration",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.application-id" => Some((
                    "settings.entraidConfig.applicationId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.kind" => Some((
                    "settings.entraidConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.tenant-id" => Some((
                    "settings.entraidConfig.tenantId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.final-backup-config.enabled" => Some((
                    "settings.finalBackupConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.final-backup-config.retention-days" => Some((
                    "settings.finalBackupConfig.retentionDays",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-insights-enabled" => Some((
                    "settings.insightsConfig.queryInsightsEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-plans-per-minute" => Some((
                    "settings.insightsConfig.queryPlansPerMinute",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-string-length" => Some((
                    "settings.insightsConfig.queryStringLength",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.record-application-tags" => Some((
                    "settings.insightsConfig.recordApplicationTags",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.record-client-address" => Some((
                    "settings.insightsConfig.recordClientAddress",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.allocated-ip-range" => Some((
                    "settings.ipConfiguration.allocatedIpRange",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.custom-subject-alternative-names" => Some((
                    "settings.ipConfiguration.customSubjectAlternativeNames",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.ip-configuration.enable-private-path-for-google-cloud-services" => {
                    Some((
                        "settings.ipConfiguration.enablePrivatePathForGoogleCloudServices",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "settings.ip-configuration.ipv4-enabled" => Some((
                    "settings.ipConfiguration.ipv4Enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.private-network" => Some((
                    "settings.ipConfiguration.privateNetwork",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.psc-config.allowed-consumer-projects" => Some((
                    "settings.ipConfiguration.pscConfig.allowedConsumerProjects",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.ip-configuration.psc-config.network-attachment-uri" => Some((
                    "settings.ipConfiguration.pscConfig.networkAttachmentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.psc-config.psc-enabled" => Some((
                    "settings.ipConfiguration.pscConfig.pscEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.require-ssl" => Some((
                    "settings.ipConfiguration.requireSsl",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-ca-mode" => Some((
                    "settings.ipConfiguration.serverCaMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-ca-pool" => Some((
                    "settings.ipConfiguration.serverCaPool",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-certificate-rotation-mode" => Some((
                    "settings.ipConfiguration.serverCertificateRotationMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.ssl-mode" => Some((
                    "settings.ipConfiguration.sslMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.kind" => Some((
                    "settings.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.follow-gae-application" => Some((
                    "settings.locationPreference.followGaeApplication",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.kind" => Some((
                    "settings.locationPreference.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.secondary-zone" => Some((
                    "settings.locationPreference.secondaryZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.zone" => Some((
                    "settings.locationPreference.zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.day" => Some((
                    "settings.maintenanceWindow.day",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.hour" => Some((
                    "settings.maintenanceWindow.hour",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.kind" => Some((
                    "settings.maintenanceWindow.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.update-track" => Some((
                    "settings.maintenanceWindow.updateTrack",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.complexity" => Some((
                    "settings.passwordValidationPolicy.complexity",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.disallow-compromised-credentials" => Some((
                    "settings.passwordValidationPolicy.disallowCompromisedCredentials",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.disallow-username-substring" => Some((
                    "settings.passwordValidationPolicy.disallowUsernameSubstring",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.enable-password-policy" => Some((
                    "settings.passwordValidationPolicy.enablePasswordPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.min-length" => Some((
                    "settings.passwordValidationPolicy.minLength",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.password-change-interval" => Some((
                    "settings.passwordValidationPolicy.passwordChangeInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.reuse-interval" => Some((
                    "settings.passwordValidationPolicy.reuseInterval",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.enabled" => Some((
                    "settings.performanceCaptureConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.probe-threshold" => Some((
                    "settings.performanceCaptureConfig.probeThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.probing-interval-seconds" => Some((
                    "settings.performanceCaptureConfig.probingIntervalSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.running-threads-threshold" => Some((
                    "settings.performanceCaptureConfig.runningThreadsThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.seconds-behind-source-threshold" => Some((
                    "settings.performanceCaptureConfig.secondsBehindSourceThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.transaction-duration-threshold" => Some((
                    "settings.performanceCaptureConfig.transactionDurationThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.pricing-plan" => Some((
                    "settings.pricingPlan",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.disable-scale-in" => Some((
                    "settings.readPoolAutoScaleConfig.disableScaleIn",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.enabled" => Some((
                    "settings.readPoolAutoScaleConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.max-node-count" => Some((
                    "settings.readPoolAutoScaleConfig.maxNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.min-node-count" => Some((
                    "settings.readPoolAutoScaleConfig.minNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.scale-in-cooldown-seconds" => Some((
                    "settings.readPoolAutoScaleConfig.scaleInCooldownSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.scale-out-cooldown-seconds" => Some((
                    "settings.readPoolAutoScaleConfig.scaleOutCooldownSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.replication-lag-max-seconds" => Some((
                    "settings.replicationLagMaxSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.replication-type" => Some((
                    "settings.replicationType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.retain-backups-on-delete" => Some((
                    "settings.retainBackupsOnDelete",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.settings-version" => Some((
                    "settings.settingsVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.bucket" => Some((
                    "settings.sqlServerAuditConfig.bucket",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.kind" => Some((
                    "settings.sqlServerAuditConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.retention-interval" => Some((
                    "settings.sqlServerAuditConfig.retentionInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.upload-interval" => Some((
                    "settings.sqlServerAuditConfig.uploadInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.storage-auto-resize" => Some((
                    "settings.storageAutoResize",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.storage-auto-resize-limit" => Some((
                    "settings.storageAutoResizeLimit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.tier" => Some((
                    "settings.tier",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.time-zone" => Some((
                    "settings.timeZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.user-labels" => Some((
                    "settings.userLabels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "sql-network-architecture" => Some((
                    "sqlNetworkArchitecture",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "state" => Some((
                    "state",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "suspension-reason" => Some((
                    "suspensionReason",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "switch-transaction-logs-to-cloud-storage-enabled" => Some((
                    "switchTransactionLogsToCloudStorageEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "tags" => Some((
                    "tags",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "write-endpoint" => Some((
                    "writeEndpoint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "activation-policy",
                            "active-directory-config",
                            "active-query-enabled",
                            "admin-credential-secret-name",
                            "advanced-machine-features",
                            "allocated-ip-range",
                            "allowed-consumer-projects",
                            "application-id",
                            "authorized-gae-applications",
                            "auto-upgrade-enabled",
                            "availability-type",
                            "available",
                            "available-maintenance-versions",
                            "backend-type",
                            "backup-configuration",
                            "backup-retention-settings",
                            "backup-tier",
                            "binary-log-enabled",
                            "bucket",
                            "ca-certificate",
                            "can-defer",
                            "can-reschedule",
                            "cascadable-replica",
                            "cert",
                            "cert-serial-number",
                            "client-certificate",
                            "client-key",
                            "collation",
                            "common-name",
                            "complexity",
                            "connect-retry-interval",
                            "connection-name",
                            "connection-pool-config",
                            "connection-pooling-enabled",
                            "connector-enforcement",
                            "crash-safe-replication-enabled",
                            "create-time",
                            "current-disk-size",
                            "custom-subject-alternative-names",
                            "data-api-access",
                            "data-cache-config",
                            "data-cache-enabled",
                            "data-disk-provisioned-iops",
                            "data-disk-provisioned-throughput",
                            "data-disk-size-gb",
                            "data-disk-type",
                            "database-installed-version",
                            "database-replication-enabled",
                            "database-version",
                            "day",
                            "deletion-protection-enabled",
                            "disable-scale-in",
                            "disallow-compromised-credentials",
                            "disallow-username-substring",
                            "disk-encryption-configuration",
                            "disk-encryption-status",
                            "dns-name",
                            "dns-servers",
                            "domain",
                            "dr-replica",
                            "dump-file-path",
                            "edition",
                            "enable-dataplex-integration",
                            "enable-google-ml-integration",
                            "enable-password-policy",
                            "enable-private-path-for-google-cloud-services",
                            "enabled",
                            "entitled",
                            "entraid-config",
                            "etag",
                            "expiration-time",
                            "failover-dr-replica-name",
                            "failover-replica",
                            "failover-target",
                            "final-backup-config",
                            "flag-recommender-enabled",
                            "follow-gae-application",
                            "gce-zone",
                            "gemini-config",
                            "google-vacuum-mgmt-enabled",
                            "host-port",
                            "hour",
                            "include-replicas-for-major-version-upgrade",
                            "index-advisor-enabled",
                            "insights-config",
                            "instance",
                            "instance-type",
                            "ip-configuration",
                            "ipv4-enabled",
                            "ipv6-address",
                            "kind",
                            "kms-key-name",
                            "kms-key-version-name",
                            "location",
                            "location-preference",
                            "maintenance-version",
                            "maintenance-window",
                            "master-heartbeat-period",
                            "master-instance-name",
                            "max-disk-size",
                            "max-node-count",
                            "min-length",
                            "min-node-count",
                            "mode",
                            "mysql-replica-configuration",
                            "name",
                            "network-attachment-uri",
                            "node-count",
                            "on-premises-configuration",
                            "oom-session-cancel-enabled",
                            "organizational-unit",
                            "out-of-disk-report",
                            "password",
                            "password-change-interval",
                            "password-validation-policy",
                            "performance-capture-config",
                            "point-in-time-recovery-enabled",
                            "pooler-count",
                            "pricing-plan",
                            "primary-dns-name",
                            "private-network",
                            "probe-threshold",
                            "probing-interval-seconds",
                            "project",
                            "psa-write-endpoint",
                            "psc-config",
                            "psc-enabled",
                            "psc-service-attachment-link",
                            "query-insights-enabled",
                            "query-plans-per-minute",
                            "query-string-length",
                            "read-pool-auto-scale-config",
                            "record-application-tags",
                            "record-client-address",
                            "region",
                            "replica-configuration",
                            "replica-names",
                            "replication-cluster",
                            "replication-lag-max-seconds",
                            "replication-log-archiving-enabled",
                            "replication-type",
                            "require-ssl",
                            "retain-backups-on-delete",
                            "retained-backups",
                            "retention-days",
                            "retention-interval",
                            "retention-unit",
                            "reuse-interval",
                            "root-password",
                            "running-threads-threshold",
                            "satisfies-pzi",
                            "satisfies-pzs",
                            "scale-in-cooldown-seconds",
                            "scale-out-cooldown-seconds",
                            "schedule-deadline-time",
                            "scheduled-maintenance",
                            "secondary-gce-zone",
                            "secondary-zone",
                            "seconds-behind-source-threshold",
                            "self-link",
                            "server-ca-cert",
                            "server-ca-mode",
                            "server-ca-pool",
                            "server-certificate-rotation-mode",
                            "service-account-email-address",
                            "settings",
                            "settings-version",
                            "sha1-fingerprint",
                            "source-instance",
                            "sql-min-recommended-increase-size-gb",
                            "sql-network-architecture",
                            "sql-out-of-disk-state",
                            "sql-server-audit-config",
                            "ssl-cipher",
                            "ssl-mode",
                            "ssl-option",
                            "start-time",
                            "state",
                            "storage-auto-resize",
                            "storage-auto-resize-limit",
                            "suspension-reason",
                            "switch-transaction-logs-to-cloud-storage-enabled",
                            "tags",
                            "tenant-id",
                            "threads-per-core",
                            "tier",
                            "time-zone",
                            "transaction-duration-threshold",
                            "transaction-log-retention-days",
                            "transactional-log-storage-state",
                            "update-track",
                            "upload-interval",
                            "user-labels",
                            "username",
                            "verify-server-certificate",
                            "write-endpoint",
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
        let mut request: api::DatabaseInstance = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().patch(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_point_in_time_restore(
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
                "allocated-ip-range" => Some((
                    "allocatedIpRange",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "datasource" => Some((
                    "datasource",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "point-in-time" => Some((
                    "pointInTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "preferred-secondary-zone" => Some((
                    "preferredSecondaryZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "preferred-zone" => Some((
                    "preferredZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "private-network" => Some((
                    "privateNetwork",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "target-instance" => Some((
                    "targetInstance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "allocated-ip-range",
                            "datasource",
                            "point-in-time",
                            "preferred-secondary-zone",
                            "preferred-zone",
                            "private-network",
                            "target-instance",
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
        let mut request: api::PointInTimeRestoreContext =
            serde_json::value::from_value(object).unwrap();
        let mut call = self
            .hub
            .instances()
            .point_in_time_restore(request, opt.value_of("parent").unwrap_or(""));
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

    async fn _instances_pre_check_major_version_upgrade(
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
                "pre-check-major-version-upgrade-context.kind" => Some((
                    "preCheckMajorVersionUpgradeContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "pre-check-major-version-upgrade-context.target-database-version" => Some((
                    "preCheckMajorVersionUpgradeContext.targetDatabaseVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "kind",
                            "pre-check-major-version-upgrade-context",
                            "target-database-version",
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
        let mut request: api::InstancesPreCheckMajorVersionUpgradeRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().pre_check_major_version_upgrade(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_promote_replica(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().promote_replica(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "failover" => {
                    call = call.failover(
                        value
                            .map(|v| arg_from_str(v, err, "failover", "boolean"))
                            .unwrap_or(false),
                    );
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
                                v.extend(["failover"].iter().map(|v| *v));
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

    async fn _instances_reencrypt(
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
                "backup-reencryption-config.backup-limit" => Some((
                    "backupReencryptionConfig.backupLimit",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "backup-reencryption-config.backup-type" => Some((
                    "backupReencryptionConfig.backupType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["backup-limit", "backup-reencryption-config", "backup-type"],
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
        let mut request: api::InstancesReencryptRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().reencrypt(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_release_ssrs_lease(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().release_ssrs_lease(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_reset_ssl_config(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().reset_ssl_config(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "mode" => {
                    call = call.mode(value.unwrap_or(""));
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
                                v.extend(["mode"].iter().map(|v| *v));
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

    async fn _instances_restart(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().restart(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_restore_backup(
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
                    "backup" => Some(("backup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "backupdr-backup" => Some(("backupdrBackup", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-backup-context.backup-run-id" => Some(("restoreBackupContext.backupRunId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-backup-context.instance-id" => Some(("restoreBackupContext.instanceId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-backup-context.kind" => Some(("restoreBackupContext.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-backup-context.project" => Some(("restoreBackupContext.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-clear-overrides-field-names" => Some(("restoreInstanceClearOverridesFieldNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.available-maintenance-versions" => Some(("restoreInstanceSettings.availableMaintenanceVersions", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.backend-type" => Some(("restoreInstanceSettings.backendType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.connection-name" => Some(("restoreInstanceSettings.connectionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.create-time" => Some(("restoreInstanceSettings.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.current-disk-size" => Some(("restoreInstanceSettings.currentDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.database-installed-version" => Some(("restoreInstanceSettings.databaseInstalledVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.database-version" => Some(("restoreInstanceSettings.databaseVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.disk-encryption-configuration.kind" => Some(("restoreInstanceSettings.diskEncryptionConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.disk-encryption-configuration.kms-key-name" => Some(("restoreInstanceSettings.diskEncryptionConfiguration.kmsKeyName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.disk-encryption-status.kind" => Some(("restoreInstanceSettings.diskEncryptionStatus.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.disk-encryption-status.kms-key-version-name" => Some(("restoreInstanceSettings.diskEncryptionStatus.kmsKeyVersionName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.dns-name" => Some(("restoreInstanceSettings.dnsName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.etag" => Some(("restoreInstanceSettings.etag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.failover-replica.available" => Some(("restoreInstanceSettings.failoverReplica.available", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.failover-replica.name" => Some(("restoreInstanceSettings.failoverReplica.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.gce-zone" => Some(("restoreInstanceSettings.gceZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.gemini-config.active-query-enabled" => Some(("restoreInstanceSettings.geminiConfig.activeQueryEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.gemini-config.entitled" => Some(("restoreInstanceSettings.geminiConfig.entitled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.gemini-config.flag-recommender-enabled" => Some(("restoreInstanceSettings.geminiConfig.flagRecommenderEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.gemini-config.google-vacuum-mgmt-enabled" => Some(("restoreInstanceSettings.geminiConfig.googleVacuumMgmtEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.gemini-config.index-advisor-enabled" => Some(("restoreInstanceSettings.geminiConfig.indexAdvisorEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.gemini-config.oom-session-cancel-enabled" => Some(("restoreInstanceSettings.geminiConfig.oomSessionCancelEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.include-replicas-for-major-version-upgrade" => Some(("restoreInstanceSettings.includeReplicasForMajorVersionUpgrade", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.instance-type" => Some(("restoreInstanceSettings.instanceType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.ipv6-address" => Some(("restoreInstanceSettings.ipv6Address", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.kind" => Some(("restoreInstanceSettings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.maintenance-version" => Some(("restoreInstanceSettings.maintenanceVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.master-instance-name" => Some(("restoreInstanceSettings.masterInstanceName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.max-disk-size" => Some(("restoreInstanceSettings.maxDiskSize", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.name" => Some(("restoreInstanceSettings.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.node-count" => Some(("restoreInstanceSettings.nodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.ca-certificate" => Some(("restoreInstanceSettings.onPremisesConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.client-certificate" => Some(("restoreInstanceSettings.onPremisesConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.client-key" => Some(("restoreInstanceSettings.onPremisesConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.dump-file-path" => Some(("restoreInstanceSettings.onPremisesConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.host-port" => Some(("restoreInstanceSettings.onPremisesConfiguration.hostPort", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.kind" => Some(("restoreInstanceSettings.onPremisesConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.password" => Some(("restoreInstanceSettings.onPremisesConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.source-instance.name" => Some(("restoreInstanceSettings.onPremisesConfiguration.sourceInstance.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.source-instance.project" => Some(("restoreInstanceSettings.onPremisesConfiguration.sourceInstance.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.source-instance.region" => Some(("restoreInstanceSettings.onPremisesConfiguration.sourceInstance.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.ssl-option" => Some(("restoreInstanceSettings.onPremisesConfiguration.sslOption", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.on-premises-configuration.username" => Some(("restoreInstanceSettings.onPremisesConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.out-of-disk-report.sql-min-recommended-increase-size-gb" => Some(("restoreInstanceSettings.outOfDiskReport.sqlMinRecommendedIncreaseSizeGb", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.out-of-disk-report.sql-out-of-disk-state" => Some(("restoreInstanceSettings.outOfDiskReport.sqlOutOfDiskState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.primary-dns-name" => Some(("restoreInstanceSettings.primaryDnsName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.project" => Some(("restoreInstanceSettings.project", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.psc-service-attachment-link" => Some(("restoreInstanceSettings.pscServiceAttachmentLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.region" => Some(("restoreInstanceSettings.region", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.cascadable-replica" => Some(("restoreInstanceSettings.replicaConfiguration.cascadableReplica", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.failover-target" => Some(("restoreInstanceSettings.replicaConfiguration.failoverTarget", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.kind" => Some(("restoreInstanceSettings.replicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.ca-certificate" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.caCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.client-certificate" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.clientCertificate", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.client-key" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.clientKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.connect-retry-interval" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.dump-file-path" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.kind" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.master-heartbeat-period" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.password" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.password", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.ssl-cipher" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.sslCipher", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.username" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.username", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-configuration.mysql-replica-configuration.verify-server-certificate" => Some(("restoreInstanceSettings.replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replica-names" => Some(("restoreInstanceSettings.replicaNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.replication-cluster.dr-replica" => Some(("restoreInstanceSettings.replicationCluster.drReplica", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replication-cluster.failover-dr-replica-name" => Some(("restoreInstanceSettings.replicationCluster.failoverDrReplicaName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.replication-cluster.psa-write-endpoint" => Some(("restoreInstanceSettings.replicationCluster.psaWriteEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.root-password" => Some(("restoreInstanceSettings.rootPassword", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.satisfies-pzi" => Some(("restoreInstanceSettings.satisfiesPzi", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.satisfies-pzs" => Some(("restoreInstanceSettings.satisfiesPzs", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.scheduled-maintenance.can-defer" => Some(("restoreInstanceSettings.scheduledMaintenance.canDefer", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.scheduled-maintenance.can-reschedule" => Some(("restoreInstanceSettings.scheduledMaintenance.canReschedule", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.scheduled-maintenance.schedule-deadline-time" => Some(("restoreInstanceSettings.scheduledMaintenance.scheduleDeadlineTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.scheduled-maintenance.start-time" => Some(("restoreInstanceSettings.scheduledMaintenance.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.secondary-gce-zone" => Some(("restoreInstanceSettings.secondaryGceZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.self-link" => Some(("restoreInstanceSettings.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.cert" => Some(("restoreInstanceSettings.serverCaCert.cert", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.cert-serial-number" => Some(("restoreInstanceSettings.serverCaCert.certSerialNumber", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.common-name" => Some(("restoreInstanceSettings.serverCaCert.commonName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.create-time" => Some(("restoreInstanceSettings.serverCaCert.createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.expiration-time" => Some(("restoreInstanceSettings.serverCaCert.expirationTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.instance" => Some(("restoreInstanceSettings.serverCaCert.instance", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.kind" => Some(("restoreInstanceSettings.serverCaCert.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.self-link" => Some(("restoreInstanceSettings.serverCaCert.selfLink", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.server-ca-cert.sha1-fingerprint" => Some(("restoreInstanceSettings.serverCaCert.sha1Fingerprint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.service-account-email-address" => Some(("restoreInstanceSettings.serviceAccountEmailAddress", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.activation-policy" => Some(("restoreInstanceSettings.settings.activationPolicy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.active-directory-config.admin-credential-secret-name" => Some(("restoreInstanceSettings.settings.activeDirectoryConfig.adminCredentialSecretName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.active-directory-config.dns-servers" => Some(("restoreInstanceSettings.settings.activeDirectoryConfig.dnsServers", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.settings.active-directory-config.domain" => Some(("restoreInstanceSettings.settings.activeDirectoryConfig.domain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.active-directory-config.kind" => Some(("restoreInstanceSettings.settings.activeDirectoryConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.active-directory-config.mode" => Some(("restoreInstanceSettings.settings.activeDirectoryConfig.mode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.active-directory-config.organizational-unit" => Some(("restoreInstanceSettings.settings.activeDirectoryConfig.organizationalUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.advanced-machine-features.threads-per-core" => Some(("restoreInstanceSettings.settings.advancedMachineFeatures.threadsPerCore", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.authorized-gae-applications" => Some(("restoreInstanceSettings.settings.authorizedGaeApplications", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.settings.auto-upgrade-enabled" => Some(("restoreInstanceSettings.settings.autoUpgradeEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.availability-type" => Some(("restoreInstanceSettings.settings.availabilityType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.backup-retention-settings.retained-backups" => Some(("restoreInstanceSettings.settings.backupConfiguration.backupRetentionSettings.retainedBackups", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.backup-retention-settings.retention-unit" => Some(("restoreInstanceSettings.settings.backupConfiguration.backupRetentionSettings.retentionUnit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.backup-tier" => Some(("restoreInstanceSettings.settings.backupConfiguration.backupTier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.binary-log-enabled" => Some(("restoreInstanceSettings.settings.backupConfiguration.binaryLogEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.enabled" => Some(("restoreInstanceSettings.settings.backupConfiguration.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.kind" => Some(("restoreInstanceSettings.settings.backupConfiguration.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.location" => Some(("restoreInstanceSettings.settings.backupConfiguration.location", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.point-in-time-recovery-enabled" => Some(("restoreInstanceSettings.settings.backupConfiguration.pointInTimeRecoveryEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.replication-log-archiving-enabled" => Some(("restoreInstanceSettings.settings.backupConfiguration.replicationLogArchivingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.start-time" => Some(("restoreInstanceSettings.settings.backupConfiguration.startTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.transaction-log-retention-days" => Some(("restoreInstanceSettings.settings.backupConfiguration.transactionLogRetentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.backup-configuration.transactional-log-storage-state" => Some(("restoreInstanceSettings.settings.backupConfiguration.transactionalLogStorageState", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.collation" => Some(("restoreInstanceSettings.settings.collation", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.connection-pool-config.connection-pooling-enabled" => Some(("restoreInstanceSettings.settings.connectionPoolConfig.connectionPoolingEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.connection-pool-config.pooler-count" => Some(("restoreInstanceSettings.settings.connectionPoolConfig.poolerCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.connector-enforcement" => Some(("restoreInstanceSettings.settings.connectorEnforcement", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.crash-safe-replication-enabled" => Some(("restoreInstanceSettings.settings.crashSafeReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.data-api-access" => Some(("restoreInstanceSettings.settings.dataApiAccess", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.data-cache-config.data-cache-enabled" => Some(("restoreInstanceSettings.settings.dataCacheConfig.dataCacheEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.data-disk-provisioned-iops" => Some(("restoreInstanceSettings.settings.dataDiskProvisionedIops", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.data-disk-provisioned-throughput" => Some(("restoreInstanceSettings.settings.dataDiskProvisionedThroughput", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.data-disk-size-gb" => Some(("restoreInstanceSettings.settings.dataDiskSizeGb", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.data-disk-type" => Some(("restoreInstanceSettings.settings.dataDiskType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.database-replication-enabled" => Some(("restoreInstanceSettings.settings.databaseReplicationEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.deletion-protection-enabled" => Some(("restoreInstanceSettings.settings.deletionProtectionEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.edition" => Some(("restoreInstanceSettings.settings.edition", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.enable-dataplex-integration" => Some(("restoreInstanceSettings.settings.enableDataplexIntegration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.enable-google-ml-integration" => Some(("restoreInstanceSettings.settings.enableGoogleMlIntegration", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.entraid-config.application-id" => Some(("restoreInstanceSettings.settings.entraidConfig.applicationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.entraid-config.kind" => Some(("restoreInstanceSettings.settings.entraidConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.entraid-config.tenant-id" => Some(("restoreInstanceSettings.settings.entraidConfig.tenantId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.final-backup-config.enabled" => Some(("restoreInstanceSettings.settings.finalBackupConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.final-backup-config.retention-days" => Some(("restoreInstanceSettings.settings.finalBackupConfig.retentionDays", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.insights-config.query-insights-enabled" => Some(("restoreInstanceSettings.settings.insightsConfig.queryInsightsEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.insights-config.query-plans-per-minute" => Some(("restoreInstanceSettings.settings.insightsConfig.queryPlansPerMinute", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.insights-config.query-string-length" => Some(("restoreInstanceSettings.settings.insightsConfig.queryStringLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.insights-config.record-application-tags" => Some(("restoreInstanceSettings.settings.insightsConfig.recordApplicationTags", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.insights-config.record-client-address" => Some(("restoreInstanceSettings.settings.insightsConfig.recordClientAddress", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.allocated-ip-range" => Some(("restoreInstanceSettings.settings.ipConfiguration.allocatedIpRange", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.custom-subject-alternative-names" => Some(("restoreInstanceSettings.settings.ipConfiguration.customSubjectAlternativeNames", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.settings.ip-configuration.enable-private-path-for-google-cloud-services" => Some(("restoreInstanceSettings.settings.ipConfiguration.enablePrivatePathForGoogleCloudServices", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.ipv4-enabled" => Some(("restoreInstanceSettings.settings.ipConfiguration.ipv4Enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.private-network" => Some(("restoreInstanceSettings.settings.ipConfiguration.privateNetwork", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.psc-config.allowed-consumer-projects" => Some(("restoreInstanceSettings.settings.ipConfiguration.pscConfig.allowedConsumerProjects", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.settings.ip-configuration.psc-config.network-attachment-uri" => Some(("restoreInstanceSettings.settings.ipConfiguration.pscConfig.networkAttachmentUri", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.psc-config.psc-enabled" => Some(("restoreInstanceSettings.settings.ipConfiguration.pscConfig.pscEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.require-ssl" => Some(("restoreInstanceSettings.settings.ipConfiguration.requireSsl", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.server-ca-mode" => Some(("restoreInstanceSettings.settings.ipConfiguration.serverCaMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.server-ca-pool" => Some(("restoreInstanceSettings.settings.ipConfiguration.serverCaPool", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.server-certificate-rotation-mode" => Some(("restoreInstanceSettings.settings.ipConfiguration.serverCertificateRotationMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.ip-configuration.ssl-mode" => Some(("restoreInstanceSettings.settings.ipConfiguration.sslMode", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.kind" => Some(("restoreInstanceSettings.settings.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.location-preference.follow-gae-application" => Some(("restoreInstanceSettings.settings.locationPreference.followGaeApplication", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.location-preference.kind" => Some(("restoreInstanceSettings.settings.locationPreference.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.location-preference.secondary-zone" => Some(("restoreInstanceSettings.settings.locationPreference.secondaryZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.location-preference.zone" => Some(("restoreInstanceSettings.settings.locationPreference.zone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.maintenance-window.day" => Some(("restoreInstanceSettings.settings.maintenanceWindow.day", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.maintenance-window.hour" => Some(("restoreInstanceSettings.settings.maintenanceWindow.hour", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.maintenance-window.kind" => Some(("restoreInstanceSettings.settings.maintenanceWindow.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.maintenance-window.update-track" => Some(("restoreInstanceSettings.settings.maintenanceWindow.updateTrack", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.password-validation-policy.complexity" => Some(("restoreInstanceSettings.settings.passwordValidationPolicy.complexity", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.password-validation-policy.disallow-compromised-credentials" => Some(("restoreInstanceSettings.settings.passwordValidationPolicy.disallowCompromisedCredentials", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.password-validation-policy.disallow-username-substring" => Some(("restoreInstanceSettings.settings.passwordValidationPolicy.disallowUsernameSubstring", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.password-validation-policy.enable-password-policy" => Some(("restoreInstanceSettings.settings.passwordValidationPolicy.enablePasswordPolicy", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.password-validation-policy.min-length" => Some(("restoreInstanceSettings.settings.passwordValidationPolicy.minLength", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.password-validation-policy.password-change-interval" => Some(("restoreInstanceSettings.settings.passwordValidationPolicy.passwordChangeInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.password-validation-policy.reuse-interval" => Some(("restoreInstanceSettings.settings.passwordValidationPolicy.reuseInterval", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.performance-capture-config.enabled" => Some(("restoreInstanceSettings.settings.performanceCaptureConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.performance-capture-config.probe-threshold" => Some(("restoreInstanceSettings.settings.performanceCaptureConfig.probeThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.performance-capture-config.probing-interval-seconds" => Some(("restoreInstanceSettings.settings.performanceCaptureConfig.probingIntervalSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.performance-capture-config.running-threads-threshold" => Some(("restoreInstanceSettings.settings.performanceCaptureConfig.runningThreadsThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.performance-capture-config.seconds-behind-source-threshold" => Some(("restoreInstanceSettings.settings.performanceCaptureConfig.secondsBehindSourceThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.performance-capture-config.transaction-duration-threshold" => Some(("restoreInstanceSettings.settings.performanceCaptureConfig.transactionDurationThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.pricing-plan" => Some(("restoreInstanceSettings.settings.pricingPlan", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.read-pool-auto-scale-config.disable-scale-in" => Some(("restoreInstanceSettings.settings.readPoolAutoScaleConfig.disableScaleIn", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.read-pool-auto-scale-config.enabled" => Some(("restoreInstanceSettings.settings.readPoolAutoScaleConfig.enabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.read-pool-auto-scale-config.max-node-count" => Some(("restoreInstanceSettings.settings.readPoolAutoScaleConfig.maxNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.read-pool-auto-scale-config.min-node-count" => Some(("restoreInstanceSettings.settings.readPoolAutoScaleConfig.minNodeCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.read-pool-auto-scale-config.scale-in-cooldown-seconds" => Some(("restoreInstanceSettings.settings.readPoolAutoScaleConfig.scaleInCooldownSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.read-pool-auto-scale-config.scale-out-cooldown-seconds" => Some(("restoreInstanceSettings.settings.readPoolAutoScaleConfig.scaleOutCooldownSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.replication-lag-max-seconds" => Some(("restoreInstanceSettings.settings.replicationLagMaxSeconds", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.replication-type" => Some(("restoreInstanceSettings.settings.replicationType", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.retain-backups-on-delete" => Some(("restoreInstanceSettings.settings.retainBackupsOnDelete", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.settings-version" => Some(("restoreInstanceSettings.settings.settingsVersion", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.sql-server-audit-config.bucket" => Some(("restoreInstanceSettings.settings.sqlServerAuditConfig.bucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.sql-server-audit-config.kind" => Some(("restoreInstanceSettings.settings.sqlServerAuditConfig.kind", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.sql-server-audit-config.retention-interval" => Some(("restoreInstanceSettings.settings.sqlServerAuditConfig.retentionInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.sql-server-audit-config.upload-interval" => Some(("restoreInstanceSettings.settings.sqlServerAuditConfig.uploadInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.storage-auto-resize" => Some(("restoreInstanceSettings.settings.storageAutoResize", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.storage-auto-resize-limit" => Some(("restoreInstanceSettings.settings.storageAutoResizeLimit", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.tier" => Some(("restoreInstanceSettings.settings.tier", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.time-zone" => Some(("restoreInstanceSettings.settings.timeZone", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.settings.user-labels" => Some(("restoreInstanceSettings.settings.userLabels", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "restore-instance-settings.sql-network-architecture" => Some(("restoreInstanceSettings.sqlNetworkArchitecture", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.state" => Some(("restoreInstanceSettings.state", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "restore-instance-settings.suspension-reason" => Some(("restoreInstanceSettings.suspensionReason", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "restore-instance-settings.switch-transaction-logs-to-cloud-storage-enabled" => Some(("restoreInstanceSettings.switchTransactionLogsToCloudStorageEnabled", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "restore-instance-settings.tags" => Some(("restoreInstanceSettings.tags", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "restore-instance-settings.write-endpoint" => Some(("restoreInstanceSettings.writeEndpoint", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["activation-policy", "active-directory-config", "active-query-enabled", "admin-credential-secret-name", "advanced-machine-features", "allocated-ip-range", "allowed-consumer-projects", "application-id", "authorized-gae-applications", "auto-upgrade-enabled", "availability-type", "available", "available-maintenance-versions", "backend-type", "backup", "backup-configuration", "backup-retention-settings", "backup-run-id", "backup-tier", "backupdr-backup", "binary-log-enabled", "bucket", "ca-certificate", "can-defer", "can-reschedule", "cascadable-replica", "cert", "cert-serial-number", "client-certificate", "client-key", "collation", "common-name", "complexity", "connect-retry-interval", "connection-name", "connection-pool-config", "connection-pooling-enabled", "connector-enforcement", "crash-safe-replication-enabled", "create-time", "current-disk-size", "custom-subject-alternative-names", "data-api-access", "data-cache-config", "data-cache-enabled", "data-disk-provisioned-iops", "data-disk-provisioned-throughput", "data-disk-size-gb", "data-disk-type", "database-installed-version", "database-replication-enabled", "database-version", "day", "deletion-protection-enabled", "disable-scale-in", "disallow-compromised-credentials", "disallow-username-substring", "disk-encryption-configuration", "disk-encryption-status", "dns-name", "dns-servers", "domain", "dr-replica", "dump-file-path", "edition", "enable-dataplex-integration", "enable-google-ml-integration", "enable-password-policy", "enable-private-path-for-google-cloud-services", "enabled", "entitled", "entraid-config", "etag", "expiration-time", "failover-dr-replica-name", "failover-replica", "failover-target", "final-backup-config", "flag-recommender-enabled", "follow-gae-application", "gce-zone", "gemini-config", "google-vacuum-mgmt-enabled", "host-port", "hour", "include-replicas-for-major-version-upgrade", "index-advisor-enabled", "insights-config", "instance", "instance-id", "instance-type", "ip-configuration", "ipv4-enabled", "ipv6-address", "kind", "kms-key-name", "kms-key-version-name", "location", "location-preference", "maintenance-version", "maintenance-window", "master-heartbeat-period", "master-instance-name", "max-disk-size", "max-node-count", "min-length", "min-node-count", "mode", "mysql-replica-configuration", "name", "network-attachment-uri", "node-count", "on-premises-configuration", "oom-session-cancel-enabled", "organizational-unit", "out-of-disk-report", "password", "password-change-interval", "password-validation-policy", "performance-capture-config", "point-in-time-recovery-enabled", "pooler-count", "pricing-plan", "primary-dns-name", "private-network", "probe-threshold", "probing-interval-seconds", "project", "psa-write-endpoint", "psc-config", "psc-enabled", "psc-service-attachment-link", "query-insights-enabled", "query-plans-per-minute", "query-string-length", "read-pool-auto-scale-config", "record-application-tags", "record-client-address", "region", "replica-configuration", "replica-names", "replication-cluster", "replication-lag-max-seconds", "replication-log-archiving-enabled", "replication-type", "require-ssl", "restore-backup-context", "restore-instance-clear-overrides-field-names", "restore-instance-settings", "retain-backups-on-delete", "retained-backups", "retention-days", "retention-interval", "retention-unit", "reuse-interval", "root-password", "running-threads-threshold", "satisfies-pzi", "satisfies-pzs", "scale-in-cooldown-seconds", "scale-out-cooldown-seconds", "schedule-deadline-time", "scheduled-maintenance", "secondary-gce-zone", "secondary-zone", "seconds-behind-source-threshold", "self-link", "server-ca-cert", "server-ca-mode", "server-ca-pool", "server-certificate-rotation-mode", "service-account-email-address", "settings", "settings-version", "sha1-fingerprint", "source-instance", "sql-min-recommended-increase-size-gb", "sql-network-architecture", "sql-out-of-disk-state", "sql-server-audit-config", "ssl-cipher", "ssl-mode", "ssl-option", "start-time", "state", "storage-auto-resize", "storage-auto-resize-limit", "suspension-reason", "switch-transaction-logs-to-cloud-storage-enabled", "tags", "tenant-id", "threads-per-core", "tier", "time-zone", "transaction-duration-threshold", "transaction-log-retention-days", "transactional-log-storage-state", "update-track", "upload-interval", "user-labels", "username", "verify-server-certificate", "write-endpoint", "zone"]);
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
        let mut request: api::InstancesRestoreBackupRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().restore_backup(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_rotate_server_ca(
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
                "rotate-server-ca-context.kind" => Some((
                    "rotateServerCaContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "rotate-server-ca-context.next-version" => Some((
                    "rotateServerCaContext.nextVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["kind", "next-version", "rotate-server-ca-context"],
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
        let mut request: api::InstancesRotateServerCaRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().rotate_server_ca(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_start_replica(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().start_replica(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_stop_replica(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().stop_replica(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_switchover(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.instances().switchover(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "db-timeout" => {
                    call = call.db_timeout(
                        value
                            .map(|v| arg_from_str(v, err, "db-timeout", "google-duration"))
                            .unwrap_or(chrono::Duration::seconds(0)),
                    );
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
                                v.extend(["db-timeout"].iter().map(|v| *v));
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

    async fn _instances_truncate_log(
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
                "truncate-log-context.kind" => Some((
                    "truncateLogContext.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "truncate-log-context.log-type" => Some((
                    "truncateLogContext.logType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["kind", "log-type", "truncate-log-context"],
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
        let mut request: api::InstancesTruncateLogRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().truncate_log(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _instances_update(
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
                "available-maintenance-versions" => Some((
                    "availableMaintenanceVersions",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "backend-type" => Some((
                    "backendType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "connection-name" => Some((
                    "connectionName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "create-time" => Some((
                    "createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "current-disk-size" => Some((
                    "currentDiskSize",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database-installed-version" => Some((
                    "databaseInstalledVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "database-version" => Some((
                    "databaseVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kind" => Some((
                    "diskEncryptionConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-configuration.kms-key-name" => Some((
                    "diskEncryptionConfiguration.kmsKeyName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kind" => Some((
                    "diskEncryptionStatus.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "disk-encryption-status.kms-key-version-name" => Some((
                    "diskEncryptionStatus.kmsKeyVersionName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "dns-name" => Some((
                    "dnsName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "failover-replica.available" => Some((
                    "failoverReplica.available",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "failover-replica.name" => Some((
                    "failoverReplica.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gce-zone" => Some((
                    "gceZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.active-query-enabled" => Some((
                    "geminiConfig.activeQueryEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.entitled" => Some((
                    "geminiConfig.entitled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.flag-recommender-enabled" => Some((
                    "geminiConfig.flagRecommenderEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.google-vacuum-mgmt-enabled" => Some((
                    "geminiConfig.googleVacuumMgmtEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.index-advisor-enabled" => Some((
                    "geminiConfig.indexAdvisorEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "gemini-config.oom-session-cancel-enabled" => Some((
                    "geminiConfig.oomSessionCancelEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "include-replicas-for-major-version-upgrade" => Some((
                    "includeReplicasForMajorVersionUpgrade",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance-type" => Some((
                    "instanceType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "ipv6-address" => Some((
                    "ipv6Address",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "maintenance-version" => Some((
                    "maintenanceVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "master-instance-name" => Some((
                    "masterInstanceName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "max-disk-size" => Some((
                    "maxDiskSize",
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
                "on-premises-configuration.ca-certificate" => Some((
                    "onPremisesConfiguration.caCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.client-certificate" => Some((
                    "onPremisesConfiguration.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.client-key" => Some((
                    "onPremisesConfiguration.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.dump-file-path" => Some((
                    "onPremisesConfiguration.dumpFilePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.host-port" => Some((
                    "onPremisesConfiguration.hostPort",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.kind" => Some((
                    "onPremisesConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.password" => Some((
                    "onPremisesConfiguration.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.name" => Some((
                    "onPremisesConfiguration.sourceInstance.name",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.project" => Some((
                    "onPremisesConfiguration.sourceInstance.project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.source-instance.region" => Some((
                    "onPremisesConfiguration.sourceInstance.region",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.ssl-option" => Some((
                    "onPremisesConfiguration.sslOption",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "on-premises-configuration.username" => Some((
                    "onPremisesConfiguration.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "out-of-disk-report.sql-min-recommended-increase-size-gb" => Some((
                    "outOfDiskReport.sqlMinRecommendedIncreaseSizeGb",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "out-of-disk-report.sql-out-of-disk-state" => Some((
                    "outOfDiskReport.sqlOutOfDiskState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "primary-dns-name" => Some((
                    "primaryDnsName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "psc-service-attachment-link" => Some((
                    "pscServiceAttachmentLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "region" => Some((
                    "region",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.cascadable-replica" => Some((
                    "replicaConfiguration.cascadableReplica",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.failover-target" => Some((
                    "replicaConfiguration.failoverTarget",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.kind" => Some((
                    "replicaConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.ca-certificate" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.caCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.client-certificate" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.clientCertificate",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.client-key" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.clientKey",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.connect-retry-interval" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.connectRetryInterval",
                        JsonTypeInfo {
                            jtype: JsonType::Int,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-configuration.mysql-replica-configuration.dump-file-path" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.dumpFilePath",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.kind" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.master-heartbeat-period" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.masterHeartbeatPeriod",
                        JsonTypeInfo {
                            jtype: JsonType::String,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-configuration.mysql-replica-configuration.password" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.ssl-cipher" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.sslCipher",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.username" => Some((
                    "replicaConfiguration.mysqlReplicaConfiguration.username",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-configuration.mysql-replica-configuration.verify-server-certificate" => {
                    Some((
                        "replicaConfiguration.mysqlReplicaConfiguration.verifyServerCertificate",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "replica-names" => Some((
                    "replicaNames",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "replication-cluster.dr-replica" => Some((
                    "replicationCluster.drReplica",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replication-cluster.failover-dr-replica-name" => Some((
                    "replicationCluster.failoverDrReplicaName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replication-cluster.psa-write-endpoint" => Some((
                    "replicationCluster.psaWriteEndpoint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "root-password" => Some((
                    "rootPassword",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "satisfies-pzi" => Some((
                    "satisfiesPzi",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "satisfies-pzs" => Some((
                    "satisfiesPzs",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.can-defer" => Some((
                    "scheduledMaintenance.canDefer",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.can-reschedule" => Some((
                    "scheduledMaintenance.canReschedule",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.schedule-deadline-time" => Some((
                    "scheduledMaintenance.scheduleDeadlineTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "scheduled-maintenance.start-time" => Some((
                    "scheduledMaintenance.startTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "secondary-gce-zone" => Some((
                    "secondaryGceZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "self-link" => Some((
                    "selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.cert" => Some((
                    "serverCaCert.cert",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.cert-serial-number" => Some((
                    "serverCaCert.certSerialNumber",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.common-name" => Some((
                    "serverCaCert.commonName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.create-time" => Some((
                    "serverCaCert.createTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.expiration-time" => Some((
                    "serverCaCert.expirationTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.instance" => Some((
                    "serverCaCert.instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.kind" => Some((
                    "serverCaCert.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.self-link" => Some((
                    "serverCaCert.selfLink",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "server-ca-cert.sha1-fingerprint" => Some((
                    "serverCaCert.sha1Fingerprint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "service-account-email-address" => Some((
                    "serviceAccountEmailAddress",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.activation-policy" => Some((
                    "settings.activationPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.admin-credential-secret-name" => Some((
                    "settings.activeDirectoryConfig.adminCredentialSecretName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.dns-servers" => Some((
                    "settings.activeDirectoryConfig.dnsServers",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.active-directory-config.domain" => Some((
                    "settings.activeDirectoryConfig.domain",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.kind" => Some((
                    "settings.activeDirectoryConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.mode" => Some((
                    "settings.activeDirectoryConfig.mode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.active-directory-config.organizational-unit" => Some((
                    "settings.activeDirectoryConfig.organizationalUnit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.advanced-machine-features.threads-per-core" => Some((
                    "settings.advancedMachineFeatures.threadsPerCore",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.authorized-gae-applications" => Some((
                    "settings.authorizedGaeApplications",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.auto-upgrade-enabled" => Some((
                    "settings.autoUpgradeEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.availability-type" => Some((
                    "settings.availabilityType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.backup-retention-settings.retained-backups" => {
                    Some((
                        "settings.backupConfiguration.backupRetentionSettings.retainedBackups",
                        JsonTypeInfo {
                            jtype: JsonType::Int,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "settings.backup-configuration.backup-retention-settings.retention-unit" => Some((
                    "settings.backupConfiguration.backupRetentionSettings.retentionUnit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.backup-tier" => Some((
                    "settings.backupConfiguration.backupTier",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.binary-log-enabled" => Some((
                    "settings.backupConfiguration.binaryLogEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.enabled" => Some((
                    "settings.backupConfiguration.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.kind" => Some((
                    "settings.backupConfiguration.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.location" => Some((
                    "settings.backupConfiguration.location",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.point-in-time-recovery-enabled" => Some((
                    "settings.backupConfiguration.pointInTimeRecoveryEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.replication-log-archiving-enabled" => Some((
                    "settings.backupConfiguration.replicationLogArchivingEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.start-time" => Some((
                    "settings.backupConfiguration.startTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.transaction-log-retention-days" => Some((
                    "settings.backupConfiguration.transactionLogRetentionDays",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.backup-configuration.transactional-log-storage-state" => Some((
                    "settings.backupConfiguration.transactionalLogStorageState",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.collation" => Some((
                    "settings.collation",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connection-pool-config.connection-pooling-enabled" => Some((
                    "settings.connectionPoolConfig.connectionPoolingEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connection-pool-config.pooler-count" => Some((
                    "settings.connectionPoolConfig.poolerCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.connector-enforcement" => Some((
                    "settings.connectorEnforcement",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.crash-safe-replication-enabled" => Some((
                    "settings.crashSafeReplicationEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-api-access" => Some((
                    "settings.dataApiAccess",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-cache-config.data-cache-enabled" => Some((
                    "settings.dataCacheConfig.dataCacheEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-provisioned-iops" => Some((
                    "settings.dataDiskProvisionedIops",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-provisioned-throughput" => Some((
                    "settings.dataDiskProvisionedThroughput",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-size-gb" => Some((
                    "settings.dataDiskSizeGb",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.data-disk-type" => Some((
                    "settings.dataDiskType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.database-replication-enabled" => Some((
                    "settings.databaseReplicationEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.deletion-protection-enabled" => Some((
                    "settings.deletionProtectionEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.edition" => Some((
                    "settings.edition",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.enable-dataplex-integration" => Some((
                    "settings.enableDataplexIntegration",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.enable-google-ml-integration" => Some((
                    "settings.enableGoogleMlIntegration",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.application-id" => Some((
                    "settings.entraidConfig.applicationId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.kind" => Some((
                    "settings.entraidConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.entraid-config.tenant-id" => Some((
                    "settings.entraidConfig.tenantId",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.final-backup-config.enabled" => Some((
                    "settings.finalBackupConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.final-backup-config.retention-days" => Some((
                    "settings.finalBackupConfig.retentionDays",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-insights-enabled" => Some((
                    "settings.insightsConfig.queryInsightsEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-plans-per-minute" => Some((
                    "settings.insightsConfig.queryPlansPerMinute",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.query-string-length" => Some((
                    "settings.insightsConfig.queryStringLength",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.record-application-tags" => Some((
                    "settings.insightsConfig.recordApplicationTags",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.insights-config.record-client-address" => Some((
                    "settings.insightsConfig.recordClientAddress",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.allocated-ip-range" => Some((
                    "settings.ipConfiguration.allocatedIpRange",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.custom-subject-alternative-names" => Some((
                    "settings.ipConfiguration.customSubjectAlternativeNames",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.ip-configuration.enable-private-path-for-google-cloud-services" => {
                    Some((
                        "settings.ipConfiguration.enablePrivatePathForGoogleCloudServices",
                        JsonTypeInfo {
                            jtype: JsonType::Boolean,
                            ctype: ComplexType::Pod,
                        },
                    ))
                }
                "settings.ip-configuration.ipv4-enabled" => Some((
                    "settings.ipConfiguration.ipv4Enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.private-network" => Some((
                    "settings.ipConfiguration.privateNetwork",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.psc-config.allowed-consumer-projects" => Some((
                    "settings.ipConfiguration.pscConfig.allowedConsumerProjects",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "settings.ip-configuration.psc-config.network-attachment-uri" => Some((
                    "settings.ipConfiguration.pscConfig.networkAttachmentUri",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.psc-config.psc-enabled" => Some((
                    "settings.ipConfiguration.pscConfig.pscEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.require-ssl" => Some((
                    "settings.ipConfiguration.requireSsl",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-ca-mode" => Some((
                    "settings.ipConfiguration.serverCaMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-ca-pool" => Some((
                    "settings.ipConfiguration.serverCaPool",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.server-certificate-rotation-mode" => Some((
                    "settings.ipConfiguration.serverCertificateRotationMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.ip-configuration.ssl-mode" => Some((
                    "settings.ipConfiguration.sslMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.kind" => Some((
                    "settings.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.follow-gae-application" => Some((
                    "settings.locationPreference.followGaeApplication",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.kind" => Some((
                    "settings.locationPreference.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.secondary-zone" => Some((
                    "settings.locationPreference.secondaryZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.location-preference.zone" => Some((
                    "settings.locationPreference.zone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.day" => Some((
                    "settings.maintenanceWindow.day",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.hour" => Some((
                    "settings.maintenanceWindow.hour",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.kind" => Some((
                    "settings.maintenanceWindow.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.maintenance-window.update-track" => Some((
                    "settings.maintenanceWindow.updateTrack",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.complexity" => Some((
                    "settings.passwordValidationPolicy.complexity",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.disallow-compromised-credentials" => Some((
                    "settings.passwordValidationPolicy.disallowCompromisedCredentials",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.disallow-username-substring" => Some((
                    "settings.passwordValidationPolicy.disallowUsernameSubstring",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.enable-password-policy" => Some((
                    "settings.passwordValidationPolicy.enablePasswordPolicy",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.min-length" => Some((
                    "settings.passwordValidationPolicy.minLength",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.password-change-interval" => Some((
                    "settings.passwordValidationPolicy.passwordChangeInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.password-validation-policy.reuse-interval" => Some((
                    "settings.passwordValidationPolicy.reuseInterval",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.enabled" => Some((
                    "settings.performanceCaptureConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.probe-threshold" => Some((
                    "settings.performanceCaptureConfig.probeThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.probing-interval-seconds" => Some((
                    "settings.performanceCaptureConfig.probingIntervalSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.running-threads-threshold" => Some((
                    "settings.performanceCaptureConfig.runningThreadsThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.seconds-behind-source-threshold" => Some((
                    "settings.performanceCaptureConfig.secondsBehindSourceThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.performance-capture-config.transaction-duration-threshold" => Some((
                    "settings.performanceCaptureConfig.transactionDurationThreshold",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.pricing-plan" => Some((
                    "settings.pricingPlan",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.disable-scale-in" => Some((
                    "settings.readPoolAutoScaleConfig.disableScaleIn",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.enabled" => Some((
                    "settings.readPoolAutoScaleConfig.enabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.max-node-count" => Some((
                    "settings.readPoolAutoScaleConfig.maxNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.min-node-count" => Some((
                    "settings.readPoolAutoScaleConfig.minNodeCount",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.scale-in-cooldown-seconds" => Some((
                    "settings.readPoolAutoScaleConfig.scaleInCooldownSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.read-pool-auto-scale-config.scale-out-cooldown-seconds" => Some((
                    "settings.readPoolAutoScaleConfig.scaleOutCooldownSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.replication-lag-max-seconds" => Some((
                    "settings.replicationLagMaxSeconds",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.replication-type" => Some((
                    "settings.replicationType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.retain-backups-on-delete" => Some((
                    "settings.retainBackupsOnDelete",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.settings-version" => Some((
                    "settings.settingsVersion",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.bucket" => Some((
                    "settings.sqlServerAuditConfig.bucket",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.kind" => Some((
                    "settings.sqlServerAuditConfig.kind",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.retention-interval" => Some((
                    "settings.sqlServerAuditConfig.retentionInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.sql-server-audit-config.upload-interval" => Some((
                    "settings.sqlServerAuditConfig.uploadInterval",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.storage-auto-resize" => Some((
                    "settings.storageAutoResize",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.storage-auto-resize-limit" => Some((
                    "settings.storageAutoResizeLimit",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.tier" => Some((
                    "settings.tier",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.time-zone" => Some((
                    "settings.timeZone",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "settings.user-labels" => Some((
                    "settings.userLabels",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "sql-network-architecture" => Some((
                    "sqlNetworkArchitecture",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "state" => Some((
                    "state",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "suspension-reason" => Some((
                    "suspensionReason",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "switch-transaction-logs-to-cloud-storage-enabled" => Some((
                    "switchTransactionLogsToCloudStorageEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "tags" => Some((
                    "tags",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Map,
                    },
                )),
                "write-endpoint" => Some((
                    "writeEndpoint",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "activation-policy",
                            "active-directory-config",
                            "active-query-enabled",
                            "admin-credential-secret-name",
                            "advanced-machine-features",
                            "allocated-ip-range",
                            "allowed-consumer-projects",
                            "application-id",
                            "authorized-gae-applications",
                            "auto-upgrade-enabled",
                            "availability-type",
                            "available",
                            "available-maintenance-versions",
                            "backend-type",
                            "backup-configuration",
                            "backup-retention-settings",
                            "backup-tier",
                            "binary-log-enabled",
                            "bucket",
                            "ca-certificate",
                            "can-defer",
                            "can-reschedule",
                            "cascadable-replica",
                            "cert",
                            "cert-serial-number",
                            "client-certificate",
                            "client-key",
                            "collation",
                            "common-name",
                            "complexity",
                            "connect-retry-interval",
                            "connection-name",
                            "connection-pool-config",
                            "connection-pooling-enabled",
                            "connector-enforcement",
                            "crash-safe-replication-enabled",
                            "create-time",
                            "current-disk-size",
                            "custom-subject-alternative-names",
                            "data-api-access",
                            "data-cache-config",
                            "data-cache-enabled",
                            "data-disk-provisioned-iops",
                            "data-disk-provisioned-throughput",
                            "data-disk-size-gb",
                            "data-disk-type",
                            "database-installed-version",
                            "database-replication-enabled",
                            "database-version",
                            "day",
                            "deletion-protection-enabled",
                            "disable-scale-in",
                            "disallow-compromised-credentials",
                            "disallow-username-substring",
                            "disk-encryption-configuration",
                            "disk-encryption-status",
                            "dns-name",
                            "dns-servers",
                            "domain",
                            "dr-replica",
                            "dump-file-path",
                            "edition",
                            "enable-dataplex-integration",
                            "enable-google-ml-integration",
                            "enable-password-policy",
                            "enable-private-path-for-google-cloud-services",
                            "enabled",
                            "entitled",
                            "entraid-config",
                            "etag",
                            "expiration-time",
                            "failover-dr-replica-name",
                            "failover-replica",
                            "failover-target",
                            "final-backup-config",
                            "flag-recommender-enabled",
                            "follow-gae-application",
                            "gce-zone",
                            "gemini-config",
                            "google-vacuum-mgmt-enabled",
                            "host-port",
                            "hour",
                            "include-replicas-for-major-version-upgrade",
                            "index-advisor-enabled",
                            "insights-config",
                            "instance",
                            "instance-type",
                            "ip-configuration",
                            "ipv4-enabled",
                            "ipv6-address",
                            "kind",
                            "kms-key-name",
                            "kms-key-version-name",
                            "location",
                            "location-preference",
                            "maintenance-version",
                            "maintenance-window",
                            "master-heartbeat-period",
                            "master-instance-name",
                            "max-disk-size",
                            "max-node-count",
                            "min-length",
                            "min-node-count",
                            "mode",
                            "mysql-replica-configuration",
                            "name",
                            "network-attachment-uri",
                            "node-count",
                            "on-premises-configuration",
                            "oom-session-cancel-enabled",
                            "organizational-unit",
                            "out-of-disk-report",
                            "password",
                            "password-change-interval",
                            "password-validation-policy",
                            "performance-capture-config",
                            "point-in-time-recovery-enabled",
                            "pooler-count",
                            "pricing-plan",
                            "primary-dns-name",
                            "private-network",
                            "probe-threshold",
                            "probing-interval-seconds",
                            "project",
                            "psa-write-endpoint",
                            "psc-config",
                            "psc-enabled",
                            "psc-service-attachment-link",
                            "query-insights-enabled",
                            "query-plans-per-minute",
                            "query-string-length",
                            "read-pool-auto-scale-config",
                            "record-application-tags",
                            "record-client-address",
                            "region",
                            "replica-configuration",
                            "replica-names",
                            "replication-cluster",
                            "replication-lag-max-seconds",
                            "replication-log-archiving-enabled",
                            "replication-type",
                            "require-ssl",
                            "retain-backups-on-delete",
                            "retained-backups",
                            "retention-days",
                            "retention-interval",
                            "retention-unit",
                            "reuse-interval",
                            "root-password",
                            "running-threads-threshold",
                            "satisfies-pzi",
                            "satisfies-pzs",
                            "scale-in-cooldown-seconds",
                            "scale-out-cooldown-seconds",
                            "schedule-deadline-time",
                            "scheduled-maintenance",
                            "secondary-gce-zone",
                            "secondary-zone",
                            "seconds-behind-source-threshold",
                            "self-link",
                            "server-ca-cert",
                            "server-ca-mode",
                            "server-ca-pool",
                            "server-certificate-rotation-mode",
                            "service-account-email-address",
                            "settings",
                            "settings-version",
                            "sha1-fingerprint",
                            "source-instance",
                            "sql-min-recommended-increase-size-gb",
                            "sql-network-architecture",
                            "sql-out-of-disk-state",
                            "sql-server-audit-config",
                            "ssl-cipher",
                            "ssl-mode",
                            "ssl-option",
                            "start-time",
                            "state",
                            "storage-auto-resize",
                            "storage-auto-resize-limit",
                            "suspension-reason",
                            "switch-transaction-logs-to-cloud-storage-enabled",
                            "tags",
                            "tenant-id",
                            "threads-per-core",
                            "tier",
                            "time-zone",
                            "transaction-duration-threshold",
                            "transaction-log-retention-days",
                            "transactional-log-storage-state",
                            "update-track",
                            "upload-interval",
                            "user-labels",
                            "username",
                            "verify-server-certificate",
                            "write-endpoint",
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
        let mut request: api::DatabaseInstance = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.instances().update(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _operations_cancel(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.operations().cancel(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("operation").unwrap_or(""),
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

    async fn _operations_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.operations().get(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("operation").unwrap_or(""),
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

    async fn _operations_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self
            .hub
            .operations()
            .list(opt.value_of("project").unwrap_or(""));
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
                "max-results" => {
                    call = call.max_results(
                        value
                            .map(|v| arg_from_str(v, err, "max-results", "uint32"))
                            .unwrap_or(0),
                    );
                }
                "instance" => {
                    call = call.instance(value.unwrap_or(""));
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
                                    ["instance", "max-results", "page-token"].iter().map(|v| *v),
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

    async fn _projects_instances_get_disk_shrink_config(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().instances_get_disk_shrink_config(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _projects_instances_get_latest_recovery_time(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.projects().instances_get_latest_recovery_time(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
        );
        for parg in opt
            .values_of("v")
            .map(|i| i.collect())
            .unwrap_or(Vec::new())
            .iter()
        {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "source-instance-deletion-time" => {
                    call = call.source_instance_deletion_time(
                        value
                            .map(|v| {
                                arg_from_str(
                                    v,
                                    err,
                                    "source-instance-deletion-time",
                                    "google-datetime",
                                )
                            })
                            .unwrap_or(chrono::Utc::now()),
                    );
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
                                v.extend(["source-instance-deletion-time"].iter().map(|v| *v));
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

    async fn _projects_instances_perform_disk_shrink(
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
                "target-size-gb" => Some((
                    "targetSizeGb",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["target-size-gb"]);
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
        let mut request: api::PerformDiskShrinkContext =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().instances_perform_disk_shrink(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _projects_instances_reschedule_maintenance(
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
                "reschedule.reschedule-type" => Some((
                    "reschedule.rescheduleType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "reschedule.schedule-time" => Some((
                    "reschedule.scheduleTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec!["reschedule", "reschedule-type", "schedule-time"],
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
        let mut request: api::SqlInstancesRescheduleMaintenanceRequestBody =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().instances_reschedule_maintenance(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _projects_instances_reset_replica_size(
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
        let mut request: api::SqlInstancesResetReplicaSizeRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().instances_reset_replica_size(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _projects_instances_start_external_sync(
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
                "migration-type" => Some((
                    "migrationType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "replica-overwrite-enabled" => Some((
                    "replicaOverwriteEnabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "skip-verification" => Some((
                    "skipVerification",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sync-mode" => Some((
                    "syncMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sync-parallel-level" => Some((
                    "syncParallelLevel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "migration-type",
                            "replica-overwrite-enabled",
                            "skip-verification",
                            "sync-mode",
                            "sync-parallel-level",
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
        let mut request: api::SqlInstancesStartExternalSyncRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().instances_start_external_sync(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _projects_instances_verify_external_sync_settings(
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
                "migration-type" => Some((
                    "migrationType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sync-mode" => Some((
                    "syncMode",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sync-parallel-level" => Some((
                    "syncParallelLevel",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "verify-connection-only" => Some((
                    "verifyConnectionOnly",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "verify-replication-only" => Some((
                    "verifyReplicationOnly",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "migration-type",
                            "sync-mode",
                            "sync-parallel-level",
                            "verify-connection-only",
                            "verify-replication-only",
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
        let mut request: api::SqlInstancesVerifyExternalSyncSettingsRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.projects().instances_verify_external_sync_settings(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _ssl_certs_create_ephemeral(
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
                "access-token" => Some((
                    "access_token",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "public-key" => Some((
                    "public_key",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion =
                        FieldCursor::did_you_mean(key, &vec!["access-token", "public-key"]);
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
        let mut request: api::SslCertsCreateEphemeralRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.ssl_certs().create_ephemeral(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _ssl_certs_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().delete(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            opt.value_of("sha1-fingerprint").unwrap_or(""),
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

    async fn _ssl_certs_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().get(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
            opt.value_of("sha1-fingerprint").unwrap_or(""),
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

    async fn _ssl_certs_insert(
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
                "common-name" => Some((
                    "commonName",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(key, &vec!["common-name"]);
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
        let mut request: api::SslCertsInsertRequest =
            serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.ssl_certs().insert(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _ssl_certs_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.ssl_certs().list(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _tiers_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.tiers().list(opt.value_of("project").unwrap_or(""));
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

    async fn _users_delete(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.users().delete(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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
                "host" => {
                    call = call.host(value.unwrap_or(""));
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
                                v.extend(["host", "name"].iter().map(|v| *v));
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

    async fn _users_get(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.users().get(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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
                "host" => {
                    call = call.host(value.unwrap_or(""));
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
                                v.extend(["host"].iter().map(|v| *v));
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

    async fn _users_insert(
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
                "database-roles" => Some((
                    "databaseRoles",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "dual-password-type" => Some((
                    "dualPasswordType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "host" => Some((
                    "host",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "iam-email" => Some((
                    "iamEmail",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "iam-status" => Some((
                    "iamStatus",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance" => Some((
                    "instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
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
                "password" => Some((
                    "password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.allowed-failed-attempts" => Some((
                    "passwordPolicy.allowedFailedAttempts",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.enable-failed-attempts-check" => Some((
                    "passwordPolicy.enableFailedAttemptsCheck",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.enable-password-verification" => Some((
                    "passwordPolicy.enablePasswordVerification",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.password-expiration-duration" => Some((
                    "passwordPolicy.passwordExpirationDuration",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.status.locked" => Some((
                    "passwordPolicy.status.locked",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.status.password-expiration-time" => Some((
                    "passwordPolicy.status.passwordExpirationTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-user-details.disabled" => Some((
                    "sqlserverUserDetails.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-user-details.server-roles" => Some((
                    "sqlserverUserDetails.serverRoles",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "type" => Some((
                    "type",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "allowed-failed-attempts",
                            "database-roles",
                            "disabled",
                            "dual-password-type",
                            "enable-failed-attempts-check",
                            "enable-password-verification",
                            "etag",
                            "host",
                            "iam-email",
                            "iam-status",
                            "instance",
                            "kind",
                            "locked",
                            "name",
                            "password",
                            "password-expiration-duration",
                            "password-expiration-time",
                            "password-policy",
                            "project",
                            "server-roles",
                            "sqlserver-user-details",
                            "status",
                            "type",
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
        let mut request: api::User = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.users().insert(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _users_list(
        &self,
        opt: &ArgMatches<'n>,
        dry_run: bool,
        err: &mut InvalidOptionsError,
    ) -> Result<(), DoitError> {
        let mut call = self.hub.users().list(
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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

    async fn _users_update(
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
                "database-roles" => Some((
                    "databaseRoles",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "dual-password-type" => Some((
                    "dualPasswordType",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "etag" => Some((
                    "etag",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "host" => Some((
                    "host",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "iam-email" => Some((
                    "iamEmail",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "iam-status" => Some((
                    "iamStatus",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "instance" => Some((
                    "instance",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "kind" => Some((
                    "kind",
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
                "password" => Some((
                    "password",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.allowed-failed-attempts" => Some((
                    "passwordPolicy.allowedFailedAttempts",
                    JsonTypeInfo {
                        jtype: JsonType::Int,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.enable-failed-attempts-check" => Some((
                    "passwordPolicy.enableFailedAttemptsCheck",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.enable-password-verification" => Some((
                    "passwordPolicy.enablePasswordVerification",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.password-expiration-duration" => Some((
                    "passwordPolicy.passwordExpirationDuration",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.status.locked" => Some((
                    "passwordPolicy.status.locked",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "password-policy.status.password-expiration-time" => Some((
                    "passwordPolicy.status.passwordExpirationTime",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "project" => Some((
                    "project",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-user-details.disabled" => Some((
                    "sqlserverUserDetails.disabled",
                    JsonTypeInfo {
                        jtype: JsonType::Boolean,
                        ctype: ComplexType::Pod,
                    },
                )),
                "sqlserver-user-details.server-roles" => Some((
                    "sqlserverUserDetails.serverRoles",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Vec,
                    },
                )),
                "type" => Some((
                    "type",
                    JsonTypeInfo {
                        jtype: JsonType::String,
                        ctype: ComplexType::Pod,
                    },
                )),
                _ => {
                    let suggestion = FieldCursor::did_you_mean(
                        key,
                        &vec![
                            "allowed-failed-attempts",
                            "database-roles",
                            "disabled",
                            "dual-password-type",
                            "enable-failed-attempts-check",
                            "enable-password-verification",
                            "etag",
                            "host",
                            "iam-email",
                            "iam-status",
                            "instance",
                            "kind",
                            "locked",
                            "name",
                            "password",
                            "password-expiration-duration",
                            "password-expiration-time",
                            "password-policy",
                            "project",
                            "server-roles",
                            "sqlserver-user-details",
                            "status",
                            "type",
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
        let mut request: api::User = serde_json::value::from_value(object).unwrap();
        let mut call = self.hub.users().update(
            request,
            opt.value_of("project").unwrap_or(""),
            opt.value_of("instance").unwrap_or(""),
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
                "host" => {
                    call = call.host(value.unwrap_or(""));
                }
                "database-roles" => {
                    call = call.add_database_roles(value.unwrap_or(""));
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
                                v.extend(["database-roles", "host", "name"].iter().map(|v| *v));
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
            ("backups", Some(opt)) => match opt.subcommand() {
                ("create-backup", Some(opt)) => {
                    call_result = self._backups_create_backup(opt, dry_run, &mut err).await;
                }
                ("delete-backup", Some(opt)) => {
                    call_result = self._backups_delete_backup(opt, dry_run, &mut err).await;
                }
                ("get-backup", Some(opt)) => {
                    call_result = self._backups_get_backup(opt, dry_run, &mut err).await;
                }
                ("list-backups", Some(opt)) => {
                    call_result = self._backups_list_backups(opt, dry_run, &mut err).await;
                }
                ("update-backup", Some(opt)) => {
                    call_result = self._backups_update_backup(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("backups".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("backup-runs", Some(opt)) => match opt.subcommand() {
                ("delete", Some(opt)) => {
                    call_result = self._backup_runs_delete(opt, dry_run, &mut err).await;
                }
                ("get", Some(opt)) => {
                    call_result = self._backup_runs_get(opt, dry_run, &mut err).await;
                }
                ("insert", Some(opt)) => {
                    call_result = self._backup_runs_insert(opt, dry_run, &mut err).await;
                }
                ("list", Some(opt)) => {
                    call_result = self._backup_runs_list(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("backup-runs".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("connect", Some(opt)) => match opt.subcommand() {
                ("generate-ephemeral", Some(opt)) => {
                    call_result = self
                        ._connect_generate_ephemeral(opt, dry_run, &mut err)
                        .await;
                }
                ("get", Some(opt)) => {
                    call_result = self._connect_get(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("connect".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("databases", Some(opt)) => match opt.subcommand() {
                ("delete", Some(opt)) => {
                    call_result = self._databases_delete(opt, dry_run, &mut err).await;
                }
                ("get", Some(opt)) => {
                    call_result = self._databases_get(opt, dry_run, &mut err).await;
                }
                ("insert", Some(opt)) => {
                    call_result = self._databases_insert(opt, dry_run, &mut err).await;
                }
                ("list", Some(opt)) => {
                    call_result = self._databases_list(opt, dry_run, &mut err).await;
                }
                ("patch", Some(opt)) => {
                    call_result = self._databases_patch(opt, dry_run, &mut err).await;
                }
                ("update", Some(opt)) => {
                    call_result = self._databases_update(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("databases".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("flags", Some(opt)) => match opt.subcommand() {
                ("list", Some(opt)) => {
                    call_result = self._flags_list(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("flags".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("instances", Some(opt)) => match opt.subcommand() {
                ("list-entra-id-certificates", Some(opt)) => {
                    call_result = self
                        ._instances_list_entra_id_certificates(opt, dry_run, &mut err)
                        .await;
                }
                ("list-server-certificates", Some(opt)) => {
                    call_result = self
                        ._instances_list_server_certificates(opt, dry_run, &mut err)
                        .await;
                }
                ("rotate-entra-id-certificate", Some(opt)) => {
                    call_result = self
                        ._instances_rotate_entra_id_certificate(opt, dry_run, &mut err)
                        .await;
                }
                ("rotate-server-certificate", Some(opt)) => {
                    call_result = self
                        ._instances_rotate_server_certificate(opt, dry_run, &mut err)
                        .await;
                }
                ("acquire-ssrs-lease", Some(opt)) => {
                    call_result = self
                        ._instances_acquire_ssrs_lease(opt, dry_run, &mut err)
                        .await;
                }
                ("add-entra-id-certificate", Some(opt)) => {
                    call_result = self
                        ._instances_add_entra_id_certificate(opt, dry_run, &mut err)
                        .await;
                }
                ("add-server-ca", Some(opt)) => {
                    call_result = self._instances_add_server_ca(opt, dry_run, &mut err).await;
                }
                ("add-server-certificate", Some(opt)) => {
                    call_result = self
                        ._instances_add_server_certificate(opt, dry_run, &mut err)
                        .await;
                }
                ("clone", Some(opt)) => {
                    call_result = self._instances_clone(opt, dry_run, &mut err).await;
                }
                ("delete", Some(opt)) => {
                    call_result = self._instances_delete(opt, dry_run, &mut err).await;
                }
                ("demote", Some(opt)) => {
                    call_result = self._instances_demote(opt, dry_run, &mut err).await;
                }
                ("demote-master", Some(opt)) => {
                    call_result = self._instances_demote_master(opt, dry_run, &mut err).await;
                }
                ("execute-sql", Some(opt)) => {
                    call_result = self._instances_execute_sql(opt, dry_run, &mut err).await;
                }
                ("export", Some(opt)) => {
                    call_result = self._instances_export(opt, dry_run, &mut err).await;
                }
                ("failover", Some(opt)) => {
                    call_result = self._instances_failover(opt, dry_run, &mut err).await;
                }
                ("get", Some(opt)) => {
                    call_result = self._instances_get(opt, dry_run, &mut err).await;
                }
                ("import", Some(opt)) => {
                    call_result = self._instances_import(opt, dry_run, &mut err).await;
                }
                ("insert", Some(opt)) => {
                    call_result = self._instances_insert(opt, dry_run, &mut err).await;
                }
                ("list", Some(opt)) => {
                    call_result = self._instances_list(opt, dry_run, &mut err).await;
                }
                ("list-server-cas", Some(opt)) => {
                    call_result = self
                        ._instances_list_server_cas(opt, dry_run, &mut err)
                        .await;
                }
                ("patch", Some(opt)) => {
                    call_result = self._instances_patch(opt, dry_run, &mut err).await;
                }
                ("point-in-time-restore", Some(opt)) => {
                    call_result = self
                        ._instances_point_in_time_restore(opt, dry_run, &mut err)
                        .await;
                }
                ("pre-check-major-version-upgrade", Some(opt)) => {
                    call_result = self
                        ._instances_pre_check_major_version_upgrade(opt, dry_run, &mut err)
                        .await;
                }
                ("promote-replica", Some(opt)) => {
                    call_result = self
                        ._instances_promote_replica(opt, dry_run, &mut err)
                        .await;
                }
                ("reencrypt", Some(opt)) => {
                    call_result = self._instances_reencrypt(opt, dry_run, &mut err).await;
                }
                ("release-ssrs-lease", Some(opt)) => {
                    call_result = self
                        ._instances_release_ssrs_lease(opt, dry_run, &mut err)
                        .await;
                }
                ("reset-ssl-config", Some(opt)) => {
                    call_result = self
                        ._instances_reset_ssl_config(opt, dry_run, &mut err)
                        .await;
                }
                ("restart", Some(opt)) => {
                    call_result = self._instances_restart(opt, dry_run, &mut err).await;
                }
                ("restore-backup", Some(opt)) => {
                    call_result = self._instances_restore_backup(opt, dry_run, &mut err).await;
                }
                ("rotate-server-ca", Some(opt)) => {
                    call_result = self
                        ._instances_rotate_server_ca(opt, dry_run, &mut err)
                        .await;
                }
                ("start-replica", Some(opt)) => {
                    call_result = self._instances_start_replica(opt, dry_run, &mut err).await;
                }
                ("stop-replica", Some(opt)) => {
                    call_result = self._instances_stop_replica(opt, dry_run, &mut err).await;
                }
                ("switchover", Some(opt)) => {
                    call_result = self._instances_switchover(opt, dry_run, &mut err).await;
                }
                ("truncate-log", Some(opt)) => {
                    call_result = self._instances_truncate_log(opt, dry_run, &mut err).await;
                }
                ("update", Some(opt)) => {
                    call_result = self._instances_update(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("instances".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("operations", Some(opt)) => match opt.subcommand() {
                ("cancel", Some(opt)) => {
                    call_result = self._operations_cancel(opt, dry_run, &mut err).await;
                }
                ("get", Some(opt)) => {
                    call_result = self._operations_get(opt, dry_run, &mut err).await;
                }
                ("list", Some(opt)) => {
                    call_result = self._operations_list(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("operations".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("projects", Some(opt)) => match opt.subcommand() {
                ("instances-get-disk-shrink-config", Some(opt)) => {
                    call_result = self
                        ._projects_instances_get_disk_shrink_config(opt, dry_run, &mut err)
                        .await;
                }
                ("instances-get-latest-recovery-time", Some(opt)) => {
                    call_result = self
                        ._projects_instances_get_latest_recovery_time(opt, dry_run, &mut err)
                        .await;
                }
                ("instances-perform-disk-shrink", Some(opt)) => {
                    call_result = self
                        ._projects_instances_perform_disk_shrink(opt, dry_run, &mut err)
                        .await;
                }
                ("instances-reschedule-maintenance", Some(opt)) => {
                    call_result = self
                        ._projects_instances_reschedule_maintenance(opt, dry_run, &mut err)
                        .await;
                }
                ("instances-reset-replica-size", Some(opt)) => {
                    call_result = self
                        ._projects_instances_reset_replica_size(opt, dry_run, &mut err)
                        .await;
                }
                ("instances-start-external-sync", Some(opt)) => {
                    call_result = self
                        ._projects_instances_start_external_sync(opt, dry_run, &mut err)
                        .await;
                }
                ("instances-verify-external-sync-settings", Some(opt)) => {
                    call_result = self
                        ._projects_instances_verify_external_sync_settings(opt, dry_run, &mut err)
                        .await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("projects".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("ssl-certs", Some(opt)) => match opt.subcommand() {
                ("create-ephemeral", Some(opt)) => {
                    call_result = self
                        ._ssl_certs_create_ephemeral(opt, dry_run, &mut err)
                        .await;
                }
                ("delete", Some(opt)) => {
                    call_result = self._ssl_certs_delete(opt, dry_run, &mut err).await;
                }
                ("get", Some(opt)) => {
                    call_result = self._ssl_certs_get(opt, dry_run, &mut err).await;
                }
                ("insert", Some(opt)) => {
                    call_result = self._ssl_certs_insert(opt, dry_run, &mut err).await;
                }
                ("list", Some(opt)) => {
                    call_result = self._ssl_certs_list(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("ssl-certs".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("tiers", Some(opt)) => match opt.subcommand() {
                ("list", Some(opt)) => {
                    call_result = self._tiers_list(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("tiers".to_string()));
                    writeln!(std::io::stderr(), "{}\n", opt.usage()).ok();
                }
            },
            ("users", Some(opt)) => match opt.subcommand() {
                ("delete", Some(opt)) => {
                    call_result = self._users_delete(opt, dry_run, &mut err).await;
                }
                ("get", Some(opt)) => {
                    call_result = self._users_get(opt, dry_run, &mut err).await;
                }
                ("insert", Some(opt)) => {
                    call_result = self._users_insert(opt, dry_run, &mut err).await;
                }
                ("list", Some(opt)) => {
                    call_result = self._users_list(opt, dry_run, &mut err).await;
                }
                ("update", Some(opt)) => {
                    call_result = self._users_update(opt, dry_run, &mut err).await;
                }
                _ => {
                    err.issues
                        .push(CLIError::MissingMethodError("users".to_string()));
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

            match common::application_secret_from_directory(&config_dir, "sqladmin1-secret.json",
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
        .persist_tokens_to_disk(format!("{}/sqladmin1", config_dir))
        .build()
        .await
        .unwrap();

        let engine = Engine {
            opt,
            hub: api::SQLAdmin::new(client, auth),
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
        ("backups", "methods: 'create-backup', 'delete-backup', 'get-backup', 'list-backups' and 'update-backup'", vec![
            ("create-backup",
                    Some(r##"Creates a backup for a Cloud SQL instance. This API can be used only to create on-demand backups."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backups_create-backup",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where this backup is created. Format: projects/{project}"##),
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
            ("delete-backup",
                    Some(r##"Deletes the backup."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backups_delete-backup",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the backup to delete. Format: projects/{project}/backups/{backup}"##),
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
            ("get-backup",
                    Some(r##"Retrieves a resource containing information about a backup."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backups_get-backup",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Required. The name of the backup to retrieve. Format: projects/{project}/backups/{backup}"##),
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
            ("list-backups",
                    Some(r##"Lists all backups associated with the project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backups_list-backups",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent that owns this collection of backups. Format: projects/{project}"##),
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
            ("update-backup",
                    Some(r##"Updates the retention period and description of the backup. You can use this API to update final backups only."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backups_update-backup",
                  vec![
                    (Some(r##"name"##),
                     None,
                     Some(r##"Output only. The resource name of the backup. Format: projects/{project}/backups/{backup}."##),
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
            ]),
            ("backup-runs", "methods: 'delete', 'get', 'insert' and 'list'", vec![
            ("delete",
                    Some(r##"Deletes the backup taken by a backup run."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backup-runs_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of the backup run to delete. To find a backup run ID, use the [list](https://cloud.google.com/sql/docs/mysql/admin-api/rest/v1/backupRuns/list) method."##),
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
            ("get",
                    Some(r##"Retrieves a resource containing information about a backup run."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backup-runs_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"id"##),
                     None,
                     Some(r##"The ID of this backup run."##),
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
            ("insert",
                    Some(r##"Creates a new backup run on demand."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backup-runs_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("list",
                    Some(r##"Lists all backup runs associated with the project or a given instance and configuration in the reverse chronological order of the backup initiation time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/backup-runs_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID, or "-" for all instances. This does not include the project ID."##),
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
            ("connect", "methods: 'generate-ephemeral' and 'get'", vec![
            ("generate-ephemeral",
                    Some(r##"Generates a short-lived X509 certificate containing the provided public key and signed by a private key specific to the target instance. Users may use the certificate to authenticate as themselves when connecting to the database."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/connect_generate-ephemeral",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("get",
                    Some(r##"Retrieves connect settings about a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/connect_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("databases", "methods: 'delete', 'get', 'insert', 'list', 'patch' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a database from a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/databases_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be deleted in the instance."##),
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
            ("get",
                    Some(r##"Retrieves a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/databases_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database in the instance."##),
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
            ("insert",
                    Some(r##"Inserts a resource containing information about a database inside a Cloud SQL instance. **Note:** You can't modify the default character set and collation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/databases_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
            ("list",
                    Some(r##"Lists databases in the specified Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/databases_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("patch",
                    Some(r##"Partially updates a resource containing information about a database inside a Cloud SQL instance. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/databases_patch",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be updated in the instance."##),
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
            ("update",
                    Some(r##"Updates a resource containing information about a database inside a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/databases_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"database"##),
                     None,
                     Some(r##"Name of the database to be updated in the instance."##),
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
            ]),
            ("flags", "methods: 'list'", vec![
            ("list",
                    Some(r##"Lists all available database flags for Cloud SQL instances."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/flags_list",
                  vec![
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
            ("instances", "methods: 'list-entra-id-certificates', 'list-server-certificates', 'rotate-entra-id-certificate', 'rotate-server-certificate', 'acquire-ssrs-lease', 'add-entra-id-certificate', 'add-server-ca', 'add-server-certificate', 'clone', 'delete', 'demote', 'demote-master', 'execute-sql', 'export', 'failover', 'get', 'import', 'insert', 'list', 'list-server-cas', 'patch', 'point-in-time-restore', 'pre-check-major-version-upgrade', 'promote-replica', 'reencrypt', 'release-ssrs-lease', 'reset-ssl-config', 'restart', 'restore-backup', 'rotate-server-ca', 'start-replica', 'stop-replica', 'switchover', 'truncate-log' and 'update'", vec![
            ("list-entra-id-certificates",
                    Some(r##"Lists all versions of EntraID certificates for the specified instance. There can be up to three sets of certificates listed: the certificate that is currently in use, a future that has been added but not yet used to sign a certificate, and a certificate that has been rotated out."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_list-entra-id-certificates",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance ID. This does not include the project ID."##),
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
            ("list-server-certificates",
                    Some(r##"Lists all versions of server certificates and certificate authorities (CAs) for the specified instance. There can be up to three sets of certs listed: the certificate that is currently in use, a future that has been added but not yet used to sign a certificate, and a certificate that has been rotated out. For instances not using Certificate Authority Service (CAS) server CA, use ListServerCas instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_list-server-certificates",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance ID. This does not include the project ID."##),
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
            ("rotate-entra-id-certificate",
                    Some(r##"Rotates the server certificate version to one previously added with the addEntraIdCertificate method."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_rotate-entra-id-certificate",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance ID. This does not include the project ID."##),
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
            ("rotate-server-certificate",
                    Some(r##"Rotates the server certificate version to one previously added with the addServerCertificate method. For instances not using Certificate Authority Service (CAS) server CA, use RotateServerCa instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_rotate-server-certificate",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance ID. This does not include the project ID."##),
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
            ("acquire-ssrs-lease",
                    Some(r##"Acquire a lease for the setup of SQL Server Reporting Services (SSRS)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_acquire-ssrs-lease",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance (Example: project-id)."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance ID. This doesn't include the project ID. It's composed of lowercase letters, numbers, and hyphens, and it must start with a letter. The total length must be 98 characters or less (Example: instance-id)."##),
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
            ("add-entra-id-certificate",
                    Some(r##"Adds a new Entra ID certificate for the specified instance. If an Entra ID certificate was previously added but never used in a certificate rotation, this operation replaces that version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_add-entra-id-certificate",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance ID. This does not include the project ID."##),
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
            ("add-server-ca",
                    Some(r##"Adds a new trusted Certificate Authority (CA) version for the specified instance. Required to prepare for a certificate rotation. If a CA version was previously added but never used in a certificate rotation, this operation replaces that version. There cannot be more than one CA version waiting to be rotated in. For instances that have enabled Certificate Authority Service (CAS) based server CA, use AddServerCertificate to add a new server certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_add-server-ca",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("add-server-certificate",
                    Some(r##"Add a new trusted server certificate version for the specified instance using Certificate Authority Service (CAS) server CA. Required to prepare for a certificate rotation. If a server certificate version was previously added but never used in a certificate rotation, this operation replaces that version. There cannot be more than one certificate version waiting to be rotated in. For instances not using CAS server CA, use AddServerCa instead."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_add-server-certificate",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("clone",
                    Some(r##"Creates a Cloud SQL instance as a clone of the source instance. Using this operation might cause your instance to restart."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_clone",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the source as well as the clone Cloud SQL instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. The ID of the Cloud SQL instance to be cloned (source). This does not include the project ID."##),
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
            ("delete",
                    Some(r##"Deletes a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be deleted."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("demote",
                    Some(r##"Demotes an existing standalone instance to be a Cloud SQL read replica for an external database server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_demote",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance name."##),
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
            ("demote-master",
                    Some(r##"Demotes the stand-alone instance to be a Cloud SQL read replica for an external database server."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_demote-master",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance name."##),
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
            ("execute-sql",
                    Some(r##"Execute SQL statements."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_execute-sql",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Database instance ID. This does not include the project ID."##),
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
            ("export",
                    Some(r##"Exports data from a Cloud SQL instance to a Cloud Storage bucket as a SQL dump or CSV file."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_export",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be exported."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("failover",
                    Some(r##"Initiates a manual failover of a high availability (HA) primary instance to a standby instance, which becomes the primary instance. Users are then rerouted to the new primary. For more information, see the [Overview of high availability](https://cloud.google.com/sql/docs/mysql/high-availability) page in the Cloud SQL documentation. If using Legacy HA (MySQL only), this causes the instance to failover to its failover replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_failover",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("get",
                    Some(r##"Retrieves a resource containing information about a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Database instance ID. This does not include the project ID."##),
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
            ("import",
                    Some(r##"Imports data into a Cloud SQL instance from a SQL dump or CSV file in Cloud Storage."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_import",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("insert",
                    Some(r##"Creates a new Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project to which the newly created Cloud SQL instances should belong."##),
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
            ("list",
                    Some(r##"Lists instances under a given project."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list Cloud SQL instances."##),
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
            ("list-server-cas",
                    Some(r##"Lists all of the trusted Certificate Authorities (CAs) for the specified instance. There can be up to three CAs listed: the CA that was used to sign the certificate that is currently in use, a CA that has been added but not yet used to sign a certificate, and a CA used to sign a certificate that has previously rotated out."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_list-server-cas",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("patch",
                    Some(r##"Partially updates settings of a Cloud SQL instance by merging the request with the current configuration. This method supports patch semantics."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_patch",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("point-in-time-restore",
                    Some(r##"Point in time restore for an instance managed by Google Cloud Backup and Disaster Recovery."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_point-in-time-restore",
                  vec![
                    (Some(r##"parent"##),
                     None,
                     Some(r##"Required. The parent resource where you created this instance. Format: projects/{project}"##),
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
            ("pre-check-major-version-upgrade",
                    Some(r##"Execute MVU Pre-checks"##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_pre-check-major-version-upgrade",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. Cloud SQL instance ID. This does not include the project ID."##),
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
            ("promote-replica",
                    Some(r##"Promotes the read replica instance to be an independent Cloud SQL primary instance. Using this operation might cause your instance to restart."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_promote-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("reencrypt",
                    Some(r##"Reencrypt CMEK instance with latest key version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_reencrypt",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("release-ssrs-lease",
                    Some(r##"Release a lease for the setup of SQL Server Reporting Services (SSRS)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_release-ssrs-lease",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. The project ID that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Required. The Cloud SQL instance ID. This doesn't include the project ID. The instance ID contains lowercase letters, numbers, and hyphens, and it must start with a letter. This ID can have a maximum length of 98 characters."##),
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
            ("reset-ssl-config",
                    Some(r##"Deletes all client certificates and generates a new server SSL certificate for the instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_reset-ssl-config",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("restart",
                    Some(r##"Restarts a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_restart",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance to be restarted."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("restore-backup",
                    Some(r##"Restores a backup of a Cloud SQL instance. Using this operation might cause your instance to restart."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_restore-backup",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("rotate-server-ca",
                    Some(r##"Rotates the server certificate to one signed by the Certificate Authority (CA) version previously added with the addServerCA method. For instances that have enabled Certificate Authority Service (CAS) based server CA, use RotateServerCertificate to rotate the server certificate."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_rotate-server-ca",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("start-replica",
                    Some(r##"Starts the replication in the read replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_start-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("stop-replica",
                    Some(r##"Stops the replication in the read replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_stop-replica",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("switchover",
                    Some(r##"Switches over from the primary instance to the DR replica instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_switchover",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the replica."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("truncate-log",
                    Some(r##"Truncate MySQL general and slow query log tables MySQL only."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_truncate-log",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the Cloud SQL project."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("update",
                    Some(r##"Updates settings of a Cloud SQL instance. Using this operation might cause your instance to restart."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/instances_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ]),
            ("operations", "methods: 'cancel', 'get' and 'list'", vec![
            ("cancel",
                    Some(r##"Cancels an instance operation that has been performed on an instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/operations_cancel",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"operation"##),
                     None,
                     Some(r##"Instance operation ID."##),
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
            ("get",
                    Some(r##"Retrieves an instance operation that has been performed on an instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/operations_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Required. Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"operation"##),
                     None,
                     Some(r##"Required. Instance operation ID."##),
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
            ("list",
                    Some(r##"Lists all instance operations that have been performed on the given Cloud SQL instance in the reverse chronological order of the start time."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/operations_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
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
            ("projects", "methods: 'instances-get-disk-shrink-config', 'instances-get-latest-recovery-time', 'instances-perform-disk-shrink', 'instances-reschedule-maintenance', 'instances-reset-replica-size', 'instances-start-external-sync' and 'instances-verify-external-sync-settings'", vec![
            ("instances-get-disk-shrink-config",
                    Some(r##"Get Disk Shrink Config for a given instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/projects_instances-get-disk-shrink-config",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("instances-get-latest-recovery-time",
                    Some(r##"Get Latest Recovery Time for a given instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/projects_instances-get-latest-recovery-time",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("instances-perform-disk-shrink",
                    Some(r##"Perform Disk Shrink on primary instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/projects_instances-perform-disk-shrink",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("instances-reschedule-maintenance",
                    Some(r##"Reschedules the maintenance on the given instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/projects_instances-reschedule-maintenance",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("instances-reset-replica-size",
                    Some(r##"Reset Replica Size to primary instance disk size."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/projects_instances-reset-replica-size",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the read replica."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL read replica instance name."##),
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
            ("instances-start-external-sync",
                    Some(r##"Start External primary instance migration."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/projects_instances-start-external-sync",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("instances-verify-external-sync-settings",
                    Some(r##"Verify External primary instance external sync settings."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/projects_instances-verify-external-sync-settings",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ]),
            ("ssl-certs", "methods: 'create-ephemeral', 'delete', 'get', 'insert' and 'list'", vec![
            ("create-ephemeral",
                    Some(r##"Generates a short-lived X509 certificate containing the provided public key and signed by a private key specific to the target instance. Users may use the certificate to authenticate as themselves when connecting to the database."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/ssl-certs_create-ephemeral",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the Cloud SQL project."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("delete",
                    Some(r##"Deletes the SSL certificate. For First Generation instances, the certificate remains valid until the instance is restarted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/ssl-certs_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"sha1-fingerprint"##),
                     None,
                     Some(r##"Sha1 FingerPrint."##),
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
            ("get",
                    Some(r##"Retrieves a particular SSL certificate. Does not include the private key (required for usage). The private key must be saved from the response to initial creation."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/ssl-certs_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"sha1-fingerprint"##),
                     None,
                     Some(r##"Sha1 FingerPrint."##),
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
            ("insert",
                    Some(r##"Creates an SSL certificate and returns it along with the private key and server certificate authority. The new certificate will not be usable until the instance is restarted."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/ssl-certs_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("list",
                    Some(r##"Lists all of the current SSL certificates for the instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/ssl-certs_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Cloud SQL instance ID. This does not include the project ID."##),
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
            ("tiers", "methods: 'list'", vec![
            ("list",
                    Some(r##"Lists all available machine types (tiers) for Cloud SQL, for example, `db-custom-1-3840`. For more information, see https://cloud.google.com/sql/pricing."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/tiers_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project for which to list tiers."##),
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
            ("users", "methods: 'delete', 'get', 'insert', 'list' and 'update'", vec![
            ("delete",
                    Some(r##"Deletes a user from a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/users_delete",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
            ("get",
                    Some(r##"Retrieves a resource containing information about a user."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/users_get",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"name"##),
                     None,
                     Some(r##"User of the instance."##),
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
            ("insert",
                    Some(r##"Creates a new user in a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/users_insert",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
            ("list",
                    Some(r##"Lists users in the specified Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/users_list",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
            ("update",
                    Some(r##"Updates an existing user in a Cloud SQL instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_sqladmin1_cli/users_update",
                  vec![
                    (Some(r##"project"##),
                     None,
                     Some(r##"Project ID of the project that contains the instance."##),
                     Some(true),
                     Some(false)),
                    (Some(r##"instance"##),
                     None,
                     Some(r##"Database instance ID. This does not include the project ID."##),
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
            ]),
        ];

    let mut app = App::new("sqladmin1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("7.0.0+20251201")
           .about("API for Cloud SQL database instance management")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_sqladmin1_cli")
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
