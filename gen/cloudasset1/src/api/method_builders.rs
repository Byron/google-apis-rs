use super::*;
/// A builder providing access to all methods supported on *asset* resources.
/// It is not used directly, but through the [`CloudAsset`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudasset1::{CloudAsset, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudAsset::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.assets();
/// # }
/// ```
pub struct AssetMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudAsset<S>,
}

impl<'a, S> client::MethodsBuilder for AssetMethods<'a, S> {}

impl<'a, S> AssetMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assets with time and resource types and returns paged results in response.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Name of the organization, folder, or project the assets belong to. Format: "organizations/[organization-number]" (such as "organizations/123"), "projects/[project-id]" (such as "projects/my-project-id"), "projects/[project-number]" (such as "projects/12345"), or "folders/[folder-number]" (such as "folders/12345").
    pub fn list(&self, parent: &str) -> AssetListCall<'a, S> {
        AssetListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _relationship_types: Default::default(),
            _read_time: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _content_type: Default::default(),
            _asset_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *effectiveIamPolicy* resources.
/// It is not used directly, but through the [`CloudAsset`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudasset1::{CloudAsset, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudAsset::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get(...)`
/// // to build up your call.
/// let rb = hub.effective_iam_policies();
/// # }
/// ```
pub struct EffectiveIamPolicyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudAsset<S>,
}

impl<'a, S> client::MethodsBuilder for EffectiveIamPolicyMethods<'a, S> {}

impl<'a, S> EffectiveIamPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets effective IAM policies for a batch of resources.
    /// 
    /// # Arguments
    ///
    /// * `scope` - Required. Only IAM policies on or below the scope will be returned. This can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects).
    pub fn batch_get(&self, scope: &str) -> EffectiveIamPolicyBatchGetCall<'a, S> {
        EffectiveIamPolicyBatchGetCall {
            hub: self.hub,
            _scope: scope.to_string(),
            _names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *feed* resources.
/// It is not used directly, but through the [`CloudAsset`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudasset1::{CloudAsset, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudAsset::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.feeds();
/// # }
/// ```
pub struct FeedMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudAsset<S>,
}

impl<'a, S> client::MethodsBuilder for FeedMethods<'a, S> {}

impl<'a, S> FeedMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a feed in a parent project/folder/organization to listen to its asset updates.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project/folder/organization where this feed should be created in. It can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id")", or a project number (such as "projects/12345").
    pub fn create(&self, request: CreateFeedRequest, parent: &str) -> FeedCreateCall<'a, S> {
        FeedCreateCall {
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
    /// Deletes an asset feed.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the feed and it must be in the format of: projects/project_number/feeds/feed_id folders/folder_number/feeds/feed_id organizations/organization_number/feeds/feed_id
    pub fn delete(&self, name: &str) -> FeedDeleteCall<'a, S> {
        FeedDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about an asset feed.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the Feed and it must be in the format of: projects/project_number/feeds/feed_id folders/folder_number/feeds/feed_id organizations/organization_number/feeds/feed_id
    pub fn get(&self, name: &str) -> FeedGetCall<'a, S> {
        FeedGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all asset feeds in a parent project/folder/organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project/folder/organization whose feeds are to be listed. It can only be using project/folder/organization number (such as "folders/12345")", or a project ID (such as "projects/my-project-id").
    pub fn list(&self, parent: &str) -> FeedListCall<'a, S> {
        FeedListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an asset feed configuration.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The format will be projects/{project_number}/feeds/{client-assigned_feed_identifier} or folders/{folder_number}/feeds/{client-assigned_feed_identifier} or organizations/{organization_number}/feeds/{client-assigned_feed_identifier} The client-assigned feed identifier must be unique within the parent project/folder/organization.
    pub fn patch(&self, request: UpdateFeedRequest, name: &str) -> FeedPatchCall<'a, S> {
        FeedPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *operation* resources.
/// It is not used directly, but through the [`CloudAsset`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudasset1::{CloudAsset, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudAsset::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.operations();
/// # }
/// ```
pub struct OperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudAsset<S>,
}

impl<'a, S> client::MethodsBuilder for OperationMethods<'a, S> {}

impl<'a, S> OperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the operation resource.
    pub fn get(&self, name: &str) -> OperationGetCall<'a, S> {
        OperationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *savedQuery* resources.
/// It is not used directly, but through the [`CloudAsset`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudasset1::{CloudAsset, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudAsset::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.saved_queries();
/// # }
/// ```
pub struct SavedQueryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudAsset<S>,
}

impl<'a, S> client::MethodsBuilder for SavedQueryMethods<'a, S> {}

impl<'a, S> SavedQueryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a saved query in a parent project/folder/organization.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the project/folder/organization where this saved_query should be created in. It can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id")", or a project number (such as "projects/12345").
    pub fn create(&self, request: SavedQuery, parent: &str) -> SavedQueryCreateCall<'a, S> {
        SavedQueryCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _saved_query_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a saved query.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the saved query to delete. It must be in the format of: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id
    pub fn delete(&self, name: &str) -> SavedQueryDeleteCall<'a, S> {
        SavedQueryDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets details about a saved query.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the saved query and it must be in the format of: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id
    pub fn get(&self, name: &str) -> SavedQueryGetCall<'a, S> {
        SavedQueryGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all saved queries in a parent project/folder/organization.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent project/folder/organization whose savedQueries are to be listed. It can only be using project/folder/organization number (such as "folders/12345")", or a project ID (such as "projects/my-project-id").
    pub fn list(&self, parent: &str) -> SavedQueryListCall<'a, S> {
        SavedQueryListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a saved query.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the saved query. The format must be: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id
    pub fn patch(&self, request: SavedQuery, name: &str) -> SavedQueryPatchCall<'a, S> {
        SavedQueryPatchCall {
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



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`CloudAsset`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudasset1 as cloudasset1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudasset1::{CloudAsset, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudAsset::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `analyze_iam_policy(...)`, `analyze_iam_policy_longrunning(...)`, `analyze_move(...)`, `analyze_org_policies(...)`, `analyze_org_policy_governed_assets(...)`, `analyze_org_policy_governed_containers(...)`, `batch_get_assets_history(...)`, `export_assets(...)`, `query_assets(...)`, `search_all_iam_policies(...)` and `search_all_resources(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudAsset<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes IAM policies to answer which identities have what accesses on which resources.
    /// 
    /// # Arguments
    ///
    /// * `scope` - Required. The relative name of the root asset. Only resources and IAM policies within the scope will be analyzed. This can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects).
    pub fn analyze_iam_policy(&self, scope: &str) -> MethodAnalyzeIamPolicyCall<'a, S> {
        MethodAnalyzeIamPolicyCall {
            hub: self.hub,
            _scope: scope.to_string(),
            _saved_analysis_query: Default::default(),
            _execution_timeout: Default::default(),
            _analysis_query_resource_selector_full_resource_name: Default::default(),
            _analysis_query_options_output_resource_edges: Default::default(),
            _analysis_query_options_output_group_edges: Default::default(),
            _analysis_query_options_expand_roles: Default::default(),
            _analysis_query_options_expand_resources: Default::default(),
            _analysis_query_options_expand_groups: Default::default(),
            _analysis_query_options_analyze_service_account_impersonation: Default::default(),
            _analysis_query_identity_selector_identity: Default::default(),
            _analysis_query_condition_context_access_time: Default::default(),
            _analysis_query_access_selector_roles: Default::default(),
            _analysis_query_access_selector_permissions: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes IAM policies asynchronously to answer which identities have what accesses on which resources, and writes the analysis results to a Google Cloud Storage or a BigQuery destination. For Cloud Storage destination, the output format is the JSON format that represents a AnalyzeIamPolicyResponse. This method implements the google.longrunning.Operation, which allows you to track the operation status. We recommend intervals of at least 2 seconds with exponential backoff retry to poll the operation result. The metadata contains the metadata for the long-running operation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `scope` - Required. The relative name of the root asset. Only resources and IAM policies within the scope will be analyzed. This can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects).
    pub fn analyze_iam_policy_longrunning(&self, request: AnalyzeIamPolicyLongrunningRequest, scope: &str) -> MethodAnalyzeIamPolicyLongrunningCall<'a, S> {
        MethodAnalyzeIamPolicyLongrunningCall {
            hub: self.hub,
            _request: request,
            _scope: scope.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyze moving a resource to a specified destination without kicking off the actual move. The analysis is best effort depending on the user's permissions of viewing different hierarchical policies and configurations. The policies and configuration are subject to change before the actual resource migration takes place.
    /// 
    /// # Arguments
    ///
    /// * `resource` - Required. Name of the resource to perform the analysis against. Only Google Cloud projects are supported as of today. Hence, this can only be a project ID (such as "projects/my-project-id") or a project number (such as "projects/12345").
    pub fn analyze_move(&self, resource: &str) -> MethodAnalyzeMoveCall<'a, S> {
        MethodAnalyzeMoveCall {
            hub: self.hub,
            _resource: resource.to_string(),
            _view: Default::default(),
            _destination_parent: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes organization policies under a scope.
    /// 
    /// # Arguments
    ///
    /// * `scope` - Required. The organization to scope the request. Only organization policies within the scope will be analyzed. * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    pub fn analyze_org_policies(&self, scope: &str) -> MethodAnalyzeOrgPolicyCall<'a, S> {
        MethodAnalyzeOrgPolicyCall {
            hub: self.hub,
            _scope: scope.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _constraint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes organization policies governed assets (Google Cloud resources or policies) under a scope. This RPC supports custom constraints and the following 10 canned constraints: * storage.uniformBucketLevelAccess * iam.disableServiceAccountKeyCreation * iam.allowedPolicyMemberDomains * compute.vmExternalIpAccess * appengine.enforceServiceAccountActAsCheck * gcp.resourceLocations * compute.trustedImageProjects * compute.skipDefaultNetworkCreation * compute.requireOsLogin * compute.disableNestedVirtualization This RPC only returns either resources of types supported by [searchable asset types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types), or IAM policies.
    /// 
    /// # Arguments
    ///
    /// * `scope` - Required. The organization to scope the request. Only organization policies within the scope will be analyzed. The output assets will also be limited to the ones governed by those in-scope organization policies. * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    pub fn analyze_org_policy_governed_assets(&self, scope: &str) -> MethodAnalyzeOrgPolicyGovernedAssetCall<'a, S> {
        MethodAnalyzeOrgPolicyGovernedAssetCall {
            hub: self.hub,
            _scope: scope.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _constraint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Analyzes organization policies governed containers (projects, folders or organization) under a scope.
    /// 
    /// # Arguments
    ///
    /// * `scope` - Required. The organization to scope the request. Only organization policies within the scope will be analyzed. The output containers will also be limited to the ones governed by those in-scope organization policies. * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    pub fn analyze_org_policy_governed_containers(&self, scope: &str) -> MethodAnalyzeOrgPolicyGovernedContainerCall<'a, S> {
        MethodAnalyzeOrgPolicyGovernedContainerCall {
            hub: self.hub,
            _scope: scope.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _constraint: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Batch gets the update history of assets that overlap a time window. For IAM_POLICY content, this API outputs history when the asset and its attached IAM POLICY both exist. This can create gaps in the output history. Otherwise, this API outputs history with asset in both non-delete or deleted status. If a specified asset does not exist, this API returns an INVALID_ARGUMENT error.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The relative name of the root asset. It can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id")", or a project number (such as "projects/12345").
    pub fn batch_get_assets_history(&self, parent: &str) -> MethodBatchGetAssetsHistoryCall<'a, S> {
        MethodBatchGetAssetsHistoryCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _relationship_types: Default::default(),
            _read_time_window_start_time: Default::default(),
            _read_time_window_end_time: Default::default(),
            _content_type: Default::default(),
            _asset_names: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exports assets with time and resource types to a given Cloud Storage location/BigQuery table. For Cloud Storage location destinations, the output format is newline-delimited JSON. Each line represents a google.cloud.asset.v1.Asset in the JSON format; for BigQuery table destinations, the output table stores the fields in asset Protobuf as columns. This API implements the google.longrunning.Operation API, which allows you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. For regular-size resource parent, the export operation usually finishes within 5 minutes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"), or a folder number (such as "folders/123").
    pub fn export_assets(&self, request: ExportAssetsRequest, parent: &str) -> MethodExportAssetCall<'a, S> {
        MethodExportAssetCall {
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
    /// Issue a job that queries assets using a SQL statement compatible with [BigQuery Standard SQL](http://cloud/bigquery/docs/reference/standard-sql/enabling-standard-sql). If the query execution finishes within timeout and there's no pagination, the full query results will be returned in the `QueryAssetsResponse`. Otherwise, full query results can be obtained by issuing extra requests with the `job_reference` from the a previous `QueryAssets` call. Note, the query result has approximately 10 GB limitation enforced by BigQuery https://cloud.google.com/bigquery/docs/best-practices-performance-output, queries return larger results will result in errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"), or a folder number (such as "folders/123"). Only assets belonging to the `parent` will be returned.
    pub fn query_assets(&self, request: QueryAssetsRequest, parent: &str) -> MethodQueryAssetCall<'a, S> {
        MethodQueryAssetCall {
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
    /// Searches all IAM policies within the specified scope, such as a project, folder, or organization. The caller must be granted the `cloudasset.assets.searchAllIamPolicies` permission on the desired scope, otherwise the request will be rejected.
    /// 
    /// # Arguments
    ///
    /// * `scope` - Required. A scope can be a project, a folder, or an organization. The search is limited to the IAM policies within the `scope`. The caller must be granted the [`cloudasset.assets.searchAllIamPolicies`](https://cloud.google.com/asset-inventory/docs/access-control#required_permissions) permission on the desired scope. The allowed values are: * projects/{PROJECT_ID} (e.g., "projects/foo-bar") * projects/{PROJECT_NUMBER} (e.g., "projects/12345678") * folders/{FOLDER_NUMBER} (e.g., "folders/1234567") * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    pub fn search_all_iam_policies(&self, scope: &str) -> MethodSearchAllIamPolicyCall<'a, S> {
        MethodSearchAllIamPolicyCall {
            hub: self.hub,
            _scope: scope.to_string(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _asset_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches all Google Cloud resources within the specified scope, such as a project, folder, or organization. The caller must be granted the `cloudasset.assets.searchAllResources` permission on the desired scope, otherwise the request will be rejected.
    /// 
    /// # Arguments
    ///
    /// * `scope` - Required. A scope can be a project, a folder, or an organization. The search is limited to the resources within the `scope`. The caller must be granted the [`cloudasset.assets.searchAllResources`](https://cloud.google.com/asset-inventory/docs/access-control#required_permissions) permission on the desired scope. The allowed values are: * projects/{PROJECT_ID} (e.g., "projects/foo-bar") * projects/{PROJECT_NUMBER} (e.g., "projects/12345678") * folders/{FOLDER_NUMBER} (e.g., "folders/1234567") * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    pub fn search_all_resources(&self, scope: &str) -> MethodSearchAllResourceCall<'a, S> {
        MethodSearchAllResourceCall {
            hub: self.hub,
            _scope: scope.to_string(),
            _read_mask: Default::default(),
            _query: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _asset_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



