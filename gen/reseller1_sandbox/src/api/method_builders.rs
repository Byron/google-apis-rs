use super::*;
/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the [`Reseller`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_reseller1_sandbox as reseller1_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use reseller1_sandbox::{Reseller, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Reseller::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `insert(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Reseller<S>,
}

impl<'a, S> client::MethodsBuilder for CustomerMethods<'a, S> {}

impl<'a, S> CustomerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a customer resource if one exists and is owned by the reseller.
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Id of the Customer
    pub fn get(&self, customer_id: &str) -> CustomerGetCall<'a, S> {
        CustomerGetCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a customer resource if one does not already exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Customer) -> CustomerInsertCall<'a, S> {
        CustomerInsertCall {
            hub: self.hub,
            _request: request,
            _customer_auth_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a customer resource if one it exists and is owned by the reseller. This method supports patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customerId` - Id of the Customer
    pub fn patch(&self, request: Customer, customer_id: &str) -> CustomerPatchCall<'a, S> {
        CustomerPatchCall {
            hub: self.hub,
            _request: request,
            _customer_id: customer_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Update a customer resource if one it exists and is owned by the reseller.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customerId` - Id of the Customer
    pub fn update(&self, request: Customer, customer_id: &str) -> CustomerUpdateCall<'a, S> {
        CustomerUpdateCall {
            hub: self.hub,
            _request: request,
            _customer_id: customer_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *subscription* resources.
/// It is not used directly, but through the [`Reseller`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_reseller1_sandbox as reseller1_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use reseller1_sandbox::{Reseller, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Reseller::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `activate(...)`, `change_plan(...)`, `change_renewal_settings(...)`, `change_seats(...)`, `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `start_paid_service(...)` and `suspend(...)`
/// // to build up your call.
/// let rb = hub.subscriptions();
/// # }
/// ```
pub struct SubscriptionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Reseller<S>,
}

impl<'a, S> client::MethodsBuilder for SubscriptionMethods<'a, S> {}

impl<'a, S> SubscriptionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Activates a subscription previously suspended by the reseller
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    pub fn activate(&self, customer_id: &str, subscription_id: &str) -> SubscriptionActivateCall<'a, S> {
        SubscriptionActivateCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the plan of a subscription
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    pub fn change_plan(&self, request: ChangePlanRequest, customer_id: &str, subscription_id: &str) -> SubscriptionChangePlanCall<'a, S> {
        SubscriptionChangePlanCall {
            hub: self.hub,
            _request: request,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the renewal settings of a subscription
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    pub fn change_renewal_settings(&self, request: RenewalSettings, customer_id: &str, subscription_id: &str) -> SubscriptionChangeRenewalSettingCall<'a, S> {
        SubscriptionChangeRenewalSettingCall {
            hub: self.hub,
            _request: request,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Changes the seats configuration of a subscription
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    pub fn change_seats(&self, request: Seats, customer_id: &str, subscription_id: &str) -> SubscriptionChangeSeatCall<'a, S> {
        SubscriptionChangeSeatCall {
            hub: self.hub,
            _request: request,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels/Downgrades a subscription.
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    /// * `deletionType` - Whether the subscription is to be fully cancelled or downgraded
    pub fn delete(&self, customer_id: &str, subscription_id: &str, deletion_type: &str) -> SubscriptionDeleteCall<'a, S> {
        SubscriptionDeleteCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _deletion_type: deletion_type.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a subscription of the customer.
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    pub fn get(&self, customer_id: &str, subscription_id: &str) -> SubscriptionGetCall<'a, S> {
        SubscriptionGetCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates/Transfers a subscription for the customer.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `customerId` - Id of the Customer
    pub fn insert(&self, request: Subscription, customer_id: &str) -> SubscriptionInsertCall<'a, S> {
        SubscriptionInsertCall {
            hub: self.hub,
            _request: request,
            _customer_id: customer_id.to_string(),
            _customer_auth_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists subscriptions of a reseller, optionally filtered by a customer name prefix.
    pub fn list(&self) -> SubscriptionListCall<'a, S> {
        SubscriptionListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _customer_name_prefix: Default::default(),
            _customer_id: Default::default(),
            _customer_auth_token: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts paid service of a trial subscription
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    pub fn start_paid_service(&self, customer_id: &str, subscription_id: &str) -> SubscriptionStartPaidServiceCall<'a, S> {
        SubscriptionStartPaidServiceCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Suspends an active subscription
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Id of the Customer
    /// * `subscriptionId` - Id of the subscription, which is unique for a customer
    pub fn suspend(&self, customer_id: &str, subscription_id: &str) -> SubscriptionSuspendCall<'a, S> {
        SubscriptionSuspendCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _subscription_id: subscription_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



