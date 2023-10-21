use super::*;
/// A builder providing access to all methods supported on *partner* resources.
/// It is not used directly, but through the [`PaymentsResellerSubscription`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_paymentsresellersubscription1 as paymentsresellersubscription1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use paymentsresellersubscription1::{PaymentsResellerSubscription, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PaymentsResellerSubscription::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `products_list(...)`, `promotions_find_eligible(...)`, `promotions_list(...)`, `subscriptions_cancel(...)`, `subscriptions_create(...)`, `subscriptions_entitle(...)`, `subscriptions_extend(...)`, `subscriptions_get(...)`, `subscriptions_provision(...)` and `subscriptions_undo_cancel(...)`
/// // to build up your call.
/// let rb = hub.partners();
/// # }
/// ```
pub struct PartnerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a PaymentsResellerSubscription<S>,
}

impl<'a, S> client::MethodsBuilder for PartnerMethods<'a, S> {}

impl<'a, S> PartnerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// To retrieve the products that can be resold by the partner. It should be autenticated with a service account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, the partner that can resell. Format: partners/{partner}
    pub fn products_list(&self, parent: &str) -> PartnerProductListCall<'a, S> {
        PartnerProductListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// To find eligible promotions for the current user. The API requires user authorization via OAuth. The user is inferred from the authenticated OAuth credential.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent, the partner that can resell. Format: partners/{partner}
    pub fn promotions_find_eligible(&self, request: GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest, parent: &str) -> PartnerPromotionFindEligibleCall<'a, S> {
        PartnerPromotionFindEligibleCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// To retrieve the promotions, such as free trial, that can be used by the partner. It should be autenticated with a service account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The parent, the partner that can resell. Format: partners/{partner}
    pub fn promotions_list(&self, parent: &str) -> PartnerPromotionListCall<'a, S> {
        PartnerPromotionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to cancel a subscription service either immediately or by the end of the current billing cycle for their customers. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource to be cancelled. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_cancel(&self, request: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest, name: &str) -> PartnerSubscriptionCancelCall<'a, S> {
        PartnerSubscriptionCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to create a subscription for their customers. The created subscription is associated with the end user inferred from the end user credentials. This API must be authorized by the end user using OAuth.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}".
    pub fn subscriptions_create(&self, request: GoogleCloudPaymentsResellerSubscriptionV1Subscription, parent: &str) -> PartnerSubscriptionCreateCall<'a, S> {
        PartnerSubscriptionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _subscription_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to entitle a previously provisioned subscription to the current end user. The end user identity is inferred from the authorized credential of the request. This API must be authorized by the end user using OAuth.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource that is entitled to the current end user. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_entitle(&self, request: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest, name: &str) -> PartnerSubscriptionEntitleCall<'a, S> {
        PartnerSubscriptionEntitleCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// [Deprecated] New partners should be on auto-extend by default. Used by partners to extend a subscription service for their customers on an ongoing basis for the subscription to remain active and renewable. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource to be extended. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}".
    pub fn subscriptions_extend(&self, request: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest, name: &str) -> PartnerSubscriptionExtendCall<'a, S> {
        PartnerSubscriptionExtendCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to get a subscription by id. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. The name of the subscription resource to retrieve. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_get(&self, name: &str) -> PartnerSubscriptionGetCall<'a, S> {
        PartnerSubscriptionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to provision a subscription for their customers. This creates a subscription without associating it with the end user account. EntitleSubscription must be called separately using OAuth in order for the end user account to be associated with the subscription. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}".
    pub fn subscriptions_provision(&self, request: GoogleCloudPaymentsResellerSubscriptionV1Subscription, parent: &str) -> PartnerSubscriptionProvisionCall<'a, S> {
        PartnerSubscriptionProvisionCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _subscription_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Used by partners to revoke the pending cancellation of a subscription, which is currently in `STATE_CANCEL_AT_END_OF_CYCLE` state. If the subscription is already cancelled, the request will fail. It should be called directly by the partner using service accounts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the subscription resource whose pending cancellation needs to be undone. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}"
    pub fn subscriptions_undo_cancel(&self, request: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest, name: &str) -> PartnerSubscriptionUndoCancelCall<'a, S> {
        PartnerSubscriptionUndoCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



