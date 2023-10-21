use super::*;
/// A builder providing access to all methods supported on *spreadsheet* resources.
/// It is not used directly, but through the [`Sheets`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_sheets4 as sheets4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_update(...)`, `create(...)`, `developer_metadata_get(...)`, `developer_metadata_search(...)`, `get(...)`, `get_by_data_filter(...)`, `sheets_copy_to(...)`, `values_append(...)`, `values_batch_clear(...)`, `values_batch_clear_by_data_filter(...)`, `values_batch_get(...)`, `values_batch_get_by_data_filter(...)`, `values_batch_update(...)`, `values_batch_update_by_data_filter(...)`, `values_clear(...)`, `values_get(...)` and `values_update(...)`
/// // to build up your call.
/// let rb = hub.spreadsheets();
/// # }
/// ```
pub struct SpreadsheetMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Sheets<S>,
}

impl<'a, S> client::MethodsBuilder for SpreadsheetMethods<'a, S> {}

impl<'a, S> SpreadsheetMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the developer metadata with the specified ID. The caller must specify the spreadsheet ID and the developer metadata's unique metadataId.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve metadata from.
    /// * `metadataId` - The ID of the developer metadata to retrieve.
    pub fn developer_metadata_get(&self, spreadsheet_id: &str, metadata_id: i32) -> SpreadsheetDeveloperMetadataGetCall<'a, S> {
        SpreadsheetDeveloperMetadataGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _metadata_id: metadata_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all developer metadata matching the specified DataFilter. If the provided DataFilter represents a DeveloperMetadataLookup object, this will return all DeveloperMetadata entries selected by it. If the DataFilter represents a location in a spreadsheet, this will return all developer metadata associated with locations intersecting that region.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve metadata from.
    pub fn developer_metadata_search(&self, request: SearchDeveloperMetadataRequest, spreadsheet_id: &str) -> SpreadsheetDeveloperMetadataSearchCall<'a, S> {
        SpreadsheetDeveloperMetadataSearchCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet containing the sheet to copy.
    /// * `sheetId` - The ID of the sheet to copy.
    pub fn sheets_copy_to(&self, request: CopySheetToAnotherSpreadsheetRequest, spreadsheet_id: &str, sheet_id: i32) -> SpreadsheetSheetCopyToCall<'a, S> {
        SpreadsheetSheetCopyToCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _sheet_id: sheet_id,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Appends values to a spreadsheet. The input range is used to search for existing data and find a “table” within that range. Values will be appended to the next row of the table, starting with the first column of the table. See the [guide](https://developers.google.com/sheets/api/guides/values#appending_values) and [sample code](https://developers.google.com/sheets/api/samples/writing#append_values) for specific details of how tables are detected and data is appended. The caller must specify the spreadsheet ID, range, and a valueInputOption. The `valueInputOption` only controls how the input data will be added to the sheet (column-wise or row-wise), it does not influence what cell the data starts being written to.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    /// * `range` - The [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of a range to search for a logical table of data. Values are appended after the last row of the table.
    pub fn values_append(&self, request: ValueRange, spreadsheet_id: &str, range: &str) -> SpreadsheetValueAppendCall<'a, S> {
        SpreadsheetValueAppendCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _range: range.to_string(),
            _value_input_option: Default::default(),
            _response_value_render_option: Default::default(),
            _response_date_time_render_option: Default::default(),
            _insert_data_option: Default::default(),
            _include_values_in_response: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges. Only values are cleared -- all other properties of the cell (such as formatting and data validation) are kept.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_clear(&self, request: BatchClearValuesRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchClearCall<'a, S> {
        SpreadsheetValueBatchClearCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_clear_by_data_filter(&self, request: BatchClearValuesByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S> {
        SpreadsheetValueBatchClearByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve data from.
    pub fn values_batch_get(&self, spreadsheet_id: &str) -> SpreadsheetValueBatchGetCall<'a, S> {
        SpreadsheetValueBatchGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _value_render_option: Default::default(),
            _ranges: Default::default(),
            _major_dimension: Default::default(),
            _date_time_render_option: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns one or more ranges of values that match the specified data filters. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges that match any of the data filters in the request will be returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve data from.
    pub fn values_batch_get_by_data_filter(&self, request: BatchGetValuesByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S> {
        SpreadsheetValueBatchGetByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more ValueRanges.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_update(&self, request: BatchUpdateValuesRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchUpdateCall<'a, S> {
        SpreadsheetValueBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_update_by_data_filter(&self, request: BatchUpdateValuesByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S> {
        SpreadsheetValueBatchUpdateByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears values from a spreadsheet. The caller must specify the spreadsheet ID and range. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    /// * `range` - The [A1 notation or R1C1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of the values to clear.
    pub fn values_clear(&self, request: ClearValuesRequest, spreadsheet_id: &str, range: &str) -> SpreadsheetValueClearCall<'a, S> {
        SpreadsheetValueClearCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _range: range.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a range of values from a spreadsheet. The caller must specify the spreadsheet ID and a range.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve data from.
    /// * `range` - The [A1 notation or R1C1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of the range to retrieve values from.
    pub fn values_get(&self, spreadsheet_id: &str, range: &str) -> SpreadsheetValueGetCall<'a, S> {
        SpreadsheetValueGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _range: range.to_string(),
            _value_render_option: Default::default(),
            _major_dimension: Default::default(),
            _date_time_render_option: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets values in a range of a spreadsheet. The caller must specify the spreadsheet ID, range, and a valueInputOption.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    /// * `range` - The [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of the values to update.
    pub fn values_update(&self, request: ValueRange, spreadsheet_id: &str, range: &str) -> SpreadsheetValueUpdateCall<'a, S> {
        SpreadsheetValueUpdateCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _range: range.to_string(),
            _value_input_option: Default::default(),
            _response_value_render_option: Default::default(),
            _response_date_time_render_option: Default::default(),
            _include_values_in_response: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies one or more updates to the spreadsheet. Each request is validated before being applied. If any request is not valid then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. The replies will mirror the requests. For example, if you applied 4 updates and the 3rd one had a reply, then the response will have 2 empty replies, the actual reply, and another empty reply, in that order. Due to the collaborative nature of spreadsheets, it is not guaranteed that the spreadsheet will reflect exactly your changes after this completes, however it is guaranteed that the updates in the request will be applied together atomically. Your changes may be altered with respect to collaborator changes. If there are no collaborators, the spreadsheet should reflect your changes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The spreadsheet to apply the updates to.
    pub fn batch_update(&self, request: BatchUpdateSpreadsheetRequest, spreadsheet_id: &str) -> SpreadsheetBatchUpdateCall<'a, S> {
        SpreadsheetBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a spreadsheet, returning the newly created spreadsheet.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Spreadsheet) -> SpreadsheetCreateCall<'a, S> {
        SpreadsheetCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. By default, data within grids is not returned. You can include grid data in one of 2 ways: * Specify a [field mask](https://developers.google.com/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData URL parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, as a best practice, retrieve only the specific spreadsheet fields that you want. To retrieve only subsets of spreadsheet data, use the ranges URL parameter. Ranges are specified using [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell). You can define a single cell (for example, `A1`) or multiple cells (for example, `A1:D5`). You can also get cells from other sheets within the same spreadsheet (for example, `Sheet2!A1:C4`) or retrieve multiple ranges at once (for example, `?ranges=A1:D5&ranges=Sheet2!A1:C4`). Limiting the range returns only the portions of the spreadsheet that intersect the requested ranges.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The spreadsheet to request.
    pub fn get(&self, spreadsheet_id: &str) -> SpreadsheetGetCall<'a, S> {
        SpreadsheetGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _ranges: Default::default(),
            _include_grid_data: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. This method differs from GetSpreadsheet in that it allows selecting which subsets of spreadsheet data to return by specifying a dataFilters parameter. Multiple DataFilters can be specified. Specifying one or more data filters returns the portions of the spreadsheet that intersect ranges matched by any of the filters. By default, data within grids is not returned. You can include grid data one of 2 ways: * Specify a [field mask](https://developers.google.com/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, as a best practice, retrieve only the specific spreadsheet fields that you want.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The spreadsheet to request.
    pub fn get_by_data_filter(&self, request: GetSpreadsheetByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetGetByDataFilterCall<'a, S> {
        SpreadsheetGetByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



