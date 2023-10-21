use super::*;
/// CreateProfileRequest describes a profile resource online creation request. The deployment field must be populated. The profile_type specifies the list of profile types supported by the agent. The creation call will hang until a profile of one of these types needs to be collected. 
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [profiles create projects](ProjectProfileCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateProfileRequest {
    /// Required. Deployment details.
    
    pub deployment: Option<Deployment>,
    /// Required. One or more profile types that the agent is capable of providing.
    #[serde(rename="profileType")]
    
    pub profile_type: Option<Vec<String>>,
}

impl client::RequestValue for CreateProfileRequest {}


/// Deployment contains the deployment identification information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    /// Labels identify the deployment within the user universe and same target. Validation regex for label names: `^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$`. Value for an individual label must be \<= 512 bytes, the total size of all label names and values must be \<= 1024 bytes. Label named “language” can be used to record the programming language of the profiled deployment. The standard choices for the value include “java”, “go”, “python”, “ruby”, “nodejs”, “php”, “dotnet”. For deployments running on Google Cloud Platform, “zone” or “region” label should be present describing the deployment location. An example of a zone is “us-central1-a”, an example of a region is “us-central1” or “us-central”.
    
    pub labels: Option<HashMap<String, String>>,
    /// Project ID is the ID of a cloud project. Validation regex: `^a-z{4,61}[a-z0-9]$`.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Target is the service name used to group related deployments: * Service name for App Engine Flex / Standard. * Cluster and container name for GKE. * User-specified string for direct Compute Engine profiling (e.g. Java). * Job name for Dataflow. Validation regex: `^[a-z0-9]([-a-z0-9_.]{0,253}[a-z0-9])?$`.
    
    pub target: Option<String>,
}

impl client::Part for Deployment {}


/// Profile resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [profiles create projects](ProjectProfileCreateCall) (response)
/// * [profiles create offline projects](ProjectProfileCreateOfflineCall) (request|response)
/// * [profiles patch projects](ProjectProfilePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    /// Deployment this profile corresponds to.
    
    pub deployment: Option<Deployment>,
    /// Duration of the profiling session. Input (for the offline mode) or output (for the online mode). The field represents requested profiling duration. It may slightly differ from the effective profiling duration, which is recorded in the profile data, in case the profiling can't be stopped immediately (e.g. in case stopping the profiling is handled asynchronously).
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub duration: Option<client::chrono::Duration>,
    /// Input only. Labels associated to this specific profile. These labels will get merged with the deployment labels for the final data set. See documentation on deployment labels for validation rules and limits.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. Opaque, server-assigned, unique ID for this profile.
    
    pub name: Option<String>,
    /// Input only. Profile bytes, as a gzip compressed serialized proto, the format is https://github.com/google/pprof/blob/master/proto/profile.proto.
    #[serde(rename="profileBytes")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub profile_bytes: Option<Vec<u8>>,
    /// Type of profile. For offline mode, this must be specified when creating the profile. For online mode it is assigned and returned by the server.
    #[serde(rename="profileType")]
    
    pub profile_type: Option<String>,
}

impl client::RequestValue for Profile {}
impl client::ResponseResult for Profile {}


