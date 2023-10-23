use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateBucket {
    /// Available for Bucket.Type.ACTIVITY_TYPE, Bucket.Type.ACTIVITY_SEGMENT
    
    pub activity: Option<i32>,
    /// There will be one dataset per AggregateBy in the request.
    
    pub dataset: Option<Vec<Dataset>>,
    /// The end time for the aggregated data, in milliseconds since epoch, inclusive.
    #[serde(rename="endTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time_millis: Option<i64>,
    /// Available for Bucket.Type.SESSION
    
    pub session: Option<Session>,
    /// The start time for the aggregated data, in milliseconds since epoch, inclusive.
    #[serde(rename="startTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_millis: Option<i64>,
    /// The type of a bucket signifies how the data aggregation is performed in the bucket.
    #[serde(rename="type")]
    
    pub type_: Option<AggregateBucketTypeEnum>,
}

impl client::Part for AggregateBucket {}


/// The specification of which data to aggregate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateBy {
    /// A data source ID to aggregate. Only data from the specified data source ID will be included in the aggregation. If specified, this data source must exist; the OAuth scopes in the supplied credentials must grant read access to this data type. The dataset in the response will have the same data source ID. Note: Data can be aggregated by either the dataTypeName or the dataSourceId, not both.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// The data type to aggregate. All data sources providing this data type will contribute data to the aggregation. The response will contain a single dataset for this data type name. The dataset will have a data source ID of derived::com.google.android.gms:aggregated. If the user has no data for this data type, an empty data set will be returned. Note: Data can be aggregated by either the dataTypeName or the dataSourceId, not both.
    #[serde(rename="dataTypeName")]
    
    pub data_type_name: Option<String>,
}

impl client::Part for AggregateBy {}


/// Next id: 10
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dataset aggregate users](UserDatasetAggregateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateRequest {
    /// The specification of data to be aggregated. At least one aggregateBy spec must be provided. All data that is specified will be aggregated using the same bucketing criteria. There will be one dataset in the response for every aggregateBy spec.
    #[serde(rename="aggregateBy")]
    
    pub aggregate_by: Option<Vec<AggregateBy>>,
    /// Specifies that data be aggregated each activity segment recorded for a user. Similar to bucketByActivitySegment, but bucketing is done for each activity segment rather than all segments of the same type. Mutually exclusive of other bucketing specifications.
    #[serde(rename="bucketByActivitySegment")]
    
    pub bucket_by_activity_segment: Option<BucketByActivity>,
    /// Specifies that data be aggregated by the type of activity being performed when the data was recorded. All data that was recorded during a certain activity type (.for the given time range) will be aggregated into the same bucket. Data that was recorded while the user was not active will not be included in the response. Mutually exclusive of other bucketing specifications.
    #[serde(rename="bucketByActivityType")]
    
    pub bucket_by_activity_type: Option<BucketByActivity>,
    /// Specifies that data be aggregated by user sessions. Data that does not fall within the time range of a session will not be included in the response. Mutually exclusive of other bucketing specifications.
    #[serde(rename="bucketBySession")]
    
    pub bucket_by_session: Option<BucketBySession>,
    /// Specifies that data be aggregated by a single time interval. Mutually exclusive of other bucketing specifications.
    #[serde(rename="bucketByTime")]
    
    pub bucket_by_time: Option<BucketByTime>,
    /// The end of a window of time. Data that intersects with this time window will be aggregated. The time is in milliseconds since epoch, inclusive. The maximum allowed difference between start_time_millis // and end_time_millis is 7776000000 (roughly 90 days).
    #[serde(rename="endTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time_millis: Option<i64>,
    /// DO NOT POPULATE THIS FIELD. It is ignored.
    #[serde(rename="filteredDataQualityStandard")]
    
    pub filtered_data_quality_standard: Option<Vec<AggregateRequestFilteredDataQualityStandardEnum>>,
    /// The start of a window of time. Data that intersects with this time window will be aggregated. The time is in milliseconds since epoch, inclusive.
    #[serde(rename="startTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_millis: Option<i64>,
}

impl client::RequestValue for AggregateRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [dataset aggregate users](UserDatasetAggregateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateResponse {
    /// A list of buckets containing the aggregated data.
    
    pub bucket: Option<Vec<AggregateBucket>>,
}

impl client::ResponseResult for AggregateResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Application {
    /// An optional URI that can be used to link back to the application.
    #[serde(rename="detailsUrl")]
    
    pub details_url: Option<String>,
    /// The name of this application. This is required for REST clients, but we do not enforce uniqueness of this name. It is provided as a matter of convenience for other developers who would like to identify which REST created an Application or Data Source.
    
    pub name: Option<String>,
    /// Package name for this application. This is used as a unique identifier when created by Android applications, but cannot be specified by REST clients. REST clients will have their developer project number reflected into the Data Source data stream IDs, instead of the packageName.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Version of the application. You should update this field whenever the application changes in a way that affects the computation of the data.
    
    pub version: Option<String>,
}

impl client::Part for Application {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketByActivity {
    /// The default activity stream will be used if a specific activityDataSourceId is not specified.
    #[serde(rename="activityDataSourceId")]
    
    pub activity_data_source_id: Option<String>,
    /// Specifies that only activity segments of duration longer than minDurationMillis are considered and used as a container for aggregated data.
    #[serde(rename="minDurationMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_duration_millis: Option<i64>,
}

impl client::Part for BucketByActivity {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketBySession {
    /// Specifies that only sessions of duration longer than minDurationMillis are considered and used as a container for aggregated data.
    #[serde(rename="minDurationMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_duration_millis: Option<i64>,
}

impl client::Part for BucketBySession {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketByTime {
    /// Specifies that result buckets aggregate data by exactly durationMillis time frames. Time frames that contain no data will be included in the response with an empty dataset.
    #[serde(rename="durationMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_millis: Option<i64>,
    /// no description provided
    
    pub period: Option<BucketByTimePeriod>,
}

impl client::Part for BucketByTime {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketByTimePeriod {
    /// org.joda.timezone.DateTimeZone
    #[serde(rename="timeZoneId")]
    
    pub time_zone_id: Option<String>,
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<BucketByTimePeriodTypeEnum>,
    /// no description provided
    
    pub value: Option<i32>,
}

impl client::Part for BucketByTimePeriod {}


/// Represents a single data point, generated by a particular data source. A data point holds a value for each field, an end timestamp and an optional start time. The exact semantics of each of these attributes are specified in the documentation for the particular data type. A data point can represent an instantaneous measurement, reading or input observation, as well as averages or aggregates over a time interval. Check the data type documentation to determine which is the case for a particular data type. Data points always contain one value for each field of the data type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataPoint {
    /// DO NOT USE THIS FIELD. It is ignored, and not stored.
    #[serde(rename="computationTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub computation_time_millis: Option<i64>,
    /// The data type defining the format of the values in this data point.
    #[serde(rename="dataTypeName")]
    
    pub data_type_name: Option<String>,
    /// The end time of the interval represented by this data point, in nanoseconds since epoch.
    #[serde(rename="endTimeNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time_nanos: Option<i64>,
    /// Indicates the last time this data point was modified. Useful only in contexts where we are listing the data changes, rather than representing the current state of the data.
    #[serde(rename="modifiedTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub modified_time_millis: Option<i64>,
    /// If the data point is contained in a dataset for a derived data source, this field will be populated with the data source stream ID that created the data point originally. WARNING: do not rely on this field for anything other than debugging. The value of this field, if it is set at all, is an implementation detail and is not guaranteed to remain consistent.
    #[serde(rename="originDataSourceId")]
    
    pub origin_data_source_id: Option<String>,
    /// The raw timestamp from the original SensorEvent.
    #[serde(rename="rawTimestampNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub raw_timestamp_nanos: Option<i64>,
    /// The start time of the interval represented by this data point, in nanoseconds since epoch.
    #[serde(rename="startTimeNanos")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_nanos: Option<i64>,
    /// Values of each data type field for the data point. It is expected that each value corresponding to a data type field will occur in the same order that the field is listed with in the data type specified in a data source. Only one of integer and floating point fields will be populated, depending on the format enum value within data source's type field.
    
    pub value: Option<Vec<Value>>,
}

impl client::Part for DataPoint {}


/// Definition of a unique source of sensor data. Data sources can expose raw data coming from hardware sensors on local or companion devices. They can also expose derived data, created by transforming or merging other data sources. Multiple data sources can exist for the same data type. Every data point inserted into or read from this service has an associated data source. The data source contains enough information to uniquely identify its data, including the hardware device and the application that collected and/or transformed the data. It also holds useful metadata, such as the hardware and application versions, and the device type. Each data source produces a unique stream of data, with a unique identifier. Not all changes to data source affect the stream identifier, so that data collected by updated versions of the same application/device can still be considered to belong to the same data stream.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources create users](UserDataSourceCreateCall) (request|response)
/// * [data sources delete users](UserDataSourceDeleteCall) (response)
/// * [data sources get users](UserDataSourceGetCall) (response)
/// * [data sources update users](UserDataSourceUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// Information about an application which feeds sensor data into the platform.
    
    pub application: Option<Application>,
    /// DO NOT POPULATE THIS FIELD. It is never populated in responses from the platform, and is ignored in queries. It will be removed in a future version entirely.
    #[serde(rename="dataQualityStandard")]
    
    pub data_quality_standard: Option<Vec<DataSourceDataQualityStandardEnum>>,
    /// A unique identifier for the data stream produced by this data source. The identifier includes: - The physical device's manufacturer, model, and serial number (UID). - The application's package name or name. Package name is used when the data source was created by an Android application. The developer project number is used when the data source was created by a REST client. - The data source's type. - The data source's stream name. Note that not all attributes of the data source are used as part of the stream identifier. In particular, the version of the hardware/the application isn't used. This allows us to preserve the same stream through version updates. This also means that two DataSource objects may represent the same data stream even if they're not equal. The exact format of the data stream ID created by an Android application is: type:dataType.name:application.packageName:device.manufacturer:device.model:device.uid:dataStreamName The exact format of the data stream ID created by a REST client is: type:dataType.name:developer project number:device.manufacturer:device.model:device.uid:dataStreamName When any of the optional fields that make up the data stream ID are absent, they will be omitted from the data stream ID. The minimum viable data stream ID would be: type:dataType.name:developer project number Finally, the developer project number and device UID are obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the developer project number in clear and normal form. This means a client will see a different set of data_stream_ids than another client with different credentials.
    #[serde(rename="dataStreamId")]
    
    pub data_stream_id: Option<String>,
    /// The stream name uniquely identifies this particular data source among other data sources of the same type from the same underlying producer. Setting the stream name is optional, but should be done whenever an application exposes two streams for the same data type, or when a device has two equivalent sensors.
    #[serde(rename="dataStreamName")]
    
    pub data_stream_name: Option<String>,
    /// The data type defines the schema for a stream of data being collected by, inserted into, or queried from the Fitness API.
    #[serde(rename="dataType")]
    
    pub data_type: Option<DataType>,
    /// Representation of an integrated device (such as a phone or a wearable) that can hold sensors.
    
    pub device: Option<Device>,
    /// An end-user visible name for this data source.
    
    pub name: Option<String>,
    /// A constant describing the type of this data source. Indicates whether this data source produces raw or derived data.
    #[serde(rename="type")]
    
    pub type_: Option<DataSourceTypeEnum>,
}

impl client::RequestValue for DataSource {}
impl client::ResponseResult for DataSource {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataType {
    /// A field represents one dimension of a data type.
    
    pub field: Option<Vec<DataTypeField>>,
    /// Each data type has a unique, namespaced, name. All data types in the com.google namespace are shared as part of the platform.
    
    pub name: Option<String>,
}

impl client::Part for DataType {}


/// In case of multi-dimensional data (such as an accelerometer with x, y, and z axes) each field represents one dimension. Each data type field has a unique name which identifies it. The field also defines the format of the data (int, float, etc.). This message is only instantiated in code and not used for wire comms or stored in any way.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataTypeField {
    /// The different supported formats for each field in a data type.
    
    pub format: Option<DataTypeFieldFormatEnum>,
    /// Defines the name and format of data. Unlike data type names, field names are not namespaced, and only need to be unique within the data type.
    
    pub name: Option<String>,
    /// no description provided
    
    pub optional: Option<bool>,
}

impl client::Part for DataTypeField {}


/// A dataset represents a projection container for data points. They do not carry any info of their own. Datasets represent a set of data points from a particular data source. A data point can be found in more than one dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources datasets get users](UserDataSourceDatasetGetCall) (response)
/// * [data sources datasets patch users](UserDataSourceDatasetPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    /// The data stream ID of the data source that created the points in this dataset.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// The largest end time of all data points in this possibly partial representation of the dataset. Time is in nanoseconds from epoch. This should also match the second part of the dataset identifier.
    #[serde(rename="maxEndTimeNs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_end_time_ns: Option<i64>,
    /// The smallest start time of all data points in this possibly partial representation of the dataset. Time is in nanoseconds from epoch. This should also match the first part of the dataset identifier.
    #[serde(rename="minStartTimeNs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_start_time_ns: Option<i64>,
    /// This token will be set when a dataset is received in response to a GET request and the dataset is too large to be included in a single response. Provide this value in a subsequent GET request to return the next page of data points within this dataset.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A partial list of data points contained in the dataset, ordered by endTimeNanos. This list is considered complete when retrieving a small dataset and partial when patching a dataset or retrieving a dataset that is too large to include in a single response.
    
    pub point: Option<Vec<DataPoint>>,
}

impl client::RequestValue for Dataset {}
impl client::ResponseResult for Dataset {}


/// Representation of an integrated device (such as a phone or a wearable) that can hold sensors. Each sensor is exposed as a data source. The main purpose of the device information contained in this class is to identify the hardware of a particular data source. This can be useful in different ways, including: - Distinguishing two similar sensors on different devices (the step counter on two nexus 5 phones, for instance) - Display the source of data to the user (by using the device make / model) - Treat data differently depending on sensor type (accelerometers on a watch may give different patterns than those on a phone) - Build different analysis models for each device/version. 
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    /// Manufacturer of the product/hardware.
    
    pub manufacturer: Option<String>,
    /// End-user visible model name for the device.
    
    pub model: Option<String>,
    /// A constant representing the type of the device.
    #[serde(rename="type")]
    
    pub type_: Option<DeviceTypeEnum>,
    /// The serial number or other unique ID for the hardware. This field is obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the uid field in clear and normal form. The obfuscation preserves equality; that is, given two IDs, if id1 == id2, obfuscated(id1) == obfuscated(id2).
    
    pub uid: Option<String>,
    /// Version string for the device hardware/software.
    
    pub version: Option<String>,
}

impl client::Part for Device {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources data point changes list users](UserDataSourceDataPointChangeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDataPointChangesResponse {
    /// The data stream ID of the data source with data point changes.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// Deleted data points for the user. Note, for modifications this should be parsed before handling insertions.
    #[serde(rename="deletedDataPoint")]
    
    pub deleted_data_point: Option<Vec<DataPoint>>,
    /// Inserted data points for the user.
    #[serde(rename="insertedDataPoint")]
    
    pub inserted_data_point: Option<Vec<DataPoint>>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDataPointChangesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [data sources list users](UserDataSourceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDataSourcesResponse {
    /// A previously created data source.
    #[serde(rename="dataSource")]
    
    pub data_source: Option<Vec<DataSource>>,
}

impl client::ResponseResult for ListDataSourcesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sessions list users](UserSessionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSessionsResponse {
    /// If includeDeleted is set to true in the request, and startTime and endTime are omitted, this will include sessions which were deleted since the last sync.
    #[serde(rename="deletedSession")]
    
    pub deleted_session: Option<Vec<Session>>,
    /// Flag to indicate server has more data to transfer. DO NOT USE THIS FIELD. It is never populated in responses from the server.
    #[serde(rename="hasMoreData")]
    
    pub has_more_data: Option<bool>,
    /// The sync token which is used to sync further changes. This will only be provided if both startTime and endTime are omitted from the request.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Sessions with an end time that is between startTime and endTime of the request.
    
    pub session: Option<Vec<Session>>,
}

impl client::ResponseResult for ListSessionsResponse {}


/// Holder object for the value of an entry in a map field of a data point. A map value supports a subset of the formats that the regular Value supports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MapValue {
    /// Floating point value.
    #[serde(rename="fpVal")]
    
    pub fp_val: Option<f64>,
}

impl client::Part for MapValue {}


/// Sessions contain metadata, such as a user-friendly name and time interval information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sessions update users](UserSessionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    /// Session active time. While start_time_millis and end_time_millis define the full session time, the active time can be shorter and specified by active_time_millis. If the inactive time during the session is known, it should also be inserted via a com.google.activity.segment data point with a STILL activity value
    #[serde(rename="activeTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub active_time_millis: Option<i64>,
    /// The type of activity this session represents.
    #[serde(rename="activityType")]
    
    pub activity_type: Option<i32>,
    /// The application that created the session.
    
    pub application: Option<Application>,
    /// A description for this session.
    
    pub description: Option<String>,
    /// An end time, in milliseconds since epoch, inclusive.
    #[serde(rename="endTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time_millis: Option<i64>,
    /// A client-generated identifier that is unique across all sessions owned by this particular user.
    
    pub id: Option<String>,
    /// A timestamp that indicates when the session was last modified.
    #[serde(rename="modifiedTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub modified_time_millis: Option<i64>,
    /// A human readable name of the session.
    
    pub name: Option<String>,
    /// A start time, in milliseconds since epoch, inclusive.
    #[serde(rename="startTimeMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_millis: Option<i64>,
}

impl client::RequestValue for Session {}
impl client::ResponseResult for Session {}


/// Holder object for the value of a single field in a data point. A field value has a particular format and is only ever set to one of an integer or a floating point value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    /// Floating point value. When this is set, other values must not be set.
    #[serde(rename="fpVal")]
    
    pub fp_val: Option<f64>,
    /// Integer value. When this is set, other values must not be set.
    #[serde(rename="intVal")]
    
    pub int_val: Option<i32>,
    /// Map value. The valid key space and units for the corresponding value of each entry should be documented as part of the data type definition. Keys should be kept small whenever possible. Data streams with large keys and high data frequency may be down sampled.
    #[serde(rename="mapVal")]
    
    pub map_val: Option<Vec<ValueMapValEntry>>,
    /// String value. When this is set, other values must not be set. Strings should be kept small whenever possible. Data streams with large string values and high data frequency may be down sampled.
    #[serde(rename="stringVal")]
    
    pub string_val: Option<String>,
}

impl client::Part for Value {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueMapValEntry {
    /// no description provided
    
    pub key: Option<String>,
    /// no description provided
    
    pub value: Option<MapValue>,
}

impl client::Part for ValueMapValEntry {}


