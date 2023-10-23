use super::*;
/// The response from ListSettings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings list folders](FolderSettingListCall) (response)
/// * [settings list organizations](OrganizationSettingListCall) (response)
/// * [settings list projects](ProjectSettingListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudResourcesettingsV1ListSettingsResponse {
    /// Unused. A page token used to retrieve the next page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of settings that are available at the specified Cloud resource.
    
    pub settings: Option<Vec<GoogleCloudResourcesettingsV1Setting>>,
}

impl client::ResponseResult for GoogleCloudResourcesettingsV1ListSettingsResponse {}


/// The schema for settings.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings get folders](FolderSettingGetCall) (response)
/// * [settings patch folders](FolderSettingPatchCall) (request|response)
/// * [settings get organizations](OrganizationSettingGetCall) (response)
/// * [settings patch organizations](OrganizationSettingPatchCall) (request|response)
/// * [settings get projects](ProjectSettingGetCall) (response)
/// * [settings patch projects](ProjectSettingPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudResourcesettingsV1Setting {
    /// Output only. The effective value of the setting at the given parent resource, evaluated based on the resource hierarchy The effective value evaluates to one of the following options, in this order. If an option is not valid or doesn't exist, then the next option is used: 1. The local setting value on the given resource: Setting.local_value 2. If one of the given resource's ancestors in the resource hierarchy have a local setting value, the local value at the nearest such ancestor. 3. The setting's default value: SettingMetadata.default_value 4. An empty value, defined as a `Value` with all fields unset. The data type of Value must always be consistent with the data type defined in Setting.metadata.
    #[serde(rename="effectiveValue")]
    
    pub effective_value: Option<GoogleCloudResourcesettingsV1Value>,
    /// A fingerprint used for optimistic concurrency. See UpdateSetting for more details.
    
    pub etag: Option<String>,
    /// The configured value of the setting at the given parent resource, ignoring the resource hierarchy. The data type of Value must always be consistent with the data type defined in Setting.metadata.
    #[serde(rename="localValue")]
    
    pub local_value: Option<GoogleCloudResourcesettingsV1Value>,
    /// Output only. Metadata about a setting which is not editable by the end user.
    
    pub metadata: Option<GoogleCloudResourcesettingsV1SettingMetadata>,
    /// The resource name of the setting. Must be in one of the following forms: * `projects/{project_number}/settings/{setting_name}` * `folders/{folder_id}/settings/{setting_name}` * `organizations/{organization_id}/settings/{setting_name}` For example, "/projects/123/settings/gcp-enableMyFeature"
    
    pub name: Option<String>,
}

impl client::RequestValue for GoogleCloudResourcesettingsV1Setting {}
impl client::ResponseResult for GoogleCloudResourcesettingsV1Setting {}


/// Metadata about a setting which is not editable by the end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudResourcesettingsV1SettingMetadata {
    /// The data type for this setting.
    #[serde(rename="dataType")]
    
    pub data_type: Option<GoogleCloudResourcesettingsV1SettingMetadataDataTypeEnum>,
    /// The value provided by Setting.effective_value if no setting value is explicitly set. Note: not all settings have a default value.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<GoogleCloudResourcesettingsV1Value>,
    /// A detailed description of what this setting does.
    
    pub description: Option<String>,
    /// The human readable name for this setting.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// A flag indicating that values of this setting cannot be modified. See documentation for the specific setting for updates and reasons.
    #[serde(rename="readOnly")]
    
    pub read_only: Option<bool>,
}

impl client::Part for GoogleCloudResourcesettingsV1SettingMetadata {}


/// The data in a setting value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudResourcesettingsV1Value {
    /// Defines this value as being a boolean value.
    #[serde(rename="booleanValue")]
    
    pub boolean_value: Option<bool>,
    /// Defines this value as being a Duration.
    #[serde(rename="durationValue")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration_value: Option<client::chrono::Duration>,
    /// Defines this value as being a Enum.
    #[serde(rename="enumValue")]
    
    pub enum_value: Option<GoogleCloudResourcesettingsV1ValueEnumValue>,
    /// Defines this value as being a StringMap.
    #[serde(rename="stringMapValue")]
    
    pub string_map_value: Option<GoogleCloudResourcesettingsV1ValueStringMap>,
    /// Defines this value as being a StringSet.
    #[serde(rename="stringSetValue")]
    
    pub string_set_value: Option<GoogleCloudResourcesettingsV1ValueStringSet>,
    /// Defines this value as being a string value.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for GoogleCloudResourcesettingsV1Value {}


/// A enum value that can hold any enum type setting values. Each enum type is represented by a number, this representation is stored in the definitions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudResourcesettingsV1ValueEnumValue {
    /// The value of this enum
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudResourcesettingsV1ValueEnumValue {}


/// A string->string map value that can hold a map of string keys to string values. The maximum length of each string is 200 characters and there can be a maximum of 50 key-value pairs in the map.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudResourcesettingsV1ValueStringMap {
    /// The key-value pairs in the map
    
    pub mappings: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudResourcesettingsV1ValueStringMap {}


/// A string set value that can hold a set of strings. The maximum length of each string is 200 characters and there can be a maximum of 50 strings in the string set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudResourcesettingsV1ValueStringSet {
    /// The strings in the set
    
    pub values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudResourcesettingsV1ValueStringSet {}


