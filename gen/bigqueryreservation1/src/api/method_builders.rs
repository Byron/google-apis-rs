use super::*;
/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`BigQueryReservation`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigqueryreservation1 as bigqueryreservation1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigqueryreservation1::{BigQueryReservation, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = BigQueryReservation::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_capacity_commitments_create(...)`, `locations_capacity_commitments_delete(...)`, `locations_capacity_commitments_get(...)`, `locations_capacity_commitments_list(...)`, `locations_capacity_commitments_merge(...)`, `locations_capacity_commitments_patch(...)`, `locations_capacity_commitments_split(...)`, `locations_get_bi_reservation(...)`, `locations_reservations_assignments_create(...)`, `locations_reservations_assignments_delete(...)`, `locations_reservations_assignments_list(...)`, `locations_reservations_assignments_move(...)`, `locations_reservations_assignments_patch(...)`, `locations_reservations_create(...)`, `locations_reservations_delete(...)`, `locations_reservations_get(...)`, `locations_reservations_list(...)`, `locations_reservations_patch(...)`, `locations_search_all_assignments(...)`, `locations_search_assignments(...)` and `locations_update_bi_reservation(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a BigQueryReservation<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new capacity commitment resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Resource name of the parent reservation. E.g., `projects/myproject/locations/US`
    pub fn locations_capacity_commitments_create(&self, request: CapacityCommitment, parent: &str) -> ProjectLocationCapacityCommitmentCreateCall<'a, S> {
        ProjectLocationCapacityCommitmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _enforce_single_admin_project_per_org: Default::default(),
            _capacity_commitment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a capacity commitment. Attempting to delete capacity commitment before its commitment_end_time will fail with the error code `google.rpc.Code.FAILED_PRECONDITION`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the capacity commitment to delete. E.g., `projects/myproject/locations/US/capacityCommitments/123`
    pub fn locations_capacity_commitments_delete(&self, name: &str) -> ProjectLocationCapacityCommitmentDeleteCall<'a, S> {
        ProjectLocationCapacityCommitmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _force: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about the capacity commitment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the capacity commitment to retrieve. E.g., `projects/myproject/locations/US/capacityCommitments/123`
    pub fn locations_capacity_commitments_get(&self, name: &str) -> ProjectLocationCapacityCommitmentGetCall<'a, S> {
        ProjectLocationCapacityCommitmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the capacity commitments for the admin project.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Resource name of the parent reservation. E.g., `projects/myproject/locations/US`
    pub fn locations_capacity_commitments_list(&self, parent: &str) -> ProjectLocationCapacityCommitmentListCall<'a, S> {
        ProjectLocationCapacityCommitmentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Merges capacity commitments of the same plan into a single commitment. The resulting capacity commitment has the greater commitment_end_time out of the to-be-merged capacity commitments. Attempting to merge capacity commitments of different plan will fail with the error code `google.rpc.Code.FAILED_PRECONDITION`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent resource that identifies admin project and location e.g., `projects/myproject/locations/us`
    pub fn locations_capacity_commitments_merge(&self, request: MergeCapacityCommitmentsRequest, parent: &str) -> ProjectLocationCapacityCommitmentMergeCall<'a, S> {
        ProjectLocationCapacityCommitmentMergeCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing capacity commitment. Only `plan` and `renewal_plan` fields can be updated. Plan can only be changed to a plan of a longer commitment period. Attempting to change to a plan with shorter commitment period will fail with the error code `google.rpc.Code.FAILED_PRECONDITION`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the capacity commitment, e.g., `projects/myproject/locations/US/capacityCommitments/123` The commitment_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters.
    pub fn locations_capacity_commitments_patch(&self, request: CapacityCommitment, name: &str) -> ProjectLocationCapacityCommitmentPatchCall<'a, S> {
        ProjectLocationCapacityCommitmentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Splits capacity commitment to two commitments of the same plan and `commitment_end_time`. A common use case is to enable downgrading commitments. For example, in order to downgrade from 10000 slots to 8000, you might split a 10000 capacity commitment into commitments of 2000 and 8000. Then, you delete the first one after the commitment end time passes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name e.g.,: `projects/myproject/locations/US/capacityCommitments/123`
    pub fn locations_capacity_commitments_split(&self, request: SplitCapacityCommitmentRequest, name: &str) -> ProjectLocationCapacityCommitmentSplitCall<'a, S> {
        ProjectLocationCapacityCommitmentSplitCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an assignment object which allows the given project to submit jobs of a certain type using slots from the specified reservation. Currently a resource (project, folder, organization) can only have one assignment per each (job_type, location) combination, and that reservation will be used for all jobs of the matching type. Different assignments can be created on different levels of the projects, folders or organization hierarchy. During query execution, the assignment is looked up at the project, folder and organization levels in that order. The first assignment found is applied to the query. When creating assignments, it does not matter if other assignments exist at higher levels. Example: * The organization `organizationA` contains two projects, `project1` and `project2`. * Assignments for all three entities (`organizationA`, `project1`, and `project2`) could all be created and mapped to the same or different reservations. "None" assignments represent an absence of the assignment. Projects assigned to None use on-demand pricing. To create a "None" assignment, use "none" as a reservation_id in the parent. Example parent: `projects/myproject/locations/US/reservations/none`. Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have 'bigquery.admin' permissions on the project using the reservation and the project that owns this reservation. Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment does not match location of the reservation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name of the assignment E.g. `projects/myproject/locations/US/reservations/team1-prod`
    pub fn locations_reservations_assignments_create(&self, request: Assignment, parent: &str) -> ProjectLocationReservationAssignmentCreateCall<'a, S> {
        ProjectLocationReservationAssignmentCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _assignment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a assignment. No expansion will happen. Example: * Organization `organizationA` contains two projects, `project1` and `project2`. * Reservation `res1` exists and was created previously. * CreateAssignment was used previously to define the following associations between entities and reservations: `` and `` In this example, deletion of the `` assignment won't affect the other assignment ``. After said deletion, queries from `project1` will still use `res1` while queries from `project2` will switch to use on-demand mode.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the resource, e.g. `projects/myproject/locations/US/reservations/team1-prod/assignments/123`
    pub fn locations_reservations_assignments_delete(&self, name: &str) -> ProjectLocationReservationAssignmentDeleteCall<'a, S> {
        ProjectLocationReservationAssignmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assignments. Only explicitly created assignments will be returned. Example: * Organization `organizationA` contains two projects, `project1` and `project2`. * Reservation `res1` exists and was created previously. * CreateAssignment was used previously to define the following associations between entities and reservations: `` and `` In this example, ListAssignments will just return the above two assignments for reservation `res1`, and no expansion/merge will happen. The wildcard "-" can be used for reservations in the request. In that case all assignments belongs to the specified project and location will be listed. **Note** "-" cannot be used for projects nor locations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name e.g.: `projects/myproject/locations/US/reservations/team1-prod` Or: `projects/myproject/locations/US/reservations/-`
    pub fn locations_reservations_assignments_list(&self, parent: &str) -> ProjectLocationReservationAssignmentListCall<'a, S> {
        ProjectLocationReservationAssignmentListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves an assignment under a new reservation. This differs from removing an existing assignment and recreating a new one by providing a transactional change that ensures an assignee always has an associated reservation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the assignment, e.g. `projects/myproject/locations/US/reservations/team1-prod/assignments/123`
    pub fn locations_reservations_assignments_move(&self, request: MoveAssignmentRequest, name: &str) -> ProjectLocationReservationAssignmentMoveCall<'a, S> {
        ProjectLocationReservationAssignmentMoveCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing assignment. Only the `priority` field can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. Name of the resource. E.g.: `projects/myproject/locations/US/reservations/team1-prod/assignments/123`. The assignment_id must only contain lower case alphanumeric characters or dashes and the max length is 64 characters.
    pub fn locations_reservations_assignments_patch(&self, request: Assignment, name: &str) -> ProjectLocationReservationAssignmentPatchCall<'a, S> {
        ProjectLocationReservationAssignmentPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new reservation resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. Project, location. E.g., `projects/myproject/locations/US`
    pub fn locations_reservations_create(&self, request: Reservation, parent: &str) -> ProjectLocationReservationCreateCall<'a, S> {
        ProjectLocationReservationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _reservation_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a reservation. Returns `google.rpc.Code.FAILED_PRECONDITION` when reservation has assignments.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the reservation to retrieve. E.g., `projects/myproject/locations/US/reservations/team1-prod`
    pub fn locations_reservations_delete(&self, name: &str) -> ProjectLocationReservationDeleteCall<'a, S> {
        ProjectLocationReservationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about the reservation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Resource name of the reservation to retrieve. E.g., `projects/myproject/locations/US/reservations/team1-prod`
    pub fn locations_reservations_get(&self, name: &str) -> ProjectLocationReservationGetCall<'a, S> {
        ProjectLocationReservationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the reservations for the project in the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent resource name containing project and location, e.g.: `projects/myproject/locations/US`
    pub fn locations_reservations_list(&self, parent: &str) -> ProjectLocationReservationListCall<'a, S> {
        ProjectLocationReservationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing reservation resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the reservation, e.g., `projects/*/locations/*/reservations/team1-prod`. The reservation_id must only contain lower case alphanumeric characters or dashes. It must start with a letter and must not end with a dash. Its maximum length is 64 characters.
    pub fn locations_reservations_patch(&self, request: Reservation, name: &str) -> ProjectLocationReservationPatchCall<'a, S> {
        ProjectLocationReservationPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a BI reservation.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the requested reservation, for example: `projects/{project_id}/locations/{location_id}/biReservation`
    pub fn locations_get_bi_reservation(&self, name: &str) -> ProjectLocationGetBiReservationCall<'a, S> {
        ProjectLocationGetBiReservationCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up assignments for a specified resource for a particular region. If the request is about a project: 1. Assignments created on the project will be returned if they exist. 2. Otherwise assignments created on the closest ancestor will be returned. 3. Assignments for different JobTypes will all be returned. The same logic applies if the request is about a folder. If the request is about an organization, then assignments created on the organization will be returned (organization doesn't have ancestors). Comparing to ListAssignments, there are some behavior differences: 1. permission on the assignee will be verified in this API. 2. Hierarchy lookup (project->folder->organization) happens in this API. 3. Parent here is `projects/*/locations/*`, instead of `projects/*/locations/*reservations/*`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name with location (project name could be the wildcard '-'), e.g.: `projects/-/locations/US`.
    pub fn locations_search_all_assignments(&self, parent: &str) -> ProjectLocationSearchAllAssignmentCall<'a, S> {
        ProjectLocationSearchAllAssignmentCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated: Looks up assignments for a specified resource for a particular region. If the request is about a project: 1. Assignments created on the project will be returned if they exist. 2. Otherwise assignments created on the closest ancestor will be returned. 3. Assignments for different JobTypes will all be returned. The same logic applies if the request is about a folder. If the request is about an organization, then assignments created on the organization will be returned (organization doesn't have ancestors). Comparing to ListAssignments, there are some behavior differences: 1. permission on the assignee will be verified in this API. 2. Hierarchy lookup (project->folder->organization) happens in this API. 3. Parent here is `projects/*/locations/*`, instead of `projects/*/locations/*reservations/*`. **Note** "-" cannot be used for projects nor locations.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The resource name of the admin project(containing project and location), e.g.: `projects/myproject/locations/US`.
    pub fn locations_search_assignments(&self, parent: &str) -> ProjectLocationSearchAssignmentCall<'a, S> {
        ProjectLocationSearchAssignmentCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a BI reservation. Only fields specified in the `field_mask` are updated. A singleton BI reservation always exists with default size 0. In order to reserve BI capacity it needs to be updated to an amount greater than 0. In order to release BI capacity reservation size must be set to 0.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the singleton BI reservation. Reservation names have the form `projects/{project_id}/locations/{location_id}/biReservation`.
    pub fn locations_update_bi_reservation(&self, request: BiReservation, name: &str) -> ProjectLocationUpdateBiReservationCall<'a, S> {
        ProjectLocationUpdateBiReservationCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



