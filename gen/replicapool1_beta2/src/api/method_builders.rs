use super::*;
/// A builder providing access to all methods supported on *instanceGroupManager* resources.
/// It is not used directly, but through the [`Replicapool`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `abandon_instances(...)`, `delete(...)`, `delete_instances(...)`, `get(...)`, `insert(...)`, `list(...)`, `recreate_instances(...)`, `resize(...)`, `set_instance_template(...)` and `set_target_pools(...)`
/// // to build up your call.
/// let rb = hub.instance_group_managers();
/// # }
/// ```
pub struct InstanceGroupManagerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Replicapool<S>,
}

impl<'a, S> client::MethodsBuilder for InstanceGroupManagerMethods<'a, S> {}

impl<'a, S> InstanceGroupManagerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified instances from the managed instance group, and from any target pools of which they were members, without deleting the instances.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn abandon_instances(&self, request: InstanceGroupManagersAbandonInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerAbandonInstanceCall<'a, S> {
        InstanceGroupManagerAbandonInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the instance group manager and all instances contained within. If you'd like to delete the manager without deleting the instances, you must first abandon the instances to remove them from the group.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - Name of the Instance Group Manager resource to delete.
    pub fn delete(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerDeleteCall<'a, S> {
        InstanceGroupManagerDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified instances. The instances are deleted, then removed from the instance group and any target pools of which they were a member. The targetSize of the instance group manager is reduced by the number of instances deleted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn delete_instances(&self, request: InstanceGroupManagersDeleteInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerDeleteInstanceCall<'a, S> {
        InstanceGroupManagerDeleteInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Instance Group Manager resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - Name of the instance resource to return.
    pub fn get(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerGetCall<'a, S> {
        InstanceGroupManagerGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an instance group manager, as well as the instance group and the specified number of instances.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `size` - Number of instances that should exist.
    pub fn insert(&self, request: InstanceGroupManager, project: &str, zone: &str, size: i32) -> InstanceGroupManagerInsertCall<'a, S> {
        InstanceGroupManagerInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _size: size,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of Instance Group Manager resources contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    pub fn list(&self, project: &str, zone: &str) -> InstanceGroupManagerListCall<'a, S> {
        InstanceGroupManagerListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Recreates the specified instances. The instances are deleted, then recreated using the instance group manager's current instance template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn recreate_instances(&self, request: InstanceGroupManagersRecreateInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerRecreateInstanceCall<'a, S> {
        InstanceGroupManagerRecreateInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resizes the managed instance group up or down. If resized up, new instances are created using the current instance template. If resized down, instances are removed in the order outlined in Resizing a managed instance group.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    /// * `size` - Number of instances that should exist in this Instance Group Manager.
    pub fn resize(&self, project: &str, zone: &str, instance_group_manager: &str, size: i32) -> InstanceGroupManagerResizeCall<'a, S> {
        InstanceGroupManagerResizeCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _size: size,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the instance template to use when creating new instances in this group. Existing instances are not affected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn set_instance_template(&self, request: InstanceGroupManagersSetInstanceTemplateRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerSetInstanceTemplateCall<'a, S> {
        InstanceGroupManagerSetInstanceTemplateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the target pools to which all new instances in this group are assigned. Existing instances in the group are not affected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn set_target_pools(&self, request: InstanceGroupManagersSetTargetPoolsRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerSetTargetPoolCall<'a, S> {
        InstanceGroupManagerSetTargetPoolCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *zoneOperation* resources.
/// It is not used directly, but through the [`Replicapool`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.zone_operations();
/// # }
/// ```
pub struct ZoneOperationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Replicapool<S>,
}

impl<'a, S> client::MethodsBuilder for ZoneOperationMethods<'a, S> {}

impl<'a, S> ZoneOperationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified zone-specific operation resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    /// * `zone` - Name of the zone scoping this request.
    /// * `operation` - Name of the operation resource to return.
    pub fn get(&self, project: &str, zone: &str, operation: &str) -> ZoneOperationGetCall<'a, S> {
        ZoneOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of operation resources contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    /// * `zone` - Name of the zone scoping this request.
    pub fn list(&self, project: &str, zone: &str) -> ZoneOperationListCall<'a, S> {
        ZoneOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



