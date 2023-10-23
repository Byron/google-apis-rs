use super::*;
/// Request message for TablesService.BatchCreateRows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows batch create tables](TableRowBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateRowsRequest {
    /// Required. The request message specifying the rows to create. A maximum of 500 rows can be created in a single batch.
    
    pub requests: Option<Vec<CreateRowRequest>>,
}

impl client::RequestValue for BatchCreateRowsRequest {}


/// Response message for TablesService.BatchCreateRows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows batch create tables](TableRowBatchCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateRowsResponse {
    /// The created rows.
    
    pub rows: Option<Vec<Row>>,
}

impl client::ResponseResult for BatchCreateRowsResponse {}


/// Request message for TablesService.BatchDeleteRows
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows batch delete tables](TableRowBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteRowsRequest {
    /// Required. The names of the rows to delete. All rows must belong to the parent table or else the entire batch will fail. A maximum of 500 rows can be deleted in a batch. Format: tables/{table}/rows/{row}
    
    pub names: Option<Vec<String>>,
}

impl client::RequestValue for BatchDeleteRowsRequest {}


/// Request message for TablesService.BatchUpdateRows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows batch update tables](TableRowBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateRowsRequest {
    /// Required. The request messages specifying the rows to update. A maximum of 500 rows can be modified in a single batch.
    
    pub requests: Option<Vec<UpdateRowRequest>>,
}

impl client::RequestValue for BatchUpdateRowsRequest {}


/// Response message for TablesService.BatchUpdateRows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows batch update tables](TableRowBatchUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateRowsResponse {
    /// The updated rows.
    
    pub rows: Option<Vec<Row>>,
}

impl client::ResponseResult for BatchUpdateRowsResponse {}


/// Details on a column in the table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnDescription {
    /// Data type of the column Supported types are auto_id, boolean, boolean_list, creator, create_timestamp, date, dropdown, location, integer, integer_list, number, number_list, person, person_list, tags, check_list, text, text_list, update_timestamp, updater, relationship, file_attachment_list. These types directly map to the column types supported on Tables website.
    #[serde(rename="dataType")]
    
    pub data_type: Option<String>,
    /// Optional. Additional details about a date column.
    #[serde(rename="dateDetails")]
    
    pub date_details: Option<DateDetails>,
    /// Internal id for a column.
    
    pub id: Option<String>,
    /// Optional. Range of labeled values for the column. Some columns like tags and drop-downs limit the values to a set of possible values. We return the range of values in such cases to help clients implement better user data validation.
    
    pub labels: Option<Vec<LabeledItem>>,
    /// Optional. Indicates that this is a lookup column whose value is derived from the relationship column specified in the details. Lookup columns can not be updated directly. To change the value you must update the associated relationship column.
    #[serde(rename="lookupDetails")]
    
    pub lookup_details: Option<LookupDetails>,
    /// Optional. Indicates whether or not multiple values are allowed for array types where such a restriction is possible.
    #[serde(rename="multipleValuesDisallowed")]
    
    pub multiple_values_disallowed: Option<bool>,
    /// column name
    
    pub name: Option<String>,
    /// Optional. Indicates that values for the column cannot be set by the user.
    
    pub readonly: Option<bool>,
    /// Optional. Additional details about a relationship column. Specified when data_type is relationship.
    #[serde(rename="relationshipDetails")]
    
    pub relationship_details: Option<RelationshipDetails>,
}

impl client::Part for ColumnDescription {}


/// Request message for TablesService.CreateRow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateRowRequest {
    /// Required. The parent table where this row will be created. Format: tables/{table}
    
    pub parent: Option<String>,
    /// Required. The row to create.
    
    pub row: Option<Row>,
    /// Optional. Column key to use for values in the row. Defaults to user entered name.
    
    pub view: Option<CreateRowRequestViewEnum>,
}

impl client::Part for CreateRowRequest {}


/// Details about a date column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateDetails {
    /// Whether the date column includes time.
    #[serde(rename="hasTime")]
    
    pub has_time: Option<bool>,
}

impl client::Part for DateDetails {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows batch delete tables](TableRowBatchDeleteCall) (response)
/// * [rows delete tables](TableRowDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A single item in a labeled column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabeledItem {
    /// Internal id associated with the item.
    
    pub id: Option<String>,
    /// Display string as entered by user.
    
    pub name: Option<String>,
}

impl client::Part for LabeledItem {}


/// Response message for TablesService.ListRows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows list tables](TableRowListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRowsResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The rows from the specified table.
    
    pub rows: Option<Vec<Row>>,
}

impl client::ResponseResult for ListRowsResponse {}


/// Response message for TablesService.ListTables.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tables](TableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTablesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of tables.
    
    pub tables: Option<Vec<Table>>,
}

impl client::ResponseResult for ListTablesResponse {}


/// Response message for TablesService.ListWorkspaces.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list workspaces](WorkspaceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkspacesResponse {
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of workspaces.
    
    pub workspaces: Option<Vec<Workspace>>,
}

impl client::ResponseResult for ListWorkspacesResponse {}


/// Details about a lookup column whose value comes from the associated relationship.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LookupDetails {
    /// The name of the relationship column associated with the lookup.
    #[serde(rename="relationshipColumn")]
    
    pub relationship_column: Option<String>,
    /// The id of the relationship column.
    #[serde(rename="relationshipColumnId")]
    
    pub relationship_column_id: Option<String>,
}

impl client::Part for LookupDetails {}


/// Details about a relationship column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipDetails {
    /// The name of the table this relationship is linked to.
    #[serde(rename="linkedTable")]
    
    pub linked_table: Option<String>,
}

impl client::Part for RelationshipDetails {}


/// A single row in a table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows create tables](TableRowCreateCall) (request|response)
/// * [rows get tables](TableRowGetCall) (response)
/// * [rows patch tables](TableRowPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    /// Time when the row was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The resource name of the row. Row names have the form `tables/{table}/rows/{row}`. The name is ignored when creating a row.
    
    pub name: Option<String>,
    /// Time when the row was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The values of the row. This is a map of column key to value. Key is user entered name(default) or the internal column id based on the view in the request.
    
    pub values: Option<HashMap<String, json::Value>>,
}

impl client::RequestValue for Row {}
impl client::ResponseResult for Row {}


/// A saved view of a table. NextId: 3
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SavedView {
    /// Internal id associated with the saved view.
    
    pub id: Option<String>,
    /// Display name of the saved view.
    
    pub name: Option<String>,
}

impl client::Part for SavedView {}


/// A single table. NextId: 8
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rows batch create tables](TableRowBatchCreateCall) (none)
/// * [rows batch delete tables](TableRowBatchDeleteCall) (none)
/// * [rows batch update tables](TableRowBatchUpdateCall) (none)
/// * [rows create tables](TableRowCreateCall) (none)
/// * [rows delete tables](TableRowDeleteCall) (none)
/// * [rows get tables](TableRowGetCall) (none)
/// * [rows list tables](TableRowListCall) (none)
/// * [rows patch tables](TableRowPatchCall) (none)
/// * [get tables](TableGetCall) (response)
/// * [list tables](TableListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// List of columns in this table. Order of columns matches the display order.
    
    pub columns: Option<Vec<ColumnDescription>>,
    /// Time when the table was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The human readable title of the table.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the table. Table names have the form `tables/{table}`.
    
    pub name: Option<String>,
    /// Saved views for this table.
    #[serde(rename="savedViews")]
    
    pub saved_views: Option<Vec<SavedView>>,
    /// The time zone of the table. IANA Time Zone Database time zone, e.g. "America/New_York".
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// Time when the table was last updated excluding updates to individual rows
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Table {}
impl client::ResponseResult for Table {}


/// Request message for TablesService.UpdateRow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRowRequest {
    /// Required. The row to update.
    
    pub row: Option<Row>,
    /// The list of fields to update.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
    /// Optional. Column key to use for values in the row. Defaults to user entered name.
    
    pub view: Option<UpdateRowRequestViewEnum>,
}

impl client::Part for UpdateRowRequest {}


/// A single workspace.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get workspaces](WorkspaceGetCall) (response)
/// * [list workspaces](WorkspaceListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// Time when the workspace was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The human readable title of the workspace.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the workspace. Workspace names have the form `workspaces/{workspace}`.
    
    pub name: Option<String>,
    /// The list of tables in the workspace.
    
    pub tables: Option<Vec<Table>>,
    /// Time when the workspace was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Resource for Workspace {}
impl client::ResponseResult for Workspace {}


