use super::*;
/// A builder providing access to all methods supported on *orderinvoice* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `createchargeinvoice(...)` and `createrefundinvoice(...)`
/// // to build up your call.
/// let rb = hub.orderinvoices();
/// # }
/// ```
pub struct OrderinvoiceMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderinvoiceMethods<'a, S> {}

impl<'a, S> OrderinvoiceMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a charge invoice for a shipment group, and triggers a charge capture for non-facilitated payment orders.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createchargeinvoice(&self, request: OrderinvoicesCreateChargeInvoiceRequest, merchant_id: u64, order_id: &str) -> OrderinvoiceCreatechargeinvoiceCall<'a, S> {
        OrderinvoiceCreatechargeinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a refund invoice for one or more shipment groups, and triggers a refund for non-facilitated payment orders. This can only be used for line items that have previously been charged using createChargeInvoice. All amounts (except for the summary) are incremental with respect to the previous invoice.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createrefundinvoice(&self, request: OrderinvoicesCreateRefundInvoiceRequest, merchant_id: u64, order_id: &str) -> OrderinvoiceCreaterefundinvoiceCall<'a, S> {
        OrderinvoiceCreaterefundinvoiceCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderpayment* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `notifyauthapproved(...)`, `notifyauthdeclined(...)`, `notifycharge(...)` and `notifyrefund(...)`
/// // to build up your call.
/// let rb = hub.orderpayments();
/// # }
/// ```
pub struct OrderpaymentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderpaymentMethods<'a, S> {}

impl<'a, S> OrderpaymentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about successfully authorizing user's payment method for a given amount.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for for which payment authorization is happening.
    pub fn notifyauthapproved(&self, request: OrderpaymentsNotifyAuthApprovedRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifyauthapprovedCall<'a, S> {
        OrderpaymentNotifyauthapprovedCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about failure to authorize user's payment method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for which payment authorization was declined.
    pub fn notifyauthdeclined(&self, request: OrderpaymentsNotifyAuthDeclinedRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifyauthdeclinedCall<'a, S> {
        OrderpaymentNotifyauthdeclinedCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about charge on user's selected payments method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for which charge is happening.
    pub fn notifycharge(&self, request: OrderpaymentsNotifyChargeRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifychargeCall<'a, S> {
        OrderpaymentNotifychargeCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notify about refund on user's selected payments method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order for which charge is happening.
    pub fn notifyrefund(&self, request: OrderpaymentsNotifyRefundRequest, merchant_id: u64, order_id: &str) -> OrderpaymentNotifyrefundCall<'a, S> {
        OrderpaymentNotifyrefundCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *orderreturn* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.orderreturns();
/// # }
/// ```
pub struct OrderreturnMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderreturnMethods<'a, S> {}

impl<'a, S> OrderreturnMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order return from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `returnId` - Merchant order return ID generated by Google.
    pub fn get(&self, merchant_id: u64, return_id: &str) -> OrderreturnGetCall<'a, S> {
        OrderreturnGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _return_id: return_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists order returns in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> OrderreturnListCall<'a, S> {
        OrderreturnListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _created_start_date: Default::default(),
            _created_end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *order* resources.
/// It is not used directly, but through the [`ShoppingContent`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_content2_sandbox as content2_sandbox;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use content2_sandbox::{ShoppingContent, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ShoppingContent::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `acknowledge(...)`, `advancetestorder(...)`, `cancel(...)`, `cancellineitem(...)`, `canceltestorderbycustomer(...)`, `createtestorder(...)`, `createtestreturn(...)`, `custombatch(...)`, `get(...)`, `getbymerchantorderid(...)`, `gettestordertemplate(...)`, `instorerefundlineitem(...)`, `list(...)`, `refund(...)`, `rejectreturnlineitem(...)`, `returnlineitem(...)`, `returnrefundlineitem(...)`, `setlineitemmetadata(...)`, `shiplineitems(...)`, `updatelineitemshippingdetails(...)`, `updatemerchantorderid(...)` and `updateshipment(...)`
/// // to build up your call.
/// let rb = hub.orders();
/// # }
/// ```
pub struct OrderMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a ShoppingContent<S>,
}

impl<'a, S> client::MethodsBuilder for OrderMethods<'a, S> {}

impl<'a, S> OrderMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks an order as acknowledged.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn acknowledge(&self, request: OrdersAcknowledgeRequest, merchant_id: u64, order_id: &str) -> OrderAcknowledgeCall<'a, S> {
        OrderAcknowledgeCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Moves a test order from state "inProgress" to state "pendingShipment".
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the test order to modify.
    pub fn advancetestorder(&self, merchant_id: u64, order_id: &str) -> OrderAdvancetestorderCall<'a, S> {
        OrderAdvancetestorderCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels all line items in an order, making a full refund.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order to cancel.
    pub fn cancel(&self, request: OrdersCancelRequest, merchant_id: u64, order_id: &str) -> OrderCancelCall<'a, S> {
        OrderCancelCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Cancels a line item, making a full refund.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn cancellineitem(&self, request: OrdersCancelLineItemRequest, merchant_id: u64, order_id: &str) -> OrderCancellineitemCall<'a, S> {
        OrderCancellineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Cancels a test order for customer-initiated cancellation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the test order to cancel.
    pub fn canceltestorderbycustomer(&self, request: OrdersCancelTestOrderByCustomerRequest, merchant_id: u64, order_id: &str) -> OrderCanceltestorderbycustomerCall<'a, S> {
        OrderCanceltestorderbycustomerCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Creates a test order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that should manage the order. This cannot be a multi-client account.
    pub fn createtestorder(&self, request: OrdersCreateTestOrderRequest, merchant_id: u64) -> OrderCreatetestorderCall<'a, S> {
        OrderCreatetestorderCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Creates a test return.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn createtestreturn(&self, request: OrdersCreateTestReturnRequest, merchant_id: u64, order_id: &str) -> OrderCreatetestreturnCall<'a, S> {
        OrderCreatetestreturnCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves or modifies multiple orders in a single request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn custombatch(&self, request: OrdersCustomBatchRequest) -> OrderCustombatchCall<'a, S> {
        OrderCustombatchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order from your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn get(&self, merchant_id: u64, order_id: &str) -> OrderGetCall<'a, S> {
        OrderGetCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an order using merchant order id.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `merchantOrderId` - The merchant order id to be looked for.
    pub fn getbymerchantorderid(&self, merchant_id: u64, merchant_order_id: &str) -> OrderGetbymerchantorderidCall<'a, S> {
        OrderGetbymerchantorderidCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _merchant_order_id: merchant_order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sandbox only. Retrieves an order template that can be used to quickly create a new order in sandbox.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that should manage the order. This cannot be a multi-client account.
    /// * `templateName` - The name of the template to retrieve.
    pub fn gettestordertemplate(&self, merchant_id: u64, template_name: &str) -> OrderGettestordertemplateCall<'a, S> {
        OrderGettestordertemplateCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _template_name: template_name.to_string(),
            _country: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Notifies that item return and refund was handled directly by merchant outside of Google payments processing (e.g. cash refund done in store).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn instorerefundlineitem(&self, request: OrdersInStoreRefundLineItemRequest, merchant_id: u64, order_id: &str) -> OrderInstorerefundlineitemCall<'a, S> {
        OrderInstorerefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the orders in your Merchant Center account.
    /// 
    /// # Arguments
    ///
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    pub fn list(&self, merchant_id: u64) -> OrderListCall<'a, S> {
        OrderListCall {
            hub: self.hub,
            _merchant_id: merchant_id,
            _statuses: Default::default(),
            _placed_date_start: Default::default(),
            _placed_date_end: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _acknowledged: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deprecated, please use returnRefundLineItem instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order to refund.
    pub fn refund(&self, request: OrdersRefundRequest, merchant_id: u64, order_id: &str) -> OrderRefundCall<'a, S> {
        OrderRefundCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rejects return on an line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn rejectreturnlineitem(&self, request: OrdersRejectReturnLineItemRequest, merchant_id: u64, order_id: &str) -> OrderRejectreturnlineitemCall<'a, S> {
        OrderRejectreturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn returnlineitem(&self, request: OrdersReturnLineItemRequest, merchant_id: u64, order_id: &str) -> OrderReturnlineitemCall<'a, S> {
        OrderReturnlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns and refunds a line item. Note that this method can only be called on fully shipped orders.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn returnrefundlineitem(&self, request: OrdersReturnRefundLineItemRequest, merchant_id: u64, order_id: &str) -> OrderReturnrefundlineitemCall<'a, S> {
        OrderReturnrefundlineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets (overrides) merchant provided annotations on the line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn setlineitemmetadata(&self, request: OrdersSetLineItemMetadataRequest, merchant_id: u64, order_id: &str) -> OrderSetlineitemmetadataCall<'a, S> {
        OrderSetlineitemmetadataCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks line item(s) as shipped.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn shiplineitems(&self, request: OrdersShipLineItemsRequest, merchant_id: u64, order_id: &str) -> OrderShiplineitemCall<'a, S> {
        OrderShiplineitemCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates ship by and delivery by dates for a line item.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updatelineitemshippingdetails(&self, request: OrdersUpdateLineItemShippingDetailsRequest, merchant_id: u64, order_id: &str) -> OrderUpdatelineitemshippingdetailCall<'a, S> {
        OrderUpdatelineitemshippingdetailCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the merchant order ID for a given order.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updatemerchantorderid(&self, request: OrdersUpdateMerchantOrderIdRequest, merchant_id: u64, order_id: &str) -> OrderUpdatemerchantorderidCall<'a, S> {
        OrderUpdatemerchantorderidCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a shipment's status, carrier, and/or tracking ID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `merchantId` - The ID of the account that manages the order. This cannot be a multi-client account.
    /// * `orderId` - The ID of the order.
    pub fn updateshipment(&self, request: OrdersUpdateShipmentRequest, merchant_id: u64, order_id: &str) -> OrderUpdateshipmentCall<'a, S> {
        OrderUpdateshipmentCall {
            hub: self.hub,
            _request: request,
            _merchant_id: merchant_id,
            _order_id: order_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



