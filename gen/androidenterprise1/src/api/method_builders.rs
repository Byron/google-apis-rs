use super::*;
/// A builder providing access to all methods supported on *device* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `force_report_upload(...)`, `get(...)`, `get_state(...)`, `list(...)`, `set_state(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.devices();
/// # }
/// ```
pub struct DeviceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for DeviceMethods<'a, S> {}

impl<'a, S> DeviceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a report containing any changes in app states on the device since the last report was generated. You can call this method up to 3 times every 24 hours for a given device. If you exceed the quota, then the Google Play EMM API returns HTTP 429 Too Many Requests.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The ID of the device.
    pub fn force_report_upload(&self, enterprise_id: &str, user_id: &str, device_id: &str) -> DeviceForceReportUploadCall<'a, S> {
        DeviceForceReportUploadCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the details of a device.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The ID of the device.
    pub fn get(&self, enterprise_id: &str, user_id: &str, device_id: &str) -> DeviceGetCall<'a, S> {
        DeviceGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves whether a device's access to Google services is enabled or disabled. The device state takes effect only if enforcing EMM policies on Android devices is enabled in the Google Admin Console. Otherwise, the device state is ignored and all devices are allowed access to Google services. This is only supported for Google-managed users.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The ID of the device.
    pub fn get_state(&self, enterprise_id: &str, user_id: &str, device_id: &str) -> DeviceGetStateCall<'a, S> {
        DeviceGetStateCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the IDs of all of a user's devices.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn list(&self, enterprise_id: &str, user_id: &str) -> DeviceListCall<'a, S> {
        DeviceListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets whether a device's access to Google services is enabled or disabled. The device state takes effect only if enforcing EMM policies on Android devices is enabled in the Google Admin Console. Otherwise, the device state is ignored and all devices are allowed access to Google services. This is only supported for Google-managed users.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The ID of the device.
    pub fn set_state(&self, request: DeviceState, enterprise_id: &str, user_id: &str, device_id: &str) -> DeviceSetStateCall<'a, S> {
        DeviceSetStateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the device policy. To ensure the policy is properly enforced, you need to prevent unmanaged accounts from accessing Google Play by setting the allowed_accounts in the managed configuration for the Google Play package. See restrict accounts in Google Play. When provisioning a new device, you should set the device policy using this method before adding the managed Google Play Account to the device, otherwise the policy will not be applied for a short period of time after adding the account to the device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The ID of the device.
    pub fn update(&self, request: Device, enterprise_id: &str, user_id: &str, device_id: &str) -> DeviceUpdateCall<'a, S> {
        DeviceUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *enterprise* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `acknowledge_notification_set(...)`, `complete_signup(...)`, `create_enrollment_token(...)`, `create_web_token(...)`, `enroll(...)`, `generate_signup_url(...)`, `get(...)`, `get_service_account(...)`, `get_store_layout(...)`, `list(...)`, `pull_notification_set(...)`, `send_test_push_notification(...)`, `set_account(...)`, `set_store_layout(...)` and `unenroll(...)`
/// // to build up your call.
/// let rb = hub.enterprises();
/// # }
/// ```
pub struct EnterpriseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for EnterpriseMethods<'a, S> {}

impl<'a, S> EnterpriseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Acknowledges notifications that were received from Enterprises.PullNotificationSet to prevent subsequent calls from returning the same notifications.
    pub fn acknowledge_notification_set(&self) -> EnterpriseAcknowledgeNotificationSetCall<'a, S> {
        EnterpriseAcknowledgeNotificationSetCall {
            hub: self.hub,
            _notification_set_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes the signup flow, by specifying the Completion token and Enterprise token. This request must not be called multiple times for a given Enterprise Token.
    pub fn complete_signup(&self) -> EnterpriseCompleteSignupCall<'a, S> {
        EnterpriseCompleteSignupCall {
            hub: self.hub,
            _enterprise_token: Default::default(),
            _completion_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a token for device enrollment. The DPC can encode this token within the QR/NFC/zero-touch enrollment payload or fetch it before calling the on-device API to authenticate the user. The token can be generated for each device or reused across multiple devices.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn create_enrollment_token(&self, enterprise_id: &str) -> EnterpriseCreateEnrollmentTokenCall<'a, S> {
        EnterpriseCreateEnrollmentTokenCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _device_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a unique token to access an embeddable UI. To generate a web UI, pass the generated token into the managed Google Play javascript API. Each token may only be used to start one UI session. See the javascript API documentation for further information.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn create_web_token(&self, request: AdministratorWebTokenSpec, enterprise_id: &str) -> EnterpriseCreateWebTokenCall<'a, S> {
        EnterpriseCreateWebTokenCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Enrolls an enterprise with the calling EMM.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `token` - Required. The token provided by the enterprise to register the EMM.
    pub fn enroll(&self, request: Enterprise, token: &str) -> EnterpriseEnrollCall<'a, S> {
        EnterpriseEnrollCall {
            hub: self.hub,
            _request: request,
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a sign-up URL.
    pub fn generate_signup_url(&self) -> EnterpriseGenerateSignupUrlCall<'a, S> {
        EnterpriseGenerateSignupUrlCall {
            hub: self.hub,
            _callback_url: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the name and domain of an enterprise.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn get(&self, enterprise_id: &str) -> EnterpriseGetCall<'a, S> {
        EnterpriseGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a service account and credentials. The service account can be bound to the enterprise by calling setAccount. The service account is unique to this enterprise and EMM, and will be deleted if the enterprise is unbound. The credentials contain private key data and are not stored server-side. This method can only be called after calling Enterprises.Enroll or Enterprises.CompleteSignup, and before Enterprises.SetAccount; at other times it will return an error. Subsequent calls after the first will generate a new, unique set of credentials, and invalidate the previously generated credentials. Once the service account is bound to the enterprise, it can be managed using the serviceAccountKeys resource.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn get_service_account(&self, enterprise_id: &str) -> EnterpriseGetServiceAccountCall<'a, S> {
        EnterpriseGetServiceAccountCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _key_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the store layout for the enterprise. If the store layout has not been set, returns "basic" as the store layout type and no homepage.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn get_store_layout(&self, enterprise_id: &str) -> EnterpriseGetStoreLayoutCall<'a, S> {
        EnterpriseGetStoreLayoutCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up an enterprise by domain name. This is only supported for enterprises created via the Google-initiated creation flow. Lookup of the id is not needed for enterprises created via the EMM-initiated flow since the EMM learns the enterprise ID in the callback specified in the Enterprises.generateSignupUrl call.
    /// 
    /// # Arguments
    ///
    /// * `domain` - Required. The exact primary domain name of the enterprise to look up.
    pub fn list(&self, domain: &str) -> EnterpriseListCall<'a, S> {
        EnterpriseListCall {
            hub: self.hub,
            _domain: domain.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Pulls and returns a notification set for the enterprises associated with the service account authenticated for the request. The notification set may be empty if no notification are pending. A notification set returned needs to be acknowledged within 20 seconds by calling Enterprises.AcknowledgeNotificationSet, unless the notification set is empty. Notifications that are not acknowledged within the 20 seconds will eventually be included again in the response to another PullNotificationSet request, and those that are never acknowledged will ultimately be deleted according to the Google Cloud Platform Pub/Sub system policy. Multiple requests might be performed concurrently to retrieve notifications, in which case the pending notifications (if any) will be split among each caller, if any are pending. If no notifications are present, an empty notification list is returned. Subsequent requests may return more notifications once they become available.
    pub fn pull_notification_set(&self) -> EnterprisePullNotificationSetCall<'a, S> {
        EnterprisePullNotificationSetCall {
            hub: self.hub,
            _request_mode: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sends a test notification to validate the EMM integration with the Google Cloud Pub/Sub service for this enterprise.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn send_test_push_notification(&self, enterprise_id: &str) -> EnterpriseSendTestPushNotificationCall<'a, S> {
        EnterpriseSendTestPushNotificationCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the account that will be used to authenticate to the API as the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn set_account(&self, request: EnterpriseAccount, enterprise_id: &str) -> EnterpriseSetAccountCall<'a, S> {
        EnterpriseSetAccountCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the store layout for the enterprise. By default, storeLayoutType is set to "basic" and the basic store layout is enabled. The basic layout only contains apps approved by the admin, and that have been added to the available product set for a user (using the setAvailableProductSet call). Apps on the page are sorted in order of their product ID value. If you create a custom store layout (by setting storeLayoutType = "custom" and setting a homepage), the basic store layout is disabled.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn set_store_layout(&self, request: StoreLayout, enterprise_id: &str) -> EnterpriseSetStoreLayoutCall<'a, S> {
        EnterpriseSetStoreLayoutCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unenrolls an enterprise from the calling EMM.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn unenroll(&self, enterprise_id: &str) -> EnterpriseUnenrollCall<'a, S> {
        EnterpriseUnenrollCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *entitlement* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.entitlements();
/// # }
/// ```
pub struct EntitlementMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for EntitlementMethods<'a, S> {}

impl<'a, S> EntitlementMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes an entitlement to an app for a user. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `entitlementId` - The ID of the entitlement (a product ID), e.g. "app:com.google.android.gm".
    pub fn delete(&self, enterprise_id: &str, user_id: &str, entitlement_id: &str) -> EntitlementDeleteCall<'a, S> {
        EntitlementDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _entitlement_id: entitlement_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of an entitlement. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `entitlementId` - The ID of the entitlement (a product ID), e.g. "app:com.google.android.gm".
    pub fn get(&self, enterprise_id: &str, user_id: &str, entitlement_id: &str) -> EntitlementGetCall<'a, S> {
        EntitlementGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _entitlement_id: entitlement_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all entitlements for the specified user. Only the ID is set. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn list(&self, enterprise_id: &str, user_id: &str) -> EntitlementListCall<'a, S> {
        EntitlementListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds or updates an entitlement to an app for a user. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `entitlementId` - The ID of the entitlement (a product ID), e.g. "app:com.google.android.gm".
    pub fn update(&self, request: Entitlement, enterprise_id: &str, user_id: &str, entitlement_id: &str) -> EntitlementUpdateCall<'a, S> {
        EntitlementUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _entitlement_id: entitlement_id.to_string(),
            _install: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *grouplicense* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.grouplicenses();
/// # }
/// ```
pub struct GrouplicenseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for GrouplicenseMethods<'a, S> {}

impl<'a, S> GrouplicenseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of an enterprise's group license for a product. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `groupLicenseId` - The ID of the product the group license is for, e.g. "app:com.google.android.gm".
    pub fn get(&self, enterprise_id: &str, group_license_id: &str) -> GrouplicenseGetCall<'a, S> {
        GrouplicenseGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _group_license_id: group_license_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves IDs of all products for which the enterprise has a group license. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn list(&self, enterprise_id: &str) -> GrouplicenseListCall<'a, S> {
        GrouplicenseListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *grouplicenseuser* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.grouplicenseusers();
/// # }
/// ```
pub struct GrouplicenseuserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for GrouplicenseuserMethods<'a, S> {}

impl<'a, S> GrouplicenseuserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the IDs of the users who have been granted entitlements under the license. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `groupLicenseId` - The ID of the product the group license is for, e.g. "app:com.google.android.gm".
    pub fn list(&self, enterprise_id: &str, group_license_id: &str) -> GrouplicenseuserListCall<'a, S> {
        GrouplicenseuserListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _group_license_id: group_license_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *install* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.installs();
/// # }
/// ```
pub struct InstallMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for InstallMethods<'a, S> {}

impl<'a, S> InstallMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests to remove an app from a device. A call to get or list will still show the app as installed on the device until it is actually removed.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    /// * `installId` - The ID of the product represented by the install, e.g. "app:com.google.android.gm".
    pub fn delete(&self, enterprise_id: &str, user_id: &str, device_id: &str, install_id: &str) -> InstallDeleteCall<'a, S> {
        InstallDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _install_id: install_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of an installation of an app on a device.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    /// * `installId` - The ID of the product represented by the install, e.g. "app:com.google.android.gm".
    pub fn get(&self, enterprise_id: &str, user_id: &str, device_id: &str, install_id: &str) -> InstallGetCall<'a, S> {
        InstallGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _install_id: install_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the details of all apps installed on the specified device.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    pub fn list(&self, enterprise_id: &str, user_id: &str, device_id: &str) -> InstallListCall<'a, S> {
        InstallListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests to install the latest version of an app to a device. If the app is already installed, then it is updated to the latest version if necessary.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    /// * `installId` - The ID of the product represented by the install, e.g. "app:com.google.android.gm".
    pub fn update(&self, request: Install, enterprise_id: &str, user_id: &str, device_id: &str, install_id: &str) -> InstallUpdateCall<'a, S> {
        InstallUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _install_id: install_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *managedconfigurationsfordevice* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.managedconfigurationsfordevice();
/// # }
/// ```
pub struct ManagedconfigurationsfordeviceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for ManagedconfigurationsfordeviceMethods<'a, S> {}

impl<'a, S> ManagedconfigurationsfordeviceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a per-device managed configuration for an app for the specified device.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    /// * `managedConfigurationForDeviceId` - The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm".
    pub fn delete(&self, enterprise_id: &str, user_id: &str, device_id: &str, managed_configuration_for_device_id: &str) -> ManagedconfigurationsfordeviceDeleteCall<'a, S> {
        ManagedconfigurationsfordeviceDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _managed_configuration_for_device_id: managed_configuration_for_device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of a per-device managed configuration.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    /// * `managedConfigurationForDeviceId` - The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm".
    pub fn get(&self, enterprise_id: &str, user_id: &str, device_id: &str, managed_configuration_for_device_id: &str) -> ManagedconfigurationsfordeviceGetCall<'a, S> {
        ManagedconfigurationsfordeviceGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _managed_configuration_for_device_id: managed_configuration_for_device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the per-device managed configurations for the specified device. Only the ID is set.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    pub fn list(&self, enterprise_id: &str, user_id: &str, device_id: &str) -> ManagedconfigurationsfordeviceListCall<'a, S> {
        ManagedconfigurationsfordeviceListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds or updates a per-device managed configuration for an app for the specified device.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `deviceId` - The Android ID of the device.
    /// * `managedConfigurationForDeviceId` - The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm".
    pub fn update(&self, request: ManagedConfiguration, enterprise_id: &str, user_id: &str, device_id: &str, managed_configuration_for_device_id: &str) -> ManagedconfigurationsfordeviceUpdateCall<'a, S> {
        ManagedconfigurationsfordeviceUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _device_id: device_id.to_string(),
            _managed_configuration_for_device_id: managed_configuration_for_device_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *managedconfigurationsforuser* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.managedconfigurationsforuser();
/// # }
/// ```
pub struct ManagedconfigurationsforuserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for ManagedconfigurationsforuserMethods<'a, S> {}

impl<'a, S> ManagedconfigurationsforuserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes a per-user managed configuration for an app for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `managedConfigurationForUserId` - The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm".
    pub fn delete(&self, enterprise_id: &str, user_id: &str, managed_configuration_for_user_id: &str) -> ManagedconfigurationsforuserDeleteCall<'a, S> {
        ManagedconfigurationsforuserDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _managed_configuration_for_user_id: managed_configuration_for_user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of a per-user managed configuration for an app for the specified user.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `managedConfigurationForUserId` - The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm".
    pub fn get(&self, enterprise_id: &str, user_id: &str, managed_configuration_for_user_id: &str) -> ManagedconfigurationsforuserGetCall<'a, S> {
        ManagedconfigurationsforuserGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _managed_configuration_for_user_id: managed_configuration_for_user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the per-user managed configurations for the specified user. Only the ID is set.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn list(&self, enterprise_id: &str, user_id: &str) -> ManagedconfigurationsforuserListCall<'a, S> {
        ManagedconfigurationsforuserListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds or updates the managed configuration settings for an app for the specified user. If you support the Managed configurations iframe, you can apply managed configurations to a user by specifying an mcmId and its associated configuration variables (if any) in the request. Alternatively, all EMMs can apply managed configurations by passing a list of managed properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    /// * `managedConfigurationForUserId` - The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm".
    pub fn update(&self, request: ManagedConfiguration, enterprise_id: &str, user_id: &str, managed_configuration_for_user_id: &str) -> ManagedconfigurationsforuserUpdateCall<'a, S> {
        ManagedconfigurationsforuserUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _managed_configuration_for_user_id: managed_configuration_for_user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *managedconfigurationssetting* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.managedconfigurationssettings();
/// # }
/// ```
pub struct ManagedconfigurationssettingMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for ManagedconfigurationssettingMethods<'a, S> {}

impl<'a, S> ManagedconfigurationssettingMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the managed configurations settings for the specified app.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `productId` - The ID of the product for which the managed configurations settings applies to.
    pub fn list(&self, enterprise_id: &str, product_id: &str) -> ManagedconfigurationssettingListCall<'a, S> {
        ManagedconfigurationssettingListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *permission* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.permissions();
/// # }
/// ```
pub struct PermissionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for PermissionMethods<'a, S> {}

impl<'a, S> PermissionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of an Android app permission for display to an enterprise admin.
    /// 
    /// # Arguments
    ///
    /// * `permissionId` - The ID of the permission.
    pub fn get(&self, permission_id: &str) -> PermissionGetCall<'a, S> {
        PermissionGetCall {
            hub: self.hub,
            _permission_id: permission_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *product* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `approve(...)`, `generate_approval_url(...)`, `get(...)`, `get_app_restrictions_schema(...)`, `get_permissions(...)`, `list(...)` and `unapprove(...)`
/// // to build up your call.
/// let rb = hub.products();
/// # }
/// ```
pub struct ProductMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for ProductMethods<'a, S> {}

impl<'a, S> ProductMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    ///  Approves the specified product and the relevant app permissions, if any. The maximum number of products that you can approve per enterprise customer is 1,000. To learn how to use managed Google Play to design and create a store layout to display approved products to your users, see Store Layout Design. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations. 
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `productId` - The ID of the product.
    pub fn approve(&self, request: ProductsApproveRequest, enterprise_id: &str, product_id: &str) -> ProductApproveCall<'a, S> {
        ProductApproveCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a URL that can be rendered in an iframe to display the permissions (if any) of a product. An enterprise admin must view these permissions and accept them on behalf of their organization in order to approve that product. Admins should accept the displayed permissions by interacting with a separate UI element in the EMM console, which in turn should trigger the use of this URL as the approvalUrlInfo.approvalUrl property in a Products.approve call to approve the product. This URL can only be used to display permissions for up to 1 day. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations. 
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `productId` - The ID of the product.
    pub fn generate_approval_url(&self, enterprise_id: &str, product_id: &str) -> ProductGenerateApprovalUrlCall<'a, S> {
        ProductGenerateApprovalUrlCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _product_id: product_id.to_string(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of a product for display to an enterprise admin.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `productId` - The ID of the product, e.g. "app:com.google.android.gm".
    pub fn get(&self, enterprise_id: &str, product_id: &str) -> ProductGetCall<'a, S> {
        ProductGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _product_id: product_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the schema that defines the configurable properties for this product. All products have a schema, but this schema may be empty if no managed configurations have been defined. This schema can be used to populate a UI that allows an admin to configure the product. To apply a managed configuration based on the schema obtained using this API, see Managed Configurations through Play.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `productId` - The ID of the product.
    pub fn get_app_restrictions_schema(&self, enterprise_id: &str, product_id: &str) -> ProductGetAppRestrictionsSchemaCall<'a, S> {
        ProductGetAppRestrictionsSchemaCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _product_id: product_id.to_string(),
            _language: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the Android app permissions required by this app.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `productId` - The ID of the product.
    pub fn get_permissions(&self, enterprise_id: &str, product_id: &str) -> ProductGetPermissionCall<'a, S> {
        ProductGetPermissionCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds approved products that match a query, or all approved products if there is no query. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations. 
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn list(&self, enterprise_id: &str) -> ProductListCall<'a, S> {
        ProductListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _token: Default::default(),
            _query: Default::default(),
            _max_results: Default::default(),
            _language: Default::default(),
            _approved: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Unapproves the specified product (and the relevant app permissions, if any) **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `productId` - The ID of the product.
    pub fn unapprove(&self, enterprise_id: &str, product_id: &str) -> ProductUnapproveCall<'a, S> {
        ProductUnapproveCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _product_id: product_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *serviceaccountkey* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.serviceaccountkeys();
/// # }
/// ```
pub struct ServiceaccountkeyMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for ServiceaccountkeyMethods<'a, S> {}

impl<'a, S> ServiceaccountkeyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes and invalidates the specified credentials for the service account associated with this enterprise. The calling service account must have been retrieved by calling Enterprises.GetServiceAccount and must have been set as the enterprise service account by calling Enterprises.SetAccount.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `keyId` - The ID of the key.
    pub fn delete(&self, enterprise_id: &str, key_id: &str) -> ServiceaccountkeyDeleteCall<'a, S> {
        ServiceaccountkeyDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _key_id: key_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates new credentials for the service account associated with this enterprise. The calling service account must have been retrieved by calling Enterprises.GetServiceAccount and must have been set as the enterprise service account by calling Enterprises.SetAccount. Only the type of the key should be populated in the resource to be inserted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn insert(&self, request: ServiceAccountKey, enterprise_id: &str) -> ServiceaccountkeyInsertCall<'a, S> {
        ServiceaccountkeyInsertCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all active credentials for the service account associated with this enterprise. Only the ID and key type are returned. The calling service account must have been retrieved by calling Enterprises.GetServiceAccount and must have been set as the enterprise service account by calling Enterprises.SetAccount.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn list(&self, enterprise_id: &str) -> ServiceaccountkeyListCall<'a, S> {
        ServiceaccountkeyListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *storelayoutcluster* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.storelayoutclusters();
/// # }
/// ```
pub struct StorelayoutclusterMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for StorelayoutclusterMethods<'a, S> {}

impl<'a, S> StorelayoutclusterMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a cluster.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    /// * `clusterId` - The ID of the cluster.
    pub fn delete(&self, enterprise_id: &str, page_id: &str, cluster_id: &str) -> StorelayoutclusterDeleteCall<'a, S> {
        StorelayoutclusterDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of a cluster.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    /// * `clusterId` - The ID of the cluster.
    pub fn get(&self, enterprise_id: &str, page_id: &str, cluster_id: &str) -> StorelayoutclusterGetCall<'a, S> {
        StorelayoutclusterGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new cluster in a page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    pub fn insert(&self, request: StoreCluster, enterprise_id: &str, page_id: &str) -> StorelayoutclusterInsertCall<'a, S> {
        StorelayoutclusterInsertCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the details of all clusters on the specified page.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    pub fn list(&self, enterprise_id: &str, page_id: &str) -> StorelayoutclusterListCall<'a, S> {
        StorelayoutclusterListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a cluster.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    /// * `clusterId` - The ID of the cluster.
    pub fn update(&self, request: StoreCluster, enterprise_id: &str, page_id: &str, cluster_id: &str) -> StorelayoutclusterUpdateCall<'a, S> {
        StorelayoutclusterUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _cluster_id: cluster_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *storelayoutpage* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.storelayoutpages();
/// # }
/// ```
pub struct StorelayoutpageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for StorelayoutpageMethods<'a, S> {}

impl<'a, S> StorelayoutpageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a store page.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    pub fn delete(&self, enterprise_id: &str, page_id: &str) -> StorelayoutpageDeleteCall<'a, S> {
        StorelayoutpageDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves details of a store page.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    pub fn get(&self, enterprise_id: &str, page_id: &str) -> StorelayoutpageGetCall<'a, S> {
        StorelayoutpageGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new store page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn insert(&self, request: StorePage, enterprise_id: &str) -> StorelayoutpageInsertCall<'a, S> {
        StorelayoutpageInsertCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the details of all pages in the store.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn list(&self, enterprise_id: &str) -> StorelayoutpageListCall<'a, S> {
        StorelayoutpageListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the content of a store page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `pageId` - The ID of the page.
    pub fn update(&self, request: StorePage, enterprise_id: &str, page_id: &str) -> StorelayoutpageUpdateCall<'a, S> {
        StorelayoutpageUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _page_id: page_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `generate_authentication_token(...)`, `get(...)`, `get_available_product_set(...)`, `insert(...)`, `list(...)`, `revoke_device_access(...)`, `set_available_product_set(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deleted an EMM-managed user.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn delete(&self, enterprise_id: &str, user_id: &str) -> UserDeleteCall<'a, S> {
        UserDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates an authentication token which the device policy client can use to provision the given EMM-managed user account on a device. The generated token is single-use and expires after a few minutes. You can provision a maximum of 10 devices per user. This call only works with EMM-managed accounts.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn generate_authentication_token(&self, enterprise_id: &str, user_id: &str) -> UserGenerateAuthenticationTokenCall<'a, S> {
        UserGenerateAuthenticationTokenCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a user's details.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn get(&self, enterprise_id: &str, user_id: &str) -> UserGetCall<'a, S> {
        UserGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the set of products a user is entitled to access. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn get_available_product_set(&self, enterprise_id: &str, user_id: &str) -> UserGetAvailableProductSetCall<'a, S> {
        UserGetAvailableProductSetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new EMM-managed user. The Users resource passed in the body of the request should include an accountIdentifier and an accountType. If a corresponding user already exists with the same account identifier, the user will be updated with the resource. In this case only the displayName field can be changed.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn insert(&self, request: User, enterprise_id: &str) -> UserInsertCall<'a, S> {
        UserInsertCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up a user by primary email address. This is only supported for Google-managed users. Lookup of the id is not needed for EMM-managed users because the id is already returned in the result of the Users.insert call.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `email` - Required. The exact primary email address of the user to look up.
    pub fn list(&self, enterprise_id: &str, email: &str) -> UserListCall<'a, S> {
        UserListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _email: email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Revokes access to all devices currently provisioned to the user. The user will no longer be able to use the managed Play store on any of their managed devices. This call only works with EMM-managed accounts.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn revoke_device_access(&self, enterprise_id: &str, user_id: &str) -> UserRevokeDeviceAccesCall<'a, S> {
        UserRevokeDeviceAccesCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the set of products that a user is entitled to access (referred to as *whitelisted* products). Only products that are approved or products that were previously approved (products with revoked approval) can be whitelisted. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn set_available_product_set(&self, request: ProductSet, enterprise_id: &str, user_id: &str) -> UserSetAvailableProductSetCall<'a, S> {
        UserSetAvailableProductSetCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the details of an EMM-managed user. Can be used with EMM-managed users only (not Google managed users). Pass the new details in the Users resource in the request body. Only the displayName field can be changed. Other fields must either be unset or have the currently active value.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `userId` - The ID of the user.
    pub fn update(&self, request: User, enterprise_id: &str, user_id: &str) -> UserUpdateCall<'a, S> {
        UserUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *webapp* resources.
/// It is not used directly, but through the [`AndroidEnterprise`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_androidenterprise1 as androidenterprise1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use androidenterprise1::{AndroidEnterprise, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AndroidEnterprise::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.webapps();
/// # }
/// ```
pub struct WebappMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AndroidEnterprise<S>,
}

impl<'a, S> client::MethodsBuilder for WebappMethods<'a, S> {}

impl<'a, S> WebappMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing web app.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `webAppId` - The ID of the web app.
    pub fn delete(&self, enterprise_id: &str, web_app_id: &str) -> WebappDeleteCall<'a, S> {
        WebappDeleteCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _web_app_id: web_app_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets an existing web app.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `webAppId` - The ID of the web app.
    pub fn get(&self, enterprise_id: &str, web_app_id: &str) -> WebappGetCall<'a, S> {
        WebappGetCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _web_app_id: web_app_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new web app for the enterprise.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn insert(&self, request: WebApp, enterprise_id: &str) -> WebappInsertCall<'a, S> {
        WebappInsertCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the details of all web apps for a given enterprise.
    /// 
    /// # Arguments
    ///
    /// * `enterpriseId` - The ID of the enterprise.
    pub fn list(&self, enterprise_id: &str) -> WebappListCall<'a, S> {
        WebappListCall {
            hub: self.hub,
            _enterprise_id: enterprise_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing web app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `enterpriseId` - The ID of the enterprise.
    /// * `webAppId` - The ID of the web app.
    pub fn update(&self, request: WebApp, enterprise_id: &str, web_app_id: &str) -> WebappUpdateCall<'a, S> {
        WebappUpdateCall {
            hub: self.hub,
            _request: request,
            _enterprise_id: enterprise_id.to_string(),
            _web_app_id: web_app_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



