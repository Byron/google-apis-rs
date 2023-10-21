use super::*;
/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the [`ChromeManagement`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_chromemanagement1 as chromemanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use chromemanagement1::{ChromeManagement, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `apps_android_get(...)`, `apps_chrome_get(...)`, `apps_count_chrome_app_requests(...)`, `apps_web_get(...)`, `reports_count_chrome_devices_reaching_auto_expiration_date(...)`, `reports_count_chrome_devices_that_need_attention(...)`, `reports_count_chrome_hardware_fleet_devices(...)`, `reports_count_chrome_versions(...)`, `reports_count_installed_apps(...)`, `reports_find_installed_app_devices(...)`, `telemetry_devices_get(...)`, `telemetry_devices_list(...)` and `telemetry_events_list(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ChromeManagement<S>,
}

impl<'a, S> client::MethodsBuilder for CustomerMethods<'a, S> {}

impl<'a, S> CustomerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_android_get(&self, name: &str) -> CustomerAppAndroidGetCall<'a, S> {
        CustomerAppAndroidGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_chrome_get(&self, name: &str) -> CustomerAppChromeGetCall<'a, S> {
        CustomerAppChromeGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get a specific app for a customer by its resource name.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The app for which details are being queried. Examples: "customers/my_customer/apps/chrome/gmbmikajjgmnabiglmofipeabaddhgne@2.1.2" for the Save to Google Drive Chrome extension version 2.1.2, "customers/my_customer/apps/android/com.google.android.apps.docs" for the Google Drive Android app's latest version.
    pub fn apps_web_get(&self, name: &str) -> CustomerAppWebGetCall<'a, S> {
        CustomerAppWebGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate summary of app installation requests.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn apps_count_chrome_app_requests(&self, customer: &str) -> CustomerAppCountChromeAppRequestCall<'a, S> {
        CustomerAppCountChromeAppRequestCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of the number of devices expiring in each month of the selected time frame. Devices are grouped by auto update expiration date and model. Further information can be found [here](https://support.google.com/chrome/a/answer/10564947).
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The customer ID or "my_customer" prefixed with "customers/".
    pub fn reports_count_chrome_devices_reaching_auto_expiration_date(&self, customer: &str) -> CustomerReportCountChromeDevicesReachingAutoExpirationDateCall<'a, S> {
        CustomerReportCountChromeDevicesReachingAutoExpirationDateCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _org_unit_id: Default::default(),
            _min_aue_date: Default::default(),
            _max_aue_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Counts of ChromeOS devices that have not synced policies or have lacked user activity in the past 28 days, are out of date, or are not complaint. Further information can be found here https://support.google.com/chrome/a/answer/10564947
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The customer ID or "my_customer" prefixed with "customers/".
    pub fn reports_count_chrome_devices_that_need_attention(&self, customer: &str) -> CustomerReportCountChromeDevicesThatNeedAttentionCall<'a, S> {
        CustomerReportCountChromeDevicesThatNeedAttentionCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _read_mask: Default::default(),
            _org_unit_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Counts of devices with a specific hardware specification from the requested hardware type (for example model name, processor type). Further information can be found here https://support.google.com/chrome/a/answer/10564947
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. The customer ID or "my_customer".
    pub fn reports_count_chrome_hardware_fleet_devices(&self, customer: &str) -> CustomerReportCountChromeHardwareFleetDeviceCall<'a, S> {
        CustomerReportCountChromeHardwareFleetDeviceCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _read_mask: Default::default(),
            _org_unit_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of installed Chrome versions.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_chrome_versions(&self, customer: &str) -> CustomerReportCountChromeVersionCall<'a, S> {
        CustomerReportCountChromeVersionCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of app installations.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_installed_apps(&self, customer: &str) -> CustomerReportCountInstalledAppCall<'a, S> {
        CustomerReportCountInstalledAppCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of devices that have a specified app installed.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_find_installed_app_devices(&self, customer: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, S> {
        CustomerReportFindInstalledAppDeviceCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _app_type: Default::default(),
            _app_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get telemetry device.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Name of the `TelemetryDevice` to return.
    pub fn telemetry_devices_get(&self, name: &str) -> CustomerTelemetryDeviceGetCall<'a, S> {
        CustomerTelemetryDeviceGetCall {
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
    /// List all telemetry devices.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn telemetry_devices_list(&self, parent: &str) -> CustomerTelemetryDeviceListCall<'a, S> {
        CustomerTelemetryDeviceListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_mask: Default::default(),
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
    /// List telemetry events.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn telemetry_events_list(&self, parent: &str) -> CustomerTelemetryEventListCall<'a, S> {
        CustomerTelemetryEventListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



