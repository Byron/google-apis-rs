use super::*;
/// An assignment allows a project to submit jobs of a certain type using slots from the specified reservation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reservations assignments create projects](ProjectLocationReservationAssignmentCreateCall) (request|response)
/// * [locations reservations assignments move projects](ProjectLocationReservationAssignmentMoveCall) (response)
/// * [locations reservations assignments patch projects](ProjectLocationReservationAssignmentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Assignment {
    /// The resource which will use the reservation. E.g. `projects/myproject`, `folders/123`, or `organizations/456`.
    
    pub assignee: Option<String>,
    /// Which type of jobs will use the reservation.
    #[serde(rename="jobType")]
    
    pub job_type: Option<AssignmentJobTypeEnum>,
    /// Output only. Name of the resource. E.g.: `projects/myproject/locations/US/reservations/team1-prod/assignments/123`. The assignment_id must only contain lower case alphanumeric characters or dashes and the max length is 64 characters.
    
    pub name: Option<String>,
    /// Output only. State of the assignment.
    
    pub state: Option<AssignmentStateEnum>,
}

impl client::RequestValue for Assignment {}
impl client::ResponseResult for Assignment {}


/// Represents a BI Reservation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get bi reservation projects](ProjectLocationGetBiReservationCall) (response)
/// * [locations update bi reservation projects](ProjectLocationUpdateBiReservationCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BiReservation {
    /// The resource name of the singleton BI reservation. Reservation names have the form `projects/{project_id}/locations/{location_id}/biReservation`.
    
    pub name: Option<String>,
    /// Preferred tables to use BI capacity for.
    #[serde(rename="preferredTables")]
    
    pub preferred_tables: Option<Vec<TableReference>>,
    /// Size of a reservation, in bytes.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<i64>,
    /// Output only. The last update timestamp of a reservation.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for BiReservation {}
impl client::ResponseResult for BiReservation {}


/// Capacity commitment is a way to purchase compute capacity for BigQuery jobs (in the form of slots) with some committed period of usage. Annual commitments renew by default. Commitments can be removed after their commitment end time passes. In order to remove annual commitment, its plan needs to be changed to monthly or flex first. A capacity commitment resource exists as a child resource of the admin project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations capacity commitments create projects](ProjectLocationCapacityCommitmentCreateCall) (request|response)
/// * [locations capacity commitments get projects](ProjectLocationCapacityCommitmentGetCall) (response)
/// * [locations capacity commitments merge projects](ProjectLocationCapacityCommitmentMergeCall) (response)
/// * [locations capacity commitments patch projects](ProjectLocationCapacityCommitmentPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CapacityCommitment {
    /// Output only. The end of the current commitment period. It is applicable only for ACTIVE capacity commitments.
    #[serde(rename="commitmentEndTime")]
    
    pub commitment_end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The start of the current commitment period. It is applicable only for ACTIVE capacity commitments.
    #[serde(rename="commitmentStartTime")]
    
    pub commitment_start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. For FAILED commitment plan, provides the reason of failure.
    #[serde(rename="failureStatus")]
    
    pub failure_status: Option<Status>,
    /// Applicable only for commitments located within one of the BigQuery multi-regions (US or EU). If set to true, this commitment is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this commitment is placed in the organization's default region.
    #[serde(rename="multiRegionAuxiliary")]
    
    pub multi_region_auxiliary: Option<bool>,
    /// Output only. The resource name of the capacity commitment, e.g., `projects/myproject/locations/US/capacityCommitments/123` The commitment_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters.
    
    pub name: Option<String>,
    /// Capacity commitment commitment plan.
    
    pub plan: Option<CapacityCommitmentPlanEnum>,
    /// The plan this capacity commitment is converted to after commitment_end_time passes. Once the plan is changed, committed period is extended according to commitment plan. Only applicable for ANNUAL and TRIAL commitments.
    #[serde(rename="renewalPlan")]
    
    pub renewal_plan: Option<CapacityCommitmentRenewalPlanEnum>,
    /// Number of slots in this commitment.
    #[serde(rename="slotCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub slot_count: Option<i64>,
    /// Output only. State of the commitment.
    
    pub state: Option<CapacityCommitmentStateEnum>,
}

impl client::RequestValue for CapacityCommitment {}
impl client::ResponseResult for CapacityCommitment {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations capacity commitments delete projects](ProjectLocationCapacityCommitmentDeleteCall) (response)
/// * [locations reservations assignments delete projects](ProjectLocationReservationAssignmentDeleteCall) (response)
/// * [locations reservations delete projects](ProjectLocationReservationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The response for ReservationService.ListAssignments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reservations assignments list projects](ProjectLocationReservationAssignmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAssignmentsResponse {
    /// List of assignments visible to the user.
    
    pub assignments: Option<Vec<Assignment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAssignmentsResponse {}


/// The response for ReservationService.ListCapacityCommitments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations capacity commitments list projects](ProjectLocationCapacityCommitmentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCapacityCommitmentsResponse {
    /// List of capacity commitments visible to the user.
    #[serde(rename="capacityCommitments")]
    
    pub capacity_commitments: Option<Vec<CapacityCommitment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCapacityCommitmentsResponse {}


/// The response for ReservationService.ListReservations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reservations list projects](ProjectLocationReservationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReservationsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of reservations visible to the user.
    
    pub reservations: Option<Vec<Reservation>>,
}

impl client::ResponseResult for ListReservationsResponse {}


/// The request for ReservationService.MergeCapacityCommitments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations capacity commitments merge projects](ProjectLocationCapacityCommitmentMergeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MergeCapacityCommitmentsRequest {
    /// Ids of capacity commitments to merge. These capacity commitments must exist under admin project and location specified in the parent. ID is the last portion of capacity commitment name e.g., 'abc' for projects/myproject/locations/US/capacityCommitments/abc
    #[serde(rename="capacityCommitmentIds")]
    
    pub capacity_commitment_ids: Option<Vec<String>>,
}

impl client::RequestValue for MergeCapacityCommitmentsRequest {}


/// The request for ReservationService.MoveAssignment. **Note**: “bigquery.reservationAssignments.create” permission is required on the destination_id. **Note**: “bigquery.reservationAssignments.create” and “bigquery.reservationAssignments.delete” permission are required on the related assignee.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reservations assignments move projects](ProjectLocationReservationAssignmentMoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveAssignmentRequest {
    /// The new reservation ID, e.g.: `projects/myotherproject/locations/US/reservations/team2-prod`
    #[serde(rename="destinationId")]
    
    pub destination_id: Option<String>,
}

impl client::RequestValue for MoveAssignmentRequest {}


/// A reservation is a mechanism used to guarantee slots to users.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations reservations create projects](ProjectLocationReservationCreateCall) (request|response)
/// * [locations reservations get projects](ProjectLocationReservationGetCall) (response)
/// * [locations reservations patch projects](ProjectLocationReservationPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Reservation {
    /// Job concurrency target which sets a soft upper bound on the number of jobs that can run concurrently in this reservation. This is a soft target due to asynchronous nature of the system and various optimizations for small queries. Default value is 0 which means that concurrency target will be automatically computed by the system. NOTE: this field is exposed as `target_job_concurrency` in the Information Schema, DDL and BQ CLI.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub concurrency: Option<i64>,
    /// Output only. Creation time of the reservation.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// If false, any query or pipeline job using this reservation will use idle slots from other reservations within the same admin project. If true, a query or pipeline job using this reservation will execute with the slot capacity specified in the slot_capacity field at most.
    #[serde(rename="ignoreIdleSlots")]
    
    pub ignore_idle_slots: Option<bool>,
    /// Applicable only for reservations located within one of the BigQuery multi-regions (US or EU). If set to true, this reservation is placed in the organization's secondary region which is designated for disaster recovery purposes. If false, this reservation is placed in the organization's default region.
    #[serde(rename="multiRegionAuxiliary")]
    
    pub multi_region_auxiliary: Option<bool>,
    /// The resource name of the reservation, e.g., `projects/*/locations/*/reservations/team1-prod`. The reservation_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters.
    
    pub name: Option<String>,
    /// Minimum slots available to this reservation. A slot is a unit of computational power in BigQuery, and serves as the unit of parallelism. Queries using this reservation might use more slots during runtime if ignore_idle_slots is set to false. If total slot_capacity of the reservation and its siblings exceeds the total slot_count of all capacity commitments, the request will fail with `google.rpc.Code.RESOURCE_EXHAUSTED`. NOTE: for reservations in US or EU multi-regions, slot capacity constraints are checked separately for default and auxiliary regions. See multi_region_auxiliary flag for more details.
    #[serde(rename="slotCapacity")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub slot_capacity: Option<i64>,
    /// Output only. Last update time of the reservation.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Reservation {}
impl client::ResponseResult for Reservation {}


/// The response for ReservationService.SearchAllAssignments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations search all assignments projects](ProjectLocationSearchAllAssignmentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchAllAssignmentsResponse {
    /// List of assignments visible to the user.
    
    pub assignments: Option<Vec<Assignment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchAllAssignmentsResponse {}


/// The response for ReservationService.SearchAssignments.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations search assignments projects](ProjectLocationSearchAssignmentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchAssignmentsResponse {
    /// List of assignments visible to the user.
    
    pub assignments: Option<Vec<Assignment>>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchAssignmentsResponse {}


/// The request for ReservationService.SplitCapacityCommitment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations capacity commitments split projects](ProjectLocationCapacityCommitmentSplitCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SplitCapacityCommitmentRequest {
    /// Number of slots in the capacity commitment after the split.
    #[serde(rename="slotCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub slot_count: Option<i64>,
}

impl client::RequestValue for SplitCapacityCommitmentRequest {}


/// The response for ReservationService.SplitCapacityCommitment.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations capacity commitments split projects](ProjectLocationCapacityCommitmentSplitCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SplitCapacityCommitmentResponse {
    /// First capacity commitment, result of a split.
    
    pub first: Option<CapacityCommitment>,
    /// Second capacity commitment, result of a split.
    
    pub second: Option<CapacityCommitment>,
}

impl client::ResponseResult for SplitCapacityCommitmentResponse {}


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


/// Fully qualified reference to BigQuery table. Internally stored as google.cloud.bi.v1.BqTableReference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableReference {
    /// The ID of the dataset in the above project.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// The assigned project ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The ID of the table in the above dataset.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::Part for TableReference {}


