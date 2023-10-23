use super::*;
/// Represents locales that are available for a web property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaAvailableLocale {
    /// A string in BCP 47 format, without a resource prefix.
    
    pub locale: Option<String>,
    /// A string in BCP 47 format, prefixed with the platform and property name, and "locales/". Format: platforms/{platform}/properties/{property}/locales/{locale}
    
    pub name: Option<String>,
}

impl client::Part for GoogleSearchIdeahubV1betaAvailableLocale {}


/// A single Idea that we want to show the end user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaIdea {
    /// Unique identifier for the idea. Format: ideas/{ideaId}
    
    pub name: Option<String>,
    /// The idea’s text.
    
    pub text: Option<String>,
    /// The Topics that match the idea.
    
    pub topics: Option<Vec<GoogleSearchIdeahubV1betaTopic>>,
}

impl client::Part for GoogleSearchIdeahubV1betaIdea {}


/// An idea activity entry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [properties idea activities create platforms](PlatformPropertyIdeaActivityCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaIdeaActivity {
    /// The Idea IDs for this entry. If empty, topics should be set.
    
    pub ideas: Option<Vec<String>>,
    /// Unique identifier for the idea activity. The name is ignored when creating an idea activity. Format: platforms/{platform}/properties/{property}/ideaActivities/{idea_activity}
    
    pub name: Option<String>,
    /// The Topic IDs for this entry. If empty, ideas should be set.
    
    pub topics: Option<Vec<String>>,
    /// The type of activity performed.
    #[serde(rename="type")]
    
    pub type_: Option<GoogleSearchIdeahubV1betaIdeaActivityTypeEnum>,
    /// The uri the activity relates to.
    
    pub uri: Option<String>,
}

impl client::RequestValue for GoogleSearchIdeahubV1betaIdeaActivity {}
impl client::ResponseResult for GoogleSearchIdeahubV1betaIdeaActivity {}


/// Represents idea state specific to a web property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [properties idea states patch platforms](PlatformPropertyIdeaStatePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaIdeaState {
    /// Whether the idea is dismissed.
    
    pub dismissed: Option<bool>,
    /// Unique identifier for the idea state. Format: platforms/{platform}/properties/{property}/ideaStates/{idea_state}
    
    pub name: Option<String>,
    /// Whether the idea is saved.
    
    pub saved: Option<bool>,
}

impl client::RequestValue for GoogleSearchIdeahubV1betaIdeaState {}
impl client::ResponseResult for GoogleSearchIdeahubV1betaIdeaState {}


/// Response for whether ideas are available for a given web property on a platform, for the currently logged-in user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [properties locales list platforms](PlatformPropertyLocaleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaListAvailableLocalesResponse {
    /// Locales for which ideas are available for the given Creator.
    #[serde(rename="availableLocales")]
    
    pub available_locales: Option<Vec<GoogleSearchIdeahubV1betaAvailableLocale>>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleSearchIdeahubV1betaListAvailableLocalesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [properties ideas list platforms](PlatformPropertyIdeaListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaListIdeasResponse {
    /// Results for the ListIdeasRequest.
    
    pub ideas: Option<Vec<GoogleSearchIdeahubV1betaIdea>>,
    /// Used to fetch the next page in a subsequent request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleSearchIdeahubV1betaListIdeasResponse {}


/// Represents a Topic umbrella for a list of questions that a Creator may want to respond to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaTopic {
    /// String displayed to the creator indicating the name of the Topic.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The mID of the topic.
    
    pub mid: Option<String>,
    /// Unique identifier for the topic. Format: topics/{topic}
    
    pub name: Option<String>,
}

impl client::Part for GoogleSearchIdeahubV1betaTopic {}


/// Represents topic state specific to a web property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [properties topic states patch platforms](PlatformPropertyTopicStatePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSearchIdeahubV1betaTopicState {
    /// Whether the topic is dismissed.
    
    pub dismissed: Option<bool>,
    /// Unique identifier for the topic state. Format: platforms/{platform}/properties/{property}/topicStates/{topic_state}
    
    pub name: Option<String>,
    /// Whether the topic is saved.
    
    pub saved: Option<bool>,
}

impl client::RequestValue for GoogleSearchIdeahubV1betaTopicState {}
impl client::ResponseResult for GoogleSearchIdeahubV1betaTopicState {}


