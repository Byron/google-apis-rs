use super::*;
/// Device resource represents an instance of enterprise managed device in the property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices get enterprises](EnterpriseDeviceGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1Device {
    /// Required. The resource name of the device. For example: "enterprises/XYZ/devices/123".
    
    pub name: Option<String>,
    /// Assignee details of the device.
    #[serde(rename="parentRelations")]
    
    pub parent_relations: Option<Vec<GoogleHomeEnterpriseSdmV1ParentRelation>>,
    /// Output only. Device traits.
    
    pub traits: Option<HashMap<String, json::Value>>,
    /// Output only. Type of the device for general display purposes. For example: "THERMOSTAT". The device type should not be used to deduce or infer functionality of the actual device it is assigned to. Instead, use the returned traits for the device.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for GoogleHomeEnterpriseSdmV1Device {}


/// Request message for SmartDeviceManagementService.ExecuteDeviceCommand
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices execute command enterprises](EnterpriseDeviceExecuteCommandCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequest {
    /// The command name to execute, represented by the fully qualified protobuf message name.
    
    pub command: Option<String>,
    /// The command message to execute, represented as a Struct.
    
    pub params: Option<HashMap<String, json::Value>>,
}

impl client::RequestValue for GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequest {}


/// Response message for SmartDeviceManagementService.ExecuteDeviceCommand
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices execute command enterprises](EnterpriseDeviceExecuteCommandCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandResponse {
    /// The results of executing the command.
    
    pub results: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandResponse {}


/// Response message for SmartDeviceManagementService.ListDevices
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [devices list enterprises](EnterpriseDeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1ListDevicesResponse {
    /// The list of devices.
    
    pub devices: Option<Vec<GoogleHomeEnterpriseSdmV1Device>>,
    /// The pagination token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleHomeEnterpriseSdmV1ListDevicesResponse {}


/// Response message for SmartDeviceManagementService.ListRooms
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [structures rooms list enterprises](EnterpriseStructureRoomListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1ListRoomsResponse {
    /// The pagination token to retrieve the next page of results. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of rooms.
    
    pub rooms: Option<Vec<GoogleHomeEnterpriseSdmV1Room>>,
}

impl client::ResponseResult for GoogleHomeEnterpriseSdmV1ListRoomsResponse {}


/// Response message for SmartDeviceManagementService.ListStructures
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [structures list enterprises](EnterpriseStructureListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1ListStructuresResponse {
    /// The pagination token to retrieve the next page of results. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of structures.
    
    pub structures: Option<Vec<GoogleHomeEnterpriseSdmV1Structure>>,
}

impl client::ResponseResult for GoogleHomeEnterpriseSdmV1ListStructuresResponse {}


/// Represents device relationships, for instance, structure/room to which the device is assigned to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1ParentRelation {
    /// Output only. The custom name of the relation -- e.g., structure/room where the device is assigned to.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. The name of the relation -- e.g., structure/room where the device is assigned to. For example: "enterprises/XYZ/structures/ABC" or "enterprises/XYZ/structures/ABC/rooms/123"
    
    pub parent: Option<String>,
}

impl client::Part for GoogleHomeEnterpriseSdmV1ParentRelation {}


/// Room resource represents an instance of sub-space within a structure such as rooms in a hotel suite or rental apartment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [structures rooms get enterprises](EnterpriseStructureRoomGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1Room {
    /// Output only. The resource name of the room. For example: "enterprises/XYZ/structures/ABC/rooms/123".
    
    pub name: Option<String>,
    /// Room traits.
    
    pub traits: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleHomeEnterpriseSdmV1Room {}


/// Structure resource represents an instance of enterprise managed home or hotel room.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [structures get enterprises](EnterpriseStructureGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleHomeEnterpriseSdmV1Structure {
    /// Output only. The resource name of the structure. For example: "enterprises/XYZ/structures/ABC".
    
    pub name: Option<String>,
    /// Structure traits.
    
    pub traits: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleHomeEnterpriseSdmV1Structure {}


