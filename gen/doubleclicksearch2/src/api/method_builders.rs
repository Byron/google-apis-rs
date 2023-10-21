use super::*;
/// A builder providing access to all methods supported on *conversion* resources.
/// It is not used directly, but through the [`Doubleclicksearch`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_doubleclicksearch2 as doubleclicksearch2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use doubleclicksearch2::{Doubleclicksearch, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Doubleclicksearch::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `get_by_customer_id(...)`, `insert(...)`, `update(...)` and `update_availability(...)`
/// // to build up your call.
/// let rb = hub.conversion();
/// # }
/// ```
pub struct ConversionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Doubleclicksearch<S>,
}

impl<'a, S> client::MethodsBuilder for ConversionMethods<'a, S> {}

impl<'a, S> ConversionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of conversions from a DoubleClick Search engine account.
    /// 
    /// # Arguments
    ///
    /// * `agencyId` - Numeric ID of the agency.
    /// * `advertiserId` - Numeric ID of the advertiser.
    /// * `engineAccountId` - Numeric ID of the engine account.
    /// * `endDate` - Last date (inclusive) on which to retrieve conversions. Format is yyyymmdd.
    /// * `rowCount` - The number of conversions to return per call.
    /// * `startDate` - First date (inclusive) on which to retrieve conversions. Format is yyyymmdd.
    /// * `startRow` - The 0-based starting index for retrieving conversions results.
    pub fn get(&self, agency_id: i64, advertiser_id: i64, engine_account_id: i64, end_date: i32, row_count: i32, start_date: i32, start_row: u32) -> ConversionGetCall<'a, S> {
        ConversionGetCall {
            hub: self.hub,
            _agency_id: agency_id,
            _advertiser_id: advertiser_id,
            _engine_account_id: engine_account_id,
            _end_date: end_date,
            _row_count: row_count,
            _start_date: start_date,
            _start_row: start_row,
            _customer_id: Default::default(),
            _criterion_id: Default::default(),
            _campaign_id: Default::default(),
            _ad_id: Default::default(),
            _ad_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of conversions from a DoubleClick Search engine account.
    /// 
    /// # Arguments
    ///
    /// * `customerId` - Customer ID of a client account in the new Search Ads 360 experience.
    /// * `endDate` - Last date (inclusive) on which to retrieve conversions. Format is yyyymmdd.
    /// * `rowCount` - The number of conversions to return per call.
    /// * `startDate` - First date (inclusive) on which to retrieve conversions. Format is yyyymmdd.
    /// * `startRow` - The 0-based starting index for retrieving conversions results.
    pub fn get_by_customer_id(&self, customer_id: &str, end_date: i32, row_count: i32, start_date: i32, start_row: u32) -> ConversionGetByCustomerIdCall<'a, S> {
        ConversionGetByCustomerIdCall {
            hub: self.hub,
            _customer_id: customer_id.to_string(),
            _end_date: end_date,
            _row_count: row_count,
            _start_date: start_date,
            _start_row: start_row,
            _engine_account_id: Default::default(),
            _criterion_id: Default::default(),
            _campaign_id: Default::default(),
            _agency_id: Default::default(),
            _advertiser_id: Default::default(),
            _ad_id: Default::default(),
            _ad_group_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a batch of new conversions into DoubleClick Search.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: ConversionList) -> ConversionInsertCall<'a, S> {
        ConversionInsertCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a batch of conversions in DoubleClick Search.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: ConversionList) -> ConversionUpdateCall<'a, S> {
        ConversionUpdateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the availabilities of a batch of floodlight activities in DoubleClick Search.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update_availability(&self, request: UpdateAvailabilityRequest) -> ConversionUpdateAvailabilityCall<'a, S> {
        ConversionUpdateAvailabilityCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *report* resources.
/// It is not used directly, but through the [`Doubleclicksearch`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_doubleclicksearch2 as doubleclicksearch2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use doubleclicksearch2::{Doubleclicksearch, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Doubleclicksearch::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `generate(...)`, `get(...)`, `get_file(...)`, `get_id_mapping_file(...)` and `request(...)`
/// // to build up your call.
/// let rb = hub.reports();
/// # }
/// ```
pub struct ReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Doubleclicksearch<S>,
}

impl<'a, S> client::MethodsBuilder for ReportMethods<'a, S> {}

impl<'a, S> ReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates and returns a report immediately.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn generate(&self, request: ReportRequest) -> ReportGenerateCall<'a, S> {
        ReportGenerateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Polls for the status of a report request.
    /// 
    /// # Arguments
    ///
    /// * `reportId` - ID of the report request being polled.
    pub fn get(&self, report_id: &str) -> ReportGetCall<'a, S> {
        ReportGetCall {
            hub: self.hub,
            _report_id: report_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Downloads a report file encoded in UTF-8.
    /// 
    /// # Arguments
    ///
    /// * `reportId` - ID of the report.
    /// * `reportFragment` - The index of the report fragment to download.
    pub fn get_file(&self, report_id: &str, report_fragment: i32) -> ReportGetFileCall<'a, S> {
        ReportGetFileCall {
            hub: self.hub,
            _report_id: report_id.to_string(),
            _report_fragment: report_fragment,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Downloads a csv file(encoded in UTF-8) that contains ID mappings between legacy SA360 and new SA360. The file includes all children entities of the given advertiser(e.g. engine accounts, campaigns, ad groups, etc.) that exist in both legacy SA360 and new SA360.
    /// 
    /// # Arguments
    ///
    /// * `agencyId` - Legacy SA360 agency ID.
    /// * `advertiserId` - Legacy SA360 advertiser ID.
    pub fn get_id_mapping_file(&self, agency_id: i64, advertiser_id: i64) -> ReportGetIdMappingFileCall<'a, S> {
        ReportGetIdMappingFileCall {
            hub: self.hub,
            _agency_id: agency_id,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a report request into the reporting system.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn request(&self, request: ReportRequest) -> ReportRequestCall<'a, S> {
        ReportRequestCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *savedColumn* resources.
/// It is not used directly, but through the [`Doubleclicksearch`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_doubleclicksearch2 as doubleclicksearch2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use doubleclicksearch2::{Doubleclicksearch, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Doubleclicksearch::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.saved_columns();
/// # }
/// ```
pub struct SavedColumnMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Doubleclicksearch<S>,
}

impl<'a, S> client::MethodsBuilder for SavedColumnMethods<'a, S> {}

impl<'a, S> SavedColumnMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the list of saved columns for a specified advertiser.
    /// 
    /// # Arguments
    ///
    /// * `agencyId` - DS ID of the agency.
    /// * `advertiserId` - DS ID of the advertiser.
    pub fn list(&self, agency_id: i64, advertiser_id: i64) -> SavedColumnListCall<'a, S> {
        SavedColumnListCall {
            hub: self.hub,
            _agency_id: agency_id,
            _advertiser_id: advertiser_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



