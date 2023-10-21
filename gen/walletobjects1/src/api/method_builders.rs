use super::*;
/// A builder providing access to all methods supported on *eventticketclas* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.eventticketclass();
/// # }
/// ```
pub struct EventticketclasMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for EventticketclasMethods<'a, S> {}

impl<'a, S> EventticketclasMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the event ticket class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> EventticketclasAddmessageCall<'a, S> {
        EventticketclasAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the event ticket class with the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> EventticketclasGetCall<'a, S> {
        EventticketclasGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an event ticket class with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: EventTicketClass) -> EventticketclasInsertCall<'a, S> {
        EventticketclasInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all event ticket classes for a given issuer ID.
    pub fn list(&self) -> EventticketclasListCall<'a, S> {
        EventticketclasListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _issuer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the event ticket class referenced by the given class ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: EventTicketClass, resource_id: &str) -> EventticketclasPatchCall<'a, S> {
        EventticketclasPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the event ticket class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: EventTicketClass, resource_id: &str) -> EventticketclasUpdateCall<'a, S> {
        EventticketclasUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *eventticketobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `modifylinkedofferobjects(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.eventticketobject();
/// # }
/// ```
pub struct EventticketobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for EventticketobjectMethods<'a, S> {}

impl<'a, S> EventticketobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the event ticket object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> EventticketobjectAddmessageCall<'a, S> {
        EventticketobjectAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the event ticket object with the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> EventticketobjectGetCall<'a, S> {
        EventticketobjectGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an event ticket object with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: EventTicketObject) -> EventticketobjectInsertCall<'a, S> {
        EventticketobjectInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all event ticket objects for a given issuer ID.
    pub fn list(&self) -> EventticketobjectListCall<'a, S> {
        EventticketobjectListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _class_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies linked offer objects for the event ticket object with the given ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn modifylinkedofferobjects(&self, request: ModifyLinkedOfferObjectsRequest, resource_id: &str) -> EventticketobjectModifylinkedofferobjectCall<'a, S> {
        EventticketobjectModifylinkedofferobjectCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the event ticket object referenced by the given object ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: EventTicketObject, resource_id: &str) -> EventticketobjectPatchCall<'a, S> {
        EventticketobjectPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the event ticket object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: EventTicketObject, resource_id: &str) -> EventticketobjectUpdateCall<'a, S> {
        EventticketobjectUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *flightclas* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.flightclass();
/// # }
/// ```
pub struct FlightclasMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for FlightclasMethods<'a, S> {}

impl<'a, S> FlightclasMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the flight class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> FlightclasAddmessageCall<'a, S> {
        FlightclasAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the flight class with the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> FlightclasGetCall<'a, S> {
        FlightclasGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an flight class with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: FlightClass) -> FlightclasInsertCall<'a, S> {
        FlightclasInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all flight classes for a given issuer ID.
    pub fn list(&self) -> FlightclasListCall<'a, S> {
        FlightclasListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _issuer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the flight class referenced by the given class ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: FlightClass, resource_id: &str) -> FlightclasPatchCall<'a, S> {
        FlightclasPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the flight class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: FlightClass, resource_id: &str) -> FlightclasUpdateCall<'a, S> {
        FlightclasUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *flightobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.flightobject();
/// # }
/// ```
pub struct FlightobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for FlightobjectMethods<'a, S> {}

impl<'a, S> FlightobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the flight object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> FlightobjectAddmessageCall<'a, S> {
        FlightobjectAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the flight object with the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> FlightobjectGetCall<'a, S> {
        FlightobjectGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an flight object with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: FlightObject) -> FlightobjectInsertCall<'a, S> {
        FlightobjectInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all flight objects for a given issuer ID.
    pub fn list(&self) -> FlightobjectListCall<'a, S> {
        FlightobjectListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _class_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the flight object referenced by the given object ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: FlightObject, resource_id: &str) -> FlightobjectPatchCall<'a, S> {
        FlightobjectPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the flight object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: FlightObject, resource_id: &str) -> FlightobjectUpdateCall<'a, S> {
        FlightobjectUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *genericclas* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.genericclass();
/// # }
/// ```
pub struct GenericclasMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for GenericclasMethods<'a, S> {}

impl<'a, S> GenericclasMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the generic class with the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    pub fn get(&self, resource_id: &str) -> GenericclasGetCall<'a, S> {
        GenericclasGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a generic class with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: GenericClass) -> GenericclasInsertCall<'a, S> {
        GenericclasInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all generic classes for a given issuer ID.
    pub fn list(&self) -> GenericclasListCall<'a, S> {
        GenericclasListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _issuer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the generic class referenced by the given class ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    pub fn patch(&self, request: GenericClass, resource_id: &str) -> GenericclasPatchCall<'a, S> {
        GenericclasPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Generic class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    pub fn update(&self, request: GenericClass, resource_id: &str) -> GenericclasUpdateCall<'a, S> {
        GenericclasUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *genericobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.genericobject();
/// # }
/// ```
pub struct GenericobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for GenericobjectMethods<'a, S> {}

impl<'a, S> GenericobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the generic object with the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    pub fn get(&self, resource_id: &str) -> GenericobjectGetCall<'a, S> {
        GenericobjectGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a generic object with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: GenericObject) -> GenericobjectInsertCall<'a, S> {
        GenericobjectInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all generic objects for a given issuer ID.
    pub fn list(&self) -> GenericobjectListCall<'a, S> {
        GenericobjectListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _class_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the generic object referenced by the given object ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    pub fn patch(&self, request: GenericObject, resource_id: &str) -> GenericobjectPatchCall<'a, S> {
        GenericobjectPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the generic object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    pub fn update(&self, request: GenericObject, resource_id: &str) -> GenericobjectUpdateCall<'a, S> {
        GenericobjectUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *giftcardclas* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.giftcardclass();
/// # }
/// ```
pub struct GiftcardclasMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for GiftcardclasMethods<'a, S> {}

impl<'a, S> GiftcardclasMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the gift card class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> GiftcardclasAddmessageCall<'a, S> {
        GiftcardclasAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the gift card class with the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> GiftcardclasGetCall<'a, S> {
        GiftcardclasGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an gift card class with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: GiftCardClass) -> GiftcardclasInsertCall<'a, S> {
        GiftcardclasInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all gift card classes for a given issuer ID.
    pub fn list(&self) -> GiftcardclasListCall<'a, S> {
        GiftcardclasListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _issuer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the gift card class referenced by the given class ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: GiftCardClass, resource_id: &str) -> GiftcardclasPatchCall<'a, S> {
        GiftcardclasPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the gift card class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: GiftCardClass, resource_id: &str) -> GiftcardclasUpdateCall<'a, S> {
        GiftcardclasUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *giftcardobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.giftcardobject();
/// # }
/// ```
pub struct GiftcardobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for GiftcardobjectMethods<'a, S> {}

impl<'a, S> GiftcardobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the gift card object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> GiftcardobjectAddmessageCall<'a, S> {
        GiftcardobjectAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the gift card object with the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> GiftcardobjectGetCall<'a, S> {
        GiftcardobjectGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an gift card object with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: GiftCardObject) -> GiftcardobjectInsertCall<'a, S> {
        GiftcardobjectInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all gift card objects for a given issuer ID.
    pub fn list(&self) -> GiftcardobjectListCall<'a, S> {
        GiftcardobjectListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _class_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the gift card object referenced by the given object ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: GiftCardObject, resource_id: &str) -> GiftcardobjectPatchCall<'a, S> {
        GiftcardobjectPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the gift card object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: GiftCardObject, resource_id: &str) -> GiftcardobjectUpdateCall<'a, S> {
        GiftcardobjectUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *issuer* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.issuer();
/// # }
/// ```
pub struct IssuerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for IssuerMethods<'a, S> {}

impl<'a, S> IssuerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the issuer with the given issuer ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an issuer.
    pub fn get(&self, resource_id: i64) -> IssuerGetCall<'a, S> {
        IssuerGetCall {
            hub: self.hub,
            _resource_id: resource_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an issuer with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Issuer) -> IssuerInsertCall<'a, S> {
        IssuerInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all issuers shared to the caller.
    pub fn list(&self) -> IssuerListCall<'a, S> {
        IssuerListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the issuer referenced by the given issuer ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an issuer.
    pub fn patch(&self, request: Issuer, resource_id: i64) -> IssuerPatchCall<'a, S> {
        IssuerPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the issuer referenced by the given issuer ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an issuer.
    pub fn update(&self, request: Issuer, resource_id: i64) -> IssuerUpdateCall<'a, S> {
        IssuerUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *jwt* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.jwt();
/// # }
/// ```
pub struct JwtMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for JwtMethods<'a, S> {}

impl<'a, S> JwtMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts the resources in the JWT.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: JwtResource) -> JwtInsertCall<'a, S> {
        JwtInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *loyaltyclas* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.loyaltyclass();
/// # }
/// ```
pub struct LoyaltyclasMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for LoyaltyclasMethods<'a, S> {}

impl<'a, S> LoyaltyclasMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the loyalty class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> LoyaltyclasAddmessageCall<'a, S> {
        LoyaltyclasAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the loyalty class with the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> LoyaltyclasGetCall<'a, S> {
        LoyaltyclasGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an loyalty class with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: LoyaltyClass) -> LoyaltyclasInsertCall<'a, S> {
        LoyaltyclasInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all loyalty classes for a given issuer ID.
    pub fn list(&self) -> LoyaltyclasListCall<'a, S> {
        LoyaltyclasListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _issuer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the loyalty class referenced by the given class ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: LoyaltyClass, resource_id: &str) -> LoyaltyclasPatchCall<'a, S> {
        LoyaltyclasPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the loyalty class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: LoyaltyClass, resource_id: &str) -> LoyaltyclasUpdateCall<'a, S> {
        LoyaltyclasUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *loyaltyobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `modifylinkedofferobjects(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.loyaltyobject();
/// # }
/// ```
pub struct LoyaltyobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for LoyaltyobjectMethods<'a, S> {}

impl<'a, S> LoyaltyobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the loyalty object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> LoyaltyobjectAddmessageCall<'a, S> {
        LoyaltyobjectAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the loyalty object with the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> LoyaltyobjectGetCall<'a, S> {
        LoyaltyobjectGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an loyalty object with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: LoyaltyObject) -> LoyaltyobjectInsertCall<'a, S> {
        LoyaltyobjectInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all loyalty objects for a given issuer ID.
    pub fn list(&self) -> LoyaltyobjectListCall<'a, S> {
        LoyaltyobjectListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _class_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies linked offer objects for the loyalty object with the given ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn modifylinkedofferobjects(&self, request: ModifyLinkedOfferObjectsRequest, resource_id: &str) -> LoyaltyobjectModifylinkedofferobjectCall<'a, S> {
        LoyaltyobjectModifylinkedofferobjectCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the loyalty object referenced by the given object ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: LoyaltyObject, resource_id: &str) -> LoyaltyobjectPatchCall<'a, S> {
        LoyaltyobjectPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the loyalty object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: LoyaltyObject, resource_id: &str) -> LoyaltyobjectUpdateCall<'a, S> {
        LoyaltyobjectUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `upload(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for MediaMethods<'a, S> {}

impl<'a, S> MediaMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Uploads a private image and returns an Id to be used in its place.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `issuerId` - The ID of the issuer sending the image.
    pub fn upload(&self, request: UploadPrivateImageRequest, issuer_id: i64) -> MediaUploadCall<'a, S> {
        MediaUploadCall {
            hub: self.hub,
            _request: request,
            _issuer_id: issuer_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *offerclas* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.offerclass();
/// # }
/// ```
pub struct OfferclasMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for OfferclasMethods<'a, S> {}

impl<'a, S> OfferclasMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the offer class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> OfferclasAddmessageCall<'a, S> {
        OfferclasAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the offer class with the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> OfferclasGetCall<'a, S> {
        OfferclasGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an offer class with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: OfferClass) -> OfferclasInsertCall<'a, S> {
        OfferclasInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all offer classes for a given issuer ID.
    pub fn list(&self) -> OfferclasListCall<'a, S> {
        OfferclasListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _issuer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the offer class referenced by the given class ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: OfferClass, resource_id: &str) -> OfferclasPatchCall<'a, S> {
        OfferclasPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the offer class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: OfferClass, resource_id: &str) -> OfferclasUpdateCall<'a, S> {
        OfferclasUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *offerobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.offerobject();
/// # }
/// ```
pub struct OfferobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for OfferobjectMethods<'a, S> {}

impl<'a, S> OfferobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the offer object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> OfferobjectAddmessageCall<'a, S> {
        OfferobjectAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the offer object with the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> OfferobjectGetCall<'a, S> {
        OfferobjectGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an offer object with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: OfferObject) -> OfferobjectInsertCall<'a, S> {
        OfferobjectInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all offer objects for a given issuer ID.
    pub fn list(&self) -> OfferobjectListCall<'a, S> {
        OfferobjectListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _class_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the offer object referenced by the given object ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: OfferObject, resource_id: &str) -> OfferobjectPatchCall<'a, S> {
        OfferobjectPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the offer object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: OfferObject, resource_id: &str) -> OfferobjectUpdateCall<'a, S> {
        OfferobjectUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *permission* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.permissions();
/// # }
/// ```
pub struct PermissionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for PermissionMethods<'a, S> {}

impl<'a, S> PermissionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the permissions for the given issuer id.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an issuer. This ID must be unique across all issuers.
    pub fn get(&self, resource_id: i64) -> PermissionGetCall<'a, S> {
        PermissionGetCall {
            hub: self.hub,
            _resource_id: resource_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the permissions for the given issuer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an issuer. This ID must be unique across all issuers.
    pub fn update(&self, request: Permissions, resource_id: i64) -> PermissionUpdateCall<'a, S> {
        PermissionUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *smarttap* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.smarttap();
/// # }
/// ```
pub struct SmarttapMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for SmarttapMethods<'a, S> {}

impl<'a, S> SmarttapMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts the smart tap.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: SmartTap) -> SmarttapInsertCall<'a, S> {
        SmarttapInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *transitclas* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.transitclass();
/// # }
/// ```
pub struct TransitclasMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for TransitclasMethods<'a, S> {}

impl<'a, S> TransitclasMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the transit class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> TransitclasAddmessageCall<'a, S> {
        TransitclasAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the transit class with the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> TransitclasGetCall<'a, S> {
        TransitclasGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a transit class with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: TransitClass) -> TransitclasInsertCall<'a, S> {
        TransitclasInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all transit classes for a given issuer ID.
    pub fn list(&self) -> TransitclasListCall<'a, S> {
        TransitclasListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _issuer_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the transit class referenced by the given class ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: TransitClass, resource_id: &str) -> TransitclasPatchCall<'a, S> {
        TransitclasPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the transit class referenced by the given class ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: TransitClass, resource_id: &str) -> TransitclasUpdateCall<'a, S> {
        TransitclasUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *transitobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `addmessage(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.transitobject();
/// # }
/// ```
pub struct TransitobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for TransitobjectMethods<'a, S> {}

impl<'a, S> TransitobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a message to the transit object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn addmessage(&self, request: AddMessageRequest, resource_id: &str) -> TransitobjectAddmessageCall<'a, S> {
        TransitobjectAddmessageCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the transit object with the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn get(&self, resource_id: &str) -> TransitobjectGetCall<'a, S> {
        TransitobjectGetCall {
            hub: self.hub,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts an transit object with the given ID and properties.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: TransitObject) -> TransitobjectInsertCall<'a, S> {
        TransitobjectInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of all transit objects for a given issuer ID.
    pub fn list(&self) -> TransitobjectListCall<'a, S> {
        TransitobjectListCall {
            hub: self.hub,
            _token: Default::default(),
            _max_results: Default::default(),
            _class_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the transit object referenced by the given object ID. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn patch(&self, request: TransitObject, resource_id: &str) -> TransitobjectPatchCall<'a, S> {
        TransitobjectPatchCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the transit object referenced by the given object ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resourceId` - The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    pub fn update(&self, request: TransitObject, resource_id: &str) -> TransitobjectUpdateCall<'a, S> {
        TransitobjectUpdateCall {
            hub: self.hub,
            _request: request,
            _resource_id: resource_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *walletobject* resources.
/// It is not used directly, but through the [`Walletobjects`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_walletobjects1 as walletobjects1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use walletobjects1::{Walletobjects, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Walletobjects::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `v1_private_content_upload_private_data(...)`
/// // to build up your call.
/// let rb = hub.walletobjects();
/// # }
/// ```
pub struct WalletobjectMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Walletobjects<S>,
}

impl<'a, S> client::MethodsBuilder for WalletobjectMethods<'a, S> {}

impl<'a, S> WalletobjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Upload private data (text or URI) and returns an Id to be used in its place.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn v1_private_content_upload_private_data(&self, request: UploadPrivateDataRequest) -> WalletobjectV1PrivateContentUploadPrivateDataCall<'a, S> {
        WalletobjectV1PrivateContentUploadPrivateDataCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



