use super::*;
/// A builder providing access to all methods supported on *state* resources.
/// It is not used directly, but through the [`AppState`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_appstate1 as appstate1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use appstate1::{AppState, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = AppState::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `clear(...)`, `delete(...)`, `get(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.states();
/// # }
/// ```
pub struct StateMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a AppState<S>,
}

impl<'a, S> client::MethodsBuilder for StateMethods<'a, S> {}

impl<'a, S> StateMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears (sets to empty) the data for the passed key if and only if the passed version matches the currently stored version. This method results in a conflict error on version mismatch.
    /// 
    /// # Arguments
    ///
    /// * `stateKey` - The key for the data to be retrieved.
    pub fn clear(&self, state_key: i32) -> StateClearCall<'a, S> {
        StateClearCall {
            hub: self.hub,
            _state_key: state_key,
            _current_data_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a key and the data associated with it. The key is removed and no longer counts against the key quota. Note that since this method is not safe in the face of concurrent modifications, it should only be used for development and testing purposes. Invoking this method in shipping code can result in data loss and data corruption.
    /// 
    /// # Arguments
    ///
    /// * `stateKey` - The key for the data to be retrieved.
    pub fn delete(&self, state_key: i32) -> StateDeleteCall<'a, S> {
        StateDeleteCall {
            hub: self.hub,
            _state_key: state_key,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the data corresponding to the passed key. If the key does not exist on the server, an HTTP 404 will be returned.
    /// 
    /// # Arguments
    ///
    /// * `stateKey` - The key for the data to be retrieved.
    pub fn get(&self, state_key: i32) -> StateGetCall<'a, S> {
        StateGetCall {
            hub: self.hub,
            _state_key: state_key,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the states keys, and optionally the state data.
    pub fn list(&self) -> StateListCall<'a, S> {
        StateListCall {
            hub: self.hub,
            _include_data: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update the data associated with the input key if and only if the passed version matches the currently stored version. This method is safe in the face of concurrent writes. Maximum per-key size is 128KB.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `stateKey` - The key for the data to be retrieved.
    pub fn update(&self, request: UpdateRequest, state_key: i32) -> StateUpdateCall<'a, S> {
        StateUpdateCall {
            hub: self.hub,
            _request: request,
            _state_key: state_key,
            _current_state_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



