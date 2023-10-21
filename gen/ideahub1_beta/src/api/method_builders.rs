use super::*;
/// A builder providing access to all methods supported on *platform* resources.
/// It is not used directly, but through the [`Ideahub`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_ideahub1_beta as ideahub1_beta;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use ideahub1_beta::{Ideahub, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Ideahub::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `properties_idea_activities_create(...)`, `properties_idea_states_patch(...)`, `properties_ideas_list(...)`, `properties_locales_list(...)` and `properties_topic_states_patch(...)`
/// // to build up your call.
/// let rb = hub.platforms();
/// # }
/// ```
pub struct PlatformMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Ideahub<S>,
}

impl<'a, S> client::MethodsBuilder for PlatformMethods<'a, S> {}

impl<'a, S> PlatformMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an idea activity entry.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource where this idea activity will be created. Format: platforms/{platform}/property/{property}
    pub fn properties_idea_activities_create(&self, request: GoogleSearchIdeahubV1betaIdeaActivity, parent: &str) -> PlatformPropertyIdeaActivityCreateCall<'a, S> {
        PlatformPropertyIdeaActivityCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update an idea state resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Unique identifier for the idea state. Format: platforms/{platform}/properties/{property}/ideaStates/{idea_state}
    pub fn properties_idea_states_patch(&self, request: GoogleSearchIdeahubV1betaIdeaState, name: &str) -> PlatformPropertyIdeaStatePatchCall<'a, S> {
        PlatformPropertyIdeaStatePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List ideas for a given Creator and filter and sort options.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. If defined, specifies the creator for which to filter by. Format: publishers/{publisher}/properties/{property}
    pub fn properties_ideas_list(&self, parent: &str) -> PlatformPropertyIdeaListCall<'a, S> {
        PlatformPropertyIdeaListCall {
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
    /// Returns which locales ideas are available in for a given Creator.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The web property to check idea availability for Format: platforms/{platform}/property/{property}
    pub fn properties_locales_list(&self, parent: &str) -> PlatformPropertyLocaleListCall<'a, S> {
        PlatformPropertyLocaleListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a topic state resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Unique identifier for the topic state. Format: platforms/{platform}/properties/{property}/topicStates/{topic_state}
    pub fn properties_topic_states_patch(&self, request: GoogleSearchIdeahubV1betaTopicState, name: &str) -> PlatformPropertyTopicStatePatchCall<'a, S> {
        PlatformPropertyTopicStatePatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



