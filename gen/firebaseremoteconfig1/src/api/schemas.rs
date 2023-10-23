use super::*;
/// While default_value and conditional_values are each optional, at least one of
/// the two is required - otherwise, the parameter is meaningless (and an
/// exception will be thrown by the validation logic).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoteConfigParameter {
    /// Optional - value to set the parameter to, when none of the named conditions
    /// evaluate to <code>true</code>.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<RemoteConfigParameterValue>,
    /// Optional.
    /// A description for this Parameter. Length must be less than or equal to
    /// 100 characters (or more precisely, unicode code points, which is defined in
    /// java/com/google/wireless/android/config/ConstsExporter.java).
    /// A description may contain any Unicode characters
    
    pub description: Option<String>,
    /// Optional - a map of (condition_name, value). The condition_name of the
    /// highest priority (the one listed first in the conditions array) determines
    /// the value of this parameter.
    #[serde(rename="conditionalValues")]
    
    pub conditional_values: Option<HashMap<String, RemoteConfigParameterValue>>,
}

impl client::Part for RemoteConfigParameter {}


/// A single RemoteConfig Condition.  A list of these (because order matters) are
/// part of a single RemoteConfig template.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoteConfigCondition {
    /// Optional.
    /// The display (tag) color of this condition. This serves as part of a tag
    /// (in the future, we may add tag text as well as tag color, but that is not
    /// yet implemented in the UI).
    /// This value has no affect on the semantics of the delivered config and it
    /// is ignored by the backend, except for passing it through write/read
    /// requests.
    /// Not having this value or having the "CONDITION_DISPLAY_COLOR_UNSPECIFIED"
    /// value (0) have the same meaning:  Let the UI choose any valid color when
    /// displaying the condition.
    #[serde(rename="tagColor")]
    
    pub tag_color: Option<RemoteConfigConditionTagColorEnum>,
    /// Required.
    /// A non empty and unique name of this condition.
    
    pub name: Option<String>,
    /// Optional.
    /// A description for this Condition. Length must be less than or equal to
    /// 100 characters (or more precisely, unicode code points, which is defined in
    /// java/com/google/wireless/android/config/ConstsExporter.java).
    /// A description may contain any Unicode characters
    
    pub description: Option<String>,
    /// Required.
    
    pub expression: Option<String>,
}

impl client::Part for RemoteConfigCondition {}


/// A RemoteConfigParameter's "value" (either the default value, or the value
/// associated with a condition name) is either a string, or the
/// "use_in_app_default" indicator (which means to leave out the parameter from
/// the returned <key, value> map that is the output of the parameter fetch).
/// We represent the "use_in_app_default" as a bool, but (when using the boolean
/// instead of the string) it should always be <code>true</code>.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoteConfigParameterValue {
    /// the string to set the parameter to
    
    pub value: Option<String>,
    /// if true, omit the parameter from the map of fetched parameter values
    #[serde(rename="useInAppDefault")]
    
    pub use_in_app_default: Option<bool>,
}

impl client::Part for RemoteConfigParameterValue {}


/// * 
/// 
/// The RemoteConfig consists of a list of conditions (which can be
/// thought of as named “if” statements) and a map of parameters (parameter key
/// to a structure containing an optional default value, as well as a optional
/// submap of (condition name to value when that condition is true).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get remote config projects](ProjectGetRemoteConfigCall) (response)
/// * [update remote config projects](ProjectUpdateRemoteConfigCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoteConfig {
    /// Map of parameter keys to their optional default values and optional submap
    /// of (condition name : value). Order doesn't affect semantics, and so is
    /// sorted by the server. The 'key' values of the params must be unique.
    
    pub parameters: Option<HashMap<String, RemoteConfigParameter>>,
    /// The list of named conditions. The order *does* affect the semantics.
    /// The condition_name values of these entries must be unique.
    /// 
    /// The resolved value of a config parameter P is determined as follow:
    /// * Let Y be the set of values from the submap of P that refer to conditions
    ///   that evaluate to <code>true</code>.
    /// * If Y is non empty, the value is taken from the specific submap in Y whose
    ///   condition_name is the earliest in this condition list.
    /// * Else, if P has a default value option (condition_name is empty) then
    ///   the value is taken from that option.
    /// * Else, parameter P has no value and is omitted from the config result.
    /// 
    /// Example: parameter key "p1", default value "v1", submap specified as
    /// {"c1": v2, "c2": v3} where "c1" and "c2" are names of conditions in the
    /// condition list (where "c1" in this example appears before "c2").  The
    /// value of p1 would be v2 as long as c1 is true.  Otherwise, if c2 is true,
    /// p1 would evaluate to v3, and if c1 and c2 are both false, p1 would evaluate
    /// to v1.  If no default value was specified, and c1 and c2 were both false,
    /// no value for p1 would be generated.
    
    pub conditions: Option<Vec<RemoteConfigCondition>>,
}

impl client::RequestValue for RemoteConfig {}
impl client::ResponseResult for RemoteConfig {}


