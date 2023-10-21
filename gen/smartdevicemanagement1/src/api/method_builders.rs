use super::*;
/// A builder providing access to all methods supported on *enterprise* resources.
/// It is not used directly, but through the [`SmartDeviceManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_smartdevicemanagement1 as smartdevicemanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use smartdevicemanagement1::{SmartDeviceManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SmartDeviceManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `devices_execute_command(...)`, `devices_get(...)`, `devices_list(...)`, `structures_get(...)`, `structures_list(...)`, `structures_rooms_get(...)` and `structures_rooms_list(...)`
/// // to build up your call.
/// let rb = hub.enterprises();
/// # }
/// ```
pub struct EnterpriseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SmartDeviceManagement<S>,
}

impl<'a, S> client::MethodsBuilder for EnterpriseMethods<'a, S> {}

impl<'a, S> EnterpriseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Executes a command to device managed by the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the device requested. For example: "enterprises/XYZ/devices/123"
    pub fn devices_execute_command(&self, request: GoogleHomeEnterpriseSdmV1ExecuteDeviceCommandRequest, name: &str) -> EnterpriseDeviceExecuteCommandCall<'a, S> {
        EnterpriseDeviceExecuteCommandCall {
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
    /// Gets a device managed by the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the device requested. For example: "enterprises/XYZ/devices/123"
    pub fn devices_get(&self, name: &str) -> EnterpriseDeviceGetCall<'a, S> {
        EnterpriseDeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists devices managed by the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent enterprise to list devices under. E.g. "enterprises/XYZ".
    pub fn devices_list(&self, parent: &str) -> EnterpriseDeviceListCall<'a, S> {
        EnterpriseDeviceListCall {
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
    /// Gets a room managed by the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the room requested. For example: "enterprises/XYZ/structures/ABC/rooms/123".
    pub fn structures_rooms_get(&self, name: &str) -> EnterpriseStructureRoomGetCall<'a, S> {
        EnterpriseStructureRoomGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists rooms managed by the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent resource name of the rooms requested. For example: "enterprises/XYZ/structures/ABC".
    pub fn structures_rooms_list(&self, parent: &str) -> EnterpriseStructureRoomListCall<'a, S> {
        EnterpriseStructureRoomListCall {
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
    /// Gets a structure managed by the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the structure requested. For example: "enterprises/XYZ/structures/ABC".
    pub fn structures_get(&self, name: &str) -> EnterpriseStructureGetCall<'a, S> {
        EnterpriseStructureGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists structures managed by the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The parent enterprise to list structures under. E.g. "enterprises/XYZ".
    pub fn structures_list(&self, parent: &str) -> EnterpriseStructureListCall<'a, S> {
        EnterpriseStructureListCall {
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
}



