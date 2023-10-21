use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`MyBusinessNotificationSettings`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessnotifications1 as mybusinessnotifications1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessnotifications1::{MyBusinessNotificationSettings, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessNotificationSettings::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_notification_setting(...)` and `update_notification_setting(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessNotificationSettings<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the pubsub notification settings for the account.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the notification setting we are trying to fetch.
    pub fn get_notification_setting(&self, name: &str) -> AccountGetNotificationSettingCall<'a, S> {
        AccountGetNotificationSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the pubsub notification setting for the account informing Google which topic to send pubsub notifications for. Use the notification_types field within notification_setting to manipulate the events an account wants to subscribe to. An account will only have one notification setting resource, and only one pubsub topic can be set. To delete the setting, update with an empty notification_types
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`.
    pub fn update_notification_setting(&self, request: NotificationSetting, name: &str) -> AccountUpdateNotificationSettingCall<'a, S> {
        AccountUpdateNotificationSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



