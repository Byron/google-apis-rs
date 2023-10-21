use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`MyBusinessBusinessInformation`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessbusinessinformation1 as mybusinessbusinessinformation1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessbusinessinformation1::{MyBusinessBusinessInformation, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessBusinessInformation::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `locations_create(...)` and `locations_list(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessBusinessInformation<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new Location that will be owned by the logged in user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The name of the account in which to create this location.
    pub fn locations_create(&self, request: Location, parent: &str) -> AccountLocationCreateCall<'a, S> {
        AccountLocationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the locations for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the account to fetch locations from. If the parent Account is of AccountType PERSONAL, only Locations that are directly owned by the Account are returned, otherwise it will return all accessible locations from the Account, either directly or indirectly.
    pub fn locations_list(&self, parent: &str) -> AccountLocationListCall<'a, S> {
        AccountLocationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *attribute* resources.
/// It is not used directly, but through the [`MyBusinessBusinessInformation`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessbusinessinformation1 as mybusinessbusinessinformation1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessbusinessinformation1::{MyBusinessBusinessInformation, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessBusinessInformation::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.attributes();
/// # }
/// ```
pub struct AttributeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessBusinessInformation<S>,
}

impl<'a, S> client::MethodsBuilder for AttributeMethods<'a, S> {}

impl<'a, S> AttributeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of attributes that would be available for a location with the given primary category and country.
    pub fn list(&self) -> AttributeListCall<'a, S> {
        AttributeListCall {
            hub: self.hub,
            _show_all: Default::default(),
            _region_code: Default::default(),
            _parent: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _category_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *category* resources.
/// It is not used directly, but through the [`MyBusinessBusinessInformation`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessbusinessinformation1 as mybusinessbusinessinformation1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessbusinessinformation1::{MyBusinessBusinessInformation, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessBusinessInformation::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.categories();
/// # }
/// ```
pub struct CategoryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessBusinessInformation<S>,
}

impl<'a, S> client::MethodsBuilder for CategoryMethods<'a, S> {}

impl<'a, S> CategoryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of business categories for the provided language and GConcept ids.
    pub fn batch_get(&self) -> CategoryBatchGetCall<'a, S> {
        CategoryBatchGetCall {
            hub: self.hub,
            _view: Default::default(),
            _region_code: Default::default(),
            _names: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of business categories. Search will match the category name but not the category ID. Search only matches the front of a category name (that is, 'food' may return 'Food Court' but not 'Fast Food Restaurant').
    pub fn list(&self) -> CategoryListCall<'a, S> {
        CategoryListCall {
            hub: self.hub,
            _view: Default::default(),
            _region_code: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *chain* resources.
/// It is not used directly, but through the [`MyBusinessBusinessInformation`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessbusinessinformation1 as mybusinessbusinessinformation1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessbusinessinformation1::{MyBusinessBusinessInformation, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessBusinessInformation::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.chains();
/// # }
/// ```
pub struct ChainMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessBusinessInformation<S>,
}

impl<'a, S> client::MethodsBuilder for ChainMethods<'a, S> {}

impl<'a, S> ChainMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified chain. Returns `NOT_FOUND` if the chain does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The chain's resource name, in the format `chains/{chain_place_id}`.
    pub fn get(&self, name: &str) -> ChainGetCall<'a, S> {
        ChainGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches the chain based on chain name.
    pub fn search(&self) -> ChainSearchCall<'a, S> {
        ChainSearchCall {
            hub: self.hub,
            _page_size: Default::default(),
            _chain_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *googleLocation* resources.
/// It is not used directly, but through the [`MyBusinessBusinessInformation`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessbusinessinformation1 as mybusinessbusinessinformation1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessbusinessinformation1::{MyBusinessBusinessInformation, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessBusinessInformation::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `search(...)`
/// // to build up your call.
/// let rb = hub.google_locations();
/// # }
/// ```
pub struct GoogleLocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessBusinessInformation<S>,
}

impl<'a, S> client::MethodsBuilder for GoogleLocationMethods<'a, S> {}

impl<'a, S> GoogleLocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search all of the possible locations that are a match to the specified request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: SearchGoogleLocationsRequest) -> GoogleLocationSearchCall<'a, S> {
        GoogleLocationSearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *location* resources.
/// It is not used directly, but through the [`MyBusinessBusinessInformation`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessbusinessinformation1 as mybusinessbusinessinformation1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusinessbusinessinformation1::{MyBusinessBusinessInformation, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusinessBusinessInformation::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `associate(...)`, `attributes_get_google_updated(...)`, `clear_location_association(...)`, `delete(...)`, `get(...)`, `get_attributes(...)`, `get_google_updated(...)`, `patch(...)` and `update_attributes(...)`
/// // to build up your call.
/// let rb = hub.locations();
/// # }
/// ```
pub struct LocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusinessBusinessInformation<S>,
}

impl<'a, S> client::MethodsBuilder for LocationMethods<'a, S> {}

impl<'a, S> LocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Google-updated version of the specified location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Google identifier for this location in the form of `locations/{location_id}/attributes`.
    pub fn attributes_get_google_updated(&self, name: &str) -> LocationAttributeGetGoogleUpdatedCall<'a, S> {
        LocationAttributeGetGoogleUpdatedCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associates a location to a place ID. Any previous association is overwritten. This operation is only valid if the location is unverified. The association must be valid, that is, it appears in the list of `SearchGoogleLocations`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the location to associate.
    pub fn associate(&self, request: AssociateLocationRequest, name: &str) -> LocationAssociateCall<'a, S> {
        LocationAssociateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears an association between a location and its place ID. This operation is only valid if the location is unverified.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name of the location to disassociate.
    pub fn clear_location_association(&self, request: ClearLocationAssociationRequest, name: &str) -> LocationClearLocationAssociationCall<'a, S> {
        LocationClearLocationAssociationCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a location. If this location cannot be deleted using the API and it is marked so in the `google.mybusiness.businessinformation.v1.LocationState`, use the [Google Business Profile](https://business.google.com/manage/) website.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the location to delete.
    pub fn delete(&self, name: &str) -> LocationDeleteCall<'a, S> {
        LocationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the location to fetch.
    pub fn get(&self, name: &str) -> LocationGetCall<'a, S> {
        LocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Looks up all the attributes set for a given location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. Google identifier for this location in the form of `locations/{location_id}/attributes`.
    pub fn get_attributes(&self, name: &str) -> LocationGetAttributeCall<'a, S> {
        LocationGetAttributeCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Google-updated version of the specified location.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the location to fetch.
    pub fn get_google_updated(&self, name: &str) -> LocationGetGoogleUpdatedCall<'a, S> {
        LocationGetGoogleUpdatedCall {
            hub: self.hub,
            _name: name.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Google identifier for this location in the form: `locations/{location_id}`.
    pub fn patch(&self, request: Location, name: &str) -> LocationPatchCall<'a, S> {
        LocationPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update attributes for a given location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Google identifier for this location in the form of `locations/{location_id}/attributes`.
    pub fn update_attributes(&self, request: Attributes, name: &str) -> LocationUpdateAttributeCall<'a, S> {
        LocationUpdateAttributeCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _attribute_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



