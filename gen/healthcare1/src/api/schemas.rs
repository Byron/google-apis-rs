use super::*;
/// Activates the latest revision of the specified Consent by committing a new revision with `state` updated to `ACTIVE`. If the latest revision of the given Consent is in the `ACTIVE` state, no new revision is committed. A FAILED_PRECONDITION error occurs if the latest revision of the given consent is in the `REJECTED` or `REVOKED` state.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consents activate projects](ProjectLocationDatasetConsentStoreConsentActivateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivateConsentRequest {
    /// Required. The resource name of the Consent artifact that contains documentation of the user's consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. If the draft Consent had a Consent artifact, this Consent artifact overwrites it.
    #[serde(rename="consentArtifact")]
    
    pub consent_artifact: Option<String>,
    /// Timestamp in UTC of when this Consent is considered expired.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time to live for this Consent from when it is marked as active.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
}

impl client::RequestValue for ActivateConsentRequest {}


/// The request to analyze healthcare entities in a document.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services nlp analyze entities projects](ProjectLocationServiceNlpAnalyzeEntityCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeEntitiesRequest {
    /// document_content is a document to be annotated.
    #[serde(rename="documentContent")]
    
    pub document_content: Option<String>,
    /// A list of licensed vocabularies to use in the request, in addition to the default unlicensed vocabularies.
    #[serde(rename="licensedVocabularies")]
    
    pub licensed_vocabularies: Option<Vec<AnalyzeEntitiesRequestLicensedVocabulariesEnum>>,
}

impl client::RequestValue for AnalyzeEntitiesRequest {}


/// Includes recognized entity mentions and relationships between them.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations services nlp analyze entities projects](ProjectLocationServiceNlpAnalyzeEntityCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeEntitiesResponse {
    /// The union of all the candidate entities that the entity_mentions in this response could link to. These are UMLS concepts or normalized mention content.
    
    pub entities: Option<Vec<Entity>>,
    /// entity_mentions contains all the annotated medical entities that were mentioned in the provided document.
    #[serde(rename="entityMentions")]
    
    pub entity_mentions: Option<Vec<EntityMention>>,
    /// relationships contains all the binary relationships that were identified between entity mentions within the provided document.
    
    pub relationships: Option<Vec<EntityMentionRelationship>>,
}

impl client::ResponseResult for AnalyzeEntitiesResponse {}


/// Archives the specified User data mapping.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores user data mappings archive projects](ProjectLocationDatasetConsentStoreUserDataMappingArchiveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArchiveUserDataMappingRequest { _never_set: Option<bool> }

impl client::RequestValue for ArchiveUserDataMappingRequest {}


/// Archives the specified User data mapping.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores user data mappings archive projects](ProjectLocationDatasetConsentStoreUserDataMappingArchiveCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArchiveUserDataMappingResponse { _never_set: Option<bool> }

impl client::ResponseResult for ArchiveUserDataMappingResponse {}


/// An attribute value for a Consent or User data mapping. Each Attribute must have a corresponding AttributeDefinition in the consent store that defines the default and allowed values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    /// Indicates the name of an attribute defined in the consent store.
    #[serde(rename="attributeDefinitionId")]
    
    pub attribute_definition_id: Option<String>,
    /// Required. The value of the attribute. Must be an acceptable value as defined in the consent store. For example, if the consent store defines "data type" with acceptable values "questionnaire" and "step-count", when the attribute name is data type, this field must contain one of those values.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for Attribute {}


/// A client-defined consent attribute.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores attribute definitions create projects](ProjectLocationDatasetConsentStoreAttributeDefinitionCreateCall) (request|response)
/// * [locations datasets consent stores attribute definitions get projects](ProjectLocationDatasetConsentStoreAttributeDefinitionGetCall) (response)
/// * [locations datasets consent stores attribute definitions patch projects](ProjectLocationDatasetConsentStoreAttributeDefinitionPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AttributeDefinition {
    /// Required. Possible values for the attribute. The number of allowed values must not exceed 500. An empty list is invalid. The list can only be expanded after creation.
    #[serde(rename="allowedValues")]
    
    pub allowed_values: Option<Vec<String>>,
    /// Required. The category of the attribute. The value of this field cannot be changed after creation.
    
    pub category: Option<AttributeDefinitionCategoryEnum>,
    /// Optional. Default values of the attribute in Consents. If no default values are specified, it defaults to an empty value.
    #[serde(rename="consentDefaultValues")]
    
    pub consent_default_values: Option<Vec<String>>,
    /// Optional. Default value of the attribute in User data mappings. If no default value is specified, it defaults to an empty value. This field is only applicable to attributes of the category `RESOURCE`.
    #[serde(rename="dataMappingDefaultValue")]
    
    pub data_mapping_default_value: Option<String>,
    /// Optional. A description of the attribute.
    
    pub description: Option<String>,
    /// Resource name of the Attribute definition, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/attributeDefinitions/{attribute_definition_id}`. Cannot be changed after creation.
    
    pub name: Option<String>,
}

impl client::RequestValue for AttributeDefinition {}
impl client::ResponseResult for AttributeDefinition {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<AuditLogConfigLogTypeEnum>,
}

impl client::Part for AuditLogConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets operations cancel projects](ProjectLocationDatasetOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Mask a string by replacing its characters with a fixed character.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CharacterMaskConfig {
    /// Character to mask the sensitive values. If not supplied, defaults to "*".
    #[serde(rename="maskingCharacter")]
    
    pub masking_character: Option<String>,
}

impl client::Part for CharacterMaskConfig {}


/// Checks if a particular data_id of a User data mapping in the given consent store is consented for a given use.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores check data access projects](ProjectLocationDatasetConsentStoreCheckDataAccesCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckDataAccessRequest {
    /// Optional. Specific Consents to evaluate the access request against. These Consents must have the same `user_id` as the evaluated User data mapping, must exist in the current `consent_store`, and have a `state` of either `ACTIVE` or `DRAFT`. A maximum of 100 Consents can be provided here. If no selection is specified, the access request is evaluated against all `ACTIVE` unexpired Consents with the same `user_id` as the evaluated User data mapping.
    #[serde(rename="consentList")]
    
    pub consent_list: Option<ConsentList>,
    /// Required. The unique identifier of the resource to check access for. This identifier must correspond to a User data mapping in the given consent store.
    #[serde(rename="dataId")]
    
    pub data_id: Option<String>,
    /// The values of request attributes associated with this access request.
    #[serde(rename="requestAttributes")]
    
    pub request_attributes: Option<HashMap<String, String>>,
    /// Optional. The view for CheckDataAccessResponse. If unspecified, defaults to `BASIC` and returns `consented` as `TRUE` or `FALSE`.
    #[serde(rename="responseView")]
    
    pub response_view: Option<CheckDataAccessRequestResponseViewEnum>,
}

impl client::RequestValue for CheckDataAccessRequest {}


/// Checks if a particular data_id of a User data mapping in the given consent store is consented for a given use.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores check data access projects](ProjectLocationDatasetConsentStoreCheckDataAccesCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CheckDataAccessResponse {
    /// The resource names of all evaluated Consents mapped to their evaluation.
    #[serde(rename="consentDetails")]
    
    pub consent_details: Option<HashMap<String, ConsentEvaluation>>,
    /// Whether the requested resource is consented for the given use.
    
    pub consented: Option<bool>,
}

impl client::ResponseResult for CheckDataAccessResponse {}


/// Represents a user’s consent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consents activate projects](ProjectLocationDatasetConsentStoreConsentActivateCall) (response)
/// * [locations datasets consent stores consents create projects](ProjectLocationDatasetConsentStoreConsentCreateCall) (request|response)
/// * [locations datasets consent stores consents get projects](ProjectLocationDatasetConsentStoreConsentGetCall) (response)
/// * [locations datasets consent stores consents patch projects](ProjectLocationDatasetConsentStoreConsentPatchCall) (request|response)
/// * [locations datasets consent stores consents reject projects](ProjectLocationDatasetConsentStoreConsentRejectCall) (response)
/// * [locations datasets consent stores consents revoke projects](ProjectLocationDatasetConsentStoreConsentRevokeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Consent {
    /// Required. The resource name of the Consent artifact that contains proof of the end user's consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`.
    #[serde(rename="consentArtifact")]
    
    pub consent_artifact: Option<String>,
    /// Timestamp in UTC of when this Consent is considered expired.
    #[serde(rename="expireTime")]
    
    pub expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. User-supplied key-value pairs used to organize Consent resources. Metadata keys must: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - begin with a letter - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes Metadata values must be: - be between 1 and 63 characters long - have a UTF-8 encoding of maximum 128 bytes - consist of up to 63 characters including lowercase letters, numeric characters, underscores, and dashes No more than 64 metadata entries can be associated with a given consent.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Resource name of the Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`. Cannot be changed after creation.
    
    pub name: Option<String>,
    /// Optional. Represents a user's consent in terms of the resources that can be accessed and under what conditions.
    
    pub policies: Option<Vec<GoogleCloudHealthcareV1ConsentPolicy>>,
    /// Output only. The timestamp that the revision was created.
    #[serde(rename="revisionCreateTime")]
    
    pub revision_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The revision ID of the Consent. The format is an 8-character hexadecimal string. Refer to a specific revision of a Consent by appending `@{revision_id}` to the Consent's resource name.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Required. Indicates the current state of this Consent.
    
    pub state: Option<ConsentStateEnum>,
    /// Input only. The time to live for this Consent from when it is created.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub ttl: Option<client::chrono::Duration>,
    /// Required. User's UUID provided by the client.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for Consent {}
impl client::ResponseResult for Consent {}


/// Documentation of a user’s consent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consent artifacts create projects](ProjectLocationDatasetConsentStoreConsentArtifactCreateCall) (request|response)
/// * [locations datasets consent stores consent artifacts get projects](ProjectLocationDatasetConsentStoreConsentArtifactGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsentArtifact {
    /// Optional. Screenshots, PDFs, or other binary information documenting the user's consent.
    #[serde(rename="consentContentScreenshots")]
    
    pub consent_content_screenshots: Option<Vec<Image>>,
    /// Optional. An string indicating the version of the consent information shown to the user.
    #[serde(rename="consentContentVersion")]
    
    pub consent_content_version: Option<String>,
    /// Optional. A signature from a guardian.
    #[serde(rename="guardianSignature")]
    
    pub guardian_signature: Option<Signature>,
    /// Optional. Metadata associated with the Consent artifact. For example, the consent locale or user agent version.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Resource name of the Consent artifact, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. Cannot be changed after creation.
    
    pub name: Option<String>,
    /// Required. User's UUID provided by the client.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
    /// Optional. User's signature.
    #[serde(rename="userSignature")]
    
    pub user_signature: Option<Signature>,
    /// Optional. A signature from a witness.
    #[serde(rename="witnessSignature")]
    
    pub witness_signature: Option<Signature>,
}

impl client::RequestValue for ConsentArtifact {}
impl client::ResponseResult for ConsentArtifact {}


/// The detailed evaluation of a particular Consent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsentEvaluation {
    /// The evaluation result.
    #[serde(rename="evaluationResult")]
    
    pub evaluation_result: Option<ConsentEvaluationEvaluationResultEnum>,
}

impl client::Part for ConsentEvaluation {}


/// List of resource names of Consent resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsentList {
    /// The resource names of the Consents to evaluate against, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consents/{consent_id}`.
    
    pub consents: Option<Vec<String>>,
}

impl client::Part for ConsentList {}


/// Represents a consent store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores create projects](ProjectLocationDatasetConsentStoreCreateCall) (request|response)
/// * [locations datasets consent stores get projects](ProjectLocationDatasetConsentStoreGetCall) (response)
/// * [locations datasets consent stores patch projects](ProjectLocationDatasetConsentStorePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConsentStore {
    /// Optional. Default time to live for Consents created in this store. Must be at least 24 hours. Updating this field will not affect the expiration time of existing consents.
    #[serde(rename="defaultConsentTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub default_consent_ttl: Option<client::chrono::Duration>,
    /// Optional. If `true`, UpdateConsent creates the Consent if it does not already exist. If unspecified, defaults to `false`.
    #[serde(rename="enableConsentCreateOnUpdate")]
    
    pub enable_consent_create_on_update: Option<bool>,
    /// Optional. User-supplied key-value pairs used to organize consent stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62}. Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63}. No more than 64 labels can be associated with a given store. For more information: https://cloud.google.com/healthcare/docs/how-tos/labeling-resources
    
    pub labels: Option<HashMap<String, String>>,
    /// Resource name of the consent store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}`. Cannot be changed after creation.
    
    pub name: Option<String>,
}

impl client::RequestValue for ConsentStore {}
impl client::ResponseResult for ConsentStore {}


/// Creates a new message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores messages create projects](ProjectLocationDatasetHl7V2StoreMessageCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    /// HL7v2 message.
    
    pub message: Option<Message>,
}

impl client::RequestValue for CreateMessageRequest {}


/// Pseudonymization method that generates surrogates via cryptographic hashing. Uses SHA-256. Outputs a base64-encoded representation of the hashed output (for example, `L7k0BHmF1ha5U3NfGykjro4xWi1MPVQPjhMAZbSV9mM=`).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CryptoHashConfig {
    /// An AES 128/192/256 bit key. Causes the hash to be computed based on this key. A default key is generated for each Deidentify operation and is used when neither `crypto_key` nor `kms_wrapped` is specified. Must not be set if `kms_wrapped` is set.
    #[serde(rename="cryptoKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub crypto_key: Option<Vec<u8>>,
    /// KMS wrapped key. Must not be set if `crypto_key` is set.
    #[serde(rename="kmsWrapped")]
    
    pub kms_wrapped: Option<KmsWrappedCryptoKey>,
}

impl client::Part for CryptoHashConfig {}


/// A message representing a health dataset. A health dataset represents a collection of healthcare data pertaining to one or more patients. This may include multiple modalities of healthcare data, such as electronic medical records or medical imaging data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets create projects](ProjectLocationDatasetCreateCall) (request)
/// * [locations datasets get projects](ProjectLocationDatasetGetCall) (response)
/// * [locations datasets patch projects](ProjectLocationDatasetPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    /// Resource name of the dataset, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}`.
    
    pub name: Option<String>,
    /// The default timezone used by this dataset. Must be a either a valid IANA time zone name such as "America/New_York" or empty, which defaults to UTC. This is used for parsing times in resources, such as HL7 messages, where no explicit timezone is specified.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
}

impl client::RequestValue for Dataset {}
impl client::ResponseResult for Dataset {}


/// Shift a date forward or backward in time by a random amount which is consistent for a given patient and crypto key combination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateShiftConfig {
    /// An AES 128/192/256 bit key. The date shift is computed based on this key and the patient ID. If the patient ID is empty for a DICOM resource, the date shift is computed based on this key and the study instance UID. If `crypto_key` is not set, then `kms_wrapped` is used to calculate the date shift. If neither is set, a default key is generated for each de-identify operation. Must not be set if `kms_wrapped` is set.
    #[serde(rename="cryptoKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub crypto_key: Option<Vec<u8>>,
    /// KMS wrapped key. If `kms_wrapped` is not set, then `crypto_key` is used to calculate the date shift. If neither is set, a default key is generated for each de-identify operation. Must not be set if `crypto_key` is set.
    #[serde(rename="kmsWrapped")]
    
    pub kms_wrapped: Option<KmsWrappedCryptoKey>,
}

impl client::Part for DateShiftConfig {}


/// Contains configuration for streaming de-identified FHIR export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeidentifiedStoreDestination {
    /// The configuration to use when de-identifying resources that are added to this store.
    
    pub config: Option<DeidentifyConfig>,
    /// The full resource name of a Cloud Healthcare FHIR store, for example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`.
    
    pub store: Option<String>,
}

impl client::Part for DeidentifiedStoreDestination {}


/// Configures de-id options specific to different types of content. Each submessage customizes the handling of an https://tools.ietf.org/html/rfc6838 media type or subtype. Configs are applied in a nested manner at runtime.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeidentifyConfig {
    /// Configures de-id of application/DICOM content.
    
    pub dicom: Option<DicomConfig>,
    /// Configures de-id of application/FHIR content.
    
    pub fhir: Option<FhirConfig>,
    /// Configures de-identification of image pixels wherever they are found in the source_dataset.
    
    pub image: Option<ImageConfig>,
    /// Configures de-identification of text wherever it is found in the source_dataset.
    
    pub text: Option<TextConfig>,
}

impl client::Part for DeidentifyConfig {}


/// Redacts identifying information from the specified dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets deidentify projects](ProjectLocationDatasetDeidentifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeidentifyDatasetRequest {
    /// Deidentify configuration. Only one of `config` and `gcs_config_uri` can be specified.
    
    pub config: Option<DeidentifyConfig>,
    /// The name of the dataset resource to create and write the redacted data to. * The destination dataset must not exist. * The destination dataset must be in the same location as the source dataset. De-identifying data across multiple locations is not supported.
    #[serde(rename="destinationDataset")]
    
    pub destination_dataset: Option<String>,
    /// Cloud Storage location to read the JSON cloud.healthcare.deidentify.DeidentifyConfig from, overriding the default config. Must be of the form `gs://{bucket_id}/path/to/object`. The Cloud Storage location must grant the Cloud IAM role `roles/storage.objectViewer` to the project's Cloud Healthcare Service Agent service account. Only one of `config` and `gcs_config_uri` can be specified.
    #[serde(rename="gcsConfigUri")]
    
    pub gcs_config_uri: Option<String>,
}

impl client::RequestValue for DeidentifyDatasetRequest {}


/// Creates a new DICOM store with sensitive information de-identified.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets dicom stores deidentify projects](ProjectLocationDatasetDicomStoreDeidentifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeidentifyDicomStoreRequest {
    /// Deidentify configuration. Only one of `config` and `gcs_config_uri` can be specified.
    
    pub config: Option<DeidentifyConfig>,
    /// The name of the DICOM store to create and write the redacted data to. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`. * The destination dataset must exist. * The source dataset and destination dataset must both reside in the same location. De-identifying data across multiple locations is not supported. * The destination DICOM store must not exist. * The caller must have the necessary permissions to create the destination DICOM store.
    #[serde(rename="destinationStore")]
    
    pub destination_store: Option<String>,
    /// Filter configuration.
    #[serde(rename="filterConfig")]
    
    pub filter_config: Option<DicomFilterConfig>,
    /// Cloud Storage location to read the JSON cloud.healthcare.deidentify.DeidentifyConfig from, overriding the default config. Must be of the form `gs://{bucket_id}/path/to/object`. The Cloud Storage location must grant the Cloud IAM role `roles/storage.objectViewer` to the project's Cloud Healthcare Service Agent service account. Only one of `config` and `gcs_config_uri` can be specified.
    #[serde(rename="gcsConfigUri")]
    
    pub gcs_config_uri: Option<String>,
}

impl client::RequestValue for DeidentifyDicomStoreRequest {}


/// Creates a new FHIR store with sensitive information de-identified.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets fhir stores deidentify projects](ProjectLocationDatasetFhirStoreDeidentifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeidentifyFhirStoreRequest {
    /// Deidentify configuration. Only one of `config` and `gcs_config_uri` can be specified.
    
    pub config: Option<DeidentifyConfig>,
    /// The name of the FHIR store to create and write the redacted data to. For example, `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`. * The destination dataset must exist. * The source dataset and destination dataset must both reside in the same location. De-identifying data across multiple locations is not supported. * The destination FHIR store must exist. * The caller must have the healthcare.fhirResources.update permission to write to the destination FHIR store.
    #[serde(rename="destinationStore")]
    
    pub destination_store: Option<String>,
    /// Cloud Storage location to read the JSON cloud.healthcare.deidentify.DeidentifyConfig from, overriding the default config. Must be of the form `gs://{bucket_id}/path/to/object`. The Cloud Storage location must grant the Cloud IAM role `roles/storage.objectViewer` to the project's Cloud Healthcare Service Agent service account. Only one of `config` and `gcs_config_uri` can be specified.
    #[serde(rename="gcsConfigUri")]
    
    pub gcs_config_uri: Option<String>,
    /// A filter specifying the resources to include in the output. If not specified, all resources are included in the output.
    #[serde(rename="resourceFilter")]
    
    pub resource_filter: Option<FhirFilter>,
    /// If true, skips resources that are created or modified after the de-identify operation is created.
    #[serde(rename="skipModifiedResources")]
    
    pub skip_modified_resources: Option<bool>,
}

impl client::RequestValue for DeidentifyFhirStoreRequest {}


/// Specifies the parameters needed for de-identification of DICOM stores.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DicomConfig {
    /// Tag filtering profile that determines which tags to keep/remove.
    #[serde(rename="filterProfile")]
    
    pub filter_profile: Option<DicomConfigFilterProfileEnum>,
    /// List of tags to keep. Remove all other tags.
    #[serde(rename="keepList")]
    
    pub keep_list: Option<TagFilterList>,
    /// List of tags to remove. Keep all other tags.
    #[serde(rename="removeList")]
    
    pub remove_list: Option<TagFilterList>,
    /// If true, skip replacing StudyInstanceUID, SeriesInstanceUID, SOPInstanceUID, and MediaStorageSOPInstanceUID and leave them untouched. The Cloud Healthcare API regenerates these UIDs by default based on the DICOM Standard's reasoning: "Whilst these UIDs cannot be mapped directly to an individual out of context, given access to the original images, or to a database of the original images containing the UIDs, it would be possible to recover the individual's identity." http://dicom.nema.org/medical/dicom/current/output/chtml/part15/sect_E.3.9.html
    #[serde(rename="skipIdRedaction")]
    
    pub skip_id_redaction: Option<bool>,
}

impl client::Part for DicomConfig {}


/// Specifies the filter configuration for DICOM resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DicomFilterConfig {
    /// The Cloud Storage location of the filter configuration file. The `gcs_uri` must be in the format `gs://bucket/path/to/object`. The filter configuration file must contain a list of resource paths separated by newline characters (\n or \r\n). Each resource path must be in the format "/studies/{studyUID}[/series/{seriesUID}[/instances/{instanceUID}]]" The Cloud Healthcare API service account must have the `roles/storage.objectViewer` Cloud IAM role for this Cloud Storage location.
    #[serde(rename="resourcePathsGcsUri")]
    
    pub resource_paths_gcs_uri: Option<String>,
}

impl client::Part for DicomFilterConfig {}


/// Represents a DICOM store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets dicom stores create projects](ProjectLocationDatasetDicomStoreCreateCall) (request|response)
/// * [locations datasets dicom stores get projects](ProjectLocationDatasetDicomStoreGetCall) (response)
/// * [locations datasets dicom stores patch projects](ProjectLocationDatasetDicomStorePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DicomStore {
    /// User-supplied key-value pairs used to organize DICOM stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store.
    
    pub labels: Option<HashMap<String, String>>,
    /// Resource name of the DICOM store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/dicomStores/{dicom_store_id}`.
    
    pub name: Option<String>,
    /// Notification destination for new DICOM instances. Supplied by the client.
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<NotificationConfig>,
}

impl client::RequestValue for DicomStore {}
impl client::ResponseResult for DicomStore {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores attribute definitions delete projects](ProjectLocationDatasetConsentStoreAttributeDefinitionDeleteCall) (response)
/// * [locations datasets consent stores consent artifacts delete projects](ProjectLocationDatasetConsentStoreConsentArtifactDeleteCall) (response)
/// * [locations datasets consent stores consents delete projects](ProjectLocationDatasetConsentStoreConsentDeleteCall) (response)
/// * [locations datasets consent stores consents delete revision projects](ProjectLocationDatasetConsentStoreConsentDeleteRevisionCall) (response)
/// * [locations datasets consent stores user data mappings delete projects](ProjectLocationDatasetConsentStoreUserDataMappingDeleteCall) (response)
/// * [locations datasets consent stores delete projects](ProjectLocationDatasetConsentStoreDeleteCall) (response)
/// * [locations datasets dicom stores studies series instances delete projects](ProjectLocationDatasetDicomStoreStudySeriesInstanceDeleteCall) (response)
/// * [locations datasets dicom stores delete projects](ProjectLocationDatasetDicomStoreDeleteCall) (response)
/// * [locations datasets fhir stores fhir  resource-purge projects](ProjectLocationDatasetFhirStoreFhirResourcePurgeCall) (response)
/// * [locations datasets fhir stores delete projects](ProjectLocationDatasetFhirStoreDeleteCall) (response)
/// * [locations datasets hl7 v2 stores messages delete projects](ProjectLocationDatasetHl7V2StoreMessageDeleteCall) (response)
/// * [locations datasets hl7 v2 stores delete projects](ProjectLocationDatasetHl7V2StoreDeleteCall) (response)
/// * [locations datasets operations cancel projects](ProjectLocationDatasetOperationCancelCall) (response)
/// * [locations datasets delete projects](ProjectLocationDatasetDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The candidate entities that an entity mention could link to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    /// entity_id is a first class field entity_id uniquely identifies this concept and its meta-vocabulary. For example, "UMLS/C0000970".
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
    /// preferred_term is the preferred term for this concept. For example, "Acetaminophen". For ad hoc entities formed by normalization, this is the most popular unnormalized string.
    #[serde(rename="preferredTerm")]
    
    pub preferred_term: Option<String>,
    /// Vocabulary codes are first-class fields and differentiated from the concept unique identifier (entity_id). vocabulary_codes contains the representation of this concept in particular vocabularies, such as ICD-10, SNOMED-CT and RxNORM. These are prefixed by the name of the vocabulary, followed by the unique code within that vocabulary. For example, "RXNORM/A10334543".
    #[serde(rename="vocabularyCodes")]
    
    pub vocabulary_codes: Option<Vec<String>>,
}

impl client::Part for Entity {}


/// An entity mention in the document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMention {
    /// The certainty assessment of the entity mention. Its value is one of: LIKELY, SOMEWHAT_LIKELY, UNCERTAIN, SOMEWHAT_UNLIKELY, UNLIKELY, CONDITIONAL
    #[serde(rename="certaintyAssessment")]
    
    pub certainty_assessment: Option<Feature>,
    /// The model's confidence in this entity mention annotation. A number between 0 and 1.
    
    pub confidence: Option<f64>,
    /// linked_entities are candidate ontological concepts that this entity mention may refer to. They are sorted by decreasing confidence.
    #[serde(rename="linkedEntities")]
    
    pub linked_entities: Option<Vec<LinkedEntity>>,
    /// mention_id uniquely identifies each entity mention in a single response.
    #[serde(rename="mentionId")]
    
    pub mention_id: Option<String>,
    /// The subject this entity mention relates to. Its value is one of: PATIENT, FAMILY_MEMBER, OTHER
    
    pub subject: Option<Feature>,
    /// How this entity mention relates to the subject temporally. Its value is one of: CURRENT, CLINICAL_HISTORY, FAMILY_HISTORY, UPCOMING, ALLERGY
    #[serde(rename="temporalAssessment")]
    
    pub temporal_assessment: Option<Feature>,
    /// text is the location of the entity mention in the document.
    
    pub text: Option<TextSpan>,
    /// The semantic type of the entity: UNKNOWN_ENTITY_TYPE, ALONE, ANATOMICAL_STRUCTURE, ASSISTED_LIVING, BF_RESULT, BM_RESULT, BM_UNIT, BM_VALUE, BODY_FUNCTION, BODY_MEASUREMENT, COMPLIANT, DOESNOT_FOLLOWUP, FAMILY, FOLLOWSUP, LABORATORY_DATA, LAB_RESULT, LAB_UNIT, LAB_VALUE, MEDICAL_DEVICE, MEDICINE, MED_DOSE, MED_DURATION, MED_FORM, MED_FREQUENCY, MED_ROUTE, MED_STATUS, MED_STRENGTH, MED_TOTALDOSE, MED_UNIT, NON_COMPLIANT, OTHER_LIVINGSTATUS, PROBLEM, PROCEDURE, PROCEDURE_RESULT, PROC_METHOD, REASON_FOR_NONCOMPLIANCE, SEVERITY, SUBSTANCE_ABUSE, UNCLEAR_FOLLOWUP.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for EntityMention {}


/// Defines directed relationship from one entity mention to another.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityMentionRelationship {
    /// The model's confidence in this annotation. A number between 0 and 1.
    
    pub confidence: Option<f64>,
    /// object_id is the id of the object entity mention.
    #[serde(rename="objectId")]
    
    pub object_id: Option<String>,
    /// subject_id is the id of the subject entity mention.
    #[serde(rename="subjectId")]
    
    pub subject_id: Option<String>,
}

impl client::Part for EntityMentionRelationship {}


/// Evaluate a user’s Consents for all matching User data mappings. Note: User data mappings are indexed asynchronously, causing slight delays between the time mappings are created or updated and when they are included in EvaluateUserConsents results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores evaluate user consents projects](ProjectLocationDatasetConsentStoreEvaluateUserConsentCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EvaluateUserConsentsRequest {
    /// Optional. Specific Consents to evaluate the access request against. These Consents must have the same `user_id` as the User data mappings being evalauted, must exist in the current `consent_store`, and must have a `state` of either `ACTIVE` or `DRAFT`. A maximum of 100 Consents can be provided here. If unspecified, all `ACTIVE` unexpired Consents in the current `consent_store` will be evaluated.
    #[serde(rename="consentList")]
    
    pub consent_list: Option<ConsentList>,
    /// Optional. Limit on the number of User data mappings to return in a single response. If not specified, 100 is used. May not be larger than 1000.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// Optional. Token to retrieve the next page of results, or empty to get the first page.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Required. The values of request attributes associated with this access request.
    #[serde(rename="requestAttributes")]
    
    pub request_attributes: Option<HashMap<String, String>>,
    /// Optional. The values of resource attributes associated with the resources being requested. If no values are specified, then all resources are queried.
    #[serde(rename="resourceAttributes")]
    
    pub resource_attributes: Option<HashMap<String, String>>,
    /// Optional. The view for EvaluateUserConsentsResponse. If unspecified, defaults to `BASIC` and returns `consented` as `TRUE` or `FALSE`.
    #[serde(rename="responseView")]
    
    pub response_view: Option<EvaluateUserConsentsRequestResponseViewEnum>,
    /// Required. User ID to evaluate consents for.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for EvaluateUserConsentsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores evaluate user consents projects](ProjectLocationDatasetConsentStoreEvaluateUserConsentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EvaluateUserConsentsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list. This token is valid for 72 hours after it is created.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The consent evaluation result for each `data_id`.
    
    pub results: Option<Vec<Result>>,
}

impl client::ResponseResult for EvaluateUserConsentsResponse {}


/// Exports data from the specified DICOM store. If a given resource, such as a DICOM object with the same SOPInstance UID, already exists in the output, it is overwritten with the version in the source dataset. Exported DICOM data persists when the DICOM store from which it was exported is deleted.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets dicom stores export projects](ProjectLocationDatasetDicomStoreExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportDicomDataRequest {
    /// The BigQuery output destination. You can only export to a BigQuery dataset that's in the same project as the DICOM store you're exporting from. The Cloud Healthcare Service Agent requires two IAM roles on the BigQuery location: `roles/bigquery.dataEditor` and `roles/bigquery.jobUser`.
    #[serde(rename="bigqueryDestination")]
    
    pub bigquery_destination: Option<GoogleCloudHealthcareV1DicomBigQueryDestination>,
    /// The Cloud Storage output destination. The Cloud Healthcare Service Agent requires the `roles/storage.objectAdmin` Cloud IAM roles on the Cloud Storage location.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GoogleCloudHealthcareV1DicomGcsDestination>,
}

impl client::RequestValue for ExportDicomDataRequest {}


/// Request to schedule an export.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores export projects](ProjectLocationDatasetHl7V2StoreExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportMessagesRequest {
    /// The end of the range in `send_time` (MSH.7, https://www.hl7.org/documentcenter/public_temp_2E58C1F9-1C23-BA17-0C6126475344DA9D/wg/conf/HL7MSH.htm) to process. If not specified, the time when the export is scheduled is used. This value has to come after the `start_time` defined below. Only messages whose `send_time` lies in the range `start_time` (inclusive) to `end_time` (exclusive) are exported.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Export to a Cloud Storage destination.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsDestination>,
    /// The start of the range in `send_time` (MSH.7, https://www.hl7.org/documentcenter/public_temp_2E58C1F9-1C23-BA17-0C6126475344DA9D/wg/conf/HL7MSH.htm) to process. If not specified, the UNIX epoch (1970-01-01T00:00:00Z) is used. This value has to come before the `end_time` defined below. Only messages whose `send_time` lies in the range `start_time` (inclusive) to `end_time` (exclusive) are exported.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for ExportMessagesRequest {}


/// Request to export resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets fhir stores export projects](ProjectLocationDatasetFhirStoreExportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportResourcesRequest {
    /// If provided, only resources updated after this time are exported. The time uses the format YYYY-MM-DDThh:mm:ss.sss+zz:zz. For example, `2015-02-07T13:28:17.239+02:00` or `2017-01-01T00:00:00Z`. The time must be specified to the second and include a time zone.
    
    pub _since: Option<String>,
    /// String of comma-delimited FHIR resource types. If provided, only resources of the specified resource type(s) are exported.
    
    pub _type: Option<String>,
    /// The BigQuery output destination. The Cloud Healthcare Service Agent requires two IAM roles on the BigQuery location: `roles/bigquery.dataEditor` and `roles/bigquery.jobUser`. The output is one BigQuery table per resource type. Unlike when setting `BigQueryDestination` for `StreamConfig`, `ExportResources` does not create BigQuery views.
    #[serde(rename="bigqueryDestination")]
    
    pub bigquery_destination: Option<GoogleCloudHealthcareV1FhirBigQueryDestination>,
    /// The Cloud Storage output destination. The Healthcare Service Agent account requires the `roles/storage.objectAdmin` role on the Cloud Storage location. The exported outputs are organized by FHIR resource types. The server creates one object per resource type. Each object contains newline delimited JSON, and each line is a FHIR resource.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GoogleCloudHealthcareV1FhirGcsDestination>,
}

impl client::RequestValue for ExportResourcesRequest {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// A feature of an entity mention.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Feature {
    /// The model's confidence in this feature annotation. A number between 0 and 1.
    
    pub confidence: Option<f64>,
    /// The value of this feature annotation. Its range depends on the type of the feature.
    
    pub value: Option<String>,
}

impl client::Part for Feature {}


/// Specifies how to handle de-identification of a FHIR store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FhirConfig {
    /// The behaviour for handling FHIR extensions that aren't otherwise specified for de-identification. If true, all extensions are preserved during de-identification by default. If false or unspecified, all extensions are removed during de-identification by default.
    #[serde(rename="defaultKeepExtensions")]
    
    pub default_keep_extensions: Option<bool>,
    /// Specifies FHIR paths to match and how to transform them. Any field that is not matched by a FieldMetadata is passed through to the output dataset unmodified. All extensions will be processed according to `default_keep_extensions`.
    #[serde(rename="fieldMetadataList")]
    
    pub field_metadata_list: Option<Vec<FieldMetadata>>,
}

impl client::Part for FhirConfig {}


/// Filter configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FhirFilter {
    /// List of resources to include in the output. If this list is empty or not specified, all resources are included in the output.
    
    pub resources: Option<Resources>,
}

impl client::Part for FhirFilter {}


/// Represents a FHIR store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets fhir stores create projects](ProjectLocationDatasetFhirStoreCreateCall) (request|response)
/// * [locations datasets fhir stores get projects](ProjectLocationDatasetFhirStoreGetCall) (response)
/// * [locations datasets fhir stores patch projects](ProjectLocationDatasetFhirStorePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FhirStore {
    /// Enable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources.
    #[serde(rename="complexDataTypeReferenceParsing")]
    
    pub complex_data_type_reference_parsing: Option<FhirStoreComplexDataTypeReferenceParsingEnum>,
    /// If true, overrides the default search behavior for this FHIR store to `handling=strict` which returns an error for unrecognized search parameters. If false, uses the FHIR specification default `handling=lenient` which ignores unrecognized search parameters. The handling can always be changed from the default on an individual API call by setting the HTTP header `Prefer: handling=strict` or `Prefer: handling=lenient`.
    #[serde(rename="defaultSearchHandlingStrict")]
    
    pub default_search_handling_strict: Option<bool>,
    /// Immutable. Whether to disable referential integrity in this FHIR store. This field is immutable after FHIR store creation. The default value is false, meaning that the API enforces referential integrity and fails the requests that result in inconsistent state in the FHIR store. When this field is set to true, the API skips referential integrity checks. Consequently, operations that rely on references, such as GetPatientEverything, do not return all the results if broken references exist.
    #[serde(rename="disableReferentialIntegrity")]
    
    pub disable_referential_integrity: Option<bool>,
    /// Immutable. Whether to disable resource versioning for this FHIR store. This field can not be changed after the creation of FHIR store. If set to false, which is the default behavior, all write operations cause historical versions to be recorded automatically. The historical versions can be fetched through the history APIs, but cannot be updated. If set to true, no historical versions are kept. The server sends errors for attempts to read the historical versions.
    #[serde(rename="disableResourceVersioning")]
    
    pub disable_resource_versioning: Option<bool>,
    /// Whether this FHIR store has the [updateCreate capability](https://www.hl7.org/fhir/capabilitystatement-definitions.html#CapabilityStatement.rest.resource.updateCreate). This determines if the client can use an Update operation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through the Create operation and attempts to update a non-existent resource return errors. It is strongly advised not to include or encode any sensitive data such as patient identifiers in client-specified resource IDs. Those IDs are part of the FHIR resource path recorded in Cloud audit logs and Pub/Sub notifications. Those IDs can also be contained in reference fields within other resources.
    #[serde(rename="enableUpdateCreate")]
    
    pub enable_update_create: Option<bool>,
    /// User-supplied key-value pairs used to organize FHIR stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Resource name of the FHIR store, of the form `projects/{project_id}/datasets/{dataset_id}/fhirStores/{fhir_store_id}`.
    
    pub name: Option<String>,
    /// If non-empty, publish all resource modifications of this FHIR store to this destination. The Pub/Sub message attributes contain a map with a string describing the action that has triggered the notification. For example, "action":"CreateResource".
    #[serde(rename="notificationConfig")]
    
    pub notification_config: Option<NotificationConfig>,
    /// A list of streaming configs that configure the destinations of streaming export for every resource mutation in this FHIR store. Each store is allowed to have up to 10 streaming configs. After a new config is added, the next resource mutation is streamed to the new location in addition to the existing ones. When a location is removed from the list, the server stops streaming to that location. Before adding a new config, you must add the required [`bigquery.dataEditor`](https://cloud.google.com/bigquery/docs/access-control#bigquery.dataEditor) role to your project's **Cloud Healthcare Service Agent** [service account](https://cloud.google.com/iam/docs/service-accounts). Some lag (typically on the order of dozens of seconds) is expected before the results show up in the streaming destination.
    #[serde(rename="streamConfigs")]
    
    pub stream_configs: Option<Vec<StreamConfig>>,
    /// Configuration for how to validate incoming FHIR resources against configured profiles.
    #[serde(rename="validationConfig")]
    
    pub validation_config: Option<ValidationConfig>,
    /// Immutable. The FHIR specification version that this FHIR store supports natively. This field is immutable after store creation. Requests are rejected if they contain FHIR resources of a different version. Version is required for every FHIR store.
    
    pub version: Option<FhirStoreVersionEnum>,
}

impl client::RequestValue for FhirStore {}
impl client::ResponseResult for FhirStore {}


/// A (sub) field of a type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    /// The maximum number of times this field can be repeated. 0 or -1 means unbounded.
    #[serde(rename="maxOccurs")]
    
    pub max_occurs: Option<i32>,
    /// The minimum number of times this field must be present/repeated.
    #[serde(rename="minOccurs")]
    
    pub min_occurs: Option<i32>,
    /// The name of the field. For example, "PID-1" or just "1".
    
    pub name: Option<String>,
    /// The HL7v2 table this field refers to. For example, PID-15 (Patient's Primary Language) usually refers to table "0296".
    
    pub table: Option<String>,
    /// The type of this field. A Type with this name must be defined in an Hl7TypesConfig.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Field {}


/// Specifies FHIR paths to match, and how to handle de-identification of matching fields.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldMetadata {
    /// Deidentify action for one field.
    
    pub action: Option<FieldMetadataActionEnum>,
    /// List of paths to FHIR fields to be redacted. Each path is a period-separated list where each component is either a field name or FHIR type name, for example: Patient, HumanName. For "choice" types (those defined in the FHIR spec with the form: field[x]) we use two separate components. For example, "deceasedAge.unit" is matched by "Deceased.Age.unit". Supported types are: AdministrativeGenderCode, Base64Binary, Boolean, Code, Date, DateTime, Decimal, HumanName, Id, Instant, Integer, LanguageCode, Markdown, Oid, PositiveInt, String, UnsignedInt, Uri, Uuid, Xhtml.
    
    pub paths: Option<Vec<String>>,
}

impl client::Part for FieldMetadata {}


/// The Cloud Storage output destination. The Cloud Healthcare Service Agent requires the `roles/storage.objectAdmin` Cloud IAM roles on the Cloud Storage location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsDestination {
    /// The format of the exported HL7v2 message files.
    #[serde(rename="contentStructure")]
    
    pub content_structure: Option<GcsDestinationContentStructureEnum>,
    /// Specifies the parts of the Message resource to include in the export. If not specified, FULL is used.
    #[serde(rename="messageView")]
    
    pub message_view: Option<GcsDestinationMessageViewEnum>,
    /// URI of an existing Cloud Storage directory where the server writes result files, in the format `gs://{bucket-id}/{path/to/destination/dir}`. If there is no trailing slash, the service appends one when composing the object path.
    #[serde(rename="uriPrefix")]
    
    pub uri_prefix: Option<String>,
}

impl client::Part for GcsDestination {}


/// Specifies the configuration for importing data from Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsSource {
    /// Points to a Cloud Storage URI containing file(s) to import. The URI must be in the following format: `gs://{bucket_id}/{object_id}`. The URI can include wildcards in `object_id` and thus identify multiple files. Supported wildcards: * `*` to match 0 or more non-separator characters * `**` to match 0 or more characters (including separators). Must be used at the end of a path and with no other wildcards in the path. Can also be used with a file extension (such as .ndjson), which imports all files with the extension in the specified directory and its sub-directories. For example, `gs://my-bucket/my-directory/**.ndjson` imports all files with `.ndjson` extensions in `my-directory/` and its sub-directories. * `?` to match 1 character Files matching the wildcard are expected to contain content only, no metadata.
    
    pub uri: Option<String>,
}

impl client::Part for GcsSource {}


/// The Cloud Storage location for export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1ConsentGcsDestination {
    /// URI for a Cloud Storage directory where the server writes result files, in the format `gs://{bucket-id}/{path/to/destination/dir}`. If there is no trailing slash, the service appends one when composing the object path. The user is responsible for creating the Cloud Storage bucket and directory referenced in `uri_prefix`.
    #[serde(rename="uriPrefix")]
    
    pub uri_prefix: Option<String>,
}

impl client::Part for GoogleCloudHealthcareV1ConsentGcsDestination {}


/// Represents a user's consent in terms of the resources that can be accessed and under what conditions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1ConsentPolicy {
    /// Required. The request conditions to meet to grant access. In addition to any supported comparison operators, authorization rules may have `IN` operator as well as at most 10 logical operators that are limited to `AND` (`&&`), `OR` (`||`).
    #[serde(rename="authorizationRule")]
    
    pub authorization_rule: Option<Expr>,
    /// The resources that this policy applies to. A resource is a match if it matches all the attributes listed here. If empty, this policy applies to all User data mappings for the given user.
    #[serde(rename="resourceAttributes")]
    
    pub resource_attributes: Option<Vec<Attribute>>,
}

impl client::Part for GoogleCloudHealthcareV1ConsentPolicy {}


/// The BigQuery table where the server writes the output.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1DicomBigQueryDestination {
    /// Use `write_disposition` instead. If `write_disposition` is specified, this parameter is ignored. force=false is equivalent to write_disposition=WRITE_EMPTY and force=true is equivalent to write_disposition=WRITE_TRUNCATE.
    
    pub force: Option<bool>,
    /// BigQuery URI to a table, up to 2000 characters long, in the format `bq://projectId.bqDatasetId.tableId`
    #[serde(rename="tableUri")]
    
    pub table_uri: Option<String>,
    /// Determines whether the existing table in the destination is to be overwritten or appended to. If a write_disposition is specified, the `force` parameter is ignored.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<GoogleCloudHealthcareV1DicomBigQueryDestinationWriteDispositionEnum>,
}

impl client::Part for GoogleCloudHealthcareV1DicomBigQueryDestination {}


/// The Cloud Storage location where the server writes the output and the export configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1DicomGcsDestination {
    /// MIME types supported by DICOM spec. Each file is written in the following format: `.../{study_id}/{series_id}/{instance_id}[/{frame_number}].{extension}` The frame_number component exists only for multi-frame instances. Supported MIME types are consistent with supported formats in DICOMweb: https://cloud.google.com/healthcare/docs/dicom#retrieve_transaction. Specifically, the following are supported: - application/dicom; transfer-syntax=1.2.840.10008.1.2.1 (uncompressed DICOM) - application/dicom; transfer-syntax=1.2.840.10008.1.2.4.50 (DICOM with embedded JPEG Baseline) - application/dicom; transfer-syntax=1.2.840.10008.1.2.4.90 (DICOM with embedded JPEG 2000 Lossless Only) - application/dicom; transfer-syntax=1.2.840.10008.1.2.4.91 (DICOM with embedded JPEG 2000) - application/dicom; transfer-syntax=* (DICOM with no transcoding) - application/octet-stream; transfer-syntax=1.2.840.10008.1.2.1 (raw uncompressed PixelData) - application/octet-stream; transfer-syntax=* (raw PixelData in whatever format it was uploaded in) - image/jpeg; transfer-syntax=1.2.840.10008.1.2.4.50 (Consumer JPEG) - image/png The following extensions are used for output files: - application/dicom -> .dcm - image/jpeg -> .jpg - image/png -> .png - application/octet-stream -> no extension If unspecified, the instances are exported in the original DICOM format they were uploaded in.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The Cloud Storage destination to export to. URI for a Cloud Storage directory where the server writes the result files, in the format `gs://{bucket-id}/{path/to/destination/dir}`). If there is no trailing slash, the service appends one when composing the object path. The user is responsible for creating the Cloud Storage bucket referenced in `uri_prefix`.
    #[serde(rename="uriPrefix")]
    
    pub uri_prefix: Option<String>,
}

impl client::Part for GoogleCloudHealthcareV1DicomGcsDestination {}


/// Specifies the configuration for importing data from Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1DicomGcsSource {
    /// Points to a Cloud Storage URI containing file(s) with content only. The URI must be in the following format: `gs://{bucket_id}/{object_id}`. The URI can include wildcards in `object_id` and thus identify multiple files. Supported wildcards: * '*' to match 0 or more non-separator characters * '**' to match 0 or more characters (including separators). Must be used at the end of a path and with no other wildcards in the path. Can also be used with a file extension (such as .dcm), which imports all files with the extension in the specified directory and its sub-directories. For example, `gs://my-bucket/my-directory/**.dcm` imports all files with .dcm extensions in `my-directory/` and its sub-directories. * '?' to match 1 character. All other URI formats are invalid. Files matching the wildcard are expected to contain content only, no metadata.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudHealthcareV1DicomGcsSource {}


/// The configuration for exporting to BigQuery.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1FhirBigQueryDestination {
    /// BigQuery URI to an existing dataset, up to 2000 characters long, in the format `bq://projectId.bqDatasetId`.
    #[serde(rename="datasetUri")]
    
    pub dataset_uri: Option<String>,
    /// If this flag is `TRUE`, all tables are deleted from the dataset before the new exported tables are written. If the flag is not set and the destination dataset contains tables, the export call returns an error. If `write_disposition` is specified, this parameter is ignored. force=false is equivalent to write_disposition=WRITE_EMPTY and force=true is equivalent to write_disposition=WRITE_TRUNCATE.
    
    pub force: Option<bool>,
    /// The configuration for the exported BigQuery schema.
    #[serde(rename="schemaConfig")]
    
    pub schema_config: Option<SchemaConfig>,
    /// Determines if existing data in the destination dataset is overwritten, appended to, or not written if the tables contain data. If a write_disposition is specified, the `force` parameter is ignored.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<GoogleCloudHealthcareV1FhirBigQueryDestinationWriteDispositionEnum>,
}

impl client::Part for GoogleCloudHealthcareV1FhirBigQueryDestination {}


/// The configuration for exporting to Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1FhirGcsDestination {
    /// URI for a Cloud Storage directory where result files should be written, in the format of `gs://{bucket-id}/{path/to/destination/dir}`. If there is no trailing slash, the service appends one when composing the object path. The user is responsible for creating the Cloud Storage bucket referenced in `uri_prefix`.
    #[serde(rename="uriPrefix")]
    
    pub uri_prefix: Option<String>,
}

impl client::Part for GoogleCloudHealthcareV1FhirGcsDestination {}


/// Specifies the configuration for importing data from Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudHealthcareV1FhirGcsSource {
    /// Points to a Cloud Storage URI containing file(s) to import. The URI must be in the following format: `gs://{bucket_id}/{object_id}`. The URI can include wildcards in `object_id` and thus identify multiple files. Supported wildcards: * `*` to match 0 or more non-separator characters * `**` to match 0 or more characters (including separators). Must be used at the end of a path and with no other wildcards in the path. Can also be used with a file extension (such as .ndjson), which imports all files with the extension in the specified directory and its sub-directories. For example, `gs://my-bucket/my-directory/**.ndjson` imports all files with `.ndjson` extensions in `my-directory/` and its sub-directories. * `?` to match 1 character Files matching the wildcard are expected to contain content only, no metadata.
    
    pub uri: Option<String>,
}

impl client::Part for GoogleCloudHealthcareV1FhirGcsSource {}


/// Construct representing a logical group or a segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupOrSegment {
    /// no description provided
    
    pub group: Option<SchemaGroup>,
    /// no description provided
    
    pub segment: Option<SchemaSegment>,
}

impl client::Part for GroupOrSegment {}


/// Root config message for HL7v2 schema. This contains a schema structure of groups and segments, and filters that determine which messages to apply the schema structure to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hl7SchemaConfig {
    /// Map from each HL7v2 message type and trigger event pair, such as ADT_A04, to its schema configuration root group.
    #[serde(rename="messageSchemaConfigs")]
    
    pub message_schema_configs: Option<HashMap<String, SchemaGroup>>,
    /// Each VersionSource is tested and only if they all match is the schema used for the message.
    
    pub version: Option<Vec<VersionSource>>,
}

impl client::Part for Hl7SchemaConfig {}


/// Root config for HL7v2 datatype definitions for a specific HL7v2 version.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hl7TypesConfig {
    /// The HL7v2 type definitions.
    #[serde(rename="type")]
    
    pub type_: Option<Vec<Type>>,
    /// The version selectors that this config applies to. A message must match ALL version sources to apply.
    
    pub version: Option<Vec<VersionSource>>,
}

impl client::Part for Hl7TypesConfig {}


/// Specifies where and whether to send notifications upon changes to a data store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hl7V2NotificationConfig {
    /// Restricts notifications sent for messages matching a filter. If this is empty, all messages are matched. The following syntax is available: * A string field value can be written as text inside quotation marks, for example `"query text"`. The only valid relational operation for text fields is equality (`=`), where text is searched within the field, rather than having the field be equal to the text. For example, `"Comment = great"` returns messages with `great` in the comment field. * A number field value can be written as an integer, a decimal, or an exponential. The valid relational operators for number fields are the equality operator (`=`), along with the less than/greater than operators (`<`, `<=`, `>`, `>=`). Note that there is no inequality (`!=`) operator. You can prepend the `NOT` operator to an expression to negate it. * A date field value must be written in `yyyy-mm-dd` form. Fields with date and time use the RFC3339 time format. Leading zeros are required for one-digit months and days. The valid relational operators for date fields are the equality operator (`=`) , along with the less than/greater than operators (`<`, `<=`, `>`, `>=`). Note that there is no inequality (`!=`) operator. You can prepend the `NOT` operator to an expression to negate it. * Multiple field query expressions can be combined in one query by adding `AND` or `OR` operators between the expressions. If a boolean operator appears within a quoted string, it is not treated as special, it's just another part of the character string to be matched. You can prepend the `NOT` operator to an expression to negate it. The following fields and functions are available for filtering: * `message_type`, from the MSH-9.1 field. For example, `NOT message_type = "ADT"`. * `send_date` or `sendDate`, the YYYY-MM-DD date the message was sent in the dataset's time_zone, from the MSH-7 segment. For example, `send_date < "2017-01-02"`. * `send_time`, the timestamp when the message was sent, using the RFC3339 time format for comparisons, from the MSH-7 segment. For example, `send_time < "2017-01-02T00:00:00-05:00"`. * `create_time`, the timestamp when the message was created in the HL7v2 store. Use the RFC3339 time format for comparisons. For example, `create_time < "2017-01-02T00:00:00-05:00"`. * `send_facility`, the care center that the message came from, from the MSH-4 segment. For example, `send_facility = "ABC"`. * `PatientId(value, type)`, which matches if the message lists a patient having an ID of the given value and type in the PID-2, PID-3, or PID-4 segments. For example, `PatientId("123456", "MRN")`. * `labels.x`, a string value of the label with key `x` as set using the Message.labels map. For example, `labels."priority"="high"`. The operator `:*` can be used to assert the existence of a label. For example, `labels."priority":*`.
    
    pub filter: Option<String>,
    /// The [Pub/Sub](https://cloud.google.com/pubsub/docs/) topic that notifications of changes are published on. Supplied by the client. The notification is a `PubsubMessage` with the following fields: * `PubsubMessage.Data` contains the resource name. * `PubsubMessage.MessageId` is the ID of this notification. It's guaranteed to be unique within the topic. * `PubsubMessage.PublishTime` is the time when the message was published. Note that notifications are only sent if the topic is non-empty. [Topic names](https://cloud.google.com/pubsub/docs/overview#names) must be scoped to a project. The Cloud Healthcare API service account, service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com, must have publisher permissions on the given Pub/Sub topic. Not having adequate permissions causes the calls that send notifications to fail. If a notification cannot be published to Pub/Sub, errors are logged to Cloud Logging. For more information, see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)).
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
}

impl client::Part for Hl7V2NotificationConfig {}


/// Represents an HL7v2 store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores create projects](ProjectLocationDatasetHl7V2StoreCreateCall) (request|response)
/// * [locations datasets hl7 v2 stores get projects](ProjectLocationDatasetHl7V2StoreGetCall) (response)
/// * [locations datasets hl7 v2 stores patch projects](ProjectLocationDatasetHl7V2StorePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hl7V2Store {
    /// User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store.
    
    pub labels: Option<HashMap<String, String>>,
    /// Resource name of the HL7v2 store, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7v2_store_id}`.
    
    pub name: Option<String>,
    /// A list of notification configs. Each configuration uses a filter to determine whether to publish a message (both Ingest & Create) on the corresponding notification destination. Only the message name is sent as part of the notification. Supplied by the client.
    #[serde(rename="notificationConfigs")]
    
    pub notification_configs: Option<Vec<Hl7V2NotificationConfig>>,
    /// The configuration for the parser. It determines how the server parses the messages.
    #[serde(rename="parserConfig")]
    
    pub parser_config: Option<ParserConfig>,
    /// Determines whether to reject duplicate messages. A duplicate message is a message with the same raw bytes as a message that has already been ingested/created in this HL7v2 store. The default value is false, meaning that the store accepts the duplicate messages and it also returns the same ACK message in the IngestMessageResponse as has been returned previously. Note that only one resource is created in the store. When this field is set to true, CreateMessage/IngestMessage requests with a duplicate message will be rejected by the store, and IngestMessageErrorDetail returns a NACK message upon rejection.
    #[serde(rename="rejectDuplicateMessage")]
    
    pub reject_duplicate_message: Option<bool>,
}

impl client::RequestValue for Hl7V2Store {}
impl client::ResponseResult for Hl7V2Store {}


/// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can’t be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets dicom stores studies series instances frames retrieve frames projects](ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveFrameCall) (response)
/// * [locations datasets dicom stores studies series instances frames retrieve rendered projects](ProjectLocationDatasetDicomStoreStudySeriesInstanceFrameRetrieveRenderedCall) (response)
/// * [locations datasets dicom stores studies series instances retrieve instance projects](ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveInstanceCall) (response)
/// * [locations datasets dicom stores studies series instances retrieve metadata projects](ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveMetadataCall) (response)
/// * [locations datasets dicom stores studies series instances retrieve rendered projects](ProjectLocationDatasetDicomStoreStudySeriesInstanceRetrieveRenderedCall) (response)
/// * [locations datasets dicom stores studies series retrieve metadata projects](ProjectLocationDatasetDicomStoreStudySeriesRetrieveMetadataCall) (response)
/// * [locations datasets dicom stores studies series retrieve series projects](ProjectLocationDatasetDicomStoreStudySeriesRetrieveSeryCall) (response)
/// * [locations datasets dicom stores studies series search for instances projects](ProjectLocationDatasetDicomStoreStudySeriesSearchForInstanceCall) (response)
/// * [locations datasets dicom stores studies retrieve metadata projects](ProjectLocationDatasetDicomStoreStudyRetrieveMetadataCall) (response)
/// * [locations datasets dicom stores studies retrieve study projects](ProjectLocationDatasetDicomStoreStudyRetrieveStudyCall) (response)
/// * [locations datasets dicom stores studies search for instances projects](ProjectLocationDatasetDicomStoreStudySearchForInstanceCall) (response)
/// * [locations datasets dicom stores studies search for series projects](ProjectLocationDatasetDicomStoreStudySearchForSeryCall) (response)
/// * [locations datasets dicom stores studies store instances projects](ProjectLocationDatasetDicomStoreStudyStoreInstanceCall) (request|response)
/// * [locations datasets dicom stores search for instances projects](ProjectLocationDatasetDicomStoreSearchForInstanceCall) (response)
/// * [locations datasets dicom stores search for series projects](ProjectLocationDatasetDicomStoreSearchForSeryCall) (response)
/// * [locations datasets dicom stores search for studies projects](ProjectLocationDatasetDicomStoreSearchForStudyCall) (response)
/// * [locations datasets dicom stores store instances projects](ProjectLocationDatasetDicomStoreStoreInstanceCall) (request|response)
/// * [locations datasets fhir stores fhir  patient-everything projects](ProjectLocationDatasetFhirStoreFhirPatientEverythingCall) (response)
/// * [locations datasets fhir stores fhir  resource-validate projects](ProjectLocationDatasetFhirStoreFhirResourceValidateCall) (request|response)
/// * [locations datasets fhir stores fhir capabilities projects](ProjectLocationDatasetFhirStoreFhirCapabilityCall) (response)
/// * [locations datasets fhir stores fhir create projects](ProjectLocationDatasetFhirStoreFhirCreateCall) (request|response)
/// * [locations datasets fhir stores fhir delete projects](ProjectLocationDatasetFhirStoreFhirDeleteCall) (response)
/// * [locations datasets fhir stores fhir execute bundle projects](ProjectLocationDatasetFhirStoreFhirExecuteBundleCall) (request|response)
/// * [locations datasets fhir stores fhir history projects](ProjectLocationDatasetFhirStoreFhirHistoryCall) (response)
/// * [locations datasets fhir stores fhir patch projects](ProjectLocationDatasetFhirStoreFhirPatchCall) (request|response)
/// * [locations datasets fhir stores fhir read projects](ProjectLocationDatasetFhirStoreFhirReadCall) (response)
/// * [locations datasets fhir stores fhir search projects](ProjectLocationDatasetFhirStoreFhirSearchCall) (response)
/// * [locations datasets fhir stores fhir search-type projects](ProjectLocationDatasetFhirStoreFhirSearchTypeCall) (response)
/// * [locations datasets fhir stores fhir update projects](ProjectLocationDatasetFhirStoreFhirUpdateCall) (request|response)
/// * [locations datasets fhir stores fhir vread projects](ProjectLocationDatasetFhirStoreFhirVreadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::RequestValue for HttpBody {}
impl client::ResponseResult for HttpBody {}


/// Raw bytes representing consent artifact content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// Input only. Points to a Cloud Storage URI containing the consent artifact content. The URI must be in the following format: `gs://{bucket_id}/{object_id}`. The Cloud Healthcare API service account must have the `roles/storage.objectViewer` Cloud IAM role for this Cloud Storage location. The consent artifact content at this URI is copied to a Cloud Storage location managed by the Cloud Healthcare API. Responses to fetching requests return the consent artifact content in raw_bytes.
    #[serde(rename="gcsUri")]
    
    pub gcs_uri: Option<String>,
    /// Consent artifact content represented as a stream of bytes. This field is populated when returned in GetConsentArtifact response, but not included in CreateConsentArtifact and ListConsentArtifact response.
    #[serde(rename="rawBytes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub raw_bytes: Option<Vec<u8>>,
}

impl client::Part for Image {}


/// Specifies how to handle de-identification of image pixels.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageConfig {
    /// Determines how to redact text from image.
    #[serde(rename="textRedactionMode")]
    
    pub text_redaction_mode: Option<ImageConfigTextRedactionModeEnum>,
}

impl client::Part for ImageConfig {}


/// Imports data into the specified DICOM store. Returns an error if any of the files to import are not DICOM files. This API accepts duplicate DICOM instances by ignoring the newly-pushed instance. It does not overwrite.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets dicom stores import projects](ProjectLocationDatasetDicomStoreImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportDicomDataRequest {
    /// Cloud Storage source data location and import configuration. The Cloud Healthcare Service Agent requires the `roles/storage.objectViewer` Cloud IAM roles on the Cloud Storage location.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudHealthcareV1DicomGcsSource>,
}

impl client::RequestValue for ImportDicomDataRequest {}


/// Request to import messages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores import projects](ProjectLocationDatasetHl7V2StoreImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportMessagesRequest {
    /// Cloud Storage source data location and import configuration. The Cloud Healthcare Service Agent requires the `roles/storage.objectViewer` Cloud IAM roles on the Cloud Storage location.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsSource>,
}

impl client::RequestValue for ImportMessagesRequest {}


/// Request to import resources.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets fhir stores import projects](ProjectLocationDatasetFhirStoreImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportResourcesRequest {
    /// The content structure in the source location. If not specified, the server treats the input source files as BUNDLE.
    #[serde(rename="contentStructure")]
    
    pub content_structure: Option<ImportResourcesRequestContentStructureEnum>,
    /// Cloud Storage source data location and import configuration. The Healthcare Service Agent account requires the `roles/storage.objectAdmin` role on the Cloud Storage location. Each Cloud Storage object should be a text file that contains the format specified in ContentStructure.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GoogleCloudHealthcareV1FhirGcsSource>,
}

impl client::RequestValue for ImportResourcesRequest {}


/// A transformation to apply to text that is identified as a specific info_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InfoTypeTransformation {
    /// Config for character mask.
    #[serde(rename="characterMaskConfig")]
    
    pub character_mask_config: Option<CharacterMaskConfig>,
    /// Config for crypto hash.
    #[serde(rename="cryptoHashConfig")]
    
    pub crypto_hash_config: Option<CryptoHashConfig>,
    /// Config for date shift.
    #[serde(rename="dateShiftConfig")]
    
    pub date_shift_config: Option<DateShiftConfig>,
    /// InfoTypes to apply this transformation to. If this is not specified, the transformation applies to any info_type.
    #[serde(rename="infoTypes")]
    
    pub info_types: Option<Vec<String>>,
    /// Config for text redaction.
    #[serde(rename="redactConfig")]
    
    pub redact_config: Option<RedactConfig>,
    /// Config for replace with InfoType.
    #[serde(rename="replaceWithInfoTypeConfig")]
    
    pub replace_with_info_type_config: Option<ReplaceWithInfoTypeConfig>,
}

impl client::Part for InfoTypeTransformation {}


/// Ingests a message into the specified HL7v2 store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores messages ingest projects](ProjectLocationDatasetHl7V2StoreMessageIngestCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IngestMessageRequest {
    /// HL7v2 message to ingest.
    
    pub message: Option<Message>,
}

impl client::RequestValue for IngestMessageRequest {}


/// Acknowledges that a message has been ingested into the specified HL7v2 store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores messages ingest projects](ProjectLocationDatasetHl7V2StoreMessageIngestCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IngestMessageResponse {
    /// HL7v2 ACK message.
    #[serde(rename="hl7Ack")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub hl7_ack: Option<Vec<u8>>,
    /// Created message resource.
    
    pub message: Option<Message>,
}

impl client::ResponseResult for IngestMessageResponse {}


/// Include to use an existing data crypto key wrapped by KMS. The wrapped key must be a 128-, 192-, or 256-bit key. The key must grant the Cloud IAM permission `cloudkms.cryptoKeyVersions.useToDecrypt` to the project's Cloud Healthcare Service Agent service account. For more information, see [Creating a wrapped key] (https://cloud.google.com/dlp/docs/create-wrapped-key).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KmsWrappedCryptoKey {
    /// Required. The resource name of the KMS CryptoKey to use for unwrapping. For example, `projects/{project_id}/locations/{location_id}/keyRings/{keyring}/cryptoKeys/{key}`.
    #[serde(rename="cryptoKey")]
    
    pub crypto_key: Option<String>,
    /// Required. The wrapped data crypto key.
    #[serde(rename="wrappedKey")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub wrapped_key: Option<Vec<u8>>,
}

impl client::Part for KmsWrappedCryptoKey {}


/// EntityMentions can be linked to multiple entities using a LinkedEntity message lets us add other fields, e.g. confidence.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedEntity {
    /// entity_id is a concept unique identifier. These are prefixed by a string that identifies the entity coding system, followed by the unique identifier within that system. For example, "UMLS/C0000970". This also supports ad hoc entities, which are formed by normalizing entity mention content.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
}

impl client::Part for LinkedEntity {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores attribute definitions list projects](ProjectLocationDatasetConsentStoreAttributeDefinitionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAttributeDefinitionsResponse {
    /// The returned Attribute definitions. The maximum number of attributes returned is determined by the value of page_size in the ListAttributeDefinitionsRequest.
    #[serde(rename="attributeDefinitions")]
    
    pub attribute_definitions: Option<Vec<AttributeDefinition>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAttributeDefinitionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consent artifacts list projects](ProjectLocationDatasetConsentStoreConsentArtifactListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConsentArtifactsResponse {
    /// The returned Consent artifacts. The maximum number of artifacts returned is determined by the value of page_size in the ListConsentArtifactsRequest.
    #[serde(rename="consentArtifacts")]
    
    pub consent_artifacts: Option<Vec<ConsentArtifact>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListConsentArtifactsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consents list revisions projects](ProjectLocationDatasetConsentStoreConsentListRevisionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConsentRevisionsResponse {
    /// The returned Consent revisions. The maximum number of revisions returned is determined by the value of `page_size` in the ListConsentRevisionsRequest.
    
    pub consents: Option<Vec<Consent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListConsentRevisionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores list projects](ProjectLocationDatasetConsentStoreListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConsentStoresResponse {
    /// The returned consent stores. The maximum number of stores returned is determined by the value of page_size in the ListConsentStoresRequest.
    #[serde(rename="consentStores")]
    
    pub consent_stores: Option<Vec<ConsentStore>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListConsentStoresResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consents list projects](ProjectLocationDatasetConsentStoreConsentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListConsentsResponse {
    /// The returned Consents. The maximum number of Consents returned is determined by the value of page_size in the ListConsentsRequest.
    
    pub consents: Option<Vec<Consent>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListConsentsResponse {}


/// Lists the available datasets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets list projects](ProjectLocationDatasetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDatasetsResponse {
    /// The first page of datasets.
    
    pub datasets: Option<Vec<Dataset>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDatasetsResponse {}


/// Lists the DICOM stores in the given dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets dicom stores list projects](ProjectLocationDatasetDicomStoreListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDicomStoresResponse {
    /// The returned DICOM stores. Won't be more DICOM stores than the value of page_size in the request.
    #[serde(rename="dicomStores")]
    
    pub dicom_stores: Option<Vec<DicomStore>>,
    /// Token to retrieve the next page of results or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDicomStoresResponse {}


/// Lists the FHIR stores in the given dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets fhir stores list projects](ProjectLocationDatasetFhirStoreListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFhirStoresResponse {
    /// The returned FHIR stores. Won't be more FHIR stores than the value of page_size in the request.
    #[serde(rename="fhirStores")]
    
    pub fhir_stores: Option<Vec<FhirStore>>,
    /// Token to retrieve the next page of results or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListFhirStoresResponse {}


/// Lists the HL7v2 stores in the given dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores list projects](ProjectLocationDatasetHl7V2StoreListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHl7V2StoresResponse {
    /// The returned HL7v2 stores. Won't be more HL7v2 stores than the value of page_size in the request.
    #[serde(rename="hl7V2Stores")]
    
    pub hl7_v2_stores: Option<Vec<Hl7V2Store>>,
    /// Token to retrieve the next page of results or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListHl7V2StoresResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// Lists the messages in the specified HL7v2 store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores messages list projects](ProjectLocationDatasetHl7V2StoreMessageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMessagesResponse {
    /// The returned Messages. Won't be more Messages than the value of page_size in the request. See view for populated fields.
    #[serde(rename="hl7V2Messages")]
    
    pub hl7_v2_messages: Option<Vec<Message>>,
    /// Token to retrieve the next page of results or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListMessagesResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets operations list projects](ProjectLocationDatasetOperationListCall) (response)
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


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores user data mappings list projects](ProjectLocationDatasetConsentStoreUserDataMappingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserDataMappingsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The returned User data mappings. The maximum number of User data mappings returned is determined by the value of page_size in the ListUserDataMappingsRequest.
    #[serde(rename="userDataMappings")]
    
    pub user_data_mappings: Option<Vec<UserDataMapping>>,
}

impl client::ResponseResult for ListUserDataMappingsResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// A complete HL7v2 message. See \[Introduction to HL7 Standards\] (https://www.hl7.org/implement/standards/index.cfm?ref=common) for details on the standard.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets hl7 v2 stores messages create projects](ProjectLocationDatasetHl7V2StoreMessageCreateCall) (response)
/// * [locations datasets hl7 v2 stores messages get projects](ProjectLocationDatasetHl7V2StoreMessageGetCall) (response)
/// * [locations datasets hl7 v2 stores messages patch projects](ProjectLocationDatasetHl7V2StoreMessagePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// Output only. The datetime when the message was created. Set by the server.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Raw message bytes.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// User-supplied key-value pairs used to organize HL7v2 stores. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store.
    
    pub labels: Option<HashMap<String, String>>,
    /// The message type for this message. MSH-9.1.
    #[serde(rename="messageType")]
    
    pub message_type: Option<String>,
    /// Resource name of the Message, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/hl7V2Stores/{hl7_v2_store_id}/messages/{message_id}`. Assigned by the server.
    
    pub name: Option<String>,
    /// Output only. The parsed version of the raw message data.
    #[serde(rename="parsedData")]
    
    pub parsed_data: Option<ParsedData>,
    /// All patient IDs listed in the PID-2, PID-3, and PID-4 segments of this message.
    #[serde(rename="patientIds")]
    
    pub patient_ids: Option<Vec<PatientId>>,
    /// The parsed version of the raw message data schematized according to this store's schemas and type definitions.
    #[serde(rename="schematizedData")]
    
    pub schematized_data: Option<SchematizedData>,
    /// The hospital that this message came from. MSH-4.
    #[serde(rename="sendFacility")]
    
    pub send_facility: Option<String>,
    /// The datetime the sending application sent this message. MSH-7.
    #[serde(rename="sendTime")]
    
    pub send_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Message {}
impl client::ResponseResult for Message {}


/// Specifies where to send notifications upon changes to a data store.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// The [Pub/Sub](https://cloud.google.com/pubsub/docs/) topic that notifications of changes are published on. Supplied by the client. PubsubMessage.Data contains the resource name. PubsubMessage.MessageId is the ID of this message. It is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message was published. Notifications are only sent if the topic is non-empty. [Topic names](https://cloud.google.com/pubsub/docs/overview#names) must be scoped to a project. Cloud Healthcare API service account must have publisher permissions on the given Pub/Sub topic. Not having adequate permissions causes the calls that send notifications to fail. If a notification can't be published to Pub/Sub, errors are logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)). If the number of errors exceeds a certain rate, some aren't submitted. Note that not all operations trigger notifications, see [Configuring Pub/Sub notifications](https://cloud.google.com/healthcare/docs/how-tos/pubsub) for specific details.
    #[serde(rename="pubsubTopic")]
    
    pub pubsub_topic: Option<String>,
}

impl client::Part for NotificationConfig {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores query accessible data projects](ProjectLocationDatasetConsentStoreQueryAccessibleDataCall) (response)
/// * [locations datasets dicom stores studies series delete projects](ProjectLocationDatasetDicomStoreStudySeriesDeleteCall) (response)
/// * [locations datasets dicom stores studies delete projects](ProjectLocationDatasetDicomStoreStudyDeleteCall) (response)
/// * [locations datasets dicom stores deidentify projects](ProjectLocationDatasetDicomStoreDeidentifyCall) (response)
/// * [locations datasets dicom stores export projects](ProjectLocationDatasetDicomStoreExportCall) (response)
/// * [locations datasets dicom stores import projects](ProjectLocationDatasetDicomStoreImportCall) (response)
/// * [locations datasets fhir stores deidentify projects](ProjectLocationDatasetFhirStoreDeidentifyCall) (response)
/// * [locations datasets fhir stores export projects](ProjectLocationDatasetFhirStoreExportCall) (response)
/// * [locations datasets fhir stores import projects](ProjectLocationDatasetFhirStoreImportCall) (response)
/// * [locations datasets hl7 v2 stores export projects](ProjectLocationDatasetHl7V2StoreExportCall) (response)
/// * [locations datasets hl7 v2 stores import projects](ProjectLocationDatasetHl7V2StoreImportCall) (response)
/// * [locations datasets operations get projects](ProjectLocationDatasetOperationGetCall) (response)
/// * [locations datasets create projects](ProjectLocationDatasetCreateCall) (response)
/// * [locations datasets deidentify projects](ProjectLocationDatasetDeidentifyCall) (response)
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


/// The content of a HL7v2 message in a structured format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParsedData {
    /// no description provided
    
    pub segments: Option<Vec<Segment>>,
}

impl client::Part for ParsedData {}


/// The configuration for the parser. It determines how the server parses the messages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Determines whether messages with no header are allowed.
    #[serde(rename="allowNullHeader")]
    
    pub allow_null_header: Option<bool>,
    /// Schemas used to parse messages in this store, if schematized parsing is desired.
    
    pub schema: Option<SchemaPackage>,
    /// Byte(s) to use as the segment terminator. If this is unset, '\r' is used as segment terminator, matching the HL7 version 2 specification.
    #[serde(rename="segmentTerminator")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub segment_terminator: Option<Vec<u8>>,
    /// Immutable. Determines the version of both the default parser to be used when `schema` is not given, as well as the schematized parser used when `schema` is specified. This field is immutable after HL7v2 store creation.
    
    pub version: Option<ParserConfigVersionEnum>,
}

impl client::Part for ParserConfig {}


/// A patient identifier and associated type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PatientId {
    /// ID type. For example, MRN or NHS.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The patient's unique identifier.
    
    pub value: Option<String>,
}

impl client::Part for PatientId {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores get iam policy projects](ProjectLocationDatasetConsentStoreGetIamPolicyCall) (response)
/// * [locations datasets consent stores set iam policy projects](ProjectLocationDatasetConsentStoreSetIamPolicyCall) (response)
/// * [locations datasets dicom stores get iam policy projects](ProjectLocationDatasetDicomStoreGetIamPolicyCall) (response)
/// * [locations datasets dicom stores set iam policy projects](ProjectLocationDatasetDicomStoreSetIamPolicyCall) (response)
/// * [locations datasets fhir stores get iam policy projects](ProjectLocationDatasetFhirStoreGetIamPolicyCall) (response)
/// * [locations datasets fhir stores set iam policy projects](ProjectLocationDatasetFhirStoreSetIamPolicyCall) (response)
/// * [locations datasets hl7 v2 stores get iam policy projects](ProjectLocationDatasetHl7V2StoreGetIamPolicyCall) (response)
/// * [locations datasets hl7 v2 stores set iam policy projects](ProjectLocationDatasetHl7V2StoreSetIamPolicyCall) (response)
/// * [locations datasets get iam policy projects](ProjectLocationDatasetGetIamPolicyCall) (response)
/// * [locations datasets set iam policy projects](ProjectLocationDatasetSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Queries all data_ids that are consented for a given use in the given consent store and writes them to a specified destination. The returned Operation includes a progress counter for the number of User data mappings processed. Errors are logged to Cloud Logging (see \[Viewing error logs in Cloud Logging\] (https://cloud.google.com/healthcare/docs/how-tos/logging) and \[QueryAccessibleData\] for a sample log entry).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores query accessible data projects](ProjectLocationDatasetConsentStoreQueryAccessibleDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryAccessibleDataRequest {
    /// The Cloud Storage destination. The Cloud Healthcare API service account must have the `roles/storage.objectAdmin` Cloud IAM role for this Cloud Storage location.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GoogleCloudHealthcareV1ConsentGcsDestination>,
    /// The values of request attributes associated with this access request.
    #[serde(rename="requestAttributes")]
    
    pub request_attributes: Option<HashMap<String, String>>,
    /// Optional. The values of resource attributes associated with the type of resources being requested. If no values are specified, then all resource types are included in the output.
    #[serde(rename="resourceAttributes")]
    
    pub resource_attributes: Option<HashMap<String, String>>,
}

impl client::RequestValue for QueryAccessibleDataRequest {}


/// Define how to redact sensitive values. Default behaviour is erase. For example, "My name is Jane." becomes "My name is ."
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RedactConfig { _never_set: Option<bool> }

impl client::Part for RedactConfig {}


/// Rejects the latest revision of the specified Consent by committing a new revision with `state` updated to `REJECTED`. If the latest revision of the given Consent is in the `REJECTED` state, no new revision is committed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consents reject projects](ProjectLocationDatasetConsentStoreConsentRejectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RejectConsentRequest {
    /// Optional. The resource name of the Consent artifact that contains documentation of the user's rejection of the draft Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`. If the draft Consent had a Consent artifact, this Consent artifact overwrites it.
    #[serde(rename="consentArtifact")]
    
    pub consent_artifact: Option<String>,
}

impl client::RequestValue for RejectConsentRequest {}


/// When using the INSPECT_AND_TRANSFORM action, each match is replaced with the name of the info_type. For example, "My name is Jane" becomes "My name is [PERSON_NAME]." The TRANSFORM action is equivalent to redacting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplaceWithInfoTypeConfig { _never_set: Option<bool> }

impl client::Part for ReplaceWithInfoTypeConfig {}


/// A list of FHIR resources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resources {
    /// List of resources IDs. For example, "Patient/1234".
    
    pub resources: Option<Vec<String>>,
}

impl client::Part for Resources {}


/// The consent evaluation result for a single `data_id`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Result {
    /// The resource names of all evaluated Consents mapped to their evaluation.
    #[serde(rename="consentDetails")]
    
    pub consent_details: Option<HashMap<String, ConsentEvaluation>>,
    /// Whether the resource is consented for the given use.
    
    pub consented: Option<bool>,
    /// The unique identifier of the evaluated resource.
    #[serde(rename="dataId")]
    
    pub data_id: Option<String>,
}

impl client::Part for Result {}


/// Revokes the latest revision of the specified Consent by committing a new revision with `state` updated to `REVOKED`. If the latest revision of the given Consent is in the `REVOKED` state, no new revision is committed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores consents revoke projects](ProjectLocationDatasetConsentStoreConsentRevokeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RevokeConsentRequest {
    /// Optional. The resource name of the Consent artifact that contains proof of the user's revocation of the Consent, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/consentArtifacts/{consent_artifact_id}`.
    #[serde(rename="consentArtifact")]
    
    pub consent_artifact: Option<String>,
}

impl client::RequestValue for RevokeConsentRequest {}


/// Configuration for the FHIR BigQuery schema. Determines how the server generates the schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchemaConfig {
    /// The depth for all recursive structures in the output analytics schema. For example, `concept` in the CodeSystem resource is a recursive structure; when the depth is 2, the CodeSystem table will have a column called `concept.concept` but not `concept.concept.concept`. If not specified or set to 0, the server will use the default value 2. The maximum depth allowed is 5.
    #[serde(rename="recursiveStructureDepth")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub recursive_structure_depth: Option<i64>,
    /// Specifies the output schema type. Schema type is required.
    #[serde(rename="schemaType")]
    
    pub schema_type: Option<SchemaConfigSchemaTypeEnum>,
}

impl client::Part for SchemaConfig {}


/// An HL7v2 logical group construct.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchemaGroup {
    /// True indicates that this is a choice group, meaning that only one of its segments can exist in a given message.
    
    pub choice: Option<bool>,
    /// The maximum number of times this group can be repeated. 0 or -1 means unbounded.
    #[serde(rename="maxOccurs")]
    
    pub max_occurs: Option<i32>,
    /// Nested groups and/or segments.
    
    pub members: Option<Vec<GroupOrSegment>>,
    /// The minimum number of times this group must be present/repeated.
    #[serde(rename="minOccurs")]
    
    pub min_occurs: Option<i32>,
    /// The name of this group. For example, "ORDER_DETAIL".
    
    pub name: Option<String>,
}

impl client::Part for SchemaGroup {}


/// A schema package contains a set of schemas and type definitions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchemaPackage {
    /// Flag to ignore all min_occurs restrictions in the schema. This means that incoming messages can omit any group, segment, field, component, or subcomponent.
    #[serde(rename="ignoreMinOccurs")]
    
    pub ignore_min_occurs: Option<bool>,
    /// Schema configs that are layered based on their VersionSources that match the incoming message. Schema configs present in higher indices override those in lower indices with the same message type and trigger event if their VersionSources all match an incoming message.
    
    pub schemas: Option<Vec<Hl7SchemaConfig>>,
    /// Determines how messages that fail to parse are handled.
    #[serde(rename="schematizedParsingType")]
    
    pub schematized_parsing_type: Option<SchemaPackageSchematizedParsingTypeEnum>,
    /// Schema type definitions that are layered based on their VersionSources that match the incoming message. Type definitions present in higher indices override those in lower indices with the same type name if their VersionSources all match an incoming message.
    
    pub types: Option<Vec<Hl7TypesConfig>>,
    /// Determines how unexpected segments (segments not matched to the schema) are handled.
    #[serde(rename="unexpectedSegmentHandling")]
    
    pub unexpected_segment_handling: Option<SchemaPackageUnexpectedSegmentHandlingEnum>,
}

impl client::Part for SchemaPackage {}


/// An HL7v2 Segment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchemaSegment {
    /// The maximum number of times this segment can be present in this group. 0 or -1 means unbounded.
    #[serde(rename="maxOccurs")]
    
    pub max_occurs: Option<i32>,
    /// The minimum number of times this segment can be present in this group.
    #[serde(rename="minOccurs")]
    
    pub min_occurs: Option<i32>,
    /// The Segment type. For example, "PID".
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for SchemaSegment {}


/// The content of an HL7v2 message in a structured format as specified by a schema.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SchematizedData {
    /// JSON output of the parser.
    
    pub data: Option<String>,
    /// The error output of the parser.
    
    pub error: Option<String>,
}

impl client::Part for SchematizedData {}


/// Request to search the resources in the specified FHIR store.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets fhir stores fhir search projects](ProjectLocationDatasetFhirStoreFhirSearchCall) (request)
/// * [locations datasets fhir stores fhir search-type projects](ProjectLocationDatasetFhirStoreFhirSearchTypeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchResourcesRequest {
    /// The FHIR resource type to search, such as Patient or Observation. For a complete list, see the FHIR Resource Index ([DSTU2](http://hl7.org/implement/standards/fhir/DSTU2/resourcelist.html), [STU3](http://hl7.org/implement/standards/fhir/STU3/resourcelist.html), [R4](http://hl7.org/implement/standards/fhir/R4/resourcelist.html)).
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
}

impl client::RequestValue for SearchResourcesRequest {}


/// A segment in a structured format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Segment {
    /// A mapping from the positional location to the value. The key string uses zero-based indexes separated by dots to identify Fields, components and sub-components. A bracket notation is also used to identify different instances of a repeated field. Regex for key: (\d+)(\[\d+\])?(.\d+)?(.\d+)? Examples of (key, value) pairs: * (0.1, "hemoglobin") denotes that the first component of Field 0 has the value "hemoglobin". * (1.1.2, "CBC") denotes that the second sub-component of the first component of Field 1 has the value "CBC". * (1[0].1, "HbA1c") denotes that the first component of the first Instance of Field 1, which is repeated, has the value "HbA1c".
    
    pub fields: Option<HashMap<String, String>>,
    /// A string that indicates the type of segment. For example, EVN or PID.
    #[serde(rename="segmentId")]
    
    pub segment_id: Option<String>,
    /// Set ID for segments that can be in a set. This can be empty if it's missing or isn't applicable.
    #[serde(rename="setId")]
    
    pub set_id: Option<String>,
}

impl client::Part for Segment {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores set iam policy projects](ProjectLocationDatasetConsentStoreSetIamPolicyCall) (request)
/// * [locations datasets dicom stores set iam policy projects](ProjectLocationDatasetDicomStoreSetIamPolicyCall) (request)
/// * [locations datasets fhir stores set iam policy projects](ProjectLocationDatasetFhirStoreSetIamPolicyCall) (request)
/// * [locations datasets hl7 v2 stores set iam policy projects](ProjectLocationDatasetHl7V2StoreSetIamPolicyCall) (request)
/// * [locations datasets set iam policy projects](ProjectLocationDatasetSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// User signature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
    /// Optional. An image of the user's signature.
    
    pub image: Option<Image>,
    /// Optional. Metadata associated with the user's signature. For example, the user's name or the user's title.
    
    pub metadata: Option<HashMap<String, String>>,
    /// Optional. Timestamp of the signature.
    #[serde(rename="signatureTime")]
    
    pub signature_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. User's UUID provided by the client.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for Signature {}


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


/// Contains configuration for streaming FHIR export.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StreamConfig {
    /// The destination BigQuery structure that contains both the dataset location and corresponding schema config. The output is organized in one table per resource type. The server reuses the existing tables (if any) that are named after the resource types. For example, "Patient", "Observation". When there is no existing table for a given resource type, the server attempts to create one. When a table schema doesn't align with the schema config, either because of existing incompatible schema or out of band incompatible modification, the server does not stream in new data. BigQuery imposes a 1 MB limit on streaming insert row size, therefore any resource mutation that generates more than 1 MB of BigQuery data is not streamed. One resolution in this case is to delete the incompatible table and let the server recreate one, though the newly created table only contains data after the table recreation. Results are written to BigQuery tables according to the parameters in BigQueryDestination.WriteDisposition. Different versions of the same resource are distinguishable by the meta.versionId and meta.lastUpdated columns. The operation (CREATE/UPDATE/DELETE) that results in the new version is recorded in the meta.tag. The tables contain all historical resource versions since streaming was enabled. For query convenience, the server also creates one view per table of the same name containing only the current resource version. The streamed data in the BigQuery dataset is not guaranteed to be completely unique. The combination of the id and meta.versionId columns should ideally identify a single unique row. But in rare cases, duplicates may exist. At query time, users may use the SQL select statement to keep only one of the duplicate rows given an id and meta.versionId pair. Alternatively, the server created view mentioned above also filters out duplicates. If a resource mutation cannot be streamed to BigQuery, errors are logged to Cloud Logging. For more information, see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)).
    #[serde(rename="bigqueryDestination")]
    
    pub bigquery_destination: Option<GoogleCloudHealthcareV1FhirBigQueryDestination>,
    /// The destination FHIR store for de-identified resources. After this field is added, all subsequent creates/updates/patches to the source store will be de-identified using the provided configuration and applied to the destination store. Importing resources to the source store will not trigger the streaming. If the source store already contains resources when this option is enabled, those resources will not be copied to the destination store unless they are subsequently updated. This may result in invalid references in the destination store. Before adding this config, you must grant the healthcare.fhirResources.update permission on the destination store to your project's **Cloud Healthcare Service Agent** [service account](https://cloud.google.com/healthcare/docs/how-tos/permissions-healthcare-api-gcp-products#the_cloud_healthcare_service_agent). The destination store must set enable_update_create to true. The destination store must have disable_referential_integrity set to true. If a resource cannot be de-identified, errors will be logged to Cloud Logging (see [Viewing error logs in Cloud Logging](https://cloud.google.com/healthcare/docs/how-tos/logging)).
    #[serde(rename="deidentifiedStoreDestination")]
    
    pub deidentified_store_destination: Option<DeidentifiedStoreDestination>,
    /// Supply a FHIR resource type (such as "Patient" or "Observation"). See https://www.hl7.org/fhir/valueset-resource-types.html for a list of all FHIR resource types. The server treats an empty list as an intent to stream all the supported resource types in this FHIR store.
    #[serde(rename="resourceTypes")]
    
    pub resource_types: Option<Vec<String>>,
}

impl client::Part for StreamConfig {}


/// List of tags to be filtered.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TagFilterList {
    /// Tags to be filtered. Tags must be DICOM Data Elements, File Meta Elements, or Directory Structuring Elements, as defined at: http://dicom.nema.org/medical/dicom/current/output/html/part06.html#table_6-1,. They may be provided by "Keyword" or "Tag". For example "PatientID", "00100010".
    
    pub tags: Option<Vec<String>>,
}

impl client::Part for TagFilterList {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores test iam permissions projects](ProjectLocationDatasetConsentStoreTestIamPermissionCall) (request)
/// * [locations datasets dicom stores test iam permissions projects](ProjectLocationDatasetDicomStoreTestIamPermissionCall) (request)
/// * [locations datasets fhir stores test iam permissions projects](ProjectLocationDatasetFhirStoreTestIamPermissionCall) (request)
/// * [locations datasets hl7 v2 stores test iam permissions projects](ProjectLocationDatasetHl7V2StoreTestIamPermissionCall) (request)
/// * [locations datasets test iam permissions projects](ProjectLocationDatasetTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores test iam permissions projects](ProjectLocationDatasetConsentStoreTestIamPermissionCall) (response)
/// * [locations datasets dicom stores test iam permissions projects](ProjectLocationDatasetDicomStoreTestIamPermissionCall) (response)
/// * [locations datasets fhir stores test iam permissions projects](ProjectLocationDatasetFhirStoreTestIamPermissionCall) (response)
/// * [locations datasets hl7 v2 stores test iam permissions projects](ProjectLocationDatasetHl7V2StoreTestIamPermissionCall) (response)
/// * [locations datasets test iam permissions projects](ProjectLocationDatasetTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextConfig {
    /// The transformations to apply to the detected data.
    
    pub transformations: Option<Vec<InfoTypeTransformation>>,
}

impl client::Part for TextConfig {}


/// A span of text in the provided document.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextSpan {
    /// The unicode codepoint index of the beginning of this span.
    #[serde(rename="beginOffset")]
    
    pub begin_offset: Option<i32>,
    /// The original text contained in this span.
    
    pub content: Option<String>,
}

impl client::Part for TextSpan {}


/// A type definition for some HL7v2 type (incl. Segments and Datatypes).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Type {
    /// The (sub) fields this type has (if not primitive).
    
    pub fields: Option<Vec<Field>>,
    /// The name of this type. This would be the segment or datatype name. For example, "PID" or "XPN".
    
    pub name: Option<String>,
    /// If this is a primitive type then this field is the type of the primitive For example, STRING. Leave unspecified for composite types.
    
    pub primitive: Option<TypePrimitiveEnum>,
}

impl client::Part for Type {}


/// Maps a resource to the associated user and Attributes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations datasets consent stores user data mappings create projects](ProjectLocationDatasetConsentStoreUserDataMappingCreateCall) (request|response)
/// * [locations datasets consent stores user data mappings get projects](ProjectLocationDatasetConsentStoreUserDataMappingGetCall) (response)
/// * [locations datasets consent stores user data mappings patch projects](ProjectLocationDatasetConsentStoreUserDataMappingPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDataMapping {
    /// Output only. Indicates the time when this mapping was archived.
    #[serde(rename="archiveTime")]
    
    pub archive_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Indicates whether this mapping is archived.
    
    pub archived: Option<bool>,
    /// Required. A unique identifier for the mapped resource.
    #[serde(rename="dataId")]
    
    pub data_id: Option<String>,
    /// Resource name of the User data mapping, of the form `projects/{project_id}/locations/{location_id}/datasets/{dataset_id}/consentStores/{consent_store_id}/userDataMappings/{user_data_mapping_id}`.
    
    pub name: Option<String>,
    /// Attributes of the resource. Only explicitly set attributes are displayed here. Attribute definitions with defaults set implicitly apply to these User data mappings. Attributes listed here must be single valued, that is, exactly one value is specified for the field "values" in each Attribute.
    #[serde(rename="resourceAttributes")]
    
    pub resource_attributes: Option<Vec<Attribute>>,
    /// Required. User's UUID provided by the client.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::RequestValue for UserDataMapping {}
impl client::ResponseResult for UserDataMapping {}


/// Contains the configuration for FHIR profiles and validation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Whether to disable FHIRPath validation for incoming resources. Set this to true to disable checking incoming resources for conformance against FHIRPath requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced.
    #[serde(rename="disableFhirpathValidation")]
    
    pub disable_fhirpath_validation: Option<bool>,
    /// Whether to disable profile validation for this FHIR store. Set this to true to disable checking incoming resources for conformance against structure definitions in this FHIR store.
    #[serde(rename="disableProfileValidation")]
    
    pub disable_profile_validation: Option<bool>,
    /// Whether to disable reference type validation for incoming resources. Set this to true to disable checking incoming resources for conformance against reference type requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced.
    #[serde(rename="disableReferenceTypeValidation")]
    
    pub disable_reference_type_validation: Option<bool>,
    /// Whether to disable required fields validation for incoming resources. Set this to true to disable checking incoming resources for conformance against required fields requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced.
    #[serde(rename="disableRequiredFieldValidation")]
    
    pub disable_required_field_validation: Option<bool>,
    /// A list of implementation guide URLs in this FHIR store that are used to configure the profiles to use for validation. For example, to use the US Core profiles for validation, set `enabled_implementation_guides` to `["http://hl7.org/fhir/us/core/ImplementationGuide/ig"]`. If `enabled_implementation_guides` is empty or omitted, then incoming resources are only required to conform to the base FHIR profiles. Otherwise, a resource must conform to at least one profile listed in the `global` property of one of the enabled ImplementationGuides. The Cloud Healthcare API does not currently enforce all of the rules in a StructureDefinition. The following rules are supported: - min/max - minValue/maxValue - maxLength - type - fixed[x] - pattern[x] on simple types - slicing, when using "value" as the discriminator type When a URL cannot be resolved (for example, in a type assertion), the server does not return an error.
    #[serde(rename="enabledImplementationGuides")]
    
    pub enabled_implementation_guides: Option<Vec<String>>,
}

impl client::Part for ValidationConfig {}


/// Describes a selector for extracting and matching an MSH field to a value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VersionSource {
    /// The field to extract from the MSH segment. For example, "3.1" or "18[1].1".
    #[serde(rename="mshField")]
    
    pub msh_field: Option<String>,
    /// The value to match with the field. For example, "My Application Name" or "2.3".
    
    pub value: Option<String>,
}

impl client::Part for VersionSource {}


