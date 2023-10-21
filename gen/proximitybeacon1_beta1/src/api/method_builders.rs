use super::*;
/// A builder providing access to all methods supported on *beaconinfo* resources.
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `getforobserved(...)`
/// // to build up your call.
/// let rb = hub.beaconinfo();
/// # }
/// ```
pub struct BeaconinfoMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for BeaconinfoMethods<'a, S> {}

impl<'a, S> BeaconinfoMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Given one or more beacon observations, returns any beacon information
    /// and attachments accessible to your application. Authorize by using the
    /// [API
    /// key](https://developers.google.com/beacons/proximity/get-started#request_a_browser_api_key)
    /// for the application.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn getforobserved(&self, request: GetInfoForObservedBeaconsRequest) -> BeaconinfoGetforobservedCall<'a, S> {
        BeaconinfoGetforobservedCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *beacon* resources.
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `activate(...)`, `attachments_batch_delete(...)`, `attachments_create(...)`, `attachments_delete(...)`, `attachments_list(...)`, `deactivate(...)`, `decommission(...)`, `delete(...)`, `diagnostics_list(...)`, `get(...)`, `list(...)`, `register(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.beacons();
/// # }
/// ```
pub struct BeaconMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for BeaconMethods<'a, S> {}

impl<'a, S> BeaconMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes multiple attachments on a given beacon. This operation is
    /// permanent and cannot be undone.
    /// 
    /// You can optionally specify `namespacedType` to choose which attachments
    /// should be deleted. If you do not specify `namespacedType`,  all your
    /// attachments on the given beacon will be deleted. You also may explicitly
    /// specify `*/*` to delete all.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - The beacon whose attachments should be deleted. A beacon name has the
    ///                  format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    ///                  by the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn attachments_batch_delete(&self, beacon_name: &str) -> BeaconAttachmentBatchDeleteCall<'a, S> {
        BeaconAttachmentBatchDeleteCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _namespaced_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associates the given data with the specified beacon. Attachment data must
    /// contain two parts:
    /// <ul>
    /// <li>A namespaced type.</li>
    /// <li>The actual attachment data itself.</li>
    /// </ul>
    /// The namespaced type consists of two parts, the namespace and the type.
    /// The namespace must be one of the values returned by the `namespaces`
    /// endpoint, while the type can be a string of any characters except for the
    /// forward slash (`/`) up to 100 characters in length.
    /// 
    /// Attachment data can be up to 1024 bytes long.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `beaconName` - Beacon on which the attachment should be created. A beacon name has the
    ///                  format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    ///                  by the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn attachments_create(&self, request: BeaconAttachment, beacon_name: &str) -> BeaconAttachmentCreateCall<'a, S> {
        BeaconAttachmentCreateCall {
            hub: self.hub,
            _request: request,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified attachment for the given beacon. Each attachment has
    /// a unique attachment name (`attachmentName`) which is returned when you
    /// fetch the attachment data via this API. You specify this with the delete
    /// request to control which attachment is removed. This operation cannot be
    /// undone.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `attachmentName` - The attachment name (`attachmentName`) of
    ///                      the attachment to remove. For example:
    ///                      `beacons/3!893737abc9/attachments/c5e937-af0-494-959-ec49d12738`. For
    ///                      Eddystone-EID beacons, the beacon ID portion (`3!893737abc9`) may be the
    ///                      beacon's current EID, or its "stable" Eddystone-UID.
    ///                      Required.
    pub fn attachments_delete(&self, attachment_name: &str) -> BeaconAttachmentDeleteCall<'a, S> {
        BeaconAttachmentDeleteCall {
            hub: self.hub,
            _attachment_name: attachment_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the attachments for the specified beacon that match the specified
    /// namespaced-type pattern.
    /// 
    /// To control which namespaced types are returned, you add the
    /// `namespacedType` query parameter to the request. You must either use
    /// `*/*`, to return all attachments, or the namespace must be one of
    /// the ones returned from the  `namespaces` endpoint.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon whose attachments should be fetched. A beacon name has the
    ///                  format "beacons/N!beaconId" where the beaconId is the base16 ID broadcast
    ///                  by the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn attachments_list(&self, beacon_name: &str) -> BeaconAttachmentListCall<'a, S> {
        BeaconAttachmentListCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _namespaced_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the diagnostics for a single beacon. You can also list diagnostics for
    /// all the beacons owned by your Google Developers Console project by using
    /// the beacon name `beacons/-`.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that the diagnostics are for.
    pub fn diagnostics_list(&self, beacon_name: &str) -> BeaconDiagnosticListCall<'a, S> {
        BeaconDiagnosticListCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _alert_filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates a beacon. A beacon that is active will return information
    /// and attachment data when queried via `beaconinfo.getforobserved`.
    /// Calling this method on an already active beacon will do nothing (but
    /// will return a successful response code).
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be activated. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn activate(&self, beacon_name: &str) -> BeaconActivateCall<'a, S> {
        BeaconActivateCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deactivates a beacon. Once deactivated, the API will not return
    /// information nor attachment data for the beacon when queried via
    /// `beaconinfo.getforobserved`. Calling this method on an already inactive
    /// beacon will do nothing (but will return a successful response code).
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be deactivated. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn deactivate(&self, beacon_name: &str) -> BeaconDeactivateCall<'a, S> {
        BeaconDeactivateCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Decommissions the specified beacon in the service. This beacon will no
    /// longer be returned from `beaconinfo.getforobserved`. This operation is
    /// permanent -- you will not be able to re-register a beacon with this ID
    /// again.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be decommissioned. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID of the beacon's "stable" UID.
    ///                  Required.
    pub fn decommission(&self, beacon_name: &str) -> BeaconDecommissionCall<'a, S> {
        BeaconDecommissionCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified beacon including all diagnostics data for the beacon
    /// as well as any attachments on the beacon (including those belonging to
    /// other projects). This operation cannot be undone.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Beacon that should be deleted. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn delete(&self, beacon_name: &str) -> BeaconDeleteCall<'a, S> {
        BeaconDeleteCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns detailed information about the specified beacon.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    /// 
    /// Requests may supply an Eddystone-EID beacon name in the form:
    /// `beacons/4!beaconId` where the `beaconId` is the base16 ephemeral ID
    /// broadcast by the beacon. The returned `Beacon` object will contain the
    /// beacon's stable Eddystone-UID. Clients not authorized to resolve the
    /// beacon's ephemeral Eddystone-EID broadcast will receive an error.
    /// 
    /// # Arguments
    ///
    /// * `beaconName` - Resource name of this beacon. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone-UID, `4` for Eddystone-EID, `1` for iBeacon, or `5`
    ///                  for AltBeacon. For Eddystone-EID beacons, you may use either the
    ///                  current EID or the beacon's "stable" UID.
    ///                  Required.
    pub fn get(&self, beacon_name: &str) -> BeaconGetCall<'a, S> {
        BeaconGetCall {
            hub: self.hub,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches the beacon registry for beacons that match the given search
    /// criteria. Only those beacons that the client has permission to list
    /// will be returned.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    pub fn list(&self) -> BeaconListCall<'a, S> {
        BeaconListCall {
            hub: self.hub,
            _q: Default::default(),
            _project_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Registers a previously unregistered beacon given its `advertisedId`.
    /// These IDs are unique within the system. An ID can be registered only once.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn register(&self, request: Beacon) -> BeaconRegisterCall<'a, S> {
        BeaconRegisterCall {
            hub: self.hub,
            _request: request,
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the information about the specified beacon. **Any field that you do
    /// not populate in the submitted beacon will be permanently erased**, so you
    /// should follow the "read, modify, write" pattern to avoid inadvertently
    /// destroying data.
    /// 
    /// Changes to the beacon status via this method will be  silently ignored.
    /// To update beacon status, use the separate methods on this API for
    /// activation, deactivation, and decommissioning.
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **Is owner** or **Can edit** permissions in the Google
    /// Developers Console project.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `beaconName` - Resource name of this beacon. A beacon name has the format
    ///                  "beacons/N!beaconId" where the beaconId is the base16 ID broadcast by
    ///                  the beacon and N is a code for the beacon's type. Possible values are
    ///                  `3` for Eddystone, `1` for iBeacon, or `5` for AltBeacon.
    ///                  This field must be left empty when registering. After reading a beacon,
    ///                  clients can use the name for future operations.
    pub fn update(&self, request: Beacon, beacon_name: &str) -> BeaconUpdateCall<'a, S> {
        BeaconUpdateCall {
            hub: self.hub,
            _request: request,
            _beacon_name: beacon_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *namespace* resources.
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.namespaces();
/// # }
/// ```
pub struct NamespaceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for NamespaceMethods<'a, S> {}

impl<'a, S> NamespaceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all attachment namespaces owned by your Google Developers Console
    /// project. Attachment data associated with a beacon must include a
    /// namespaced type, and the namespace must be owned by your project.
    /// 
    /// Authenticate using an [OAuth access
    /// token](https://developers.google.com/identity/protocols/OAuth2) from a
    /// signed-in user with **viewer**, **Is owner** or **Can edit** permissions in
    /// the Google Developers Console project.
    pub fn list(&self) -> NamespaceListCall<'a, S> {
        NamespaceListCall {
            hub: self.hub,
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the information about the specified namespace. Only the namespace
    /// visibility can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `namespaceName` - Resource name of this namespace. Namespaces names have the format:
    ///                     <code>namespaces/<var>namespace</var></code>.
    pub fn update(&self, request: Namespace, namespace_name: &str) -> NamespaceUpdateCall<'a, S> {
        NamespaceUpdateCall {
            hub: self.hub,
            _request: request,
            _namespace_name: namespace_name.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`Proximitybeacon`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_proximitybeacon1_beta1 as proximitybeacon1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use proximitybeacon1_beta1::{Proximitybeacon, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Proximitybeacon::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_eidparams(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Proximitybeacon<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Proximity Beacon API's current public key and associated
    /// parameters used to initiate the Diffie-Hellman key exchange required to
    /// register a beacon that broadcasts the Eddystone-EID format. This key
    /// changes periodically; clients may cache it and re-use the same public key
    /// to provision and register multiple beacons. However, clients should be
    /// prepared to refresh this key when they encounter an error registering an
    /// Eddystone-EID beacon.
    pub fn get_eidparams(&self) -> MethodGetEidparamCall<'a, S> {
        MethodGetEidparamCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



