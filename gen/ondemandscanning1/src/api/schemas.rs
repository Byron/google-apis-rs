use super::*;
/// An alias to a repo revision.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AliasContext {
    /// The alias kind.
    
    pub kind: Option<AliasContextKindEnum>,
    /// The alias name.
    
    pub name: Option<String>,
}

impl client::Part for AliasContext {}


/// Indicates which analysis completed successfully. Multiple types of analysis can be performed on a single resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisCompleted {
    /// no description provided
    #[serde(rename="analysisType")]
    
    pub analysis_type: Option<Vec<String>>,
}

impl client::Part for AnalysisCompleted {}


/// AnalyzePackagesRequest is the request to analyze a list of packages and create Vulnerability Occurrences for it.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations scans analyze packages projects](ProjectLocationScanAnalyzePackageCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzePackagesRequestV1 {
    /// [DEPRECATED] Whether to include OSV data in the scan. For backwards compatibility reasons, this field can be neither removed nor renamed.
    #[serde(rename="includeOsvData")]
    
    pub include_osv_data: Option<bool>,
    /// The packages to analyze.
    
    pub packages: Option<Vec<PackageData>>,
    /// Required. The resource URI of the container image being scanned.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
}

impl client::RequestValue for AnalyzePackagesRequestV1 {}


/// Artifact describes a build product.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
    /// Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container.
    
    pub checksum: Option<String>,
    /// Artifact ID, if any; for container images, this will be a URL by digest like `gcr.io/projectID/imagename@sha256:123456`.
    
    pub id: Option<String>,
    /// Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image.
    
    pub names: Option<Vec<String>>,
}

impl client::Part for Artifact {}


/// Occurrence that represents a single "attestation". The authenticity of an attestation can be verified using the attached signature. If the verifier trusts the public key of the signer, then verifying the signature is sufficient to establish trust. In this circumstance, the authority to which this attestation is attached is primarily useful for lookup (how to find this attestation if you already know the authority and artifact to be verified) and intent (for which authority this attestation was intended to sign.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttestationOccurrence {
    /// One or more JWTs encoding a self-contained attestation. Each JWT encodes the payload that it verifies within the JWT itself. Verifier implementation SHOULD ignore the `serialized_payload` field when verifying these JWTs. If only JWTs are present on this AttestationOccurrence, then the `serialized_payload` SHOULD be left empty. Each JWT SHOULD encode a claim specific to the `resource_uri` of this Occurrence, but this is not validated by Grafeas metadata API implementations. The JWT itself is opaque to Grafeas.
    
    pub jwts: Option<Vec<Jwt>>,
    /// Required. The serialized payload that is verified by one or more `signatures`.
    #[serde(rename="serializedPayload")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub serialized_payload: Option<Vec<u8>>,
    /// One or more signatures over `serialized_payload`. Verifier implementations should consider this attestation message verified if at least one `signature` verifies `serialized_payload`. See `Signature` in common.proto for more details on signature structure and verification.
    
    pub signatures: Option<Vec<Signature>>,
}

impl client::Part for AttestationOccurrence {}


/// Details of a build occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildOccurrence {
    /// Deprecated. See InTotoStatement for the replacement. In-toto Provenance representation as defined in spec.
    #[serde(rename="intotoProvenance")]
    
    pub intoto_provenance: Option<InTotoProvenance>,
    /// In-toto Statement representation as defined in spec. The intoto_statement can contain any type of provenance. The serialized payload of the statement can be stored and signed in the Occurrence's envelope.
    #[serde(rename="intotoStatement")]
    
    pub intoto_statement: Option<InTotoStatement>,
    /// The actual provenance for the build.
    
    pub provenance: Option<BuildProvenance>,
    /// Serialized JSON representation of the provenance, used in generating the build signature in the corresponding build note. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes.
    #[serde(rename="provenanceBytes")]
    
    pub provenance_bytes: Option<String>,
}

impl client::Part for BuildOccurrence {}


/// Provenance of a build. Contains all information needed to verify the full details about the build from source to completion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildProvenance {
    /// Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details.
    #[serde(rename="buildOptions")]
    
    pub build_options: Option<HashMap<String, String>>,
    /// Version string of the builder at the time this build was executed.
    #[serde(rename="builderVersion")]
    
    pub builder_version: Option<String>,
    /// Output of the build.
    #[serde(rename="builtArtifacts")]
    
    pub built_artifacts: Option<Vec<Artifact>>,
    /// Commands requested by the build.
    
    pub commands: Option<Vec<Command>>,
    /// Time at which the build was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// E-mail address of the user who initiated this build. Note that this was the user's e-mail address at the time the build was initiated; this address may not represent the same end-user for all time.
    
    pub creator: Option<String>,
    /// Time at which execution of the build was finished.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Unique identifier of the build.
    
    pub id: Option<String>,
    /// URI where any logs for this provenance were written.
    #[serde(rename="logsUri")]
    
    pub logs_uri: Option<String>,
    /// ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Details of the Source input to the build.
    #[serde(rename="sourceProvenance")]
    
    pub source_provenance: Option<Source>,
    /// Time at which execution of the build was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Trigger identifier if the build was triggered automatically; empty if not.
    #[serde(rename="triggerId")]
    
    pub trigger_id: Option<String>,
}

impl client::Part for BuildProvenance {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuilderConfig {
    /// no description provided
    
    pub id: Option<String>,
}

impl client::Part for BuilderConfig {}


/// Common Vulnerability Scoring System. For details, see https://www.first.org/cvss/specification-document This is a message we will try to use for storing various versions of CVSS rather than making a separate proto for storing a specific version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CVSS {
    /// no description provided
    #[serde(rename="attackComplexity")]
    
    pub attack_complexity: Option<CVSAttackComplexityEnum>,
    /// Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments.
    #[serde(rename="attackVector")]
    
    pub attack_vector: Option<CVSAttackVectorEnum>,
    /// no description provided
    
    pub authentication: Option<CVSAuthenticationEnum>,
    /// no description provided
    #[serde(rename="availabilityImpact")]
    
    pub availability_impact: Option<CVSAvailabilityImpactEnum>,
    /// The base score is a function of the base metric scores.
    #[serde(rename="baseScore")]
    
    pub base_score: Option<f32>,
    /// no description provided
    #[serde(rename="confidentialityImpact")]
    
    pub confidentiality_impact: Option<CVSConfidentialityImpactEnum>,
    /// no description provided
    #[serde(rename="exploitabilityScore")]
    
    pub exploitability_score: Option<f32>,
    /// no description provided
    #[serde(rename="impactScore")]
    
    pub impact_score: Option<f32>,
    /// no description provided
    #[serde(rename="integrityImpact")]
    
    pub integrity_impact: Option<CVSIntegrityImpactEnum>,
    /// no description provided
    #[serde(rename="privilegesRequired")]
    
    pub privileges_required: Option<CVSPrivilegesRequiredEnum>,
    /// no description provided
    
    pub scope: Option<CVSScopeEnum>,
    /// no description provided
    #[serde(rename="userInteraction")]
    
    pub user_interaction: Option<CVSUserInteractionEnum>,
}

impl client::Part for CVSS {}


/// The category to which the update belongs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    /// The identifier of the category.
    #[serde(rename="categoryId")]
    
    pub category_id: Option<String>,
    /// The localized name of the category.
    
    pub name: Option<String>,
}

impl client::Part for Category {}


/// A CloudRepoSourceContext denotes a particular revision in a Google Cloud Source Repo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRepoSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    
    pub alias_context: Option<AliasContext>,
    /// The ID of the repo.
    #[serde(rename="repoId")]
    
    pub repo_id: Option<RepoId>,
    /// A revision ID.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for CloudRepoSourceContext {}


/// Command describes a step performed as part of the build pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Command {
    /// Command-line arguments used when executing this command.
    
    pub args: Option<Vec<String>>,
    /// Working directory (relative to project source root) used when running this command.
    
    pub dir: Option<String>,
    /// Environment variables set before running this command.
    
    pub env: Option<Vec<String>>,
    /// Optional unique identifier for this command, used in wait_for to reference this command as a dependency.
    
    pub id: Option<String>,
    /// Required. Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`.
    
    pub name: Option<String>,
    /// The ID(s) of the command(s) that this command depends on.
    #[serde(rename="waitFor")]
    
    pub wait_for: Option<Vec<String>>,
}

impl client::Part for Command {}


/// Indicates that the builder claims certain fields in this message to be complete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Completeness {
    /// If true, the builder claims that recipe.arguments is complete, meaning that all external inputs are properly captured in the recipe.
    
    pub arguments: Option<bool>,
    /// If true, the builder claims that recipe.environment is claimed to be complete.
    
    pub environment: Option<bool>,
    /// If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called "hermetic".
    
    pub materials: Option<bool>,
}

impl client::Part for Completeness {}


/// An indication that the compliance checks in the associated ComplianceNote were not satisfied for particular resources or a specified reason.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceOccurrence {
    /// no description provided
    #[serde(rename="nonComplianceReason")]
    
    pub non_compliance_reason: Option<String>,
    /// no description provided
    #[serde(rename="nonCompliantFiles")]
    
    pub non_compliant_files: Option<Vec<NonCompliantFile>>,
}

impl client::Part for ComplianceOccurrence {}


/// Deprecated. Prefer to use a regular Occurrence, and populate the Envelope at the top level of the Occurrence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DSSEAttestationOccurrence {
    /// If doing something security critical, make sure to verify the signatures in this metadata.
    
    pub envelope: Option<Envelope>,
    /// no description provided
    
    pub statement: Option<InTotoStatement>,
}

impl client::Part for DSSEAttestationOccurrence {}


/// The period during which some deployable was active in a runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentOccurrence {
    /// Address of the runtime element hosting this deployment.
    
    pub address: Option<String>,
    /// Configuration used to create this deployment.
    
    pub config: Option<String>,
    /// Required. Beginning of the lifetime of this deployment.
    #[serde(rename="deployTime")]
    
    pub deploy_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Platform hosting this deployment.
    
    pub platform: Option<DeploymentOccurrencePlatformEnum>,
    /// Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<Vec<String>>,
    /// End of the lifetime of this deployment.
    #[serde(rename="undeployTime")]
    
    pub undeploy_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identity of the user that triggered this deployment.
    #[serde(rename="userEmail")]
    
    pub user_email: Option<String>,
}

impl client::Part for DeploymentOccurrence {}


/// Provides information about the analysis status of a discovered resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoveryOccurrence {
    /// no description provided
    #[serde(rename="analysisCompleted")]
    
    pub analysis_completed: Option<AnalysisCompleted>,
    /// Indicates any errors encountered during analysis of a resource. There could be 0 or more of these errors.
    #[serde(rename="analysisError")]
    
    pub analysis_error: Option<Vec<Status>>,
    /// The status of discovery for the resource.
    #[serde(rename="analysisStatus")]
    
    pub analysis_status: Option<DiscoveryOccurrenceAnalysisStatusEnum>,
    /// When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage is output only and populated by the API.
    #[serde(rename="analysisStatusError")]
    
    pub analysis_status_error: Option<Status>,
    /// Output only. The time occurrences related to this discovery occurrence were archived.
    #[serde(rename="archiveTime")]
    
    pub archive_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Whether the resource is continuously analyzed.
    #[serde(rename="continuousAnalysis")]
    
    pub continuous_analysis: Option<DiscoveryOccurrenceContinuousAnalysisEnum>,
    /// The CPE of the resource being scanned.
    
    pub cpe: Option<String>,
    /// The last time this resource was scanned.
    #[serde(rename="lastScanTime")]
    
    pub last_scan_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for DiscoveryOccurrence {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// MUST match https://github.com/secure-systems-lab/dsse/blob/master/envelope.proto. An authenticated message of arbitrary type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Envelope {
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub payload: Option<Vec<u8>>,
    /// no description provided
    #[serde(rename="payloadType")]
    
    pub payload_type: Option<String>,
    /// no description provided
    
    pub signatures: Option<Vec<EnvelopeSignature>>,
}

impl client::Part for Envelope {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeSignature {
    /// no description provided
    
    pub keyid: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sig: Option<Vec<u8>>,
}

impl client::Part for EnvelopeSignature {}


/// Container message for hashes of byte content of files, used in source messages to verify integrity of source input to the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileHashes {
    /// Required. Collection of file hashes.
    #[serde(rename="fileHash")]
    
    pub file_hash: Option<Vec<Hash>>,
}

impl client::Part for FileHashes {}


/// Indicates the location at which a package was found.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileLocation {
    /// For jars that are contained inside .war files, this filepath can indicate the path to war file combined with the path to jar file.
    #[serde(rename="filePath")]
    
    pub file_path: Option<String>,
}

impl client::Part for FileLocation {}


/// A set of properties that uniquely identify a given Docker image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Fingerprint {
    /// Required. The layer ID of the final layer in the Docker image's v1 representation.
    #[serde(rename="v1Name")]
    
    pub v1_name: Option<String>,
    /// Required. The ordered list of v2 blobs that represent a given image.
    #[serde(rename="v2Blob")]
    
    pub v2_blob: Option<Vec<String>>,
    /// Output only. The name of the image's v2 blobs computed via: [bottom] := v2_blobbottom := sha256(v2_blob[N] + " " + v2_name[N+1]) Only the name of the final blob is kept.
    #[serde(rename="v2Name")]
    
    pub v2_name: Option<String>,
}

impl client::Part for Fingerprint {}


/// A SourceContext referring to a Gerrit project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GerritSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    
    pub alias_context: Option<AliasContext>,
    /// The full project name within the host. Projects may be nested, so "project/subproject" is a valid project name. The "repo name" is the hostURI/project.
    #[serde(rename="gerritProject")]
    
    pub gerrit_project: Option<String>,
    /// The URI of a running Gerrit instance.
    #[serde(rename="hostUri")]
    
    pub host_uri: Option<String>,
    /// A revision (commit) ID.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for GerritSourceContext {}


/// A GitSourceContext denotes a particular revision in a third party Git repository (e.g., GitHub).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitSourceContext {
    /// Git commit hash.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Git repository URL.
    
    pub url: Option<String>,
}

impl client::Part for GitSourceContext {}


/// Indicates the location at which a package was found.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1FileLocation {
    /// For jars that are contained inside .war files, this filepath can indicate the path to war file combined with the path to jar file.
    #[serde(rename="filePath")]
    
    pub file_path: Option<String>,
}

impl client::Part for GrafeasV1FileLocation {}


/// Identifies the entity that executed the recipe, which is trusted to have correctly performed the operation and populated this provenance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1SlsaProvenanceZeroTwoSlsaBuilder {
    /// no description provided
    
    pub id: Option<String>,
}

impl client::Part for GrafeasV1SlsaProvenanceZeroTwoSlsaBuilder {}


/// Indicates that the builder claims certain fields in this message to be complete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1SlsaProvenanceZeroTwoSlsaCompleteness {
    /// no description provided
    
    pub environment: Option<bool>,
    /// no description provided
    
    pub materials: Option<bool>,
    /// no description provided
    
    pub parameters: Option<bool>,
}

impl client::Part for GrafeasV1SlsaProvenanceZeroTwoSlsaCompleteness {}


/// Describes where the config file that kicked off the build came from. This is effectively a pointer to the source where buildConfig came from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1SlsaProvenanceZeroTwoSlsaConfigSource {
    /// no description provided
    
    pub digest: Option<HashMap<String, String>>,
    /// no description provided
    #[serde(rename="entryPoint")]
    
    pub entry_point: Option<String>,
    /// no description provided
    
    pub uri: Option<String>,
}

impl client::Part for GrafeasV1SlsaProvenanceZeroTwoSlsaConfigSource {}


/// Identifies the event that kicked off the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1SlsaProvenanceZeroTwoSlsaInvocation {
    /// no description provided
    #[serde(rename="configSource")]
    
    pub config_source: Option<GrafeasV1SlsaProvenanceZeroTwoSlsaConfigSource>,
    /// no description provided
    
    pub environment: Option<HashMap<String, json::Value>>,
    /// no description provided
    
    pub parameters: Option<HashMap<String, json::Value>>,
}

impl client::Part for GrafeasV1SlsaProvenanceZeroTwoSlsaInvocation {}


/// The collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1SlsaProvenanceZeroTwoSlsaMaterial {
    /// no description provided
    
    pub digest: Option<HashMap<String, String>>,
    /// no description provided
    
    pub uri: Option<String>,
}

impl client::Part for GrafeasV1SlsaProvenanceZeroTwoSlsaMaterial {}


/// Other properties of the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GrafeasV1SlsaProvenanceZeroTwoSlsaMetadata {
    /// no description provided
    #[serde(rename="buildFinishedOn")]
    
    pub build_finished_on: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// no description provided
    #[serde(rename="buildInvocationId")]
    
    pub build_invocation_id: Option<String>,
    /// no description provided
    #[serde(rename="buildStartedOn")]
    
    pub build_started_on: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// no description provided
    
    pub completeness: Option<GrafeasV1SlsaProvenanceZeroTwoSlsaCompleteness>,
    /// no description provided
    
    pub reproducible: Option<bool>,
}

impl client::Part for GrafeasV1SlsaProvenanceZeroTwoSlsaMetadata {}


/// Container message for hash values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hash {
    /// Required. The type of hash that was performed, e.g. "SHA-256".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Required. The hash value.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub value: Option<Vec<u8>>,
}

impl client::Part for Hash {}


/// The unique identifier of the update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    /// The revision number of the update.
    
    pub revision: Option<i32>,
    /// The revision independent identifier of the update.
    #[serde(rename="updateId")]
    
    pub update_id: Option<String>,
}

impl client::Part for Identity {}


/// Details of the derived image portion of the DockerImage relationship. This image would be produced from a Dockerfile with FROM .
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageOccurrence {
    /// Output only. This contains the base image URL for the derived image occurrence.
    #[serde(rename="baseResourceUrl")]
    
    pub base_resource_url: Option<String>,
    /// Output only. The number of layers by which this image differs from the associated image basis.
    
    pub distance: Option<i32>,
    /// Required. The fingerprint of the derived image.
    
    pub fingerprint: Option<Fingerprint>,
    /// This contains layer-specific metadata, if populated it has length "distance" and is ordered with [distance] being the layer immediately following the base image and [1] being the final layer.
    #[serde(rename="layerInfo")]
    
    pub layer_info: Option<Vec<Layer>>,
}

impl client::Part for ImageOccurrence {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InTotoProvenance {
    /// required
    #[serde(rename="builderConfig")]
    
    pub builder_config: Option<BuilderConfig>,
    /// The collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on. This is considered to be incomplete unless metadata.completeness.materials is true. Unset or null is equivalent to empty.
    
    pub materials: Option<Vec<String>>,
    /// no description provided
    
    pub metadata: Option<Metadata>,
    /// Identifies the configuration used for the build. When combined with materials, this SHOULD fully describe the build, such that re-running this recipe results in bit-for-bit identical output (if the build is reproducible). required
    
    pub recipe: Option<Recipe>,
}

impl client::Part for InTotoProvenance {}


/// Spec defined at https://github.com/in-toto/attestation/tree/main/spec#statement The serialized InTotoStatement will be stored as Envelope.payload. Envelope.payloadType is always "application/vnd.in-toto+json".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InTotoStatement {
    /// Always `https://in-toto.io/Statement/v0.1`.
    
    pub _type: Option<String>,
    /// `https://slsa.dev/provenance/v0.1` for SlsaProvenance.
    #[serde(rename="predicateType")]
    
    pub predicate_type: Option<String>,
    /// no description provided
    
    pub provenance: Option<InTotoProvenance>,
    /// no description provided
    #[serde(rename="slsaProvenance")]
    
    pub slsa_provenance: Option<SlsaProvenance>,
    /// no description provided
    #[serde(rename="slsaProvenanceZeroTwo")]
    
    pub slsa_provenance_zero_two: Option<SlsaProvenanceZeroTwo>,
    /// no description provided
    
    pub subject: Option<Vec<Subject>>,
}

impl client::Part for InTotoStatement {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Jwt {
    /// The compact encoding of a JWS, which is always three base64 encoded strings joined by periods. For details, see: https://tools.ietf.org/html/rfc7515.html#section-3.1
    #[serde(rename="compactJwt")]
    
    pub compact_jwt: Option<String>,
}

impl client::Part for Jwt {}


/// Indicates a language package available between this package and the customer's resource artifact.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguagePackageDependency {
    /// no description provided
    
    pub package: Option<String>,
    /// no description provided
    
    pub version: Option<String>,
}

impl client::Part for LanguagePackageDependency {}


/// Layer holds metadata specific to a layer of a Docker image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    /// The recovered arguments to the Dockerfile directive.
    
    pub arguments: Option<String>,
    /// Required. The recovered Dockerfile directive used to construct this layer. See https://docs.docker.com/engine/reference/builder/ for more information.
    
    pub directive: Option<String>,
}

impl client::Part for Layer {}


/// License information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct License {
    /// Comments
    
    pub comments: Option<String>,
    /// Often a single license can be used to represent the licensing terms. Sometimes it is necessary to include a choice of one or more licenses or some combination of license identifiers. Examples: "LGPL-2.1-only OR MIT", "LGPL-2.1-only AND MIT", "GPL-2.0-or-later WITH Bison-exception-2.2".
    
    pub expression: Option<String>,
}

impl client::Part for License {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// ListVulnerabilitiesResponse contains a single page of vulnerabilities resulting from a scan.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations scans vulnerabilities list projects](ProjectLocationScanVulnerabilityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListVulnerabilitiesResponseV1 {
    /// A page token that can be used in a subsequent call to ListVulnerabilities to continue retrieving results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Vulnerability Occurrences resulting from a scan.
    
    pub occurrences: Option<Vec<Occurrence>>,
}

impl client::ResponseResult for ListVulnerabilitiesResponseV1 {}


/// An occurrence of a particular package installation found within a system's filesystem. E.g., glibc was found in `/var/lib/dpkg/status`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Deprecated. The CPE URI in [CPE format](https://cpe.mitre.org/specification/)
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// The path from which we gathered that this package/version is installed.
    
    pub path: Option<String>,
    /// Deprecated. The version installed at this location.
    
    pub version: Option<Version>,
}

impl client::Part for Location {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    /// no description provided
    
    pub digest: Option<HashMap<String, String>>,
    /// no description provided
    
    pub uri: Option<String>,
}

impl client::Part for Material {}


/// Other properties of the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The timestamp of when the build completed.
    #[serde(rename="buildFinishedOn")]
    
    pub build_finished_on: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifies the particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The value SHOULD be globally unique, per in-toto Provenance spec.
    #[serde(rename="buildInvocationId")]
    
    pub build_invocation_id: Option<String>,
    /// The timestamp of when the build started.
    #[serde(rename="buildStartedOn")]
    
    pub build_started_on: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Indicates that the builder claims certain fields in this message to be complete.
    
    pub completeness: Option<Completeness>,
    /// If true, the builder claims that running the recipe on materials will produce bit-for-bit identical output.
    
    pub reproducible: Option<bool>,
}

impl client::Part for Metadata {}


/// Details about files that caused a compliance check to fail. display_command is a single command that can be used to display a list of non compliant files. When there is no such command, we can also iterate a list of non compliant file using 'path'.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NonCompliantFile {
    /// Command to display the non-compliant files.
    #[serde(rename="displayCommand")]
    
    pub display_command: Option<String>,
    /// Empty if `display_command` is set.
    
    pub path: Option<String>,
    /// Explains why a file is non compliant for a CIS check.
    
    pub reason: Option<String>,
}

impl client::Part for NonCompliantFile {}


/// An instance of an analysis type that has been found on a resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Occurrence {
    /// Describes an attestation of an artifact.
    
    pub attestation: Option<AttestationOccurrence>,
    /// Describes a verifiable build.
    
    pub build: Option<BuildOccurrence>,
    /// Describes a compliance violation on a linked resource.
    
    pub compliance: Option<ComplianceOccurrence>,
    /// Output only. The time this occurrence was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Describes the deployment of an artifact on a runtime.
    
    pub deployment: Option<DeploymentOccurrence>,
    /// Describes when a resource was discovered.
    
    pub discovery: Option<DiscoveryOccurrence>,
    /// Describes an attestation of an artifact using dsse.
    #[serde(rename="dsseAttestation")]
    
    pub dsse_attestation: Option<DSSEAttestationOccurrence>,
    /// https://github.com/secure-systems-lab/dsse
    
    pub envelope: Option<Envelope>,
    /// Describes how this resource derives from the basis in the associated note.
    
    pub image: Option<ImageOccurrence>,
    /// Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests.
    
    pub kind: Option<OccurrenceKindEnum>,
    /// Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    
    pub name: Option<String>,
    /// Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests.
    #[serde(rename="noteName")]
    
    pub note_name: Option<String>,
    /// Describes the installation of a package on the linked resource.
    
    pub package: Option<PackageOccurrence>,
    /// A description of actions that can be taken to remedy the note.
    
    pub remediation: Option<String>,
    /// Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
    /// Output only. The time this occurrence was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Describes an available package upgrade on the linked resource.
    
    pub upgrade: Option<UpgradeOccurrence>,
    /// Describes a security vulnerability.
    
    pub vulnerability: Option<VulnerabilityOccurrence>,
}

impl client::Part for Occurrence {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations operations wait projects](ProjectLocationOperationWaitCall) (response)
/// * [locations scans analyze packages projects](ProjectLocationScanAnalyzePackageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageData {
    /// The cpe_uri in [cpe format] (https://cpe.mitre.org/specification/) in which the vulnerability may manifest. Examples include distro or storage location for vulnerable jar.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// The dependency chain between this package and the user's artifact. List in order from the customer's package under review first, to the current package last. Inclusive of the original package and the current package.
    #[serde(rename="dependencyChain")]
    
    pub dependency_chain: Option<Vec<LanguagePackageDependency>>,
    /// The path to the jar file / go binary file.
    #[serde(rename="fileLocation")]
    
    pub file_location: Option<Vec<FileLocation>>,
    /// HashDigest stores the SHA512 hash digest of the jar file if the package is of type Maven. This field will be unset for non Maven packages.
    #[serde(rename="hashDigest")]
    
    pub hash_digest: Option<String>,
    /// The OS affected by a vulnerability Used to generate the cpe_uri for OS packages
    
    pub os: Option<String>,
    /// The version of the OS Used to generate the cpe_uri for OS packages
    #[serde(rename="osVersion")]
    
    pub os_version: Option<String>,
    /// The package being analysed for vulnerabilities
    
    pub package: Option<String>,
    /// The type of package: os, maven, go, etc.
    #[serde(rename="packageType")]
    
    pub package_type: Option<PackageDataPackageTypeEnum>,
    /// CVEs that this package is no longer vulnerable to go/drydock-dd-custom-binary-scanning
    #[serde(rename="patchedCve")]
    
    pub patched_cve: Option<Vec<String>>,
    /// no description provided
    
    pub unused: Option<String>,
    /// The version of the package being analysed
    
    pub version: Option<String>,
}

impl client::Part for PackageData {}


/// A detail for a distro and package this vulnerability occurrence was found in and its associated fix (if one is available).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageIssue {
    /// Required. The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was found in.
    #[serde(rename="affectedCpeUri")]
    
    pub affected_cpe_uri: Option<String>,
    /// Required. The package this vulnerability was found in.
    #[serde(rename="affectedPackage")]
    
    pub affected_package: Option<String>,
    /// Required. The version of the package that is installed on the resource affected by this vulnerability.
    #[serde(rename="affectedVersion")]
    
    pub affected_version: Option<Version>,
    /// Output only. The distro or language system assigned severity for this vulnerability when that is available and note provider assigned severity when it is not available.
    #[serde(rename="effectiveSeverity")]
    
    pub effective_severity: Option<PackageIssueEffectiveSeverityEnum>,
    /// The location at which this package was found.
    #[serde(rename="fileLocation")]
    
    pub file_location: Option<Vec<GrafeasV1FileLocation>>,
    /// Output only. Whether a fix is available for this package.
    #[serde(rename="fixAvailable")]
    
    pub fix_available: Option<bool>,
    /// The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was fixed in. It is possible for this to be different from the affected_cpe_uri.
    #[serde(rename="fixedCpeUri")]
    
    pub fixed_cpe_uri: Option<String>,
    /// The package this vulnerability was fixed in. It is possible for this to be different from the affected_package.
    #[serde(rename="fixedPackage")]
    
    pub fixed_package: Option<String>,
    /// Required. The version of the package this vulnerability was fixed in. Setting this to VersionKind.MAXIMUM means no fix is yet available.
    #[serde(rename="fixedVersion")]
    
    pub fixed_version: Option<Version>,
    /// The type of package (e.g. OS, MAVEN, GO).
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
}

impl client::Part for PackageIssue {}


/// Details on how a particular software package was installed on a system.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PackageOccurrence {
    /// Output only. The CPU architecture for which packages in this distribution channel were built. Architecture will be blank for language packages.
    
    pub architecture: Option<PackageOccurrenceArchitectureEnum>,
    /// Output only. The cpe_uri in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package. The cpe_uri will be blank for language packages.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// Licenses that have been declared by the authors of the package.
    
    pub license: Option<License>,
    /// All of the places within the filesystem versions of this package have been found.
    
    pub location: Option<Vec<Location>>,
    /// Required. Output only. The name of the installed package.
    
    pub name: Option<String>,
    /// Output only. The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.).
    #[serde(rename="packageType")]
    
    pub package_type: Option<String>,
    /// Output only. The version of the package.
    
    pub version: Option<Version>,
}

impl client::Part for PackageOccurrence {}


/// Selects a repo using a Google Cloud Platform project ID (e.g., winged-cargo-31) and a repo name within that project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectRepoId {
    /// The ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The name of the repo. Leave empty for the default repo.
    #[serde(rename="repoName")]
    
    pub repo_name: Option<String>,
}

impl client::Part for ProjectRepoId {}


/// Steps taken to build the artifact. For a TaskRun, typically each container corresponds to one step in the recipe.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    /// Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were "make", then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint. Since the arguments field can greatly vary in structure, depending on the builder and recipe type, this is of form "Any".
    
    pub arguments: Option<Vec<HashMap<String, json::Value>>>,
    /// Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were "make", then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn't come from a material, as zero is default unset value for int64.
    #[serde(rename="definedInMaterial")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub defined_in_material: Option<i64>,
    /// String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were "make", then this would reference the directory in which to run make as well as which target to use.
    #[serde(rename="entryPoint")]
    
    pub entry_point: Option<String>,
    /// Any other builder-controlled inputs necessary for correctly evaluating the recipe. Usually only needed for reproducing the build but not evaluated as part of policy. Since the environment field can greatly vary in structure, depending on the builder and recipe type, this is of form "Any".
    
    pub environment: Option<Vec<HashMap<String, json::Value>>>,
    /// URI indicating what type of recipe was performed. It determines the meaning of recipe.entryPoint, recipe.arguments, recipe.environment, and materials.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Recipe {}


/// Metadata for any related URL information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelatedUrl {
    /// Label to describe usage of the URL.
    
    pub label: Option<String>,
    /// Specific URL associated with the resource.
    
    pub url: Option<String>,
}

impl client::Part for RelatedUrl {}


/// A unique identifier for a Cloud Repo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepoId {
    /// A combination of a project ID and a repo name.
    #[serde(rename="projectRepoId")]
    
    pub project_repo_id: Option<ProjectRepoId>,
    /// A server-assigned, globally unique identifier.
    
    pub uid: Option<String>,
}

impl client::Part for RepoId {}


/// Verifiers (e.g. Kritis implementations) MUST verify signatures with respect to the trust anchors defined in policy (e.g. a Kritis policy). Typically this means that the verifier has been configured with a map from `public_key_id` to public key material (and any required parameters, e.g. signing algorithm). In particular, verification implementations MUST NOT treat the signature `public_key_id` as anything more than a key lookup hint. The `public_key_id` DOES NOT validate or authenticate a public key; it only provides a mechanism for quickly selecting a public key ALREADY CONFIGURED on the verifier through a trusted channel. Verification implementations MUST reject signatures in any of the following circumstances: * The `public_key_id` is not recognized by the verifier. * The public key that `public_key_id` refers to does not verify the signature with respect to the payload. The `signature` contents SHOULD NOT be "attached" (where the payload is included with the serialized `signature` bytes). Verifiers MUST ignore any "attached" payload and only verify signatures with respect to explicitly provided payload (e.g. a `payload` field on the proto message that holds this Signature, or the canonical serialization of the proto message that holds this signature).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    /// The identifier for the public key that verifies this signature. * The `public_key_id` is required. * The `public_key_id` SHOULD be an RFC3986 conformant URI. * When possible, the `public_key_id` SHOULD be an immutable reference, such as a cryptographic digest. Examples of valid `public_key_id`s: OpenPGP V4 public key fingerprint: * "openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA" See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more details on this scheme. RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization): * "ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU" * "nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5"
    #[serde(rename="publicKeyId")]
    
    pub public_key_id: Option<String>,
    /// The content of the signature, an opaque bytestring. The payload that this signature verifies MUST be unambiguously provided with the Signature during verification. A wrapper message might provide the payload explicitly. Alternatively, a message might have a canonical serialization that can always be unambiguously computed to derive the payload.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub signature: Option<Vec<u8>>,
}

impl client::Part for Signature {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlsaBuilder {
    /// no description provided
    
    pub id: Option<String>,
}

impl client::Part for SlsaBuilder {}


/// Indicates that the builder claims certain fields in this message to be complete.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlsaCompleteness {
    /// If true, the builder claims that recipe.arguments is complete, meaning that all external inputs are properly captured in the recipe.
    
    pub arguments: Option<bool>,
    /// If true, the builder claims that recipe.environment is claimed to be complete.
    
    pub environment: Option<bool>,
    /// If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called "hermetic".
    
    pub materials: Option<bool>,
}

impl client::Part for SlsaCompleteness {}


/// Other properties of the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlsaMetadata {
    /// The timestamp of when the build completed.
    #[serde(rename="buildFinishedOn")]
    
    pub build_finished_on: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Identifies the particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The value SHOULD be globally unique, per in-toto Provenance spec.
    #[serde(rename="buildInvocationId")]
    
    pub build_invocation_id: Option<String>,
    /// The timestamp of when the build started.
    #[serde(rename="buildStartedOn")]
    
    pub build_started_on: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Indicates that the builder claims certain fields in this message to be complete.
    
    pub completeness: Option<SlsaCompleteness>,
    /// If true, the builder claims that running the recipe on materials will produce bit-for-bit identical output.
    
    pub reproducible: Option<bool>,
}

impl client::Part for SlsaMetadata {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlsaProvenance {
    /// required
    
    pub builder: Option<SlsaBuilder>,
    /// The collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on. This is considered to be incomplete unless metadata.completeness.materials is true. Unset or null is equivalent to empty.
    
    pub materials: Option<Vec<Material>>,
    /// no description provided
    
    pub metadata: Option<SlsaMetadata>,
    /// Identifies the configuration used for the build. When combined with materials, this SHOULD fully describe the build, such that re-running this recipe results in bit-for-bit identical output (if the build is reproducible). required
    
    pub recipe: Option<SlsaRecipe>,
}

impl client::Part for SlsaProvenance {}


/// See full explanation of fields at slsa.dev/provenance/v0.2.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlsaProvenanceZeroTwo {
    /// no description provided
    #[serde(rename="buildConfig")]
    
    pub build_config: Option<HashMap<String, json::Value>>,
    /// no description provided
    #[serde(rename="buildType")]
    
    pub build_type: Option<String>,
    /// no description provided
    
    pub builder: Option<GrafeasV1SlsaProvenanceZeroTwoSlsaBuilder>,
    /// no description provided
    
    pub invocation: Option<GrafeasV1SlsaProvenanceZeroTwoSlsaInvocation>,
    /// no description provided
    
    pub materials: Option<Vec<GrafeasV1SlsaProvenanceZeroTwoSlsaMaterial>>,
    /// no description provided
    
    pub metadata: Option<GrafeasV1SlsaProvenanceZeroTwoSlsaMetadata>,
}

impl client::Part for SlsaProvenanceZeroTwo {}


/// Steps taken to build the artifact. For a TaskRun, typically each container corresponds to one step in the recipe.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlsaRecipe {
    /// Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were "make", then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint. Depending on the recipe Type, the structure may be different.
    
    pub arguments: Option<HashMap<String, json::Value>>,
    /// Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were "make", then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn't come from a material, as zero is default unset value for int64.
    #[serde(rename="definedInMaterial")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub defined_in_material: Option<i64>,
    /// String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were "make", then this would reference the directory in which to run make as well as which target to use.
    #[serde(rename="entryPoint")]
    
    pub entry_point: Option<String>,
    /// Any other builder-controlled inputs necessary for correctly evaluating the recipe. Usually only needed for reproducing the build but not evaluated as part of policy. Depending on the recipe Type, the structure may be different.
    
    pub environment: Option<HashMap<String, json::Value>>,
    /// URI indicating what type of recipe was performed. It determines the meaning of recipe.entryPoint, recipe.arguments, recipe.environment, and materials.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for SlsaRecipe {}


/// Source describes the location of the source used for the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field.
    #[serde(rename="additionalContexts")]
    
    pub additional_contexts: Option<Vec<SourceContext>>,
    /// If provided, the input binary artifacts for the build came from this location.
    #[serde(rename="artifactStorageSourceUri")]
    
    pub artifact_storage_source_uri: Option<String>,
    /// If provided, the source code used for the build came from this location.
    
    pub context: Option<SourceContext>,
    /// Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file.
    #[serde(rename="fileHashes")]
    
    pub file_hashes: Option<HashMap<String, FileHashes>>,
}

impl client::Part for Source {}


/// A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceContext {
    /// A SourceContext referring to a revision in a Google Cloud Source Repo.
    #[serde(rename="cloudRepo")]
    
    pub cloud_repo: Option<CloudRepoSourceContext>,
    /// A SourceContext referring to a Gerrit project.
    
    pub gerrit: Option<GerritSourceContext>,
    /// A SourceContext referring to any third party Git repo (e.g., GitHub).
    
    pub git: Option<GitSourceContext>,
    /// Labels with user defined metadata.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for SourceContext {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Subject {
    /// `"": ""` Algorithms can be e.g. sha256, sha512 See https://github.com/in-toto/attestation/blob/main/spec/field_types.md#DigestSet
    
    pub digest: Option<HashMap<String, String>>,
    /// no description provided
    
    pub name: Option<String>,
}

impl client::Part for Subject {}


/// The Upgrade Distribution represents metadata about the Upgrade for each operating system (CPE). Some distributions have additional metadata around updates, classifying them into various categories and severities.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeDistribution {
    /// The operating system classification of this Upgrade, as specified by the upstream operating system upgrade feed. For Windows the classification is one of the category_ids listed at https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ff357803(v=vs.85)
    
    pub classification: Option<String>,
    /// Required - The specific operating system this metadata applies to. See https://cpe.mitre.org/specification/.
    #[serde(rename="cpeUri")]
    
    pub cpe_uri: Option<String>,
    /// The cve tied to this Upgrade.
    
    pub cve: Option<Vec<String>>,
    /// The severity as specified by the upstream operating system.
    
    pub severity: Option<String>,
}

impl client::Part for UpgradeDistribution {}


/// An Upgrade Occurrence represents that a specific resource_url could install a specific upgrade. This presence is supplied via local sources (i.e. it is present in the mirror and the running system has noticed its availability). For Windows, both distribution and windows_update contain information for the Windows update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeOccurrence {
    /// Metadata about the upgrade for available for the specific operating system for the resource_url. This allows efficient filtering, as well as making it easier to use the occurrence.
    
    pub distribution: Option<UpgradeDistribution>,
    /// Required for non-Windows OS. The package this Upgrade is for.
    
    pub package: Option<String>,
    /// Required for non-Windows OS. The version of the package in a machine + human readable form.
    #[serde(rename="parsedVersion")]
    
    pub parsed_version: Option<Version>,
    /// Required for Windows OS. Represents the metadata about the Windows update.
    #[serde(rename="windowsUpdate")]
    
    pub windows_update: Option<WindowsUpdate>,
}

impl client::Part for UpgradeOccurrence {}


/// Version contains structured information about the version of a package.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// Used to correct mistakes in the version numbering scheme.
    
    pub epoch: Option<i32>,
    /// Human readable version string. This string is of the form :- and is only set when kind is NORMAL.
    #[serde(rename="fullName")]
    
    pub full_name: Option<String>,
    /// Whether this version is specifying part of an inclusive range. Grafeas does not have the capability to specify version ranges; instead we have fields that specify start version and end versions. At times this is insufficient - we also need to specify whether the version is included in the range or is excluded from the range. This boolean is expected to be set to true when the version is included in a range.
    
    pub inclusive: Option<bool>,
    /// Required. Distinguishes between sentinel MIN/MAX versions and normal versions.
    
    pub kind: Option<VersionKindEnum>,
    /// Required only when version kind is NORMAL. The main part of the version name.
    
    pub name: Option<String>,
    /// The iteration of the package build from the above version.
    
    pub revision: Option<String>,
}

impl client::Part for Version {}


/// An occurrence of a severity vulnerability on a resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VulnerabilityOccurrence {
    /// Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0 - 10 where 0 indicates low severity and 10 indicates high severity.
    #[serde(rename="cvssScore")]
    
    pub cvss_score: Option<f32>,
    /// Output only. CVSS version used to populate cvss_score and severity.
    #[serde(rename="cvssVersion")]
    
    pub cvss_version: Option<VulnerabilityOccurrenceCvssVersionEnum>,
    /// The cvss v3 score for the vulnerability.
    
    pub cvssv3: Option<CVSS>,
    /// The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity. When there are multiple PackageIssues for this vulnerability, they can have different effective severities because some might be provided by the distro while others are provided by the language ecosystem for a language pack. For this reason, it is advised to use the effective severity on the PackageIssue level. In the case where multiple PackageIssues have differing effective severities, this field should be the highest severity for any of the PackageIssues.
    #[serde(rename="effectiveSeverity")]
    
    pub effective_severity: Option<VulnerabilityOccurrenceEffectiveSeverityEnum>,
    /// Output only. Whether at least one of the affected packages has a fix available.
    #[serde(rename="fixAvailable")]
    
    pub fix_available: Option<bool>,
    /// Output only. A detailed description of this vulnerability.
    #[serde(rename="longDescription")]
    
    pub long_description: Option<String>,
    /// Required. The set of affected locations and their fixes (if available) within the associated resource.
    #[serde(rename="packageIssue")]
    
    pub package_issue: Option<Vec<PackageIssue>>,
    /// Output only. URLs related to this vulnerability.
    #[serde(rename="relatedUrls")]
    
    pub related_urls: Option<Vec<RelatedUrl>>,
    /// Output only. The note provider assigned severity of this vulnerability.
    
    pub severity: Option<VulnerabilityOccurrenceSeverityEnum>,
    /// Output only. A one sentence description of this vulnerability.
    #[serde(rename="shortDescription")]
    
    pub short_description: Option<String>,
    /// The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for VulnerabilityOccurrence {}


/// Windows Update represents the metadata about the update for the Windows operating system. The fields in this message come from the Windows Update API documented at https://docs.microsoft.com/en-us/windows/win32/api/wuapi/nn-wuapi-iupdate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WindowsUpdate {
    /// The list of categories to which the update belongs.
    
    pub categories: Option<Vec<Category>>,
    /// The localized description of the update.
    
    pub description: Option<String>,
    /// Required - The unique identifier for the update.
    
    pub identity: Option<Identity>,
    /// The Microsoft Knowledge Base article IDs that are associated with the update.
    #[serde(rename="kbArticleIds")]
    
    pub kb_article_ids: Option<Vec<String>>,
    /// The last published timestamp of the update.
    #[serde(rename="lastPublishedTimestamp")]
    
    pub last_published_timestamp: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The hyperlink to the support information for the update.
    #[serde(rename="supportUrl")]
    
    pub support_url: Option<String>,
    /// The localized title of the update.
    
    pub title: Option<String>,
}

impl client::Part for WindowsUpdate {}


