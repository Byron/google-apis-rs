use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updateaggregatedstats statscollection](StatscollectionUpdateaggregatedstatCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregatedStats {
    /// no description provided
    
    pub stats: Option<Vec<Stats>>,
}

impl client::RequestValue for AggregatedStats {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updateaggregatedstats statscollection](StatscollectionUpdateaggregatedstatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregatedStatsReply {
    /// no description provided
    #[serde(rename="testValue")]
    
    pub test_value: Option<String>,
}

impl client::ResponseResult for AggregatedStatsReply {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleValue {
    /// no description provided
    
    pub label: Option<String>,
    /// no description provided
    
    pub value: Option<f32>,
}

impl client::Part for DoubleValue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntValue {
    /// no description provided
    
    pub label: Option<String>,
    /// no description provided
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub value: Option<i64>,
}

impl client::Part for IntValue {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatestats statscollection](StatscollectionUpdatestatCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    /// no description provided
    #[serde(rename="doubleValues")]
    
    pub double_values: Option<Vec<DoubleValue>>,
    /// no description provided
    #[serde(rename="intValues")]
    
    pub int_values: Option<Vec<IntValue>>,
    /// no description provided
    #[serde(rename="stringValues")]
    
    pub string_values: Option<Vec<StringValue>>,
    /// no description provided
    
    pub time: Option<f64>,
}

impl client::RequestValue for Stats {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [updatestats statscollection](StatscollectionUpdatestatCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatsReply {
    /// no description provided
    #[serde(rename="testValue")]
    
    pub test_value: Option<String>,
}

impl client::ResponseResult for StatsReply {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringValue {
    /// no description provided
    
    pub label: Option<String>,
    /// no description provided
    
    pub value: Option<String>,
}

impl client::Part for StringValue {}


