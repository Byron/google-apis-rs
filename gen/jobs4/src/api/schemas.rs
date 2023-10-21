use super::*;
/// Application related details of a job posting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationInfo {
    /// Use this field to specify email address(es) to which resumes or applications can be sent. The maximum number of allowed characters for each entry is 255.
    
    pub emails: Option<Vec<String>>,
    /// Use this field to provide instructions, such as "Mail your application to ...", that a candidate can follow to apply for the job. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 3,000.
    
    pub instruction: Option<String>,
    /// Use this URI field to direct an applicant to a website, for example to link to an online application form. The maximum number of allowed characters for each entry is 2,000.
    
    pub uris: Option<Vec<String>>,
}

impl client::Part for ApplicationInfo {}


/// Request to create a batch of jobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants jobs batch create projects](ProjectTenantJobBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateJobsRequest {
    /// Required. The jobs to be created. A maximum of 200 jobs can be created in a batch.
    
    pub jobs: Option<Vec<Job>>,
}

impl client::RequestValue for BatchCreateJobsRequest {}


/// Request to delete a batch of jobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants jobs batch delete projects](ProjectTenantJobBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteJobsRequest {
    /// The names of the jobs to delete. The format is "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For example, "projects/foo/tenants/bar/jobs/baz". A maximum of 200 jobs can be deleted in a batch.
    
    pub names: Option<Vec<String>>,
}

impl client::RequestValue for BatchDeleteJobsRequest {}


/// Request to update a batch of jobs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants jobs batch update projects](ProjectTenantJobBatchUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateJobsRequest {
    /// Required. The jobs to be updated. A maximum of 200 jobs can be updated in a batch.
    
    pub jobs: Option<Vec<Job>>,
    /// Strongly recommended for the best service experience. Be aware that it will also increase latency when checking the status of a batch operation. If update_mask is provided, only the specified fields in Job are updated. Otherwise all the fields are updated. A field mask to restrict the fields that are updated. Only top level fields of Job are supported. If update_mask is provided, The Job inside JobResult will only contains fields that is updated, plus the Id of the Job. Otherwise, Job will include all fields, which can yield a very large response.
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for BatchUpdateJobsRequest {}


/// An event issued when an end user interacts with the application that implements Cloud Talent Solution. Providing this information improves the quality of results for the API clients, enabling the service to perform optimally. The number of events sent must be consistent with other calls, such as job searches, issued to the service by the client.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants client events create projects](ProjectTenantClientEventCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClientEvent {
    /// Required. The timestamp of the event.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. A unique identifier, generated by the client application.
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// Notes about the event provided by recruiters or other users, for example, feedback on why a job was bookmarked.
    #[serde(rename="eventNotes")]
    
    pub event_notes: Option<String>,
    /// An event issued when a job seeker interacts with the application that implements Cloud Talent Solution.
    #[serde(rename="jobEvent")]
    
    pub job_event: Option<JobEvent>,
    /// Strongly recommended for the best service experience. A unique ID generated in the API responses. It can be found in ResponseMetadata.request_id.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::RequestValue for ClientEvent {}
impl client::ResponseResult for ClientEvent {}


/// Parameters needed for commute search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommuteFilter {
    /// If `true`, jobs without street level addresses may also be returned. For city level addresses, the city center is used. For state and coarser level addresses, text matching is used. If this field is set to `false` or isn't specified, only jobs that include street level addresses will be returned by commute search.
    #[serde(rename="allowImpreciseAddresses")]
    
    pub allow_imprecise_addresses: Option<bool>,
    /// Required. The method of transportation to calculate the commute time for.
    #[serde(rename="commuteMethod")]
    
    pub commute_method: Option<String>,
    /// The departure time used to calculate traffic impact, represented as google.type.TimeOfDay in local time zone. Currently traffic model is restricted to hour level resolution.
    #[serde(rename="departureTime")]
    
    pub departure_time: Option<TimeOfDay>,
    /// Specifies the traffic density to use when calculating commute time.
    #[serde(rename="roadTraffic")]
    
    pub road_traffic: Option<String>,
    /// Required. The latitude and longitude of the location to calculate the commute time from.
    #[serde(rename="startCoordinates")]
    
    pub start_coordinates: Option<LatLng>,
    /// Required. The maximum travel time in seconds. The maximum allowed value is `3600s` (one hour). Format is `123s`.
    #[serde(rename="travelDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub travel_duration: Option<client::chrono::Duration>,
}

impl client::Part for CommuteFilter {}


/// Commute details related to this job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommuteInfo {
    /// Location used as the destination in the commute calculation.
    #[serde(rename="jobLocation")]
    
    pub job_location: Option<Location>,
    /// The number of seconds required to travel to the job location from the query location. A duration of 0 seconds indicates that the job isn't reachable within the requested duration, but was returned as part of an expanded query.
    #[serde(rename="travelDuration")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub travel_duration: Option<client::chrono::Duration>,
}

impl client::Part for CommuteInfo {}


/// A Company resource represents a company in the service. A company is the entity that owns job postings, that is, the hiring entity responsible for employing applicants for the job position.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants companies create projects](ProjectTenantCompanyCreateCall) (request|response)
/// * [tenants companies get projects](ProjectTenantCompanyGetCall) (response)
/// * [tenants companies patch projects](ProjectTenantCompanyPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Company {
    /// The URI to employer's career site or careers page on the employer's web site, for example, "https://careers.google.com".
    #[serde(rename="careerSiteUri")]
    
    pub career_site_uri: Option<String>,
    /// Output only. Derived details about the company.
    #[serde(rename="derivedInfo")]
    
    pub derived_info: Option<CompanyDerivedInfo>,
    /// Required. The display name of the company, for example, "Google LLC".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Equal Employment Opportunity legal disclaimer text to be associated with all jobs, and typically to be displayed in all roles. The maximum number of allowed characters is 500.
    #[serde(rename="eeoText")]
    
    pub eeo_text: Option<String>,
    /// Required. Client side company identifier, used to uniquely identify the company. The maximum number of allowed characters is 255.
    #[serde(rename="externalId")]
    
    pub external_id: Option<String>,
    /// The street address of the company's main headquarters, which may be different from the job location. The service attempts to geolocate the provided address, and populates a more specific location wherever possible in DerivedInfo.headquarters_location.
    #[serde(rename="headquartersAddress")]
    
    pub headquarters_address: Option<String>,
    /// Set to true if it is the hiring agency that post jobs for other employers. Defaults to false if not provided.
    #[serde(rename="hiringAgency")]
    
    pub hiring_agency: Option<bool>,
    /// A URI that hosts the employer's company logo.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// This field is deprecated. Please set the searchability of the custom attribute in the Job.custom_attributes going forward. A list of keys of filterable Job.custom_attributes, whose corresponding `string_values` are used in keyword searches. Jobs with `string_values` under these specified field keys are returned if any of the values match the search keyword. Custom field values with parenthesis, brackets and special symbols are not searchable as-is, and those keyword queries must be surrounded by quotes.
    #[serde(rename="keywordSearchableJobCustomAttributes")]
    
    pub keyword_searchable_job_custom_attributes: Option<Vec<String>>,
    /// Required during company update. The resource name for a company. This is generated by the service when a company is created. The format is "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for example, "projects/foo/tenants/bar/companies/baz".
    
    pub name: Option<String>,
    /// The employer's company size.
    
    pub size: Option<String>,
    /// Output only. Indicates whether a company is flagged to be suspended from public availability by the service when job content appears suspicious, abusive, or spammy.
    
    pub suspended: Option<bool>,
    /// The URI representing the company's primary web site or home page, for example, "https://www.google.com". The maximum number of allowed characters is 255.
    #[serde(rename="websiteUri")]
    
    pub website_uri: Option<String>,
}

impl client::RequestValue for Company {}
impl client::ResponseResult for Company {}


/// Derived details about the company.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompanyDerivedInfo {
    /// A structured headquarters location of the company, resolved from Company.headquarters_address if provided.
    #[serde(rename="headquartersLocation")]
    
    pub headquarters_location: Option<Location>,
}

impl client::Part for CompanyDerivedInfo {}


/// A compensation entry that represents one component of compensation, such as base pay, bonus, or other compensation type. Annualization: One compensation entry can be annualized if - it contains valid amount or range. - and its expected_units_per_year is set or can be derived. Its annualized range is determined as (amount or range) times expected_units_per_year.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationEntry {
    /// Compensation amount.
    
    pub amount: Option<Money>,
    /// Compensation description. For example, could indicate equity terms or provide additional context to an estimated bonus.
    
    pub description: Option<String>,
    /// Expected number of units paid each year. If not specified, when Job.employment_types is FULLTIME, a default value is inferred based on unit. Default values: - HOURLY: 2080 - DAILY: 260 - WEEKLY: 52 - MONTHLY: 12 - ANNUAL: 1
    #[serde(rename="expectedUnitsPerYear")]
    
    pub expected_units_per_year: Option<f64>,
    /// Compensation range.
    
    pub range: Option<CompensationRange>,
    /// Compensation type. Default is CompensationType.COMPENSATION_TYPE_UNSPECIFIED.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Frequency of the specified amount. Default is CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED.
    
    pub unit: Option<String>,
}

impl client::Part for CompensationEntry {}


/// Filter on job compensation type and amount.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationFilter {
    /// If set to true, jobs with unspecified compensation range fields are included.
    #[serde(rename="includeJobsWithUnspecifiedCompensationRange")]
    
    pub include_jobs_with_unspecified_compensation_range: Option<bool>,
    /// Compensation range.
    
    pub range: Option<CompensationRange>,
    /// Required. Type of filter.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Required. Specify desired `base compensation entry's` CompensationInfo.CompensationUnit.
    
    pub units: Option<Vec<String>>,
}

impl client::Part for CompensationFilter {}


/// Job compensation details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationInfo {
    /// Output only. Annualized base compensation range. Computed as base compensation entry's CompensationEntry.amount times CompensationEntry.expected_units_per_year. See CompensationEntry for explanation on compensation annualization.
    #[serde(rename="annualizedBaseCompensationRange")]
    
    pub annualized_base_compensation_range: Option<CompensationRange>,
    /// Output only. Annualized total compensation range. Computed as all compensation entries' CompensationEntry.amount times CompensationEntry.expected_units_per_year. See CompensationEntry for explanation on compensation annualization.
    #[serde(rename="annualizedTotalCompensationRange")]
    
    pub annualized_total_compensation_range: Option<CompensationRange>,
    /// Job compensation information. At most one entry can be of type CompensationInfo.CompensationType.BASE, which is referred as **base compensation entry** for the job.
    
    pub entries: Option<Vec<CompensationEntry>>,
}

impl client::Part for CompensationInfo {}


/// Compensation range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompensationRange {
    /// The maximum amount of compensation. If left empty, the value is set to a maximal compensation value and the currency code is set to match the currency code of min_compensation.
    #[serde(rename="maxCompensation")]
    
    pub max_compensation: Option<Money>,
    /// The minimum amount of compensation. If left empty, the value is set to zero and the currency code is set to match the currency code of max_compensation.
    #[serde(rename="minCompensation")]
    
    pub min_compensation: Option<Money>,
}

impl client::Part for CompensationRange {}


/// Response of auto-complete query.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants complete query projects](ProjectTenantCompleteQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompleteQueryResponse {
    /// Results of the matching job/company candidates.
    #[serde(rename="completionResults")]
    
    pub completion_results: Option<Vec<CompletionResult>>,
    /// Additional information for the API invocation, such as the request tracking id.
    
    pub metadata: Option<ResponseMetadata>,
}

impl client::ResponseResult for CompleteQueryResponse {}


/// Resource that represents completion results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompletionResult {
    /// The URI of the company image for COMPANY_NAME.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// The suggestion for the query.
    
    pub suggestion: Option<String>,
    /// The completion topic.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for CompletionResult {}


/// Custom attribute values that are either filterable or non-filterable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomAttribute {
    /// If the `filterable` flag is true, the custom field values may be used for custom attribute filters JobQuery.custom_attribute_filter. If false, these values may not be used for custom attribute filters. Default is false.
    
    pub filterable: Option<bool>,
    /// If the `keyword_searchable` flag is true, the keywords in custom fields are searchable by keyword match. If false, the values are not searchable by keyword match. Default is false.
    #[serde(rename="keywordSearchable")]
    
    pub keyword_searchable: Option<bool>,
    /// Exactly one of string_values or long_values must be specified. This field is used to perform number range search. (`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`. Currently at most 1 long_values is supported.
    #[serde(rename="longValues")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub long_values: Option<Vec<i64>>,
    /// Exactly one of string_values or long_values must be specified. This field is used to perform a string match (`CASE_SENSITIVE_MATCH` or `CASE_INSENSITIVE_MATCH`) search. For filterable `string_value`s, a maximum total number of 200 values is allowed, with each `string_value` has a byte size of no more than 500B. For unfilterable `string_values`, the maximum total byte size of unfilterable `string_values` is 50KB. Empty string isn't allowed.
    #[serde(rename="stringValues")]
    
    pub string_values: Option<Vec<String>>,
}

impl client::Part for CustomAttribute {}


/// Custom ranking information for SearchJobsRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomRankingInfo {
    /// Required. Controls over how important the score of CustomRankingInfo.ranking_expression gets applied to job's final ranking position. An error is thrown if not specified.
    #[serde(rename="importanceLevel")]
    
    pub importance_level: Option<String>,
    /// Required. Controls over how job documents get ranked on top of existing relevance score (determined by API algorithm). A combination of the ranking expression and relevance score is used to determine job's final ranking position. The syntax for this expression is a subset of Google SQL syntax. Supported operators are: +, -, *, /, where the left and right side of the operator is either a numeric Job.custom_attributes key, integer/double value or an expression that can be evaluated to a number. Parenthesis are supported to adjust calculation precedence. The expression must be < 200 characters in length. The expression is considered invalid for a job if the expression references custom attributes that are not populated on the job or if the expression results in a divide by zero. If an expression is invalid for a job, that job is demoted to the end of the results. Sample ranking expression (year + 25) * 0.25 - (freshness / 0.5)
    #[serde(rename="rankingExpression")]
    
    pub ranking_expression: Option<String>,
}

impl client::Part for CustomRankingInfo {}


/// Device information collected from the job seeker, candidate, or other entity conducting the job search. Providing this information improves the quality of the search results across devices.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Type of the device.
    #[serde(rename="deviceType")]
    
    pub device_type: Option<String>,
    /// A device-specific ID. The ID must be a unique identifier that distinguishes the device from other devices.
    
    pub id: Option<String>,
}

impl client::Part for DeviceInfo {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants companies delete projects](ProjectTenantCompanyDeleteCall) (response)
/// * [tenants jobs delete projects](ProjectTenantJobDeleteCall) (response)
/// * [tenants delete projects](ProjectTenantDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The histogram request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramQuery {
    /// An expression specifies a histogram request against matching jobs for searches. See SearchJobsRequest.histogram_queries for details about syntax.
    #[serde(rename="histogramQuery")]
    
    pub histogram_query: Option<String>,
}

impl client::Part for HistogramQuery {}


/// Histogram result that matches HistogramQuery specified in searches.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramQueryResult {
    /// A map from the values of the facet associated with distinct values to the number of matching entries with corresponding value. The key format is: * (for string histogram) string values stored in the field. * (for named numeric bucket) name specified in `bucket()` function, like for `bucket(0, MAX, "non-negative")`, the key will be `non-negative`. * (for anonymous numeric bucket) range formatted as `-`, for example, `0-1000`, `MIN-0`, and `0-MAX`.
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde_with::DisplayFromStr>>")]
    pub histogram: Option<HashMap<String, i64>>,
    /// Requested histogram expression.
    #[serde(rename="histogramQuery")]
    
    pub histogram_query: Option<String>,
}

impl client::Part for HistogramQueryResult {}


/// A Job resource represents a job posting (also referred to as a “job listing” or “job requisition”). A job belongs to a Company, which is the hiring entity responsible for the job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants jobs create projects](ProjectTenantJobCreateCall) (request|response)
/// * [tenants jobs get projects](ProjectTenantJobGetCall) (response)
/// * [tenants jobs patch projects](ProjectTenantJobPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Strongly recommended for the best service experience. Location(s) where the employer is looking to hire for this job posting. Specifying the full street address(es) of the hiring location enables better API results, especially job searches by commute time. At most 50 locations are allowed for best search performance. If a job has more locations, it is suggested to split it into multiple jobs with unique requisition_ids (e.g. 'ReqA' becomes 'ReqA-1', 'ReqA-2', and so on.) as multiple jobs with the same company, language_code and requisition_id are not allowed. If the original requisition_id must be preserved, a custom field should be used for storage. It is also suggested to group the locations that close to each other in the same job for better search experience. Jobs with multiple addresses must have their addresses with the same LocationType to allow location filtering to work properly. (For example, a Job with addresses "1600 Amphitheatre Parkway, Mountain View, CA, USA" and "London, UK" may not have location filters applied correctly at search time since the first is a LocationType.STREET_ADDRESS and the second is a LocationType.LOCALITY.) If a job needs to have multiple addresses, it is suggested to split it into multiple jobs with same LocationTypes. The maximum number of allowed characters is 500.
    
    pub addresses: Option<Vec<String>>,
    /// Job application information.
    #[serde(rename="applicationInfo")]
    
    pub application_info: Option<ApplicationInfo>,
    /// Required. The resource name of the company listing the job. The format is "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}". For example, "projects/foo/tenants/bar/companies/baz".
    
    pub company: Option<String>,
    /// Output only. Display name of the company listing the job.
    #[serde(rename="companyDisplayName")]
    
    pub company_display_name: Option<String>,
    /// Job compensation information (a.k.a. "pay rate") i.e., the compensation that will paid to the employee.
    #[serde(rename="compensationInfo")]
    
    pub compensation_info: Option<CompensationInfo>,
    /// A map of fields to hold both filterable and non-filterable custom job attributes that are not covered by the provided structured fields. The keys of the map are strings up to 64 bytes and must match the pattern: `a-zA-Z*`. For example, key0LikeThis or KEY_1_LIKE_THIS. At most 100 filterable and at most 100 unfilterable keys are supported. For filterable `string_values`, across all keys at most 200 values are allowed, with each string no more than 255 characters. For unfilterable `string_values`, the maximum total size of `string_values` across all keys is 50KB.
    #[serde(rename="customAttributes")]
    
    pub custom_attributes: Option<HashMap<String, CustomAttribute>>,
    /// The desired education degrees for the job, such as Bachelors, Masters.
    #[serde(rename="degreeTypes")]
    
    pub degree_types: Option<Vec<String>>,
    /// The department or functional area within the company with the open position. The maximum number of allowed characters is 255.
    
    pub department: Option<String>,
    /// Output only. Derived details about the job posting.
    #[serde(rename="derivedInfo")]
    
    pub derived_info: Option<JobDerivedInfo>,
    /// Required. The description of the job, which typically includes a multi-paragraph description of the company and related information. Separate fields are provided on the job object for responsibilities, qualifications, and other job characteristics. Use of these separate job fields is recommended. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 100,000.
    
    pub description: Option<String>,
    /// The employment type(s) of a job, for example, full time or part time.
    #[serde(rename="employmentTypes")]
    
    pub employment_types: Option<Vec<String>>,
    /// A description of bonus, commission, and other compensation incentives associated with the job not including salary or pay. The maximum number of allowed characters is 10,000.
    
    pub incentives: Option<String>,
    /// The benefits included with the job.
    #[serde(rename="jobBenefits")]
    
    pub job_benefits: Option<Vec<String>>,
    /// The end timestamp of the job. Typically this field is used for contracting engagements. Invalid timestamps are ignored.
    #[serde(rename="jobEndTime")]
    
    pub job_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The experience level associated with the job, such as "Entry Level".
    #[serde(rename="jobLevel")]
    
    pub job_level: Option<String>,
    /// The start timestamp of the job in UTC time zone. Typically this field is used for contracting engagements. Invalid timestamps are ignored.
    #[serde(rename="jobStartTime")]
    
    pub job_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The language of the posting. This field is distinct from any requirements for fluency that are associated with the job. Language codes must be in BCP-47 format, such as "en-US" or "sr-Latn". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){: class="external" target="_blank" }. If this field is unspecified and Job.description is present, detected language code based on Job.description is assigned, otherwise defaults to 'en_US'.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Required during job update. The resource name for the job. This is generated by the service when a job is created. The format is "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For example, "projects/foo/tenants/bar/jobs/baz". Use of this field in job queries and API calls is preferred over the use of requisition_id since this value is unique.
    
    pub name: Option<String>,
    /// Output only. The timestamp when this job posting was created.
    #[serde(rename="postingCreateTime")]
    
    pub posting_create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Strongly recommended for the best service experience. The expiration timestamp of the job. After this timestamp, the job is marked as expired, and it no longer appears in search results. The expired job can't be listed by the ListJobs API, but it can be retrieved with the GetJob API or updated with the UpdateJob API or deleted with the DeleteJob API. An expired job can be updated and opened again by using a future expiration timestamp. Updating an expired job fails if there is another existing open job with same company, language_code and requisition_id. The expired jobs are retained in our system for 90 days. However, the overall expired job count cannot exceed 3 times the maximum number of open jobs over previous 7 days. If this threshold is exceeded, expired jobs are cleaned out in order of earliest expire time. Expired jobs are no longer accessible after they are cleaned out. Invalid timestamps are ignored, and treated as expire time not provided. If the timestamp is before the instant request is made, the job is treated as expired immediately on creation. This kind of job can not be updated. And when creating a job with past timestamp, the posting_publish_time must be set before posting_expire_time. The purpose of this feature is to allow other objects, such as Application, to refer a job that didn't exist in the system prior to becoming expired. If you want to modify a job that was expired on creation, delete it and create a new one. If this value isn't provided at the time of job creation or is invalid, the job posting expires after 30 days from the job's creation time. For example, if the job was created on 2017/01/01 13:00AM UTC with an unspecified expiration date, the job expires after 2017/01/31 13:00AM UTC. If this value isn't provided on job update, it depends on the field masks set by UpdateJobRequest.update_mask. If the field masks include job_end_time, or the masks are empty meaning that every field is updated, the job posting expires after 30 days from the job's last update time. Otherwise the expiration date isn't updated.
    #[serde(rename="postingExpireTime")]
    
    pub posting_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The timestamp this job posting was most recently published. The default value is the time the request arrives at the server. Invalid timestamps are ignored.
    #[serde(rename="postingPublishTime")]
    
    pub posting_publish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The job PostingRegion (for example, state, country) throughout which the job is available. If this field is set, a LocationFilter in a search query within the job region finds this job posting if an exact location match isn't specified. If this field is set to PostingRegion.NATION or PostingRegion.ADMINISTRATIVE_AREA, setting job Job.addresses to the same location level as this field is strongly recommended.
    #[serde(rename="postingRegion")]
    
    pub posting_region: Option<String>,
    /// Output only. The timestamp when this job posting was last updated.
    #[serde(rename="postingUpdateTime")]
    
    pub posting_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Options for job processing.
    #[serde(rename="processingOptions")]
    
    pub processing_options: Option<ProcessingOptions>,
    /// A promotion value of the job, as determined by the client. The value determines the sort order of the jobs returned when searching for jobs using the featured jobs search call, with higher promotional values being returned first and ties being resolved by relevance sort. Only the jobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH. Default value is 0, and negative values are treated as 0.
    #[serde(rename="promotionValue")]
    
    pub promotion_value: Option<i32>,
    /// A description of the qualifications required to perform the job. The use of this field is recommended as an alternative to using the more general description field. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 10,000.
    
    pub qualifications: Option<String>,
    /// Required. The requisition ID, also referred to as the posting ID, is assigned by the client to identify a job. This field is intended to be used by clients for client identification and tracking of postings. A job isn't allowed to be created if there is another job with the same company, language_code and requisition_id. The maximum number of allowed characters is 255.
    #[serde(rename="requisitionId")]
    
    pub requisition_id: Option<String>,
    /// A description of job responsibilities. The use of this field is recommended as an alternative to using the more general description field. This field accepts and sanitizes HTML input, and also accepts bold, italic, ordered list, and unordered list markup tags. The maximum number of allowed characters is 10,000.
    
    pub responsibilities: Option<String>,
    /// Required. The title of the job, such as "Software Engineer" The maximum number of allowed characters is 500.
    
    pub title: Option<String>,
    /// Deprecated. The job is only visible to the owner. The visibility of the job. Defaults to Visibility.ACCOUNT_ONLY if not specified.
    
    pub visibility: Option<String>,
}

impl client::RequestValue for Job {}
impl client::ResponseResult for Job {}


/// Derived details about the job posting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobDerivedInfo {
    /// Job categories derived from Job.title and Job.description.
    #[serde(rename="jobCategories")]
    
    pub job_categories: Option<Vec<String>>,
    /// Structured locations of the job, resolved from Job.addresses. locations are exactly matched to Job.addresses in the same order.
    
    pub locations: Option<Vec<Location>>,
}

impl client::Part for JobDerivedInfo {}


/// An event issued when a job seeker interacts with the application that implements Cloud Talent Solution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobEvent {
    /// Required. The job name(s) associated with this event. For example, if this is an impression event, this field contains the identifiers of all jobs shown to the job seeker. If this was a view event, this field contains the identifier of the viewed job. The format is "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}", for example, "projects/foo/tenants/bar/jobs/baz".
    
    pub jobs: Option<Vec<String>>,
    /// Required. The type of the event (see JobEventType).
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for JobEvent {}


/// The query required to perform a search query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobQuery {
    /// Allows filtering jobs by commute time with different travel methods (for example, driving or public transit). Note: This only works when you specify a CommuteMethod. In this case, location_filters is ignored. Currently we don't support sorting by commute time.
    #[serde(rename="commuteFilter")]
    
    pub commute_filter: Option<CommuteFilter>,
    /// This filter specifies the company entities to search against. If a value isn't specified, jobs are searched for against all companies. If multiple values are specified, jobs are searched against the companies specified. The format is "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}". For example, "projects/foo/tenants/bar/companies/baz". At most 20 company filters are allowed.
    
    pub companies: Option<Vec<String>>,
    /// This filter specifies the company Company.display_name of the jobs to search against. The company name must match the value exactly. Alternatively, the value being searched for can be wrapped in different match operators. `SUBSTRING_MATCH([value])` The company name must contain a case insensitive substring match of the value. Using this function may increase latency. Sample Value: `SUBSTRING_MATCH(google)` `MULTI_WORD_TOKEN_MATCH([value])` The value will be treated as a multi word token and the company name must contain a case insensitive match of the value. Using this function may increase latency. Sample Value: `MULTI_WORD_TOKEN_MATCH(google)` If a value isn't specified, jobs within the search results are associated with any company. If multiple values are specified, jobs within the search results may be associated with any of the specified companies. At most 20 company display name filters are allowed.
    #[serde(rename="companyDisplayNames")]
    
    pub company_display_names: Option<Vec<String>>,
    /// This search filter is applied only to Job.compensation_info. For example, if the filter is specified as "Hourly job with per-hour compensation > $15", only jobs meeting these criteria are searched. If a filter isn't defined, all open jobs are searched.
    #[serde(rename="compensationFilter")]
    
    pub compensation_filter: Option<CompensationFilter>,
    /// This filter specifies a structured syntax to match against the Job.custom_attributes marked as `filterable`. The syntax for this expression is a subset of SQL syntax. Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the left of the operator is a custom field key and the right of the operator is a number or a quoted string. You must escape backslash (\\) and quote (\") characters. Supported functions are `LOWER([field_name])` to perform a case insensitive match and `EMPTY([field_name])` to filter on the existence of a key. Boolean expressions (AND/OR/NOT) are supported up to 3 levels of nesting (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100 comparisons or functions are allowed in the expression. The expression must be < 10000 bytes in length. Sample Query: `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND driving_years > 10`
    #[serde(rename="customAttributeFilter")]
    
    pub custom_attribute_filter: Option<String>,
    /// This flag controls the spell-check feature. If false, the service attempts to correct a misspelled query, for example, "enginee" is corrected to "engineer". Defaults to false: a spell check is performed.
    #[serde(rename="disableSpellCheck")]
    
    pub disable_spell_check: Option<bool>,
    /// The employment type filter specifies the employment type of jobs to search against, such as EmploymentType.FULL_TIME. If a value isn't specified, jobs in the search results includes any employment type. If multiple values are specified, jobs in the search results include any of the specified employment types.
    #[serde(rename="employmentTypes")]
    
    pub employment_types: Option<Vec<String>>,
    /// This filter specifies a list of job names to be excluded during search. At most 400 excluded job names are allowed.
    #[serde(rename="excludedJobs")]
    
    pub excluded_jobs: Option<Vec<String>>,
    /// The category filter specifies the categories of jobs to search against. See JobCategory for more information. If a value isn't specified, jobs from any category are searched against. If multiple values are specified, jobs from any of the specified categories are searched against.
    #[serde(rename="jobCategories")]
    
    pub job_categories: Option<Vec<String>>,
    /// This filter specifies the locale of jobs to search against, for example, "en-US". If a value isn't specified, the search results can contain jobs in any locale. Language codes should be in BCP-47 format, such as "en-US" or "sr-Latn". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47). At most 10 language code filters are allowed.
    #[serde(rename="languageCodes")]
    
    pub language_codes: Option<Vec<String>>,
    /// The location filter specifies geo-regions containing the jobs to search against. See LocationFilter for more information. If a location value isn't specified, jobs fitting the other search criteria are retrieved regardless of where they're located. If multiple values are specified, jobs are retrieved from any of the specified locations. If different values are specified for the LocationFilter.distance_in_miles parameter, the maximum provided distance is used for all locations. At most 5 location filters are allowed.
    #[serde(rename="locationFilters")]
    
    pub location_filters: Option<Vec<LocationFilter>>,
    /// Jobs published within a range specified by this filter are searched against.
    #[serde(rename="publishTimeRange")]
    
    pub publish_time_range: Option<TimestampRange>,
    /// The query string that matches against the job title, description, and location fields. The maximum number of allowed characters is 255.
    
    pub query: Option<String>,
    /// The language code of query. For example, "en-US". This field helps to better interpret the query. If a value isn't specified, the query language code is automatically detected, which may not be accurate. Language code should be in BCP-47 format, such as "en-US" or "sr-Latn". For more information, see [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    #[serde(rename="queryLanguageCode")]
    
    pub query_language_code: Option<String>,
}

impl client::Part for JobQuery {}


/// An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this object must conform to the WGS84 standard. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for LatLng {}


/// The List companies response object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants companies list projects](ProjectTenantCompanyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCompaniesResponse {
    /// Companies for the current client.
    
    pub companies: Option<Vec<Company>>,
    /// Additional information for the API invocation, such as the request tracking id.
    
    pub metadata: Option<ResponseMetadata>,
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCompaniesResponse {}


/// List jobs response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants jobs list projects](ProjectTenantJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListJobsResponse {
    /// The Jobs for a given company. The maximum number of items returned is based on the limit field provided in the request.
    
    pub jobs: Option<Vec<Job>>,
    /// Additional information for the API invocation, such as the request tracking id.
    
    pub metadata: Option<ResponseMetadata>,
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListJobsResponse {}


/// The List tenants response object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants list projects](ProjectTenantListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTenantsResponse {
    /// Additional information for the API invocation, such as the request tracking id.
    
    pub metadata: Option<ResponseMetadata>,
    /// A token to retrieve the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Tenants for the current client.
    
    pub tenants: Option<Vec<Tenant>>,
}

impl client::ResponseResult for ListTenantsResponse {}


/// A resource that represents a location with full geographic information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// An object representing a latitude/longitude pair.
    #[serde(rename="latLng")]
    
    pub lat_lng: Option<LatLng>,
    /// The type of a location, which corresponds to the address lines field of google.type.PostalAddress. For example, "Downtown, Atlanta, GA, USA" has a type of LocationType.NEIGHBORHOOD, and "Kansas City, KS, USA" has a type of LocationType.LOCALITY.
    #[serde(rename="locationType")]
    
    pub location_type: Option<String>,
    /// Postal address of the location that includes human readable information, such as postal delivery and payments addresses. Given a postal address, a postal service can deliver items to a premises, P.O. Box, or other delivery location.
    #[serde(rename="postalAddress")]
    
    pub postal_address: Option<PostalAddress>,
    /// Radius in miles of the job location. This value is derived from the location bounding box in which a circle with the specified radius centered from google.type.LatLng covers the area associated with the job location. For example, currently, "Mountain View, CA, USA" has a radius of 6.17 miles.
    #[serde(rename="radiusMiles")]
    
    pub radius_miles: Option<f64>,
}

impl client::Part for Location {}


/// Geographic region of the search.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationFilter {
    /// The address name, such as "Mountain View" or "Bay Area".
    
    pub address: Option<String>,
    /// The distance_in_miles is applied when the location being searched for is identified as a city or smaller. This field is ignored if the location being searched for is a state or larger.
    #[serde(rename="distanceInMiles")]
    
    pub distance_in_miles: Option<f64>,
    /// The latitude and longitude of the geographic center to search from. This field is ignored if `address` is provided.
    #[serde(rename="latLng")]
    
    pub lat_lng: Option<LatLng>,
    /// CLDR region code of the country/region. This field may be used in two ways: 1) If telecommute preference is not set, this field is used address ambiguity of the user-input address. For example, "Liverpool" may refer to "Liverpool, NY, US" or "Liverpool, UK". This region code biases the address resolution toward a specific country or territory. If this field is not set, address resolution is biased toward the United States by default. 2) If telecommute preference is set to TELECOMMUTE_ALLOWED, the telecommute location filter will be limited to the region specified in this field. If this field is not set, the telecommute job locations will not be See https://unicode-org.github.io/cldr-staging/charts/latest/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// Allows the client to return jobs without a set location, specifically, telecommuting jobs (telecommuting is considered by the service as a special location). Job.posting_region indicates if a job permits telecommuting. If this field is set to TelecommutePreference.TELECOMMUTE_ALLOWED, telecommuting jobs are searched, and address and lat_lng are ignored. If not set or set to TelecommutePreference.TELECOMMUTE_EXCLUDED, the telecommute status of the jobs is ignored. Jobs that have PostingRegion.TELECOMMUTE and have additional Job.addresses may still be matched based on other location filters using address or latlng. This filter can be used by itself to search exclusively for telecommuting jobs, or it can be combined with another location filter to search for a combination of job locations, such as "Mountain View" or "telecommuting" jobs. However, when used in combination with other location filters, telecommuting jobs can be treated as less relevant than other jobs in the search response. This field is only used for job search requests.
    #[serde(rename="telecommutePreference")]
    
    pub telecommute_preference: Option<String>,
}

impl client::Part for LocationFilter {}


/// Job entry with metadata inside SearchJobsResponse.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatchingJob {
    /// Commute information which is generated based on specified CommuteFilter.
    #[serde(rename="commuteInfo")]
    
    pub commute_info: Option<CommuteInfo>,
    /// Job resource that matches the specified SearchJobsRequest.
    
    pub job: Option<Job>,
    /// A summary of the job with core information that's displayed on the search results listing page.
    #[serde(rename="jobSummary")]
    
    pub job_summary: Option<String>,
    /// Contains snippets of text from the Job.title field most closely matching a search query's keywords, if available. The matching query keywords are enclosed in HTML bold tags.
    #[serde(rename="jobTitleSnippet")]
    
    pub job_title_snippet: Option<String>,
    /// Contains snippets of text from the Job.description and similar fields that most closely match a search query's keywords, if available. All HTML tags in the original fields are stripped when returned in this field, and matching query keywords are enclosed in HTML bold tags.
    #[serde(rename="searchTextSnippet")]
    
    pub search_text_snippet: Option<String>,
}

impl client::Part for MatchingJob {}


/// Represents an amount of money with its currency type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The three-letter currency code defined in ISO 4217.
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    
    pub nanos: Option<i32>,
    /// The whole units of the amount. For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub units: Option<i64>,
}

impl client::Part for Money {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations get projects](ProjectOperationGetCall) (response)
/// * [tenants jobs batch create projects](ProjectTenantJobBatchCreateCall) (response)
/// * [tenants jobs batch delete projects](ProjectTenantJobBatchDeleteCall) (response)
/// * [tenants jobs batch update projects](ProjectTenantJobBatchUpdateCall) (response)
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


/// Represents a postal address, e.g. for postal delivery or payments addresses. Given a postal address, a postal service can deliver items to a premise, P.O. Box or similar. It is not intended to model geographical locations (roads, towns, mountains). In typical usage an address would be created via user input or from importing existing data, depending on the type of process. Advice on address input / editing: - Use an internationalization-ready address widget such as https://github.com/google/libaddressinput) - Users should not be presented with UI elements for input or editing of fields outside countries where that field is used. For more guidance on how to use this schema, please see: https://support.google.com/business/answer/6397478
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    /// Unstructured address lines describing the lower levels of an address. Because values in address_lines do not have type information and may sometimes contain multiple values in a single field (e.g. "Austin, TX"), it is important that the line order is clear. The order of address lines should be "envelope order" for the country/region of the address. In places where this can vary (e.g. Japan), address_language is used to make it explicit (e.g. "ja" for large-to-small ordering and "ja-Latn" or "en" for small-to-large). This way, the most specific line of an address can be selected based on the language. The minimum permitted structural representation of an address consists of a region_code with all remaining information placed in the address_lines. It would be possible to format such an address very approximately without geocoding, but no semantic reasoning could be made about any of the address components until it was at least partially resolved. Creating an address only containing a region_code and address_lines, and then geocoding is the recommended way to handle completely unstructured addresses (as opposed to guessing which parts of the address should be localities or administrative areas).
    #[serde(rename="addressLines")]
    
    pub address_lines: Option<Vec<String>>,
    /// Optional. Highest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state, a province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community (e.g. "Barcelona" and not "Catalonia"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland this should be left unpopulated.
    #[serde(rename="administrativeArea")]
    
    pub administrative_area: Option<String>,
    /// Optional. BCP-47 language code of the contents of this address (if known). This is often the UI language of the input form or is expected to match one of the languages used in the address' country/region, or their transliterated equivalents. This can affect formatting in certain countries, but is not critical to the correctness of the data and will never affect any validation or other non-formatting related operations. If this value is not known, it should be omitted (rather than specifying a possibly incorrect default). Examples: "zh-Hant", "ja", "ja-Latn", "en".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Optional. Generally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world where localities are not well defined or do not fit into this structure well, leave locality empty and use address_lines.
    
    pub locality: Option<String>,
    /// Optional. The name of the organization at the address.
    
    pub organization: Option<String>,
    /// Optional. Postal code of the address. Not all countries use or require postal codes to be present, but where they are used, they may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.).
    #[serde(rename="postalCode")]
    
    pub postal_code: Option<String>,
    /// Optional. The recipient at the address. This field may, under certain circumstances, contain multiline information. For example, it might contain "care of" information.
    
    pub recipients: Option<Vec<String>>,
    /// Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to ensure the value is correct. See https://cldr.unicode.org/ and https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: "CH" for Switzerland.
    #[serde(rename="regionCode")]
    
    pub region_code: Option<String>,
    /// The schema revision of the `PostalAddress`. This must be set to 0, which is the latest revision. All new revisions **must** be backward compatible with old revisions.
    
    pub revision: Option<i32>,
    /// Optional. Additional, country-specific, sorting code. This is not used in most regions. Where it is used, the value is either a string like "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a number alone, representing the "sector code" (Jamaica), "delivery area indicator" (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[serde(rename="sortingCode")]
    
    pub sorting_code: Option<String>,
    /// Optional. Sublocality of the address. For example, this can be neighborhoods, boroughs, districts.
    
    pub sublocality: Option<String>,
}

impl client::Part for PostalAddress {}


/// Options for job processing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProcessingOptions {
    /// If set to `true`, the service does not attempt to resolve a more precise address for the job.
    #[serde(rename="disableStreetAddressResolution")]
    
    pub disable_street_address_resolution: Option<bool>,
    /// Option for job HTML content sanitization. Applied fields are: * description * applicationInfo.instruction * incentives * qualifications * responsibilities HTML tags in these fields may be stripped if sanitiazation isn't disabled. Defaults to HtmlSanitization.SIMPLE_FORMATTING_ONLY.
    #[serde(rename="htmlSanitization")]
    
    pub html_sanitization: Option<String>,
}

impl client::Part for ProcessingOptions {}


/// Meta information related to the job searcher or entity conducting the job search. This information is used to improve the performance of the service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestMetadata {
    /// Only set when any of domain, session_id and user_id isn't available for some reason. It is highly recommended not to set this field and provide accurate domain, session_id and user_id for the best service experience.
    #[serde(rename="allowMissingIds")]
    
    pub allow_missing_ids: Option<bool>,
    /// The type of device used by the job seeker at the time of the call to the service.
    #[serde(rename="deviceInfo")]
    
    pub device_info: Option<DeviceInfo>,
    /// Required if allow_missing_ids is unset or `false`. The client-defined scope or source of the service call, which typically is the domain on which the service has been implemented and is currently being run. For example, if the service is being run by client *Foo, Inc.*, on job board www.foo.com and career site www.bar.com, then this field is set to "foo.com" for use on the job board, and "bar.com" for use on the career site. Note that any improvements to the model for a particular tenant site rely on this field being set correctly to a unique domain. The maximum number of allowed characters is 255.
    
    pub domain: Option<String>,
    /// Required if allow_missing_ids is unset or `false`. A unique session identification string. A session is defined as the duration of an end user's interaction with the service over a certain period. Obfuscate this field for privacy concerns before providing it to the service. Note that any improvements to the model for a particular tenant site rely on this field being set correctly to a unique session ID. The maximum number of allowed characters is 255.
    #[serde(rename="sessionId")]
    
    pub session_id: Option<String>,
    /// Required if allow_missing_ids is unset or `false`. A unique user identification string, as determined by the client. To have the strongest positive impact on search quality make sure the client-level is unique. Obfuscate this field for privacy concerns before providing it to the service. Note that any improvements to the model for a particular tenant site rely on this field being set correctly to a unique user ID. The maximum number of allowed characters is 255.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for RequestMetadata {}


/// Additional information returned to client, such as debugging information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// A unique id associated with this call. This id is logged for tracking purposes.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
}

impl client::Part for ResponseMetadata {}


/// The Request body of the `SearchJobs` call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants jobs search projects](ProjectTenantJobSearchCall) (request)
/// * [tenants jobs search for alert projects](ProjectTenantJobSearchForAlertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchJobsRequest {
    /// Controls over how job documents get ranked on top of existing relevance score (determined by API algorithm).
    #[serde(rename="customRankingInfo")]
    
    pub custom_ranking_info: Option<CustomRankingInfo>,
    /// This field is deprecated. Please use SearchJobsRequest.keyword_match_mode going forward. To migrate, disable_keyword_match set to false maps to KeywordMatchMode.KEYWORD_MATCH_ALL, and disable_keyword_match set to true maps to KeywordMatchMode.KEYWORD_MATCH_DISABLED. If SearchJobsRequest.keyword_match_mode is set, this field is ignored. Controls whether to disable exact keyword match on Job.title, Job.description, Job.company_display_name, Job.addresses, Job.qualifications. When disable keyword match is turned off, a keyword match returns jobs that do not match given category filters when there are matching keywords. For example, for the query "program manager," a result is returned even if the job posting has the title "software developer," which doesn't fall into "program manager" ontology, but does have "program manager" appearing in its description. For queries like "cloud" that don't contain title or location specific ontology, jobs with "cloud" keyword matches are returned regardless of this flag's value. Use Company.keyword_searchable_job_custom_attributes if company-specific globally matched custom field/attribute string values are needed. Enabling keyword match improves recall of subsequent search requests. Defaults to false.
    #[serde(rename="disableKeywordMatch")]
    
    pub disable_keyword_match: Option<bool>,
    /// Controls whether highly similar jobs are returned next to each other in the search results. Jobs are identified as highly similar based on their titles, job categories, and locations. Highly similar results are clustered so that only one representative job of the cluster is displayed to the job seeker higher up in the results, with the other jobs being displayed lower down in the results. Defaults to DiversificationLevel.SIMPLE if no value is specified.
    #[serde(rename="diversificationLevel")]
    
    pub diversification_level: Option<String>,
    /// Controls whether to broaden the search when it produces sparse results. Broadened queries append results to the end of the matching results list. Defaults to false.
    #[serde(rename="enableBroadening")]
    
    pub enable_broadening: Option<bool>,
    /// An expression specifies a histogram request against matching jobs. Expression syntax is an aggregation function call with histogram facets and other options. Available aggregation function calls are: * `count(string_histogram_facet)`: Count the number of matching entities, for each distinct attribute value. * `count(numeric_histogram_facet, list of buckets)`: Count the number of matching entities within each bucket. A maximum of 200 histogram buckets are supported. Data types: * Histogram facet: facet names with format `a-zA-Z+`. * String: string like "any string with backslash escape for quote(\")." * Number: whole number and floating point number like 10, -1 and -0.01. * List: list of elements with comma(,) separator surrounded by square brackets, for example, [1, 2, 3] and ["one", "two", "three"]. Built-in constants: * MIN (minimum number similar to java Double.MIN_VALUE) * MAX (maximum number similar to java Double.MAX_VALUE) Built-in functions: * bucket(start, end[, label]): bucket built-in function creates a bucket with range of start, end). Note that the end is exclusive, for example, bucket(1, MAX, "positive number") or bucket(1, 10). Job histogram facets: * company_display_name: histogram by [Job.company_display_name. * employment_type: histogram by Job.employment_types, for example, "FULL_TIME", "PART_TIME". * company_size (DEPRECATED): histogram by CompanySize, for example, "SMALL", "MEDIUM", "BIG". * publish_time_in_day: histogram by the Job.posting_publish_time in days. Must specify list of numeric buckets in spec. * publish_time_in_month: histogram by the Job.posting_publish_time in months. Must specify list of numeric buckets in spec. * publish_time_in_year: histogram by the Job.posting_publish_time in years. Must specify list of numeric buckets in spec. * degree_types: histogram by the Job.degree_types, for example, "Bachelors", "Masters". * job_level: histogram by the Job.job_level, for example, "Entry Level". * country: histogram by the country code of jobs, for example, "US", "FR". * admin1: histogram by the admin1 code of jobs, which is a global placeholder referring to the state, province, or the particular term a country uses to define the geographic structure below the country level, for example, "CA", "IL". * city: histogram by a combination of the "city name, admin1 code". For example, "Mountain View, CA", "New York, NY". * admin1_country: histogram by a combination of the "admin1 code, country", for example, "CA, US", "IL, US". * city_coordinate: histogram by the city center's GPS coordinates (latitude and longitude), for example, 37.4038522,-122.0987765. Since the coordinates of a city center can change, customers may need to refresh them periodically. * locale: histogram by the Job.language_code, for example, "en-US", "fr-FR". * language: histogram by the language subtag of the Job.language_code, for example, "en", "fr". * category: histogram by the JobCategory, for example, "COMPUTER_AND_IT", "HEALTHCARE". * base_compensation_unit: histogram by the CompensationInfo.CompensationUnit of base salary, for example, "WEEKLY", "MONTHLY". * base_compensation: histogram by the base salary. Must specify list of numeric buckets to group results by. * annualized_base_compensation: histogram by the base annualized salary. Must specify list of numeric buckets to group results by. * annualized_total_compensation: histogram by the total annualized salary. Must specify list of numeric buckets to group results by. * string_custom_attribute: histogram by string Job.custom_attributes. Values can be accessed via square bracket notations like string_custom_attribute["key1"]. * numeric_custom_attribute: histogram by numeric Job.custom_attributes. Values can be accessed via square bracket notations like numeric_custom_attribute["key1"]. Must specify list of numeric buckets to group results by. Example expressions: * `count(admin1)` * `count(base_compensation, [bucket(1000, 10000), bucket(10000, 100000), bucket(100000, MAX)])` * `count(string_custom_attribute["some-string-custom-attribute"])` * `count(numeric_custom_attribute["some-numeric-custom-attribute"], [bucket(MIN, 0, "negative"), bucket(0, MAX, "non-negative")])`
    #[serde(rename="histogramQueries")]
    
    pub histogram_queries: Option<Vec<HistogramQuery>>,
    /// Query used to search against jobs, such as keyword, location filters, etc.
    #[serde(rename="jobQuery")]
    
    pub job_query: Option<JobQuery>,
    /// The desired job attributes returned for jobs in the search response. Defaults to JobView.JOB_VIEW_SMALL if no value is specified.
    #[serde(rename="jobView")]
    
    pub job_view: Option<String>,
    /// Controls what keyword match options to use. If both keyword_match_mode and disable_keyword_match are set, keyword_match_mode will take precedence. Defaults to KeywordMatchMode.KEYWORD_MATCH_ALL if no value is specified.
    #[serde(rename="keywordMatchMode")]
    
    pub keyword_match_mode: Option<String>,
    /// A limit on the number of jobs returned in the search results. Increasing this value above the default value of 10 can increase search response time. The value can be between 1 and 100.
    #[serde(rename="maxPageSize")]
    
    pub max_page_size: Option<i32>,
    /// An integer that specifies the current offset (that is, starting result location, amongst the jobs deemed by the API as relevant) in search results. This field is only considered if page_token is unset. The maximum allowed value is 5000. Otherwise an error is thrown. For example, 0 means to return results starting from the first matching job, and 10 means to return from the 11th job. This can be used for pagination, (for example, pageSize = 10 and offset = 10 means to return from the second page).
    
    pub offset: Option<i32>,
    /// The criteria determining how search results are sorted. Default is `"relevance desc"`. Supported options are: * `"relevance desc"`: By relevance descending, as determined by the API algorithms. Relevance thresholding of query results is only available with this ordering. * `"posting_publish_time desc"`: By Job.posting_publish_time descending. * `"posting_update_time desc"`: By Job.posting_update_time descending. * `"title"`: By Job.title ascending. * `"title desc"`: By Job.title descending. * `"annualized_base_compensation"`: By job's CompensationInfo.annualized_base_compensation_range ascending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `"annualized_base_compensation desc"`: By job's CompensationInfo.annualized_base_compensation_range descending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `"annualized_total_compensation"`: By job's CompensationInfo.annualized_total_compensation_range ascending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `"annualized_total_compensation desc"`: By job's CompensationInfo.annualized_total_compensation_range descending. Jobs whose annualized base compensation is unspecified are put at the end of search results. * `"custom_ranking desc"`: By the relevance score adjusted to the SearchJobsRequest.CustomRankingInfo.ranking_expression with weight factor assigned by SearchJobsRequest.CustomRankingInfo.importance_level in descending order. * Location sorting: Use the special syntax to order jobs by distance: `"distance_from('Hawaii')"`: Order by distance from Hawaii. `"distance_from(19.89, 155.5)"`: Order by distance from a coordinate. `"distance_from('Hawaii'), distance_from('Puerto Rico')"`: Order by multiple locations. See details below. `"distance_from('Hawaii'), distance_from(19.89, 155.5)"`: Order by multiple locations. See details below. The string can have a maximum of 256 characters. When multiple distance centers are provided, a job that is close to any of the distance centers would have a high rank. When a job has multiple locations, the job location closest to one of the distance centers will be used. Jobs that don't have locations will be ranked at the bottom. Distance is calculated with a precision of 11.3 meters (37.4 feet). Diversification strategy is still applied unless explicitly disabled in diversification_level.
    #[serde(rename="orderBy")]
    
    pub order_by: Option<String>,
    /// The token specifying the current offset within search results. See SearchJobsResponse.next_page_token for an explanation of how to obtain the next set of query results.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Required. The meta information collected about the job searcher, used to improve the search quality of the service. The identifiers (such as `user_id`) are provided by users, and must be unique and consistent.
    #[serde(rename="requestMetadata")]
    
    pub request_metadata: Option<RequestMetadata>,
    /// Mode of a search. Defaults to SearchMode.JOB_SEARCH.
    #[serde(rename="searchMode")]
    
    pub search_mode: Option<String>,
}

impl client::RequestValue for SearchJobsRequest {}


/// Response for SearchJob method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants jobs search projects](ProjectTenantJobSearchCall) (response)
/// * [tenants jobs search for alert projects](ProjectTenantJobSearchForAlertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchJobsResponse {
    /// If query broadening is enabled, we may append additional results from the broadened query. This number indicates how many of the jobs returned in the jobs field are from the broadened query. These results are always at the end of the jobs list. In particular, a value of 0, or if the field isn't set, all the jobs in the jobs list are from the original (without broadening) query. If this field is non-zero, subsequent requests with offset after this result set should contain all broadened results.
    #[serde(rename="broadenedQueryJobsCount")]
    
    pub broadened_query_jobs_count: Option<i32>,
    /// The histogram results that match with specified SearchJobsRequest.histogram_queries.
    #[serde(rename="histogramQueryResults")]
    
    pub histogram_query_results: Option<Vec<HistogramQueryResult>>,
    /// The location filters that the service applied to the specified query. If any filters are lat-lng based, the Location.location_type is Location.LocationType.LOCATION_TYPE_UNSPECIFIED.
    #[serde(rename="locationFilters")]
    
    pub location_filters: Option<Vec<Location>>,
    /// The Job entities that match the specified SearchJobsRequest.
    #[serde(rename="matchingJobs")]
    
    pub matching_jobs: Option<Vec<MatchingJob>>,
    /// Additional information for the API invocation, such as the request tracking id.
    
    pub metadata: Option<ResponseMetadata>,
    /// The token that specifies the starting position of the next page of results. This field is empty if there are no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The spell checking result, and correction.
    #[serde(rename="spellCorrection")]
    
    pub spell_correction: Option<SpellingCorrection>,
    /// Number of jobs that match the specified query. Note: This size is precise only if the total is less than 100,000.
    #[serde(rename="totalSize")]
    
    pub total_size: Option<i32>,
}

impl client::ResponseResult for SearchJobsResponse {}


/// Spell check result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpellingCorrection {
    /// Indicates if the query was corrected by the spell checker.
    
    pub corrected: Option<bool>,
    /// Corrected output with html tags to highlight the corrected words. Corrected words are called out with the "*...*" html tags. For example, the user input query is "software enginear", where the second word, "enginear," is incorrect. It should be "engineer". When spelling correction is enabled, this value is "software *engineer*".
    #[serde(rename="correctedHtml")]
    
    pub corrected_html: Option<String>,
    /// Correction output consisting of the corrected keyword string.
    #[serde(rename="correctedText")]
    
    pub corrected_text: Option<String>,
}

impl client::Part for SpellingCorrection {}


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


/// A Tenant resource represents a tenant in the service. A tenant is a group or entity that shares common access with specific privileges for resources like jobs. Customer may create multiple tenants to provide data isolation for different groups.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [tenants create projects](ProjectTenantCreateCall) (request|response)
/// * [tenants get projects](ProjectTenantGetCall) (response)
/// * [tenants patch projects](ProjectTenantPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Tenant {
    /// Required. Client side tenant identifier, used to uniquely identify the tenant. The maximum number of allowed characters is 255.
    #[serde(rename="externalId")]
    
    pub external_id: Option<String>,
    /// Required during tenant update. The resource name for a tenant. This is generated by the service when a tenant is created. The format is "projects/{project_id}/tenants/{tenant_id}", for example, "projects/foo/tenants/bar".
    
    pub name: Option<String>,
}

impl client::RequestValue for Tenant {}
impl client::ResponseResult for Tenant {}


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// Message representing a period of time between two timestamps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimestampRange {
    /// End of the period (exclusive).
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Begin of the period (inclusive).
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimestampRange {}


