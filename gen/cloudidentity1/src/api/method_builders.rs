use super::*;
/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the [`CloudIdentity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudidentity1 as cloudidentity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudidentity1::{CloudIdentity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIdentity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `userinvitations_cancel(...)`, `userinvitations_get(...)`, `userinvitations_is_invitable_user(...)`, `userinvitations_list(...)` and `userinvitations_send(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIdentity<S>,
}

impl<'a, S> client::MethodsBuilder for CustomerMethods<'a, S> {}

impl<'a, S> CustomerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a UserInvitation that was already sent.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}`
    pub fn userinvitations_cancel(&self, request: CancelUserInvitationRequest, name: &str) -> CustomerUserinvitationCancelCall<'a, S> {
        CustomerUserinvitationCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a UserInvitation resource. **Note:** New consumer accounts with the customer's verified domain created within the previous 48 hours will not appear in the result. This delay also applies to newly-verified domains.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}`
    pub fn userinvitations_get(&self, name: &str) -> CustomerUserinvitationGetCall<'a, S> {
        CustomerUserinvitationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Verifies whether a user account is eligible to receive a UserInvitation (is an unmanaged account). Eligibility is based on the following criteria: * the email address is a consumer account and it's the primary email address of the account, and * the domain of the email address matches an existing verified Google Workspace or Cloud Identity domain If both conditions are met, the user is eligible. **Note:** This method is not supported for Workspace Essentials customers.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}`
    pub fn userinvitations_is_invitable_user(&self, name: &str) -> CustomerUserinvitationIsInvitableUserCall<'a, S> {
        CustomerUserinvitationIsInvitableUserCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of UserInvitation resources. **Note:** New consumer accounts with the customer's verified domain created within the previous 48 hours will not appear in the result. This delay also applies to newly-verified domains.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The customer ID of the Google Workspace or Cloud Identity account the UserInvitation resources are associated with.
    pub fn userinvitations_list(&self, parent: &str) -> CustomerUserinvitationListCall<'a, S> {
        CustomerUserinvitationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sends a UserInvitation to email. If the `UserInvitation` does not exist for this request and it is a valid request, the request creates a `UserInvitation`. **Note:** The `get` and `list` methods have a 48-hour delay where newly-created consumer accounts will not appear in the results. You can still send a `UserInvitation` to those accounts if you know the unmanaged email address and IsInvitableUser==True.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. `UserInvitation` name in the format `customers/{customer}/userinvitations/{user_email_address}`
    pub fn userinvitations_send(&self, request: SendUserInvitationRequest, name: &str) -> CustomerUserinvitationSendCall<'a, S> {
        CustomerUserinvitationSendCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *device* resources.
/// It is not used directly, but through the [`CloudIdentity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudidentity1 as cloudidentity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudidentity1::{CloudIdentity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIdentity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel_wipe(...)`, `create(...)`, `delete(...)`, `device_users_approve(...)`, `device_users_block(...)`, `device_users_cancel_wipe(...)`, `device_users_client_states_get(...)`, `device_users_client_states_list(...)`, `device_users_client_states_patch(...)`, `device_users_delete(...)`, `device_users_get(...)`, `device_users_list(...)`, `device_users_lookup(...)`, `device_users_wipe(...)`, `get(...)`, `list(...)` and `wipe(...)`
/// // to build up your call.
/// let rb = hub.devices();
/// # }
/// ```
pub struct DeviceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIdentity<S>,
}

impl<'a, S> client::MethodsBuilder for DeviceMethods<'a, S> {}

impl<'a, S> DeviceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the client state for the device user
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device}/deviceUsers/{device_user}/clientStates/{partner}`, where `device` is the unique ID assigned to the Device, `device_user` is the unique ID assigned to the User and `partner` identifies the partner storing the data. To get the client state for devices belonging to your own organization, the `partnerId` is in the format: `customerId-*anystring*`. Where the `customerId` is your organization's customer ID and `anystring` is any suffix. This suffix is used in setting up Custom Access Levels in Context-Aware Access. You may use `my_customer` instead of the customer ID for devices managed by your own organization. You may specify `-` in place of the `{device}`, so the ClientState resource name can be: `devices/-/deviceUsers/{device_user_resource}/clientStates/{partner}`.
    pub fn device_users_client_states_get(&self, name: &str) -> DeviceDeviceUserClientStateGetCall<'a, S> {
        DeviceDeviceUserClientStateGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the client states for the given search query.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. To list all ClientStates, set this to "devices/-/deviceUsers/-". To list all ClientStates owned by a DeviceUser, set this to the resource name of the DeviceUser. Format: devices/{device}/deviceUsers/{deviceUser}
    pub fn device_users_client_states_list(&self, parent: &str) -> DeviceDeviceUserClientStateListCall<'a, S> {
        DeviceDeviceUserClientStateListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the client state for the device user **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the ClientState in format: `devices/{device}/deviceUsers/{device_user}/clientState/{partner}`, where partner corresponds to the partner storing the data. For partners belonging to the "BeyondCorp Alliance", this is the partner ID specified to you by Google. For all other callers, this is a string of the form: `{customer}-suffix`, where `customer` is your customer ID. The *suffix* is any string the caller specifies. This string will be displayed verbatim in the administration console. This suffix is used in setting up Custom Access Levels in Context-Aware Access. Your organization's customer ID can be obtained from the URL: `GET https://www.googleapis.com/admin/directory/v1/customers/my_customer` The `id` field in the response contains the customer ID starting with the letter 'C'. The customer ID to be used in this API is the string after the letter 'C' (not including 'C')
    pub fn device_users_client_states_patch(&self, request: GoogleAppsCloudidentityDevicesV1ClientState, name: &str) -> DeviceDeviceUserClientStatePatchCall<'a, S> {
        DeviceDeviceUserClientStatePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Approves device to access user data.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
    pub fn device_users_approve(&self, request: GoogleAppsCloudidentityDevicesV1ApproveDeviceUserRequest, name: &str) -> DeviceDeviceUserApproveCall<'a, S> {
        DeviceDeviceUserApproveCall {
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
    /// Blocks device from accessing user data
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
    pub fn device_users_block(&self, request: GoogleAppsCloudidentityDevicesV1BlockDeviceUserRequest, name: &str) -> DeviceDeviceUserBlockCall<'a, S> {
        DeviceDeviceUserBlockCall {
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
    /// Cancels an unfinished user account wipe. This operation can be used to cancel device wipe in the gap between the wipe operation returning success and the device being wiped.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
    pub fn device_users_cancel_wipe(&self, request: GoogleAppsCloudidentityDevicesV1CancelWipeDeviceUserRequest, name: &str) -> DeviceDeviceUserCancelWipeCall<'a, S> {
        DeviceDeviceUserCancelWipeCall {
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
    /// Deletes the specified DeviceUser. This also revokes the user's access to device data.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
    pub fn device_users_delete(&self, name: &str) -> DeviceDeviceUserDeleteCall<'a, S> {
        DeviceDeviceUserDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified DeviceUser
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
    pub fn device_users_get(&self, name: &str) -> DeviceDeviceUserGetCall<'a, S> {
        DeviceDeviceUserGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists/Searches DeviceUsers.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. To list all DeviceUsers, set this to "devices/-". To list all DeviceUsers owned by a device, set this to the resource name of the device. Format: devices/{device}
    pub fn device_users_list(&self, parent: &str) -> DeviceDeviceUserListCall<'a, S> {
        DeviceDeviceUserListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up resource names of the DeviceUsers associated with the caller's credentials, as well as the properties provided in the request. This method must be called with end-user credentials with the scope: https://www.googleapis.com/auth/cloud-identity.devices.lookup If multiple properties are provided, only DeviceUsers having all of these properties are considered as matches - i.e. the query behaves like an AND. Different platforms require different amounts of information from the caller to ensure that the DeviceUser is uniquely identified. - iOS: No properties need to be passed, the caller's credentials are sufficient to identify the corresponding DeviceUser. - Android: Specifying the 'android_id' field is required. - Desktop: Specifying the 'raw_resource_id' field is required.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Must be set to "devices/-/deviceUsers" to search across all DeviceUser belonging to the user.
    pub fn device_users_lookup(&self, parent: &str) -> DeviceDeviceUserLookupCall<'a, S> {
        DeviceDeviceUserLookupCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _user_id: Default::default(),
            _raw_resource_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _android_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Wipes the user's account on a device. Other data on the device that is not associated with the user's work account is not affected. For example, if a Gmail app is installed on a device that is used for personal and work purposes, and the user is logged in to the Gmail app with their personal account as well as their work account, wiping the "deviceUser" by their work administrator will not affect their personal account within Gmail or other apps such as Photos.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
    pub fn device_users_wipe(&self, request: GoogleAppsCloudidentityDevicesV1WipeDeviceUserRequest, name: &str) -> DeviceDeviceUserWipeCall<'a, S> {
        DeviceDeviceUserWipeCall {
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
    /// Cancels an unfinished device wipe. This operation can be used to cancel device wipe in the gap between the wipe operation returning success and the device being wiped. This operation is possible when the device is in a "pending wipe" state. The device enters the "pending wipe" state when a wipe device command is issued, but has not yet been sent to the device. The cancel wipe will fail if the wipe command has already been issued to the device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}`, where device is the unique ID assigned to the Device.
    pub fn cancel_wipe(&self, request: GoogleAppsCloudidentityDevicesV1CancelWipeDeviceRequest, name: &str) -> DeviceCancelWipeCall<'a, S> {
        DeviceCancelWipeCall {
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
    /// Creates a device. Only company-owned device may be created. **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: GoogleAppsCloudidentityDevicesV1Device) -> DeviceCreateCall<'a, S> {
        DeviceCreateCall {
            hub: self.hub,
            _request: request,
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}`, where device is the unique ID assigned to the Device.
    pub fn delete(&self, name: &str) -> DeviceDeleteCall<'a, S> {
        DeviceDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in the format: `devices/{device}`, where device is the unique ID assigned to the Device.
    pub fn get(&self, name: &str) -> DeviceGetCall<'a, S> {
        DeviceGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists/Searches devices.
    pub fn list(&self) -> DeviceListCall<'a, S> {
        DeviceListCall {
            hub: self.hub,
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _customer: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Wipes all data on the specified device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Device in format: `devices/{device}/deviceUsers/{device_user}`, where device is the unique ID assigned to the Device, and device_user is the unique ID assigned to the User.
    pub fn wipe(&self, request: GoogleAppsCloudidentityDevicesV1WipeDeviceRequest, name: &str) -> DeviceWipeCall<'a, S> {
        DeviceWipeCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *group* resources.
/// It is not used directly, but through the [`CloudIdentity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudidentity1 as cloudidentity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudidentity1::{CloudIdentity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIdentity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `get_security_settings(...)`, `list(...)`, `lookup(...)`, `memberships_check_transitive_membership(...)`, `memberships_create(...)`, `memberships_delete(...)`, `memberships_get(...)`, `memberships_get_membership_graph(...)`, `memberships_list(...)`, `memberships_lookup(...)`, `memberships_modify_membership_roles(...)`, `memberships_search_transitive_groups(...)`, `memberships_search_transitive_memberships(...)`, `patch(...)`, `search(...)` and `update_security_settings(...)`
/// // to build up your call.
/// let rb = hub.groups();
/// # }
/// ```
pub struct GroupMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIdentity<S>,
}

impl<'a, S> client::MethodsBuilder for GroupMethods<'a, S> {}

impl<'a, S> GroupMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Check a potential member for membership in a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the member is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. A member has membership to a group as long as there is a single viewable transitive membership between the group and the member. The actor must have view permissions to at least one transitive membership between the member and group.
    /// 
    /// # Arguments
    ///
    /// * `parent` - [Resource name](https://cloud.google.com/apis/design/resource_names) of the group to check the transitive membership in. Format: `groups/{group}`, where `group` is the unique id assigned to the Group to which the Membership belongs to.
    pub fn memberships_check_transitive_membership(&self, parent: &str) -> GroupMembershipCheckTransitiveMembershipCall<'a, S> {
        GroupMembershipCheckTransitiveMembershipCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a `Membership`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent `Group` resource under which to create the `Membership`. Must be of the form `groups/{group}`.
    pub fn memberships_create(&self, request: Membership, parent: &str) -> GroupMembershipCreateCall<'a, S> {
        GroupMembershipCreateCall {
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
    /// Deletes a `Membership`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership` to delete. Must be of the form `groups/{group}/memberships/{membership}`
    pub fn memberships_delete(&self, name: &str) -> GroupMembershipDeleteCall<'a, S> {
        GroupMembershipDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a `Membership`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership` to retrieve. Must be of the form `groups/{group}/memberships/{membership}`.
    pub fn memberships_get(&self, name: &str) -> GroupMembershipGetCall<'a, S> {
        GroupMembershipGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a membership graph of just a member or both a member and a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the member is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. Given a member, the response will contain all membership paths from the member. Given both a group and a member, the response will contain all membership paths between the group and the member.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. [Resource name](https://cloud.google.com/apis/design/resource_names) of the group to search transitive memberships in. Format: `groups/{group}`, where `group` is the unique ID assigned to the Group to which the Membership belongs to. group can be a wildcard collection id "-". When a group is specified, the membership graph will be constrained to paths between the member (defined in the query) and the parent. If a wildcard collection is provided, all membership paths connected to the member will be returned.
    pub fn memberships_get_membership_graph(&self, parent: &str) -> GroupMembershipGetMembershipGraphCall<'a, S> {
        GroupMembershipGetMembershipGraphCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _query: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the `Membership`s within a `Group`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent `Group` resource under which to lookup the `Membership` name. Must be of the form `groups/{group}`.
    pub fn memberships_list(&self, parent: &str) -> GroupMembershipListCall<'a, S> {
        GroupMembershipListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _view: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up the [resource name](https://cloud.google.com/apis/design/resource_names) of a `Membership` by its `EntityKey`.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent `Group` resource under which to lookup the `Membership` name. Must be of the form `groups/{group}`.
    pub fn memberships_lookup(&self, parent: &str) -> GroupMembershipLookupCall<'a, S> {
        GroupMembershipLookupCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _member_key_namespace: Default::default(),
            _member_key_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the `MembershipRole`s of a `Membership`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Membership` whose roles are to be modified. Must be of the form `groups/{group}/memberships/{membership}`.
    pub fn memberships_modify_membership_roles(&self, request: ModifyMembershipRolesRequest, name: &str) -> GroupMembershipModifyMembershipRoleCall<'a, S> {
        GroupMembershipModifyMembershipRoleCall {
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
    /// Search transitive groups of a member. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the member is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. A transitive group is any group that has a direct or indirect membership to the member. Actor must have view permissions all transitive groups.
    /// 
    /// # Arguments
    ///
    /// * `parent` - [Resource name](https://cloud.google.com/apis/design/resource_names) of the group to search transitive memberships in. Format: `groups/{group}`, where `group` is always '-' as this API will search across all groups for a given member.
    pub fn memberships_search_transitive_groups(&self, parent: &str) -> GroupMembershipSearchTransitiveGroupCall<'a, S> {
        GroupMembershipSearchTransitiveGroupCall {
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
    /// Search transitive memberships of a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. If the account of the group is not one of these, a 403 (PERMISSION_DENIED) HTTP status code will be returned. A transitive membership is any direct or indirect membership of a group. Actor must have view permissions to all transitive memberships.
    /// 
    /// # Arguments
    ///
    /// * `parent` - [Resource name](https://cloud.google.com/apis/design/resource_names) of the group to search transitive memberships in. Format: `groups/{group}`, where `group` is the unique ID assigned to the Group.
    pub fn memberships_search_transitive_memberships(&self, parent: &str) -> GroupMembershipSearchTransitiveMembershipCall<'a, S> {
        GroupMembershipSearchTransitiveMembershipCall {
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
    /// Creates a Group.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Group) -> GroupCreateCall<'a, S> {
        GroupCreateCall {
            hub: self.hub,
            _request: request,
            _initial_group_config: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a `Group`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group` to retrieve. Must be of the form `groups/{group}`.
    pub fn delete(&self, name: &str) -> GroupDeleteCall<'a, S> {
        GroupDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a `Group`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group` to retrieve. Must be of the form `groups/{group}`.
    pub fn get(&self, name: &str) -> GroupGetCall<'a, S> {
        GroupGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get Security Settings
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The security settings to retrieve. Format: `groups/{group_id}/securitySettings`
    pub fn get_security_settings(&self, name: &str) -> GroupGetSecuritySettingCall<'a, S> {
        GroupGetSecuritySettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the `Group` resources under a customer or namespace.
    pub fn list(&self) -> GroupListCall<'a, S> {
        GroupListCall {
            hub: self.hub,
            _view: Default::default(),
            _parent: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up the [resource name](https://cloud.google.com/apis/design/resource_names) of a `Group` by its `EntityKey`.
    pub fn lookup(&self) -> GroupLookupCall<'a, S> {
        GroupLookupCall {
            hub: self.hub,
            _group_key_namespace: Default::default(),
            _group_key_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a `Group`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The [resource name](https://cloud.google.com/apis/design/resource_names) of the `Group`. Shall be of the form `groups/{group}`.
    pub fn patch(&self, request: Group, name: &str) -> GroupPatchCall<'a, S> {
        GroupPatchCall {
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
    /// Searches for `Group` resources matching a specified query.
    pub fn search(&self) -> GroupSearchCall<'a, S> {
        GroupSearchCall {
            hub: self.hub,
            _view: Default::default(),
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
    /// Update Security Settings
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. The resource name of the security settings. Shall be of the form `groups/{group_id}/securitySettings`.
    pub fn update_security_settings(&self, request: SecuritySettings, name: &str) -> GroupUpdateSecuritySettingCall<'a, S> {
        GroupUpdateSecuritySettingCall {
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



/// A builder providing access to all methods supported on *inboundSamlSsoProfile* resources.
/// It is not used directly, but through the [`CloudIdentity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudidentity1 as cloudidentity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudidentity1::{CloudIdentity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIdentity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `idp_credentials_add(...)`, `idp_credentials_delete(...)`, `idp_credentials_get(...)`, `idp_credentials_list(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.inbound_saml_sso_profiles();
/// # }
/// ```
pub struct InboundSamlSsoProfileMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIdentity<S>,
}

impl<'a, S> client::MethodsBuilder for InboundSamlSsoProfileMethods<'a, S> {}

impl<'a, S> InboundSamlSsoProfileMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds an IdpCredential. Up to 2 credentials are allowed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The InboundSamlSsoProfile that owns the IdpCredential. Format: `inboundSamlSsoProfiles/{sso_profile_id}`
    pub fn idp_credentials_add(&self, request: AddIdpCredentialRequest, parent: &str) -> InboundSamlSsoProfileIdpCredentialAddCall<'a, S> {
        InboundSamlSsoProfileIdpCredentialAddCall {
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
    /// Deletes an IdpCredential.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the IdpCredential to delete. Format: `inboundSamlSsoProfiles/{sso_profile_id}/idpCredentials/{idp_credential_id}`
    pub fn idp_credentials_delete(&self, name: &str) -> InboundSamlSsoProfileIdpCredentialDeleteCall<'a, S> {
        InboundSamlSsoProfileIdpCredentialDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an IdpCredential.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the IdpCredential to retrieve. Format: `inboundSamlSsoProfiles/{sso_profile_id}/idpCredentials/{idp_credential_id}`
    pub fn idp_credentials_get(&self, name: &str) -> InboundSamlSsoProfileIdpCredentialGetCall<'a, S> {
        InboundSamlSsoProfileIdpCredentialGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of IdpCredentials in an InboundSamlSsoProfile.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, which owns this collection of `IdpCredential`s. Format: `inboundSamlSsoProfiles/{sso_profile_id}`
    pub fn idp_credentials_list(&self, parent: &str) -> InboundSamlSsoProfileIdpCredentialListCall<'a, S> {
        InboundSamlSsoProfileIdpCredentialListCall {
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
    /// Creates an InboundSamlSsoProfile for a customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: InboundSamlSsoProfile) -> InboundSamlSsoProfileCreateCall<'a, S> {
        InboundSamlSsoProfileCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an InboundSamlSsoProfile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the InboundSamlSsoProfile to delete. Format: `inboundSamlSsoProfiles/{sso_profile_id}`
    pub fn delete(&self, name: &str) -> InboundSamlSsoProfileDeleteCall<'a, S> {
        InboundSamlSsoProfileDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an InboundSamlSsoProfile.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the InboundSamlSsoProfile to get. Format: `inboundSamlSsoProfiles/{sso_profile_id}`
    pub fn get(&self, name: &str) -> InboundSamlSsoProfileGetCall<'a, S> {
        InboundSamlSsoProfileGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists InboundSamlSsoProfiles for a customer.
    pub fn list(&self) -> InboundSamlSsoProfileListCall<'a, S> {
        InboundSamlSsoProfileListCall {
            hub: self.hub,
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
    /// Updates an InboundSamlSsoProfile.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the SAML SSO profile.
    pub fn patch(&self, request: InboundSamlSsoProfile, name: &str) -> InboundSamlSsoProfilePatchCall<'a, S> {
        InboundSamlSsoProfilePatchCall {
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



/// A builder providing access to all methods supported on *inboundSsoAssignment* resources.
/// It is not used directly, but through the [`CloudIdentity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_cloudidentity1 as cloudidentity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use cloudidentity1::{CloudIdentity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudIdentity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.inbound_sso_assignments();
/// # }
/// ```
pub struct InboundSsoAssignmentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudIdentity<S>,
}

impl<'a, S> client::MethodsBuilder for InboundSsoAssignmentMethods<'a, S> {}

impl<'a, S> InboundSsoAssignmentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an InboundSsoAssignment for users and devices in a `Customer` under a given `Group` or `OrgUnit`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: InboundSsoAssignment) -> InboundSsoAssignmentCreateCall<'a, S> {
        InboundSsoAssignmentCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an InboundSsoAssignment. To disable SSO, Create (or Update) an assignment that has `sso_mode` == `SSO_OFF`.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the InboundSsoAssignment to delete. Format: `inboundSsoAssignments/{assignment}`
    pub fn delete(&self, name: &str) -> InboundSsoAssignmentDeleteCall<'a, S> {
        InboundSsoAssignmentDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an InboundSsoAssignment.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The [resource name](https://cloud.google.com/apis/design/resource_names) of the InboundSsoAssignment to fetch. Format: `inboundSsoAssignments/{assignment}`
    pub fn get(&self, name: &str) -> InboundSsoAssignmentGetCall<'a, S> {
        InboundSsoAssignmentGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the InboundSsoAssignments for a `Customer`.
    pub fn list(&self) -> InboundSsoAssignmentListCall<'a, S> {
        InboundSsoAssignmentListCall {
            hub: self.hub,
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
    /// Updates an InboundSsoAssignment. The body of this request is the `inbound_sso_assignment` field and the `update_mask` is relative to that. For example: a PATCH to `/v1/inboundSsoAssignments/0abcdefg1234567&update_mask=rank` with a body of `{ "rank": 1 }` moves that (presumably group-targeted) SSO assignment to the highest priority and shifts any other group-targeted assignments down in priority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Output only. [Resource name](https://cloud.google.com/apis/design/resource_names) of the Inbound SSO Assignment.
    pub fn patch(&self, request: InboundSsoAssignment, name: &str) -> InboundSsoAssignmentPatchCall<'a, S> {
        InboundSsoAssignmentPatchCall {
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



