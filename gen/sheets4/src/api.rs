use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// See, edit, create, and delete all of your Google Drive files
    Drive,

    /// See, edit, create, and delete only the specific Google Drive files you use with this app
    DriveFile,

    /// See and download all your Google Drive files
    DriveReadonly,

    /// See, edit, create, and delete all your Google Sheets spreadsheets
    Spreadsheet,

    /// See all your Google Sheets spreadsheets
    SpreadsheetReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::Spreadsheet => "https://www.googleapis.com/auth/spreadsheets",
            Scope::SpreadsheetReadonly => "https://www.googleapis.com/auth/spreadsheets.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::DriveReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Sheets related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_sheets4 as sheets4;
/// use sheets4::api::ValueRange;
/// use sheets4::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ValueRange::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
///              .value_input_option("dolor")
///              .response_value_render_option("ea")
///              .response_date_time_render_option("ipsum")
///              .insert_data_option("invidunt")
///              .include_values_in_response(true)
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
#[derive(Clone)]
pub struct Sheets<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Sheets<S> {}

impl<'a, S> Sheets<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Sheets<S> {
        Sheets {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://sheets.googleapis.com/".to_string(),
            _root_url: "https://sheets.googleapis.com/".to_string(),
        }
    }

    pub fn spreadsheets(&'a self) -> SpreadsheetMethods<'a, S> {
        SpreadsheetMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://sheets.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://sheets.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Adds a new banded range to the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddBandingRequest {
    /// The banded range to add. The bandedRangeId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a range that already exists.)
    #[serde(rename="bandedRange")]
    
    pub banded_range: Option<BandedRange>,
}

impl client::Part for AddBandingRequest {}


/// The result of adding a banded range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddBandingResponse {
    /// The banded range that was added.
    #[serde(rename="bandedRange")]
    
    pub banded_range: Option<BandedRange>,
}

impl client::Part for AddBandingResponse {}


/// Adds a chart to a sheet in the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddChartRequest {
    /// The chart that should be added to the spreadsheet, including the position where it should be placed. The chartId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of an embedded object that already exists.)
    
    pub chart: Option<EmbeddedChart>,
}

impl client::Part for AddChartRequest {}


/// The result of adding a chart to a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddChartResponse {
    /// The newly added chart.
    
    pub chart: Option<EmbeddedChart>,
}

impl client::Part for AddChartResponse {}


/// Adds a new conditional format rule at the given index. All subsequent rules' indexes are incremented.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddConditionalFormatRuleRequest {
    /// The zero-based index where the rule should be inserted.
    
    pub index: Option<i32>,
    /// The rule to add.
    
    pub rule: Option<ConditionalFormatRule>,
}

impl client::Part for AddConditionalFormatRuleRequest {}


/// Adds a data source. After the data source is added successfully, an associated DATA_SOURCE sheet is created and an execution is triggered to refresh the sheet to read data from the data source. The request requires an additional `bigquery.readonly` OAuth scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddDataSourceRequest {
    /// The data source to add.
    #[serde(rename="dataSource")]
    
    pub data_source: Option<DataSource>,
}

impl client::Part for AddDataSourceRequest {}


/// The result of adding a data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddDataSourceResponse {
    /// The data execution status.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// The data source that was created.
    #[serde(rename="dataSource")]
    
    pub data_source: Option<DataSource>,
}

impl client::Part for AddDataSourceResponse {}


/// Creates a group over the specified range. If the requested range is a superset of the range of an existing group G, then the depth of G is incremented and this new group G' has the depth of that group. For example, a group [C:D, depth 1] + [B:E] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range is a subset of the range of an existing group G, then the depth of the new group G' becomes one greater than the depth of G. For example, a group [B:E, depth 1] + [C:D] results in groups [B:E, depth 1] and [C:D, depth 2]. If the requested range starts before and ends within, or starts within and ends after, the range of an existing group G, then the range of the existing group G becomes the union of the ranges, and the new group G' has depth one greater than the depth of G and range as the intersection of the ranges. For example, a group [B:D, depth 1] + [C:E] results in groups [B:E, depth 1] and [C:D, depth 2].
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddDimensionGroupRequest {
    /// The range over which to create a group.
    
    pub range: Option<DimensionRange>,
}

impl client::Part for AddDimensionGroupRequest {}


/// The result of adding a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddDimensionGroupResponse {
    /// All groups of a dimension after adding a group to that dimension.
    #[serde(rename="dimensionGroups")]
    
    pub dimension_groups: Option<Vec<DimensionGroup>>,
}

impl client::Part for AddDimensionGroupResponse {}


/// Adds a filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddFilterViewRequest {
    /// The filter to add. The filterViewId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a filter that already exists.)
    
    pub filter: Option<FilterView>,
}

impl client::Part for AddFilterViewRequest {}


/// The result of adding a filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddFilterViewResponse {
    /// The newly added filter view.
    
    pub filter: Option<FilterView>,
}

impl client::Part for AddFilterViewResponse {}


/// Adds a named range to the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddNamedRangeRequest {
    /// The named range to add. The namedRangeId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a range that already exists.)
    #[serde(rename="namedRange")]
    
    pub named_range: Option<NamedRange>,
}

impl client::Part for AddNamedRangeRequest {}


/// The result of adding a named range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddNamedRangeResponse {
    /// The named range to add.
    #[serde(rename="namedRange")]
    
    pub named_range: Option<NamedRange>,
}

impl client::Part for AddNamedRangeResponse {}


/// Adds a new protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddProtectedRangeRequest {
    /// The protected range to be added. The protectedRangeId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a range that already exists.)
    #[serde(rename="protectedRange")]
    
    pub protected_range: Option<ProtectedRange>,
}

impl client::Part for AddProtectedRangeRequest {}


/// The result of adding a new protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddProtectedRangeResponse {
    /// The newly added protected range.
    #[serde(rename="protectedRange")]
    
    pub protected_range: Option<ProtectedRange>,
}

impl client::Part for AddProtectedRangeResponse {}


/// Adds a new sheet. When a sheet is added at a given index, all subsequent sheets' indexes are incremented. To add an object sheet, use AddChartRequest instead and specify EmbeddedObjectPosition.sheetId or EmbeddedObjectPosition.newSheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSheetRequest {
    /// The properties the new sheet should have. All properties are optional. The sheetId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a sheet that already exists.)
    
    pub properties: Option<SheetProperties>,
}

impl client::Part for AddSheetRequest {}


/// The result of adding a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSheetResponse {
    /// The properties of the newly added sheet.
    
    pub properties: Option<SheetProperties>,
}

impl client::Part for AddSheetResponse {}


/// Adds a slicer to a sheet in the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSlicerRequest {
    /// The slicer that should be added to the spreadsheet, including the position where it should be placed. The slicerId field is optional; if one is not set, an id will be randomly generated. (It is an error to specify the ID of a slicer that already exists.)
    
    pub slicer: Option<Slicer>,
}

impl client::Part for AddSlicerRequest {}


/// The result of adding a slicer to a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSlicerResponse {
    /// The newly added slicer.
    
    pub slicer: Option<Slicer>,
}

impl client::Part for AddSlicerResponse {}


/// Adds new cells after the last row with data in a sheet, inserting new rows into the sheet if necessary.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppendCellsRequest {
    /// The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The data to append.
    
    pub rows: Option<Vec<RowData>>,
    /// The sheet ID to append the data to.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for AppendCellsRequest {}


/// Appends rows or columns to the end of a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppendDimensionRequest {
    /// Whether rows or columns should be appended.
    
    pub dimension: Option<String>,
    /// The number of rows or columns to append.
    
    pub length: Option<i32>,
    /// The sheet to append rows or columns to.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for AppendDimensionRequest {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values append spreadsheets](SpreadsheetValueAppendCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppendValuesResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The range (in A1 notation) of the table that values are being appended to (before the values were appended). Empty if no table was found.
    #[serde(rename="tableRange")]
    
    pub table_range: Option<String>,
    /// Information about the updates that were applied.
    
    pub updates: Option<UpdateValuesResponse>,
}

impl client::ResponseResult for AppendValuesResponse {}


/// Fills in more data based on existing data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoFillRequest {
    /// The range to autofill. This will examine the range and detect the location that has data and automatically fill that data in to the rest of the range.
    
    pub range: Option<GridRange>,
    /// The source and destination areas to autofill. This explicitly lists the source of the autofill and where to extend that data.
    #[serde(rename="sourceAndDestination")]
    
    pub source_and_destination: Option<SourceAndDestination>,
    /// True if we should generate data with the "alternate" series. This differs based on the type and amount of source data.
    #[serde(rename="useAlternateSeries")]
    
    pub use_alternate_series: Option<bool>,
}

impl client::Part for AutoFillRequest {}


/// Automatically resizes one or more dimensions based on the contents of the cells in that dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoResizeDimensionsRequest {
    /// The dimensions on a data source sheet to automatically resize.
    #[serde(rename="dataSourceSheetDimensions")]
    
    pub data_source_sheet_dimensions: Option<DataSourceSheetDimensionRange>,
    /// The dimensions to automatically resize.
    
    pub dimensions: Option<DimensionRange>,
}

impl client::Part for AutoResizeDimensionsRequest {}


/// A banded (alternating colors) range in a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BandedRange {
    /// The ID of the banded range.
    #[serde(rename="bandedRangeId")]
    
    pub banded_range_id: Option<i32>,
    /// Properties for column bands. These properties are applied on a column- by-column basis throughout all the columns in the range. At least one of row_properties or column_properties must be specified.
    #[serde(rename="columnProperties")]
    
    pub column_properties: Option<BandingProperties>,
    /// The range over which these properties are applied.
    
    pub range: Option<GridRange>,
    /// Properties for row bands. These properties are applied on a row-by-row basis throughout all the rows in the range. At least one of row_properties or column_properties must be specified.
    #[serde(rename="rowProperties")]
    
    pub row_properties: Option<BandingProperties>,
}

impl client::Part for BandedRange {}


/// Properties referring a single dimension (either row or column). If both BandedRange.row_properties and BandedRange.column_properties are set, the fill colors are applied to cells according to the following rules: * header_color and footer_color take priority over band colors. * first_band_color takes priority over second_band_color. * row_properties takes priority over column_properties. For example, the first row color takes priority over the first column color, but the first column color takes priority over the second row color. Similarly, the row header takes priority over the column header in the top left cell, but the column header takes priority over the first row color if the row header is not set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BandingProperties {
    /// The first color that is alternating. (Required) Deprecated: Use first_band_color_style.
    #[serde(rename="firstBandColor")]
    
    pub first_band_color: Option<Color>,
    /// The first color that is alternating. (Required) If first_band_color is also set, this field takes precedence.
    #[serde(rename="firstBandColorStyle")]
    
    pub first_band_color_style: Option<ColorStyle>,
    /// The color of the last row or column. If this field is not set, the last row or column is filled with either first_band_color or second_band_color, depending on the color of the previous row or column. Deprecated: Use footer_color_style.
    #[serde(rename="footerColor")]
    
    pub footer_color: Option<Color>,
    /// The color of the last row or column. If this field is not set, the last row or column is filled with either first_band_color or second_band_color, depending on the color of the previous row or column. If footer_color is also set, this field takes precedence.
    #[serde(rename="footerColorStyle")]
    
    pub footer_color_style: Option<ColorStyle>,
    /// The color of the first row or column. If this field is set, the first row or column is filled with this color and the colors alternate between first_band_color and second_band_color starting from the second row or column. Otherwise, the first row or column is filled with first_band_color and the colors proceed to alternate as they normally would. Deprecated: Use header_color_style.
    #[serde(rename="headerColor")]
    
    pub header_color: Option<Color>,
    /// The color of the first row or column. If this field is set, the first row or column is filled with this color and the colors alternate between first_band_color and second_band_color starting from the second row or column. Otherwise, the first row or column is filled with first_band_color and the colors proceed to alternate as they normally would. If header_color is also set, this field takes precedence.
    #[serde(rename="headerColorStyle")]
    
    pub header_color_style: Option<ColorStyle>,
    /// The second color that is alternating. (Required) Deprecated: Use second_band_color_style.
    #[serde(rename="secondBandColor")]
    
    pub second_band_color: Option<Color>,
    /// The second color that is alternating. (Required) If second_band_color is also set, this field takes precedence.
    #[serde(rename="secondBandColorStyle")]
    
    pub second_band_color_style: Option<ColorStyle>,
}

impl client::Part for BandingProperties {}


/// Formatting options for baseline value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BaselineValueFormat {
    /// The comparison type of key value with baseline value.
    #[serde(rename="comparisonType")]
    
    pub comparison_type: Option<String>,
    /// Description which is appended after the baseline value. This field is optional.
    
    pub description: Option<String>,
    /// Color to be used, in case baseline value represents a negative change for key value. This field is optional. Deprecated: Use negative_color_style.
    #[serde(rename="negativeColor")]
    
    pub negative_color: Option<Color>,
    /// Color to be used, in case baseline value represents a negative change for key value. This field is optional. If negative_color is also set, this field takes precedence.
    #[serde(rename="negativeColorStyle")]
    
    pub negative_color_style: Option<ColorStyle>,
    /// Specifies the horizontal text positioning of baseline value. This field is optional. If not specified, default positioning is used.
    
    pub position: Option<TextPosition>,
    /// Color to be used, in case baseline value represents a positive change for key value. This field is optional. Deprecated: Use positive_color_style.
    #[serde(rename="positiveColor")]
    
    pub positive_color: Option<Color>,
    /// Color to be used, in case baseline value represents a positive change for key value. This field is optional. If positive_color is also set, this field takes precedence.
    #[serde(rename="positiveColorStyle")]
    
    pub positive_color_style: Option<ColorStyle>,
    /// Text formatting options for baseline value. The link field is not supported.
    #[serde(rename="textFormat")]
    
    pub text_format: Option<TextFormat>,
}

impl client::Part for BaselineValueFormat {}


/// An axis of the chart. A chart may not have more than one axis per axis position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartAxis {
    /// The format of the title. Only valid if the axis is not associated with the domain. The link field is not supported.
    
    pub format: Option<TextFormat>,
    /// The position of this axis.
    
    pub position: Option<String>,
    /// The title of this axis. If set, this overrides any title inferred from headers of the data.
    
    pub title: Option<String>,
    /// The axis title text position.
    #[serde(rename="titleTextPosition")]
    
    pub title_text_position: Option<TextPosition>,
    /// The view window options for this axis.
    #[serde(rename="viewWindowOptions")]
    
    pub view_window_options: Option<ChartAxisViewWindowOptions>,
}

impl client::Part for BasicChartAxis {}


/// The domain of a chart. For example, if charting stock prices over time, this would be the date.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartDomain {
    /// The data of the domain. For example, if charting stock prices over time, this is the data representing the dates.
    
    pub domain: Option<ChartData>,
    /// True to reverse the order of the domain values (horizontal axis).
    
    pub reversed: Option<bool>,
}

impl client::Part for BasicChartDomain {}


/// A single series of data in a chart. For example, if charting stock prices over time, multiple series may exist, one for the "Open Price", "High Price", "Low Price" and "Close Price".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartSeries {
    /// The color for elements (such as bars, lines, and points) associated with this series. If empty, a default color is used. Deprecated: Use color_style.
    
    pub color: Option<Color>,
    /// The color for elements (such as bars, lines, and points) associated with this series. If empty, a default color is used. If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    
    pub color_style: Option<ColorStyle>,
    /// Information about the data labels for this series.
    #[serde(rename="dataLabel")]
    
    pub data_label: Option<DataLabel>,
    /// The line style of this series. Valid only if the chartType is AREA, LINE, or SCATTER. COMBO charts are also supported if the series chart type is AREA or LINE.
    #[serde(rename="lineStyle")]
    
    pub line_style: Option<LineStyle>,
    /// The style for points associated with this series. Valid only if the chartType is AREA, LINE, or SCATTER. COMBO charts are also supported if the series chart type is AREA, LINE, or SCATTER. If empty, a default point style is used.
    #[serde(rename="pointStyle")]
    
    pub point_style: Option<PointStyle>,
    /// The data being visualized in this chart series.
    
    pub series: Option<ChartData>,
    /// Style override settings for series data points.
    #[serde(rename="styleOverrides")]
    
    pub style_overrides: Option<Vec<BasicSeriesDataPointStyleOverride>>,
    /// The minor axis that will specify the range of values for this series. For example, if charting stocks over time, the "Volume" series may want to be pinned to the right with the prices pinned to the left, because the scale of trading volume is different than the scale of prices. It is an error to specify an axis that isn't a valid minor axis for the chart's type.
    #[serde(rename="targetAxis")]
    
    pub target_axis: Option<String>,
    /// The type of this series. Valid only if the chartType is COMBO. Different types will change the way the series is visualized. Only LINE, AREA, and COLUMN are supported.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for BasicChartSeries {}


/// The specification for a basic chart. See BasicChartType for the list of charts this supports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartSpec {
    /// The axis on the chart.
    
    pub axis: Option<Vec<BasicChartAxis>>,
    /// The type of the chart.
    #[serde(rename="chartType")]
    
    pub chart_type: Option<String>,
    /// The behavior of tooltips and data highlighting when hovering on data and chart area.
    #[serde(rename="compareMode")]
    
    pub compare_mode: Option<String>,
    /// The domain of data this is charting. Only a single domain is supported.
    
    pub domains: Option<Vec<BasicChartDomain>>,
    /// The number of rows or columns in the data that are "headers". If not set, Google Sheets will guess how many rows are headers based on the data. (Note that BasicChartAxis.title may override the axis title inferred from the header values.)
    #[serde(rename="headerCount")]
    
    pub header_count: Option<i32>,
    /// If some values in a series are missing, gaps may appear in the chart (e.g, segments of lines in a line chart will be missing). To eliminate these gaps set this to true. Applies to Line, Area, and Combo charts.
    #[serde(rename="interpolateNulls")]
    
    pub interpolate_nulls: Option<bool>,
    /// The position of the chart legend.
    #[serde(rename="legendPosition")]
    
    pub legend_position: Option<String>,
    /// Gets whether all lines should be rendered smooth or straight by default. Applies to Line charts.
    #[serde(rename="lineSmoothing")]
    
    pub line_smoothing: Option<bool>,
    /// The data this chart is visualizing.
    
    pub series: Option<Vec<BasicChartSeries>>,
    /// The stacked type for charts that support vertical stacking. Applies to Area, Bar, Column, Combo, and Stepped Area charts.
    #[serde(rename="stackedType")]
    
    pub stacked_type: Option<String>,
    /// True to make the chart 3D. Applies to Bar and Column charts.
    #[serde(rename="threeDimensional")]
    
    pub three_dimensional: Option<bool>,
    /// Controls whether to display additional data labels on stacked charts which sum the total value of all stacked values at each value along the domain axis. These data labels can only be set when chart_type is one of AREA, BAR, COLUMN, COMBO or STEPPED_AREA and stacked_type is either STACKED or PERCENT_STACKED. In addition, for COMBO, this will only be supported if there is only one type of stackable series type or one type has more series than the others and each of the other types have no more than one series. For example, if a chart has two stacked bar series and one area series, the total data labels will be supported. If it has three bar series and two area series, total data labels are not allowed. Neither CUSTOM nor placement can be set on the total_data_label.
    #[serde(rename="totalDataLabel")]
    
    pub total_data_label: Option<DataLabel>,
}

impl client::Part for BasicChartSpec {}


/// The default filter associated with a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicFilter {
    /// The criteria for showing/hiding values per column. The map's key is the column index, and the value is the criteria for that column. This field is deprecated in favor of filter_specs.
    
    pub criteria: Option<HashMap<String, FilterCriteria>>,
    /// The filter criteria per column. Both criteria and filter_specs are populated in responses. If both fields are specified in an update request, this field takes precedence.
    #[serde(rename="filterSpecs")]
    
    pub filter_specs: Option<Vec<FilterSpec>>,
    /// The range the filter covers.
    
    pub range: Option<GridRange>,
    /// The sort order per column. Later specifications are used when values are equal in the earlier specifications.
    #[serde(rename="sortSpecs")]
    
    pub sort_specs: Option<Vec<SortSpec>>,
}

impl client::Part for BasicFilter {}


/// Style override settings for a single series data point.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicSeriesDataPointStyleOverride {
    /// Color of the series data point. If empty, the series default is used. Deprecated: Use color_style.
    
    pub color: Option<Color>,
    /// Color of the series data point. If empty, the series default is used. If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    
    pub color_style: Option<ColorStyle>,
    /// The zero-based index of the series data point.
    
    pub index: Option<i32>,
    /// Point style of the series data point. Valid only if the chartType is AREA, LINE, or SCATTER. COMBO charts are also supported if the series chart type is AREA, LINE, or SCATTER. If empty, the series default is used.
    #[serde(rename="pointStyle")]
    
    pub point_style: Option<PointStyle>,
}

impl client::Part for BasicSeriesDataPointStyleOverride {}


/// The request for clearing more than one range selected by a DataFilter in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear by data filter spreadsheets](SpreadsheetValueBatchClearByDataFilterCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesByDataFilterRequest {
    /// The DataFilters used to determine which ranges to clear.
    #[serde(rename="dataFilters")]
    
    pub data_filters: Option<Vec<DataFilter>>,
}

impl client::RequestValue for BatchClearValuesByDataFilterRequest {}


/// The response when clearing a range of values selected with DataFilters in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear by data filter spreadsheets](SpreadsheetValueBatchClearByDataFilterCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesByDataFilterResponse {
    /// The ranges that were cleared, in [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell). If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet’s limits.
    #[serde(rename="clearedRanges")]
    
    pub cleared_ranges: Option<Vec<String>>,
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
}

impl client::ResponseResult for BatchClearValuesByDataFilterResponse {}


/// The request for clearing more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear spreadsheets](SpreadsheetValueBatchClearCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesRequest {
    /// The ranges to clear, in [A1 notation or R1C1 notation](https://developers.google.com/sheets/api/guides/concepts#cell).
    
    pub ranges: Option<Vec<String>>,
}

impl client::RequestValue for BatchClearValuesRequest {}


/// The response when clearing a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear spreadsheets](SpreadsheetValueBatchClearCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesResponse {
    /// The ranges that were cleared, in A1 notation. If the requests are for an unbounded range or a ranger larger than the bounds of the sheet, this is the actual ranges that were cleared, bounded to the sheet's limits.
    #[serde(rename="clearedRanges")]
    
    pub cleared_ranges: Option<Vec<String>>,
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
}

impl client::ResponseResult for BatchClearValuesResponse {}


/// The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch get by data filter spreadsheets](SpreadsheetValueBatchGetByDataFilterCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetValuesByDataFilterRequest {
    /// The data filters used to match the ranges of values to retrieve. Ranges that match any of the specified data filters are included in the response.
    #[serde(rename="dataFilters")]
    
    pub data_filters: Option<Vec<DataFilter>>,
    /// How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    #[serde(rename="dateTimeRenderOption")]
    
    pub date_time_render_option: Option<String>,
    /// The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then a request that selects that range and sets `majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas a request that sets `majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
    #[serde(rename="majorDimension")]
    
    pub major_dimension: Option<String>,
    /// How values should be represented in the output. The default render option is FORMATTED_VALUE.
    #[serde(rename="valueRenderOption")]
    
    pub value_render_option: Option<String>,
}

impl client::RequestValue for BatchGetValuesByDataFilterRequest {}


/// The response when retrieving more than one range of values in a spreadsheet selected by DataFilters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch get by data filter spreadsheets](SpreadsheetValueBatchGetByDataFilterCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetValuesByDataFilterResponse {
    /// The ID of the spreadsheet the data was retrieved from.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The requested values with the list of data filters that matched them.
    #[serde(rename="valueRanges")]
    
    pub value_ranges: Option<Vec<MatchedValueRange>>,
}

impl client::ResponseResult for BatchGetValuesByDataFilterResponse {}


/// The response when retrieving more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch get spreadsheets](SpreadsheetValueBatchGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetValuesResponse {
    /// The ID of the spreadsheet the data was retrieved from.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The requested values. The order of the ValueRanges is the same as the order of the requested ranges.
    #[serde(rename="valueRanges")]
    
    pub value_ranges: Option<Vec<ValueRange>>,
}

impl client::ResponseResult for BatchGetValuesResponse {}


/// The request for updating any aspect of a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update spreadsheets](SpreadsheetBatchUpdateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateSpreadsheetRequest {
    /// Determines if the update response should include the spreadsheet resource.
    #[serde(rename="includeSpreadsheetInResponse")]
    
    pub include_spreadsheet_in_response: Option<bool>,
    /// A list of updates to apply to the spreadsheet. Requests will be applied in the order they are specified. If any request is not valid, no requests will be applied.
    
    pub requests: Option<Vec<Request>>,
    /// True if grid data should be returned. Meaningful only if include_spreadsheet_in_response is 'true'. This parameter is ignored if a field mask was set in the request.
    #[serde(rename="responseIncludeGridData")]
    
    pub response_include_grid_data: Option<bool>,
    /// Limits the ranges included in the response spreadsheet. Meaningful only if include_spreadsheet_in_response is 'true'.
    #[serde(rename="responseRanges")]
    
    pub response_ranges: Option<Vec<String>>,
}

impl client::RequestValue for BatchUpdateSpreadsheetRequest {}


/// The reply for batch updating a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update spreadsheets](SpreadsheetBatchUpdateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateSpreadsheetResponse {
    /// The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty.
    
    pub replies: Option<Vec<Response>>,
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The spreadsheet after updates were applied. This is only set if BatchUpdateSpreadsheetRequest.include_spreadsheet_in_response is `true`.
    #[serde(rename="updatedSpreadsheet")]
    
    pub updated_spreadsheet: Option<Spreadsheet>,
}

impl client::ResponseResult for BatchUpdateSpreadsheetResponse {}


/// The request for updating more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update by data filter spreadsheets](SpreadsheetValueBatchUpdateByDataFilterCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesByDataFilterRequest {
    /// The new values to apply to the spreadsheet. If more than one range is matched by the specified DataFilter the specified values are applied to all of those ranges.
    
    pub data: Option<Vec<DataFilterValueRange>>,
    /// Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The `updatedData` field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).
    #[serde(rename="includeValuesInResponse")]
    
    pub include_values_in_response: Option<bool>,
    /// Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    #[serde(rename="responseDateTimeRenderOption")]
    
    pub response_date_time_render_option: Option<String>,
    /// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
    #[serde(rename="responseValueRenderOption")]
    
    pub response_value_render_option: Option<String>,
    /// How the input data should be interpreted.
    #[serde(rename="valueInputOption")]
    
    pub value_input_option: Option<String>,
}

impl client::RequestValue for BatchUpdateValuesByDataFilterRequest {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update by data filter spreadsheets](SpreadsheetValueBatchUpdateByDataFilterCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesByDataFilterResponse {
    /// The response for each range updated.
    
    pub responses: Option<Vec<UpdateValuesByDataFilterResponse>>,
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The total number of cells updated.
    #[serde(rename="totalUpdatedCells")]
    
    pub total_updated_cells: Option<i32>,
    /// The total number of columns where at least one cell in the column was updated.
    #[serde(rename="totalUpdatedColumns")]
    
    pub total_updated_columns: Option<i32>,
    /// The total number of rows where at least one cell in the row was updated.
    #[serde(rename="totalUpdatedRows")]
    
    pub total_updated_rows: Option<i32>,
    /// The total number of sheets where at least one cell in the sheet was updated.
    #[serde(rename="totalUpdatedSheets")]
    
    pub total_updated_sheets: Option<i32>,
}

impl client::ResponseResult for BatchUpdateValuesByDataFilterResponse {}


/// The request for updating more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update spreadsheets](SpreadsheetValueBatchUpdateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesRequest {
    /// The new values to apply to the spreadsheet.
    
    pub data: Option<Vec<ValueRange>>,
    /// Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The `updatedData` field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).
    #[serde(rename="includeValuesInResponse")]
    
    pub include_values_in_response: Option<bool>,
    /// Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    #[serde(rename="responseDateTimeRenderOption")]
    
    pub response_date_time_render_option: Option<String>,
    /// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
    #[serde(rename="responseValueRenderOption")]
    
    pub response_value_render_option: Option<String>,
    /// How the input data should be interpreted.
    #[serde(rename="valueInputOption")]
    
    pub value_input_option: Option<String>,
}

impl client::RequestValue for BatchUpdateValuesRequest {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update spreadsheets](SpreadsheetValueBatchUpdateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesResponse {
    /// One UpdateValuesResponse per requested range, in the same order as the requests appeared.
    
    pub responses: Option<Vec<UpdateValuesResponse>>,
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The total number of cells updated.
    #[serde(rename="totalUpdatedCells")]
    
    pub total_updated_cells: Option<i32>,
    /// The total number of columns where at least one cell in the column was updated.
    #[serde(rename="totalUpdatedColumns")]
    
    pub total_updated_columns: Option<i32>,
    /// The total number of rows where at least one cell in the row was updated.
    #[serde(rename="totalUpdatedRows")]
    
    pub total_updated_rows: Option<i32>,
    /// The total number of sheets where at least one cell in the sheet was updated.
    #[serde(rename="totalUpdatedSheets")]
    
    pub total_updated_sheets: Option<i32>,
}

impl client::ResponseResult for BatchUpdateValuesResponse {}


/// The specification of a BigQuery data source that's connected to a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryDataSourceSpec {
    /// The ID of a BigQuery enabled Google Cloud project with a billing account attached. For any queries executed against the data source, the project is charged.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// A BigQueryQuerySpec.
    #[serde(rename="querySpec")]
    
    pub query_spec: Option<BigQueryQuerySpec>,
    /// A BigQueryTableSpec.
    #[serde(rename="tableSpec")]
    
    pub table_spec: Option<BigQueryTableSpec>,
}

impl client::Part for BigQueryDataSourceSpec {}


/// Specifies a custom BigQuery query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryQuerySpec {
    /// The raw query string.
    #[serde(rename="rawQuery")]
    
    pub raw_query: Option<String>,
}

impl client::Part for BigQueryQuerySpec {}


/// Specifies a BigQuery table definition. Only [native tables](https://cloud.google.com/bigquery/docs/tables-intro) are allowed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryTableSpec {
    /// The BigQuery dataset id.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// The BigQuery table id.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
    /// The ID of a BigQuery project the table belongs to. If not specified, the project_id is assumed.
    #[serde(rename="tableProjectId")]
    
    pub table_project_id: Option<String>,
}

impl client::Part for BigQueryTableSpec {}


/// A condition that can evaluate to true or false. BooleanConditions are used by conditional formatting, data validation, and the criteria in filters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooleanCondition {
    /// The type of condition.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The values of the condition. The number of supported values depends on the condition type. Some support zero values, others one or two values, and ConditionType.ONE_OF_LIST supports an arbitrary number of values.
    
    pub values: Option<Vec<ConditionValue>>,
}

impl client::Part for BooleanCondition {}


/// A rule that may or may not match, depending on the condition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooleanRule {
    /// The condition of the rule. If the condition evaluates to true, the format is applied.
    
    pub condition: Option<BooleanCondition>,
    /// The format to apply. Conditional formatting can only apply a subset of formatting: bold, italic, strikethrough, foreground color and, background color.
    
    pub format: Option<CellFormat>,
}

impl client::Part for BooleanRule {}


/// A border along a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Border {
    /// The color of the border. Deprecated: Use color_style.
    
    pub color: Option<Color>,
    /// The color of the border. If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    
    pub color_style: Option<ColorStyle>,
    /// The style of the border.
    
    pub style: Option<String>,
    /// The width of the border, in pixels. Deprecated; the width is determined by the "style" field.
    
    pub width: Option<i32>,
}

impl client::Part for Border {}


/// The borders of the cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Borders {
    /// The bottom border of the cell.
    
    pub bottom: Option<Border>,
    /// The left border of the cell.
    
    pub left: Option<Border>,
    /// The right border of the cell.
    
    pub right: Option<Border>,
    /// The top border of the cell.
    
    pub top: Option<Border>,
}

impl client::Part for Borders {}


/// A bubble chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BubbleChartSpec {
    /// The bubble border color. Deprecated: Use bubble_border_color_style.
    #[serde(rename="bubbleBorderColor")]
    
    pub bubble_border_color: Option<Color>,
    /// The bubble border color. If bubble_border_color is also set, this field takes precedence.
    #[serde(rename="bubbleBorderColorStyle")]
    
    pub bubble_border_color_style: Option<ColorStyle>,
    /// The data containing the bubble labels. These do not need to be unique.
    #[serde(rename="bubbleLabels")]
    
    pub bubble_labels: Option<ChartData>,
    /// The max radius size of the bubbles, in pixels. If specified, the field must be a positive value.
    #[serde(rename="bubbleMaxRadiusSize")]
    
    pub bubble_max_radius_size: Option<i32>,
    /// The minimum radius size of the bubbles, in pixels. If specific, the field must be a positive value.
    #[serde(rename="bubbleMinRadiusSize")]
    
    pub bubble_min_radius_size: Option<i32>,
    /// The opacity of the bubbles between 0 and 1.0. 0 is fully transparent and 1 is fully opaque.
    #[serde(rename="bubbleOpacity")]
    
    pub bubble_opacity: Option<f32>,
    /// The data containing the bubble sizes. Bubble sizes are used to draw the bubbles at different sizes relative to each other. If specified, group_ids must also be specified. This field is optional.
    #[serde(rename="bubbleSizes")]
    
    pub bubble_sizes: Option<ChartData>,
    /// The format of the text inside the bubbles. Strikethrough, underline, and link are not supported.
    #[serde(rename="bubbleTextStyle")]
    
    pub bubble_text_style: Option<TextFormat>,
    /// The data containing the bubble x-values. These values locate the bubbles in the chart horizontally.
    
    pub domain: Option<ChartData>,
    /// The data containing the bubble group IDs. All bubbles with the same group ID are drawn in the same color. If bubble_sizes is specified then this field must also be specified but may contain blank values. This field is optional.
    #[serde(rename="groupIds")]
    
    pub group_ids: Option<ChartData>,
    /// Where the legend of the chart should be drawn.
    #[serde(rename="legendPosition")]
    
    pub legend_position: Option<String>,
    /// The data containing the bubble y-values. These values locate the bubbles in the chart vertically.
    
    pub series: Option<ChartData>,
}

impl client::Part for BubbleChartSpec {}


/// Cancels one or multiple refreshes of data source objects in the spreadsheet by the specified references.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelDataSourceRefreshRequest {
    /// Reference to a DataSource. If specified, cancels all associated data source object refreshes for this data source.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// Cancels all existing data source object refreshes for all data sources in the spreadsheet.
    #[serde(rename="isAll")]
    
    pub is_all: Option<bool>,
    /// References to data source objects whose refreshes are to be cancelled.
    
    pub references: Option<DataSourceObjectReferences>,
}

impl client::Part for CancelDataSourceRefreshRequest {}


/// The response from cancelling one or multiple data source object refreshes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelDataSourceRefreshResponse {
    /// The cancellation statuses of refreshes of all data source objects specified in the request. If is_all is specified, the field contains only those in failure status. Refreshing and canceling refresh the same data source object is also not allowed in the same `batchUpdate`.
    
    pub statuses: Option<Vec<CancelDataSourceRefreshStatus>>,
}

impl client::Part for CancelDataSourceRefreshResponse {}


/// The status of cancelling a single data source object refresh.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelDataSourceRefreshStatus {
    /// Reference to the data source object whose refresh is being cancelled.
    
    pub reference: Option<DataSourceObjectReference>,
    /// The cancellation status.
    #[serde(rename="refreshCancellationStatus")]
    
    pub refresh_cancellation_status: Option<RefreshCancellationStatus>,
}

impl client::Part for CancelDataSourceRefreshStatus {}


/// A candlestick chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickChartSpec {
    /// The Candlestick chart data. Only one CandlestickData is supported.
    
    pub data: Option<Vec<CandlestickData>>,
    /// The domain data (horizontal axis) for the candlestick chart. String data will be treated as discrete labels, other data will be treated as continuous values.
    
    pub domain: Option<CandlestickDomain>,
}

impl client::Part for CandlestickChartSpec {}


/// The Candlestick chart data, each containing the low, open, close, and high values for a series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickData {
    /// The range data (vertical axis) for the close/final value for each candle. This is the top of the candle body. If greater than the open value the candle will be filled. Otherwise the candle will be hollow.
    #[serde(rename="closeSeries")]
    
    pub close_series: Option<CandlestickSeries>,
    /// The range data (vertical axis) for the high/maximum value for each candle. This is the top of the candle's center line.
    #[serde(rename="highSeries")]
    
    pub high_series: Option<CandlestickSeries>,
    /// The range data (vertical axis) for the low/minimum value for each candle. This is the bottom of the candle's center line.
    #[serde(rename="lowSeries")]
    
    pub low_series: Option<CandlestickSeries>,
    /// The range data (vertical axis) for the open/initial value for each candle. This is the bottom of the candle body. If less than the close value the candle will be filled. Otherwise the candle will be hollow.
    #[serde(rename="openSeries")]
    
    pub open_series: Option<CandlestickSeries>,
}

impl client::Part for CandlestickData {}


/// The domain of a CandlestickChart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickDomain {
    /// The data of the CandlestickDomain.
    
    pub data: Option<ChartData>,
    /// True to reverse the order of the domain values (horizontal axis).
    
    pub reversed: Option<bool>,
}

impl client::Part for CandlestickDomain {}


/// The series of a CandlestickData.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickSeries {
    /// The data of the CandlestickSeries.
    
    pub data: Option<ChartData>,
}

impl client::Part for CandlestickSeries {}


/// Data about a specific cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CellData {
    /// Output only. Information about a data source formula on the cell. The field is set if user_entered_value is a formula referencing some DATA_SOURCE sheet, e.g. `=SUM(DataSheet!Column)`.
    #[serde(rename="dataSourceFormula")]
    
    pub data_source_formula: Option<DataSourceFormula>,
    /// A data source table anchored at this cell. The size of data source table itself is computed dynamically based on its configuration. Only the first cell of the data source table contains the data source table definition. The other cells will contain the display values of the data source table result in their effective_value fields.
    #[serde(rename="dataSourceTable")]
    
    pub data_source_table: Option<DataSourceTable>,
    /// A data validation rule on the cell, if any. When writing, the new data validation rule will overwrite any prior rule.
    #[serde(rename="dataValidation")]
    
    pub data_validation: Option<DataValidationRule>,
    /// The effective format being used by the cell. This includes the results of applying any conditional formatting and, if the cell contains a formula, the computed number format. If the effective format is the default format, effective format will not be written. This field is read-only.
    #[serde(rename="effectiveFormat")]
    
    pub effective_format: Option<CellFormat>,
    /// The effective value of the cell. For cells with formulas, this is the calculated value. For cells with literals, this is the same as the user_entered_value. This field is read-only.
    #[serde(rename="effectiveValue")]
    
    pub effective_value: Option<ExtendedValue>,
    /// The formatted value of the cell. This is the value as it's shown to the user. This field is read-only.
    #[serde(rename="formattedValue")]
    
    pub formatted_value: Option<String>,
    /// A hyperlink this cell points to, if any. If the cell contains multiple hyperlinks, this field will be empty. This field is read-only. To set it, use a `=HYPERLINK` formula in the userEnteredValue.formulaValue field. A cell-level link can also be set from the userEnteredFormat.textFormat field. Alternatively, set a hyperlink in the textFormatRun.format.link field that spans the entire cell.
    
    pub hyperlink: Option<String>,
    /// Any note on the cell.
    
    pub note: Option<String>,
    /// A pivot table anchored at this cell. The size of pivot table itself is computed dynamically based on its data, grouping, filters, values, etc. Only the top-left cell of the pivot table contains the pivot table definition. The other cells will contain the calculated values of the results of the pivot in their effective_value fields.
    #[serde(rename="pivotTable")]
    
    pub pivot_table: Option<PivotTable>,
    /// Runs of rich text applied to subsections of the cell. Runs are only valid on user entered strings, not formulas, bools, or numbers. Properties of a run start at a specific index in the text and continue until the next run. Runs will inherit the properties of the cell unless explicitly changed. When writing, the new runs will overwrite any prior runs. When writing a new user_entered_value, previous runs are erased.
    #[serde(rename="textFormatRuns")]
    
    pub text_format_runs: Option<Vec<TextFormatRun>>,
    /// The format the user entered for the cell. When writing, the new format will be merged with the existing format.
    #[serde(rename="userEnteredFormat")]
    
    pub user_entered_format: Option<CellFormat>,
    /// The value the user entered in the cell. e.g., `1234`, `'Hello'`, or `=NOW()` Note: Dates, Times and DateTimes are represented as doubles in serial number format.
    #[serde(rename="userEnteredValue")]
    
    pub user_entered_value: Option<ExtendedValue>,
}

impl client::Part for CellData {}


/// The format of a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CellFormat {
    /// The background color of the cell. Deprecated: Use background_color_style.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<Color>,
    /// The background color of the cell. If background_color is also set, this field takes precedence.
    #[serde(rename="backgroundColorStyle")]
    
    pub background_color_style: Option<ColorStyle>,
    /// The borders of the cell.
    
    pub borders: Option<Borders>,
    /// The horizontal alignment of the value in the cell.
    #[serde(rename="horizontalAlignment")]
    
    pub horizontal_alignment: Option<String>,
    /// If one exists, how a hyperlink should be displayed in the cell.
    #[serde(rename="hyperlinkDisplayType")]
    
    pub hyperlink_display_type: Option<String>,
    /// A format describing how number values should be represented to the user.
    #[serde(rename="numberFormat")]
    
    pub number_format: Option<NumberFormat>,
    /// The padding of the cell.
    
    pub padding: Option<Padding>,
    /// The direction of the text in the cell.
    #[serde(rename="textDirection")]
    
    pub text_direction: Option<String>,
    /// The format of the text in the cell (unless overridden by a format run). Setting a cell-level link here clears the cell's existing links. Setting the link field in a TextFormatRun takes precedence over the cell-level link.
    #[serde(rename="textFormat")]
    
    pub text_format: Option<TextFormat>,
    /// The rotation applied to text in the cell.
    #[serde(rename="textRotation")]
    
    pub text_rotation: Option<TextRotation>,
    /// The vertical alignment of the value in the cell.
    #[serde(rename="verticalAlignment")]
    
    pub vertical_alignment: Option<String>,
    /// The wrap strategy for the value in the cell.
    #[serde(rename="wrapStrategy")]
    
    pub wrap_strategy: Option<String>,
}

impl client::Part for CellFormat {}


/// The options that define a "view window" for a chart (such as the visible values in an axis).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartAxisViewWindowOptions {
    /// The maximum numeric value to be shown in this view window. If unset, will automatically determine a maximum value that looks good for the data.
    #[serde(rename="viewWindowMax")]
    
    pub view_window_max: Option<f64>,
    /// The minimum numeric value to be shown in this view window. If unset, will automatically determine a minimum value that looks good for the data.
    #[serde(rename="viewWindowMin")]
    
    pub view_window_min: Option<f64>,
    /// The view window's mode.
    #[serde(rename="viewWindowMode")]
    
    pub view_window_mode: Option<String>,
}

impl client::Part for ChartAxisViewWindowOptions {}


/// Custom number formatting options for chart attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartCustomNumberFormatOptions {
    /// Custom prefix to be prepended to the chart attribute. This field is optional.
    
    pub prefix: Option<String>,
    /// Custom suffix to be appended to the chart attribute. This field is optional.
    
    pub suffix: Option<String>,
}

impl client::Part for ChartCustomNumberFormatOptions {}


/// The data included in a domain or series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartData {
    /// The aggregation type for the series of a data source chart. Only supported for data source charts.
    #[serde(rename="aggregateType")]
    
    pub aggregate_type: Option<String>,
    /// The reference to the data source column that the data reads from.
    #[serde(rename="columnReference")]
    
    pub column_reference: Option<DataSourceColumnReference>,
    /// The rule to group the data by if the ChartData backs the domain of a data source chart. Only supported for data source charts.
    #[serde(rename="groupRule")]
    
    pub group_rule: Option<ChartGroupRule>,
    /// The source ranges of the data.
    #[serde(rename="sourceRange")]
    
    pub source_range: Option<ChartSourceRange>,
}

impl client::Part for ChartData {}


/// Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartDateTimeRule {
    /// The type of date-time grouping to apply.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ChartDateTimeRule {}


/// An optional setting on the ChartData of the domain of a data source chart that defines buckets for the values in the domain rather than breaking out each individual value. For example, when plotting a data source chart, you can specify a histogram rule on the domain (it should only contain numeric values), grouping its values into buckets. Any values of a chart series that fall into the same bucket are aggregated based on the aggregate_type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartGroupRule {
    /// A ChartDateTimeRule.
    #[serde(rename="dateTimeRule")]
    
    pub date_time_rule: Option<ChartDateTimeRule>,
    /// A ChartHistogramRule
    #[serde(rename="histogramRule")]
    
    pub histogram_rule: Option<ChartHistogramRule>,
}

impl client::Part for ChartGroupRule {}


/// Allows you to organize numeric values in a source data column into buckets of constant size.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartHistogramRule {
    /// The size of the buckets that are created. Must be positive.
    #[serde(rename="intervalSize")]
    
    pub interval_size: Option<f64>,
    /// The maximum value at which items are placed into buckets. Values greater than the maximum are grouped into a single bucket. If omitted, it is determined by the maximum item value.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<f64>,
    /// The minimum value at which items are placed into buckets. Values that are less than the minimum are grouped into a single bucket. If omitted, it is determined by the minimum item value.
    #[serde(rename="minValue")]
    
    pub min_value: Option<f64>,
}

impl client::Part for ChartHistogramRule {}


/// Source ranges for a chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartSourceRange {
    /// The ranges of data for a series or domain. Exactly one dimension must have a length of 1, and all sources in the list must have the same dimension with length 1. The domain (if it exists) & all series must have the same number of source ranges. If using more than one source range, then the source range at a given offset must be in order and contiguous across the domain and series. For example, these are valid configurations: domain sources: A1:A5 series1 sources: B1:B5 series2 sources: D6:D10 domain sources: A1:A5, C10:C12 series1 sources: B1:B5, D10:D12 series2 sources: C1:C5, E10:E12
    
    pub sources: Option<Vec<GridRange>>,
}

impl client::Part for ChartSourceRange {}


/// The specifications of a chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartSpec {
    /// The alternative text that describes the chart. This is often used for accessibility.
    #[serde(rename="altText")]
    
    pub alt_text: Option<String>,
    /// The background color of the entire chart. Not applicable to Org charts. Deprecated: Use background_color_style.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<Color>,
    /// The background color of the entire chart. Not applicable to Org charts. If background_color is also set, this field takes precedence.
    #[serde(rename="backgroundColorStyle")]
    
    pub background_color_style: Option<ColorStyle>,
    /// A basic chart specification, can be one of many kinds of charts. See BasicChartType for the list of all charts this supports.
    #[serde(rename="basicChart")]
    
    pub basic_chart: Option<BasicChartSpec>,
    /// A bubble chart specification.
    #[serde(rename="bubbleChart")]
    
    pub bubble_chart: Option<BubbleChartSpec>,
    /// A candlestick chart specification.
    #[serde(rename="candlestickChart")]
    
    pub candlestick_chart: Option<CandlestickChartSpec>,
    /// If present, the field contains data source chart specific properties.
    #[serde(rename="dataSourceChartProperties")]
    
    pub data_source_chart_properties: Option<DataSourceChartProperties>,
    /// The filters applied to the source data of the chart. Only supported for data source charts.
    #[serde(rename="filterSpecs")]
    
    pub filter_specs: Option<Vec<FilterSpec>>,
    /// The name of the font to use by default for all chart text (e.g. title, axis labels, legend). If a font is specified for a specific part of the chart it will override this font name.
    #[serde(rename="fontName")]
    
    pub font_name: Option<String>,
    /// Determines how the charts will use hidden rows or columns.
    #[serde(rename="hiddenDimensionStrategy")]
    
    pub hidden_dimension_strategy: Option<String>,
    /// A histogram chart specification.
    #[serde(rename="histogramChart")]
    
    pub histogram_chart: Option<HistogramChartSpec>,
    /// True to make a chart fill the entire space in which it's rendered with minimum padding. False to use the default padding. (Not applicable to Geo and Org charts.)
    
    pub maximized: Option<bool>,
    /// An org chart specification.
    #[serde(rename="orgChart")]
    
    pub org_chart: Option<OrgChartSpec>,
    /// A pie chart specification.
    #[serde(rename="pieChart")]
    
    pub pie_chart: Option<PieChartSpec>,
    /// A scorecard chart specification.
    #[serde(rename="scorecardChart")]
    
    pub scorecard_chart: Option<ScorecardChartSpec>,
    /// The order to sort the chart data by. Only a single sort spec is supported. Only supported for data source charts.
    #[serde(rename="sortSpecs")]
    
    pub sort_specs: Option<Vec<SortSpec>>,
    /// The subtitle of the chart.
    
    pub subtitle: Option<String>,
    /// The subtitle text format. Strikethrough, underline, and link are not supported.
    #[serde(rename="subtitleTextFormat")]
    
    pub subtitle_text_format: Option<TextFormat>,
    /// The subtitle text position. This field is optional.
    #[serde(rename="subtitleTextPosition")]
    
    pub subtitle_text_position: Option<TextPosition>,
    /// The title of the chart.
    
    pub title: Option<String>,
    /// The title text format. Strikethrough, underline, and link are not supported.
    #[serde(rename="titleTextFormat")]
    
    pub title_text_format: Option<TextFormat>,
    /// The title text position. This field is optional.
    #[serde(rename="titleTextPosition")]
    
    pub title_text_position: Option<TextPosition>,
    /// A treemap chart specification.
    #[serde(rename="treemapChart")]
    
    pub treemap_chart: Option<TreemapChartSpec>,
    /// A waterfall chart specification.
    #[serde(rename="waterfallChart")]
    
    pub waterfall_chart: Option<WaterfallChartSpec>,
}

impl client::Part for ChartSpec {}


/// Clears the basic filter, if any exists on the sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearBasicFilterRequest {
    /// The sheet ID on which the basic filter should be cleared.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for ClearBasicFilterRequest {}


/// The request for clearing a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values clear spreadsheets](SpreadsheetValueClearCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearValuesRequest { _never_set: Option<bool> }

impl client::RequestValue for ClearValuesRequest {}


/// The response when clearing a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values clear spreadsheets](SpreadsheetValueClearCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearValuesResponse {
    /// The range (in A1 notation) that was cleared. (If the request was for an unbounded range or a ranger larger than the bounds of the sheet, this will be the actual range that was cleared, bounded to the sheet's limits.)
    #[serde(rename="clearedRange")]
    
    pub cleared_range: Option<String>,
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
}

impl client::ResponseResult for ClearValuesResponse {}


/// Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to and from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't have information about the absolute color space that should be used to interpret the RGB value—for example, sRGB, Adobe RGB, DCI-P3, and BT.2020. By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most `1e-5`. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)` This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0).
    
    pub alpha: Option<f32>,
    /// The amount of blue in the color as a value in the interval [0, 1].
    
    pub blue: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    
    pub red: Option<f32>,
}

impl client::Part for Color {}


/// A color value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColorStyle {
    /// RGB color. The [`alpha`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other#Color.FIELDS.alpha) value in the [`Color`](https://developers.google.com/sheets/api/reference/rest/v4/spreadsheets/other#color) object isn’t generally supported.
    #[serde(rename="rgbColor")]
    
    pub rgb_color: Option<Color>,
    /// Theme color.
    #[serde(rename="themeColor")]
    
    pub theme_color: Option<String>,
}

impl client::Part for ColorStyle {}


/// The value of the condition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConditionValue {
    /// A relative date (based on the current date). Valid only if the type is DATE_BEFORE, DATE_AFTER, DATE_ON_OR_BEFORE or DATE_ON_OR_AFTER. Relative dates are not supported in data validation. They are supported only in conditional formatting and conditional filters.
    #[serde(rename="relativeDate")]
    
    pub relative_date: Option<String>,
    /// A value the condition is based on. The value is parsed as if the user typed into a cell. Formulas are supported (and must begin with an `=` or a '+').
    #[serde(rename="userEnteredValue")]
    
    pub user_entered_value: Option<String>,
}

impl client::Part for ConditionValue {}


/// A rule describing a conditional format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalFormatRule {
    /// The formatting is either "on" or "off" according to the rule.
    #[serde(rename="booleanRule")]
    
    pub boolean_rule: Option<BooleanRule>,
    /// The formatting will vary based on the gradients in the rule.
    #[serde(rename="gradientRule")]
    
    pub gradient_rule: Option<GradientRule>,
    /// The ranges that are formatted if the condition is true. All the ranges must be on the same grid.
    
    pub ranges: Option<Vec<GridRange>>,
}

impl client::Part for ConditionalFormatRule {}


/// Copies data from the source to the destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopyPasteRequest {
    /// The location to paste to. If the range covers a span that's a multiple of the source's height or width, then the data will be repeated to fill in the destination range. If the range is smaller than the source range, the entire source data will still be copied (beyond the end of the destination range).
    
    pub destination: Option<GridRange>,
    /// How that data should be oriented when pasting.
    #[serde(rename="pasteOrientation")]
    
    pub paste_orientation: Option<String>,
    /// What kind of data to paste.
    #[serde(rename="pasteType")]
    
    pub paste_type: Option<String>,
    /// The source range to copy.
    
    pub source: Option<GridRange>,
}

impl client::Part for CopyPasteRequest {}


/// The request to copy a sheet across spreadsheets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sheets copy to spreadsheets](SpreadsheetSheetCopyToCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopySheetToAnotherSpreadsheetRequest {
    /// The ID of the spreadsheet to copy the sheet to.
    #[serde(rename="destinationSpreadsheetId")]
    
    pub destination_spreadsheet_id: Option<String>,
}

impl client::RequestValue for CopySheetToAnotherSpreadsheetRequest {}


/// A request to create developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateDeveloperMetadataRequest {
    /// The developer metadata to create.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<DeveloperMetadata>,
}

impl client::Part for CreateDeveloperMetadataRequest {}


/// The response from creating developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateDeveloperMetadataResponse {
    /// The developer metadata that was created.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<DeveloperMetadata>,
}

impl client::Part for CreateDeveloperMetadataResponse {}


/// Moves data from the source to the destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CutPasteRequest {
    /// The top-left coordinate where the data should be pasted.
    
    pub destination: Option<GridCoordinate>,
    /// What kind of data to paste. All the source data will be cut, regardless of what is pasted.
    #[serde(rename="pasteType")]
    
    pub paste_type: Option<String>,
    /// The source data to cut.
    
    pub source: Option<GridRange>,
}

impl client::Part for CutPasteRequest {}


/// The data execution status. A data execution is created to sync a data source object with the latest data from a DataSource. It is usually scheduled to run at background, you can check its state to tell if an execution completes There are several scenarios where a data execution is triggered to run: * Adding a data source creates an associated data source sheet as well as a data execution to sync the data from the data source to the sheet. * Updating a data source creates a data execution to refresh the associated data source sheet similarly. * You can send refresh request to explicitly refresh one or multiple data source objects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataExecutionStatus {
    /// The error code.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<String>,
    /// The error message, which may be empty.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// Gets the time the data last successfully refreshed.
    #[serde(rename="lastRefreshTime")]
    
    pub last_refresh_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The state of the data execution.
    
    pub state: Option<String>,
}

impl client::Part for DataExecutionStatus {}


/// Filter that describes what data should be selected or returned from a request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataFilter {
    /// Selects data that matches the specified A1 range.
    #[serde(rename="a1Range")]
    
    pub a1_range: Option<String>,
    /// Selects data associated with the developer metadata matching the criteria described by this DeveloperMetadataLookup.
    #[serde(rename="developerMetadataLookup")]
    
    pub developer_metadata_lookup: Option<DeveloperMetadataLookup>,
    /// Selects data that matches the range described by the GridRange.
    #[serde(rename="gridRange")]
    
    pub grid_range: Option<GridRange>,
}

impl client::Part for DataFilter {}


/// A range of values whose location is specified by a DataFilter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataFilterValueRange {
    /// The data filter describing the location of the values in the spreadsheet.
    #[serde(rename="dataFilter")]
    
    pub data_filter: Option<DataFilter>,
    /// The major dimension of the values.
    #[serde(rename="majorDimension")]
    
    pub major_dimension: Option<String>,
    /// The data to be written. If the provided values exceed any of the ranges matched by the data filter then the request fails. If the provided values are less than the matched ranges only the specified values are written, existing values in the matched ranges remain unaffected.
    
    pub values: Option<Vec<Vec<json::Value>>>,
}

impl client::Part for DataFilterValueRange {}


/// Settings for one set of data labels. Data labels are annotations that appear next to a set of data, such as the points on a line chart, and provide additional information about what the data represents, such as a text representation of the value behind that point on the graph.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataLabel {
    /// Data to use for custom labels. Only used if type is set to CUSTOM. This data must be the same length as the series or other element this data label is applied to. In addition, if the series is split into multiple source ranges, this source data must come from the next column in the source data. For example, if the series is B2:B4,E6:E8 then this data must come from C2:C4,F6:F8.
    #[serde(rename="customLabelData")]
    
    pub custom_label_data: Option<ChartData>,
    /// The placement of the data label relative to the labeled data.
    
    pub placement: Option<String>,
    /// The text format used for the data label. The link field is not supported.
    #[serde(rename="textFormat")]
    
    pub text_format: Option<TextFormat>,
    /// The type of the data label.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for DataLabel {}


/// Information about an external data source in the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSource {
    /// All calculated columns in the data source.
    #[serde(rename="calculatedColumns")]
    
    pub calculated_columns: Option<Vec<DataSourceColumn>>,
    /// The spreadsheet-scoped unique ID that identifies the data source. Example: 1080547365.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// The ID of the Sheet connected with the data source. The field cannot be changed once set. When creating a data source, an associated DATA_SOURCE sheet is also created, if the field is not specified, the ID of the created sheet will be randomly generated.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
    /// The DataSourceSpec for the data source connected with this spreadsheet.
    
    pub spec: Option<DataSourceSpec>,
}

impl client::Part for DataSource {}


/// Properties of a data source chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceChartProperties {
    /// Output only. The data execution status.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// ID of the data source that the chart is associated with.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
}

impl client::Part for DataSourceChartProperties {}


/// A column in a data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceColumn {
    /// The formula of the calculated column.
    
    pub formula: Option<String>,
    /// The column reference.
    
    pub reference: Option<DataSourceColumnReference>,
}

impl client::Part for DataSourceColumn {}


/// An unique identifier that references a data source column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceColumnReference {
    /// The display name of the column. It should be unique within a data source.
    
    pub name: Option<String>,
}

impl client::Part for DataSourceColumnReference {}


/// A data source formula.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceFormula {
    /// Output only. The data execution status.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// The ID of the data source the formula is associated with.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
}

impl client::Part for DataSourceFormula {}


/// Reference to a data source object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceObjectReference {
    /// References to a data source chart.
    #[serde(rename="chartId")]
    
    pub chart_id: Option<i32>,
    /// References to a cell containing DataSourceFormula.
    #[serde(rename="dataSourceFormulaCell")]
    
    pub data_source_formula_cell: Option<GridCoordinate>,
    /// References to a data source PivotTable anchored at the cell.
    #[serde(rename="dataSourcePivotTableAnchorCell")]
    
    pub data_source_pivot_table_anchor_cell: Option<GridCoordinate>,
    /// References to a DataSourceTable anchored at the cell.
    #[serde(rename="dataSourceTableAnchorCell")]
    
    pub data_source_table_anchor_cell: Option<GridCoordinate>,
    /// References to a DATA_SOURCE sheet.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<String>,
}

impl client::Part for DataSourceObjectReference {}


/// A list of references to data source objects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceObjectReferences {
    /// The references.
    
    pub references: Option<Vec<DataSourceObjectReference>>,
}

impl client::Part for DataSourceObjectReferences {}


/// A parameter in a data source's query. The parameter allows the user to pass in values from the spreadsheet into a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceParameter {
    /// Named parameter. Must be a legitimate identifier for the DataSource that supports it. For example, [BigQuery identifier](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#identifiers).
    
    pub name: Option<String>,
    /// ID of a NamedRange. Its size must be 1x1.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
    /// A range that contains the value of the parameter. Its size must be 1x1.
    
    pub range: Option<GridRange>,
}

impl client::Part for DataSourceParameter {}


/// A schedule for data to refresh every day in a given time interval.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceRefreshDailySchedule {
    /// The start time of a time interval in which a data source refresh is scheduled. Only `hours` part is used. The time interval size defaults to that in the Sheets editor.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for DataSourceRefreshDailySchedule {}


/// A monthly schedule for data to refresh on specific days in the month in a given time interval.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceRefreshMonthlySchedule {
    /// Days of the month to refresh. Only 1-28 are supported, mapping to the 1st to the 28th day. At least one day must be specified.
    #[serde(rename="daysOfMonth")]
    
    pub days_of_month: Option<Vec<i32>>,
    /// The start time of a time interval in which a data source refresh is scheduled. Only `hours` part is used. The time interval size defaults to that in the Sheets editor.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for DataSourceRefreshMonthlySchedule {}


/// Schedule for refreshing the data source. Data sources in the spreadsheet are refreshed within a time interval. You can specify the start time by clicking the Scheduled Refresh button in the Sheets editor, but the interval is fixed at 4 hours. For example, if you specify a start time of 8 AM , the refresh will take place between 8 AM and 12 PM every day.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceRefreshSchedule {
    /// Daily refresh schedule.
    #[serde(rename="dailySchedule")]
    
    pub daily_schedule: Option<DataSourceRefreshDailySchedule>,
    /// True if the refresh schedule is enabled, or false otherwise.
    
    pub enabled: Option<bool>,
    /// Monthly refresh schedule.
    #[serde(rename="monthlySchedule")]
    
    pub monthly_schedule: Option<DataSourceRefreshMonthlySchedule>,
    /// Output only. The time interval of the next run.
    #[serde(rename="nextRun")]
    
    pub next_run: Option<Interval>,
    /// The scope of the refresh. Must be ALL_DATA_SOURCES.
    #[serde(rename="refreshScope")]
    
    pub refresh_scope: Option<String>,
    /// Weekly refresh schedule.
    #[serde(rename="weeklySchedule")]
    
    pub weekly_schedule: Option<DataSourceRefreshWeeklySchedule>,
}

impl client::Part for DataSourceRefreshSchedule {}


/// A weekly schedule for data to refresh on specific days in a given time interval.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceRefreshWeeklySchedule {
    /// Days of the week to refresh. At least one day must be specified.
    #[serde(rename="daysOfWeek")]
    
    pub days_of_week: Option<Vec<String>>,
    /// The start time of a time interval in which a data source refresh is scheduled. Only `hours` part is used. The time interval size defaults to that in the Sheets editor.
    #[serde(rename="startTime")]
    
    pub start_time: Option<TimeOfDay>,
}

impl client::Part for DataSourceRefreshWeeklySchedule {}


/// A range along a single dimension on a DATA_SOURCE sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceSheetDimensionRange {
    /// The columns on the data source sheet.
    #[serde(rename="columnReferences")]
    
    pub column_references: Option<Vec<DataSourceColumnReference>>,
    /// The ID of the data source sheet the range is on.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for DataSourceSheetDimensionRange {}


/// Additional properties of a DATA_SOURCE sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceSheetProperties {
    /// The columns displayed on the sheet, corresponding to the values in RowData.
    
    pub columns: Option<Vec<DataSourceColumn>>,
    /// The data execution status.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// ID of the DataSource the sheet is connected to.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
}

impl client::Part for DataSourceSheetProperties {}


/// This specifies the details of the data source. For example, for BigQuery, this specifies information about the BigQuery source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceSpec {
    /// A BigQueryDataSourceSpec.
    #[serde(rename="bigQuery")]
    
    pub big_query: Option<BigQueryDataSourceSpec>,
    /// The parameters of the data source, used when querying the data source.
    
    pub parameters: Option<Vec<DataSourceParameter>>,
}

impl client::Part for DataSourceSpec {}


/// A data source table, which allows the user to import a static table of data from the DataSource into Sheets. This is also known as "Extract" in the Sheets editor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSourceTable {
    /// The type to select columns for the data source table. Defaults to SELECTED.
    #[serde(rename="columnSelectionType")]
    
    pub column_selection_type: Option<String>,
    /// Columns selected for the data source table. The column_selection_type must be SELECTED.
    
    pub columns: Option<Vec<DataSourceColumnReference>>,
    /// Output only. The data execution status.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// The ID of the data source the data source table is associated with.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// Filter specifications in the data source table.
    #[serde(rename="filterSpecs")]
    
    pub filter_specs: Option<Vec<FilterSpec>>,
    /// The limit of rows to return. If not set, a default limit is applied. Please refer to the Sheets editor for the default and max limit.
    #[serde(rename="rowLimit")]
    
    pub row_limit: Option<i32>,
    /// Sort specifications in the data source table. The result of the data source table is sorted based on the sort specifications in order.
    #[serde(rename="sortSpecs")]
    
    pub sort_specs: Option<Vec<SortSpec>>,
}

impl client::Part for DataSourceTable {}


/// A data validation rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataValidationRule {
    /// The condition that data in the cell must match.
    
    pub condition: Option<BooleanCondition>,
    /// A message to show the user when adding data to the cell.
    #[serde(rename="inputMessage")]
    
    pub input_message: Option<String>,
    /// True if the UI should be customized based on the kind of condition. If true, "List" conditions will show a dropdown.
    #[serde(rename="showCustomUi")]
    
    pub show_custom_ui: Option<bool>,
    /// True if invalid data should be rejected.
    
    pub strict: Option<bool>,
}

impl client::Part for DataValidationRule {}


/// Allows you to organize the date-time values in a source data column into buckets based on selected parts of their date or time values. For example, consider a pivot table showing sales transactions by date: +----------+--------------+ | Date | SUM of Sales | +----------+--------------+ | 1/1/2017 | $621.14 | | 2/3/2017 | $708.84 | | 5/8/2017 | $326.84 | ... +----------+--------------+ Applying a date-time group rule with a DateTimeRuleType of YEAR_MONTH results in the following pivot table. +--------------+--------------+ | Grouped Date | SUM of Sales | +--------------+--------------+ | 2017-Jan | $53,731.78 | | 2017-Feb | $83,475.32 | | 2017-Mar | $94,385.05 | ... +--------------+--------------+
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateTimeRule {
    /// The type of date-time grouping to apply.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for DateTimeRule {}


/// Removes the banded range with the given ID from the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteBandingRequest {
    /// The ID of the banded range to delete.
    #[serde(rename="bandedRangeId")]
    
    pub banded_range_id: Option<i32>,
}

impl client::Part for DeleteBandingRequest {}


/// Deletes a conditional format rule at the given index. All subsequent rules' indexes are decremented.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteConditionalFormatRuleRequest {
    /// The zero-based index of the rule to be deleted.
    
    pub index: Option<i32>,
    /// The sheet the rule is being deleted from.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for DeleteConditionalFormatRuleRequest {}


/// The result of deleting a conditional format rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteConditionalFormatRuleResponse {
    /// The rule that was deleted.
    
    pub rule: Option<ConditionalFormatRule>,
}

impl client::Part for DeleteConditionalFormatRuleResponse {}


/// Deletes a data source. The request also deletes the associated data source sheet, and unlinks all associated data source objects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDataSourceRequest {
    /// The ID of the data source to delete.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
}

impl client::Part for DeleteDataSourceRequest {}


/// A request to delete developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDeveloperMetadataRequest {
    /// The data filter describing the criteria used to select which developer metadata entry to delete.
    #[serde(rename="dataFilter")]
    
    pub data_filter: Option<DataFilter>,
}

impl client::Part for DeleteDeveloperMetadataRequest {}


/// The response from deleting developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDeveloperMetadataResponse {
    /// The metadata that was deleted.
    #[serde(rename="deletedDeveloperMetadata")]
    
    pub deleted_developer_metadata: Option<Vec<DeveloperMetadata>>,
}

impl client::Part for DeleteDeveloperMetadataResponse {}


/// Deletes a group over the specified range by decrementing the depth of the dimensions in the range. For example, assume the sheet has a depth-1 group over B:E and a depth-2 group over C:D. Deleting a group over D:E leaves the sheet with a depth-1 group over B:D and a depth-2 group over C:C.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDimensionGroupRequest {
    /// The range of the group to be deleted.
    
    pub range: Option<DimensionRange>,
}

impl client::Part for DeleteDimensionGroupRequest {}


/// The result of deleting a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDimensionGroupResponse {
    /// All groups of a dimension after deleting a group from that dimension.
    #[serde(rename="dimensionGroups")]
    
    pub dimension_groups: Option<Vec<DimensionGroup>>,
}

impl client::Part for DeleteDimensionGroupResponse {}


/// Deletes the dimensions from the sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDimensionRequest {
    /// The dimensions to delete from the sheet.
    
    pub range: Option<DimensionRange>,
}

impl client::Part for DeleteDimensionRequest {}


/// Removes rows within this range that contain values in the specified columns that are duplicates of values in any previous row. Rows with identical values but different letter cases, formatting, or formulas are considered to be duplicates. This request also removes duplicate rows hidden from view (for example, due to a filter). When removing duplicates, the first instance of each duplicate row scanning from the top downwards is kept in the resulting range. Content outside of the specified range isn't removed, and rows considered duplicates do not have to be adjacent to each other in the range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDuplicatesRequest {
    /// The columns in the range to analyze for duplicate values. If no columns are selected then all columns are analyzed for duplicates.
    #[serde(rename="comparisonColumns")]
    
    pub comparison_columns: Option<Vec<DimensionRange>>,
    /// The range to remove duplicates rows from.
    
    pub range: Option<GridRange>,
}

impl client::Part for DeleteDuplicatesRequest {}


/// The result of removing duplicates in a range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDuplicatesResponse {
    /// The number of duplicate rows removed.
    #[serde(rename="duplicatesRemovedCount")]
    
    pub duplicates_removed_count: Option<i32>,
}

impl client::Part for DeleteDuplicatesResponse {}


/// Deletes the embedded object with the given ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteEmbeddedObjectRequest {
    /// The ID of the embedded object to delete.
    #[serde(rename="objectId")]
    
    pub object_id: Option<i32>,
}

impl client::Part for DeleteEmbeddedObjectRequest {}


/// Deletes a particular filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFilterViewRequest {
    /// The ID of the filter to delete.
    #[serde(rename="filterId")]
    
    pub filter_id: Option<i32>,
}

impl client::Part for DeleteFilterViewRequest {}


/// Removes the named range with the given ID from the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteNamedRangeRequest {
    /// The ID of the named range to delete.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
}

impl client::Part for DeleteNamedRangeRequest {}


/// Deletes the protected range with the given ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteProtectedRangeRequest {
    /// The ID of the protected range to delete.
    #[serde(rename="protectedRangeId")]
    
    pub protected_range_id: Option<i32>,
}

impl client::Part for DeleteProtectedRangeRequest {}


/// Deletes a range of cells, shifting other cells into the deleted area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRangeRequest {
    /// The range of cells to delete.
    
    pub range: Option<GridRange>,
    /// The dimension from which deleted cells will be replaced with. If ROWS, existing cells will be shifted upward to replace the deleted cells. If COLUMNS, existing cells will be shifted left to replace the deleted cells.
    #[serde(rename="shiftDimension")]
    
    pub shift_dimension: Option<String>,
}

impl client::Part for DeleteRangeRequest {}


/// Deletes the requested sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteSheetRequest {
    /// The ID of the sheet to delete. If the sheet is of DATA_SOURCE type, the associated DataSource is also deleted.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for DeleteSheetRequest {}


/// Developer metadata associated with a location or object in a spreadsheet. Developer metadata may be used to associate arbitrary data with various parts of a spreadsheet and will remain associated at those locations as they move around and the spreadsheet is edited. For example, if developer metadata is associated with row 5 and another row is then subsequently inserted above row 5, that original metadata will still be associated with the row it was first associated with (what is now row 6). If the associated object is deleted its metadata is deleted too.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developer metadata get spreadsheets](SpreadsheetDeveloperMetadataGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperMetadata {
    /// The location where the metadata is associated.
    
    pub location: Option<DeveloperMetadataLocation>,
    /// The spreadsheet-scoped unique ID that identifies the metadata. IDs may be specified when metadata is created, otherwise one will be randomly generated and assigned. Must be positive.
    #[serde(rename="metadataId")]
    
    pub metadata_id: Option<i32>,
    /// The metadata key. There may be multiple metadata in a spreadsheet with the same key. Developer metadata must always have a key specified.
    #[serde(rename="metadataKey")]
    
    pub metadata_key: Option<String>,
    /// Data associated with the metadata's key.
    #[serde(rename="metadataValue")]
    
    pub metadata_value: Option<String>,
    /// The metadata visibility. Developer metadata must always have a visibility specified.
    
    pub visibility: Option<String>,
}

impl client::ResponseResult for DeveloperMetadata {}


/// A location where metadata may be associated in a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperMetadataLocation {
    /// Represents the row or column when metadata is associated with a dimension. The specified DimensionRange must represent a single row or column; it cannot be unbounded or span multiple rows or columns.
    #[serde(rename="dimensionRange")]
    
    pub dimension_range: Option<DimensionRange>,
    /// The type of location this object represents. This field is read-only.
    #[serde(rename="locationType")]
    
    pub location_type: Option<String>,
    /// The ID of the sheet when metadata is associated with an entire sheet.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
    /// True when metadata is associated with an entire spreadsheet.
    
    pub spreadsheet: Option<bool>,
}

impl client::Part for DeveloperMetadataLocation {}


/// Selects DeveloperMetadata that matches all of the specified fields. For example, if only a metadata ID is specified this considers the DeveloperMetadata with that particular unique ID. If a metadata key is specified, this considers all developer metadata with that key. If a key, visibility, and location type are all specified, this considers all developer metadata with that key and visibility that are associated with a location of that type. In general, this selects all DeveloperMetadata that matches the intersection of all the specified fields; any field or combination of fields may be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperMetadataLookup {
    /// Determines how this lookup matches the location. If this field is specified as EXACT, only developer metadata associated on the exact location specified is matched. If this field is specified to INTERSECTING, developer metadata associated on intersecting locations is also matched. If left unspecified, this field assumes a default value of INTERSECTING. If this field is specified, a metadataLocation must also be specified.
    #[serde(rename="locationMatchingStrategy")]
    
    pub location_matching_strategy: Option<String>,
    /// Limits the selected developer metadata to those entries which are associated with locations of the specified type. For example, when this field is specified as ROW this lookup only considers developer metadata associated on rows. If the field is left unspecified, all location types are considered. This field cannot be specified as SPREADSHEET when the locationMatchingStrategy is specified as INTERSECTING or when the metadataLocation is specified as a non-spreadsheet location: spreadsheet metadata cannot intersect any other developer metadata location. This field also must be left unspecified when the locationMatchingStrategy is specified as EXACT.
    #[serde(rename="locationType")]
    
    pub location_type: Option<String>,
    /// Limits the selected developer metadata to that which has a matching DeveloperMetadata.metadata_id.
    #[serde(rename="metadataId")]
    
    pub metadata_id: Option<i32>,
    /// Limits the selected developer metadata to that which has a matching DeveloperMetadata.metadata_key.
    #[serde(rename="metadataKey")]
    
    pub metadata_key: Option<String>,
    /// Limits the selected developer metadata to those entries associated with the specified location. This field either matches exact locations or all intersecting locations according the specified locationMatchingStrategy.
    #[serde(rename="metadataLocation")]
    
    pub metadata_location: Option<DeveloperMetadataLocation>,
    /// Limits the selected developer metadata to that which has a matching DeveloperMetadata.metadata_value.
    #[serde(rename="metadataValue")]
    
    pub metadata_value: Option<String>,
    /// Limits the selected developer metadata to that which has a matching DeveloperMetadata.visibility. If left unspecified, all developer metadata visibile to the requesting project is considered.
    
    pub visibility: Option<String>,
}

impl client::Part for DeveloperMetadataLookup {}


/// A group over an interval of rows or columns on a sheet, which can contain or be contained within other groups. A group can be collapsed or expanded as a unit on the sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionGroup {
    /// This field is true if this group is collapsed. A collapsed group remains collapsed if an overlapping group at a shallower depth is expanded. A true value does not imply that all dimensions within the group are hidden, since a dimension's visibility can change independently from this group property. However, when this property is updated, all dimensions within it are set to hidden if this field is true, or set to visible if this field is false.
    
    pub collapsed: Option<bool>,
    /// The depth of the group, representing how many groups have a range that wholly contains the range of this group.
    
    pub depth: Option<i32>,
    /// The range over which this group exists.
    
    pub range: Option<DimensionRange>,
}

impl client::Part for DimensionGroup {}


/// Properties about a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionProperties {
    /// Output only. If set, this is a column in a data source sheet.
    #[serde(rename="dataSourceColumnReference")]
    
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /// The developer metadata associated with a single row or column.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// True if this dimension is being filtered. This field is read-only.
    #[serde(rename="hiddenByFilter")]
    
    pub hidden_by_filter: Option<bool>,
    /// True if this dimension is explicitly hidden.
    #[serde(rename="hiddenByUser")]
    
    pub hidden_by_user: Option<bool>,
    /// The height (if a row) or width (if a column) of the dimension in pixels.
    #[serde(rename="pixelSize")]
    
    pub pixel_size: Option<i32>,
}

impl client::Part for DimensionProperties {}


/// A range along a single dimension on a sheet. All indexes are zero-based. Indexes are half open: the start index is inclusive and the end index is exclusive. Missing indexes indicate the range is unbounded on that side.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionRange {
    /// The dimension of the span.
    
    pub dimension: Option<String>,
    /// The end (exclusive) of the span, or not set if unbounded.
    #[serde(rename="endIndex")]
    
    pub end_index: Option<i32>,
    /// The sheet this span is on.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
    /// The start (inclusive) of the span, or not set if unbounded.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
}

impl client::Part for DimensionRange {}


/// Duplicates a particular filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateFilterViewRequest {
    /// The ID of the filter being duplicated.
    #[serde(rename="filterId")]
    
    pub filter_id: Option<i32>,
}

impl client::Part for DuplicateFilterViewRequest {}


/// The result of a filter view being duplicated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateFilterViewResponse {
    /// The newly created filter.
    
    pub filter: Option<FilterView>,
}

impl client::Part for DuplicateFilterViewResponse {}


/// Duplicates the contents of a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateSheetRequest {
    /// The zero-based index where the new sheet should be inserted. The index of all sheets after this are incremented.
    #[serde(rename="insertSheetIndex")]
    
    pub insert_sheet_index: Option<i32>,
    /// If set, the ID of the new sheet. If not set, an ID is chosen. If set, the ID must not conflict with any existing sheet ID. If set, it must be non-negative.
    #[serde(rename="newSheetId")]
    
    pub new_sheet_id: Option<i32>,
    /// The name of the new sheet. If empty, a new name is chosen for you.
    #[serde(rename="newSheetName")]
    
    pub new_sheet_name: Option<String>,
    /// The sheet to duplicate. If the source sheet is of DATA_SOURCE type, its backing DataSource is also duplicated and associated with the new copy of the sheet. No data execution is triggered, the grid data of this sheet is also copied over but only available after the batch request completes.
    #[serde(rename="sourceSheetId")]
    
    pub source_sheet_id: Option<i32>,
}

impl client::Part for DuplicateSheetRequest {}


/// The result of duplicating a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateSheetResponse {
    /// The properties of the duplicate sheet.
    
    pub properties: Option<SheetProperties>,
}

impl client::Part for DuplicateSheetResponse {}


/// The editors of a protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Editors {
    /// True if anyone in the document's domain has edit access to the protected range. Domain protection is only supported on documents within a domain.
    #[serde(rename="domainUsersCanEdit")]
    
    pub domain_users_can_edit: Option<bool>,
    /// The email addresses of groups with edit access to the protected range.
    
    pub groups: Option<Vec<String>>,
    /// The email addresses of users with edit access to the protected range.
    
    pub users: Option<Vec<String>>,
}

impl client::Part for Editors {}


/// A chart embedded in a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedChart {
    /// The border of the chart.
    
    pub border: Option<EmbeddedObjectBorder>,
    /// The ID of the chart.
    #[serde(rename="chartId")]
    
    pub chart_id: Option<i32>,
    /// The position of the chart.
    
    pub position: Option<EmbeddedObjectPosition>,
    /// The specification of the chart.
    
    pub spec: Option<ChartSpec>,
}

impl client::Part for EmbeddedChart {}


/// A border along an embedded object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectBorder {
    /// The color of the border. Deprecated: Use color_style.
    
    pub color: Option<Color>,
    /// The color of the border. If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    
    pub color_style: Option<ColorStyle>,
}

impl client::Part for EmbeddedObjectBorder {}


/// The position of an embedded object such as a chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectPosition {
    /// If true, the embedded object is put on a new sheet whose ID is chosen for you. Used only when writing.
    #[serde(rename="newSheet")]
    
    pub new_sheet: Option<bool>,
    /// The position at which the object is overlaid on top of a grid.
    #[serde(rename="overlayPosition")]
    
    pub overlay_position: Option<OverlayPosition>,
    /// The sheet this is on. Set only if the embedded object is on its own sheet. Must be non-negative.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for EmbeddedObjectPosition {}


/// An error in a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorValue {
    /// A message with more information about the error (in the spreadsheet's locale).
    
    pub message: Option<String>,
    /// The type of error.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for ErrorValue {}


/// The kinds of value that a cell in a spreadsheet can have.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedValue {
    /// Represents a boolean value.
    #[serde(rename="boolValue")]
    
    pub bool_value: Option<bool>,
    /// Represents an error. This field is read-only.
    #[serde(rename="errorValue")]
    
    pub error_value: Option<ErrorValue>,
    /// Represents a formula.
    #[serde(rename="formulaValue")]
    
    pub formula_value: Option<String>,
    /// Represents a double value. Note: Dates, Times and DateTimes are represented as doubles in SERIAL_NUMBER format.
    #[serde(rename="numberValue")]
    
    pub number_value: Option<f64>,
    /// Represents a string value. Leading single quotes are not included. For example, if the user typed `'123` into the UI, this would be represented as a `stringValue` of `"123"`.
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for ExtendedValue {}


/// Criteria for showing/hiding rows in a filter or filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterCriteria {
    /// A condition that must be true for values to be shown. (This does not override hidden_values -- if a value is listed there, it will still be hidden.)
    
    pub condition: Option<BooleanCondition>,
    /// Values that should be hidden.
    #[serde(rename="hiddenValues")]
    
    pub hidden_values: Option<Vec<String>>,
    /// The background fill color to filter by; only cells with this fill color are shown. Mutually exclusive with visible_foreground_color. Deprecated: Use visible_background_color_style.
    #[serde(rename="visibleBackgroundColor")]
    
    pub visible_background_color: Option<Color>,
    /// The background fill color to filter by; only cells with this fill color are shown. This field is mutually exclusive with visible_foreground_color, and must be set to an RGB-type color. If visible_background_color is also set, this field takes precedence.
    #[serde(rename="visibleBackgroundColorStyle")]
    
    pub visible_background_color_style: Option<ColorStyle>,
    /// The foreground color to filter by; only cells with this foreground color are shown. Mutually exclusive with visible_background_color. Deprecated: Use visible_foreground_color_style.
    #[serde(rename="visibleForegroundColor")]
    
    pub visible_foreground_color: Option<Color>,
    /// The foreground color to filter by; only cells with this foreground color are shown. This field is mutually exclusive with visible_background_color, and must be set to an RGB-type color. If visible_foreground_color is also set, this field takes precedence.
    #[serde(rename="visibleForegroundColorStyle")]
    
    pub visible_foreground_color_style: Option<ColorStyle>,
}

impl client::Part for FilterCriteria {}


/// The filter criteria associated with a specific column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterSpec {
    /// The zero-based column index.
    #[serde(rename="columnIndex")]
    
    pub column_index: Option<i32>,
    /// Reference to a data source column.
    #[serde(rename="dataSourceColumnReference")]
    
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /// The criteria for the column.
    #[serde(rename="filterCriteria")]
    
    pub filter_criteria: Option<FilterCriteria>,
}

impl client::Part for FilterSpec {}


/// A filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterView {
    /// The criteria for showing/hiding values per column. The map's key is the column index, and the value is the criteria for that column. This field is deprecated in favor of filter_specs.
    
    pub criteria: Option<HashMap<String, FilterCriteria>>,
    /// The filter criteria for showing/hiding values per column. Both criteria and filter_specs are populated in responses. If both fields are specified in an update request, this field takes precedence.
    #[serde(rename="filterSpecs")]
    
    pub filter_specs: Option<Vec<FilterSpec>>,
    /// The ID of the filter view.
    #[serde(rename="filterViewId")]
    
    pub filter_view_id: Option<i32>,
    /// The named range this filter view is backed by, if any. When writing, only one of range or named_range_id may be set.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
    /// The range this filter view covers. When writing, only one of range or named_range_id may be set.
    
    pub range: Option<GridRange>,
    /// The sort order per column. Later specifications are used when values are equal in the earlier specifications.
    #[serde(rename="sortSpecs")]
    
    pub sort_specs: Option<Vec<SortSpec>>,
    /// The name of the filter view.
    
    pub title: Option<String>,
}

impl client::Part for FilterView {}


/// Finds and replaces data in cells over a range, sheet, or all sheets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindReplaceRequest {
    /// True to find/replace over all sheets.
    #[serde(rename="allSheets")]
    
    pub all_sheets: Option<bool>,
    /// The value to search.
    
    pub find: Option<String>,
    /// True if the search should include cells with formulas. False to skip cells with formulas.
    #[serde(rename="includeFormulas")]
    
    pub include_formulas: Option<bool>,
    /// True if the search is case sensitive.
    #[serde(rename="matchCase")]
    
    pub match_case: Option<bool>,
    /// True if the find value should match the entire cell.
    #[serde(rename="matchEntireCell")]
    
    pub match_entire_cell: Option<bool>,
    /// The range to find/replace over.
    
    pub range: Option<GridRange>,
    /// The value to use as the replacement.
    
    pub replacement: Option<String>,
    /// True if the find value is a regex. The regular expression and replacement should follow Java regex rules at https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html. The replacement string is allowed to refer to capturing groups. For example, if one cell has the contents `"Google Sheets"` and another has `"Google Docs"`, then searching for `"o.* (.*)"` with a replacement of `"$1 Rocks"` would change the contents of the cells to `"GSheets Rocks"` and `"GDocs Rocks"` respectively.
    #[serde(rename="searchByRegex")]
    
    pub search_by_regex: Option<bool>,
    /// The sheet to find/replace over.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for FindReplaceRequest {}


/// The result of the find/replace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    /// The number of formula cells changed.
    #[serde(rename="formulasChanged")]
    
    pub formulas_changed: Option<i32>,
    /// The number of occurrences (possibly multiple within a cell) changed. For example, if replacing `"e"` with `"o"` in `"Google Sheets"`, this would be `"3"` because `"Google Sheets"` -> `"Googlo Shoots"`.
    #[serde(rename="occurrencesChanged")]
    
    pub occurrences_changed: Option<i32>,
    /// The number of rows changed.
    #[serde(rename="rowsChanged")]
    
    pub rows_changed: Option<i32>,
    /// The number of sheets changed.
    #[serde(rename="sheetsChanged")]
    
    pub sheets_changed: Option<i32>,
    /// The number of non-formula cells changed.
    #[serde(rename="valuesChanged")]
    
    pub values_changed: Option<i32>,
}

impl client::Part for FindReplaceResponse {}


/// The request for retrieving a Spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get by data filter spreadsheets](SpreadsheetGetByDataFilterCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetSpreadsheetByDataFilterRequest {
    /// The DataFilters used to select which ranges to retrieve from the spreadsheet.
    #[serde(rename="dataFilters")]
    
    pub data_filters: Option<Vec<DataFilter>>,
    /// True if grid data should be returned. This parameter is ignored if a field mask was set in the request.
    #[serde(rename="includeGridData")]
    
    pub include_grid_data: Option<bool>,
}

impl client::RequestValue for GetSpreadsheetByDataFilterRequest {}


/// A rule that applies a gradient color scale format, based on the interpolation points listed. The format of a cell will vary based on its contents as compared to the values of the interpolation points.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GradientRule {
    /// The final interpolation point.
    
    pub maxpoint: Option<InterpolationPoint>,
    /// An optional midway interpolation point.
    
    pub midpoint: Option<InterpolationPoint>,
    /// The starting interpolation point.
    
    pub minpoint: Option<InterpolationPoint>,
}

impl client::Part for GradientRule {}


/// A coordinate in a sheet. All indexes are zero-based.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridCoordinate {
    /// The column index of the coordinate.
    #[serde(rename="columnIndex")]
    
    pub column_index: Option<i32>,
    /// The row index of the coordinate.
    #[serde(rename="rowIndex")]
    
    pub row_index: Option<i32>,
    /// The sheet this coordinate is on.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for GridCoordinate {}


/// Data in the grid, as well as metadata about the dimensions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridData {
    /// Metadata about the requested columns in the grid, starting with the column in start_column.
    #[serde(rename="columnMetadata")]
    
    pub column_metadata: Option<Vec<DimensionProperties>>,
    /// The data in the grid, one entry per row, starting with the row in startRow. The values in RowData will correspond to columns starting at start_column.
    #[serde(rename="rowData")]
    
    pub row_data: Option<Vec<RowData>>,
    /// Metadata about the requested rows in the grid, starting with the row in start_row.
    #[serde(rename="rowMetadata")]
    
    pub row_metadata: Option<Vec<DimensionProperties>>,
    /// The first column this GridData refers to, zero-based.
    #[serde(rename="startColumn")]
    
    pub start_column: Option<i32>,
    /// The first row this GridData refers to, zero-based.
    #[serde(rename="startRow")]
    
    pub start_row: Option<i32>,
}

impl client::Part for GridData {}


/// Properties of a grid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridProperties {
    /// The number of columns in the grid.
    #[serde(rename="columnCount")]
    
    pub column_count: Option<i32>,
    /// True if the column grouping control toggle is shown after the group.
    #[serde(rename="columnGroupControlAfter")]
    
    pub column_group_control_after: Option<bool>,
    /// The number of columns that are frozen in the grid.
    #[serde(rename="frozenColumnCount")]
    
    pub frozen_column_count: Option<i32>,
    /// The number of rows that are frozen in the grid.
    #[serde(rename="frozenRowCount")]
    
    pub frozen_row_count: Option<i32>,
    /// True if the grid isn't showing gridlines in the UI.
    #[serde(rename="hideGridlines")]
    
    pub hide_gridlines: Option<bool>,
    /// The number of rows in the grid.
    #[serde(rename="rowCount")]
    
    pub row_count: Option<i32>,
    /// True if the row grouping control toggle is shown after the group.
    #[serde(rename="rowGroupControlAfter")]
    
    pub row_group_control_after: Option<bool>,
}

impl client::Part for GridProperties {}


/// A range on a sheet. All indexes are zero-based. Indexes are half open, i.e. the start index is inclusive and the end index is exclusive -- [start_index, end_index). Missing indexes indicate the range is unbounded on that side. For example, if `"Sheet1"` is sheet ID 123456, then: `Sheet1!A1:A1 == sheet_id: 123456, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1` `Sheet1!A3:B4 == sheet_id: 123456, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1!A:B == sheet_id: 123456, start_column_index: 0, end_column_index: 2` `Sheet1!A5:B == sheet_id: 123456, start_row_index: 4, start_column_index: 0, end_column_index: 2` `Sheet1 == sheet_id: 123456` The start index must always be less than or equal to the end index. If the start index equals the end index, then the range is empty. Empty ranges are typically not meaningful and are usually rendered in the UI as `#REF!`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridRange {
    /// The end column (exclusive) of the range, or not set if unbounded.
    #[serde(rename="endColumnIndex")]
    
    pub end_column_index: Option<i32>,
    /// The end row (exclusive) of the range, or not set if unbounded.
    #[serde(rename="endRowIndex")]
    
    pub end_row_index: Option<i32>,
    /// The sheet this range is on.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
    /// The start column (inclusive) of the range, or not set if unbounded.
    #[serde(rename="startColumnIndex")]
    
    pub start_column_index: Option<i32>,
    /// The start row (inclusive) of the range, or not set if unbounded.
    #[serde(rename="startRowIndex")]
    
    pub start_row_index: Option<i32>,
}

impl client::Part for GridRange {}


/// A histogram chart. A histogram chart groups data items into bins, displaying each bin as a column of stacked items. Histograms are used to display the distribution of a dataset. Each column of items represents a range into which those items fall. The number of bins can be chosen automatically or specified explicitly.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramChartSpec {
    /// By default the bucket size (the range of values stacked in a single column) is chosen automatically, but it may be overridden here. E.g., A bucket size of 1.5 results in buckets from 0 - 1.5, 1.5 - 3.0, etc. Cannot be negative. This field is optional.
    #[serde(rename="bucketSize")]
    
    pub bucket_size: Option<f64>,
    /// The position of the chart legend.
    #[serde(rename="legendPosition")]
    
    pub legend_position: Option<String>,
    /// The outlier percentile is used to ensure that outliers do not adversely affect the calculation of bucket sizes. For example, setting an outlier percentile of 0.05 indicates that the top and bottom 5% of values when calculating buckets. The values are still included in the chart, they will be added to the first or last buckets instead of their own buckets. Must be between 0.0 and 0.5.
    #[serde(rename="outlierPercentile")]
    
    pub outlier_percentile: Option<f64>,
    /// The series for a histogram may be either a single series of values to be bucketed or multiple series, each of the same length, containing the name of the series followed by the values to be bucketed for that series.
    
    pub series: Option<Vec<HistogramSeries>>,
    /// Whether horizontal divider lines should be displayed between items in each column.
    #[serde(rename="showItemDividers")]
    
    pub show_item_dividers: Option<bool>,
}

impl client::Part for HistogramChartSpec {}


/// Allows you to organize the numeric values in a source data column into buckets of a constant size. All values from HistogramRule.start to HistogramRule.end are placed into groups of size HistogramRule.interval. In addition, all values below HistogramRule.start are placed in one group, and all values above HistogramRule.end are placed in another. Only HistogramRule.interval is required, though if HistogramRule.start and HistogramRule.end are both provided, HistogramRule.start must be less than HistogramRule.end. For example, a pivot table showing average purchase amount by age that has 50+ rows: +-----+-------------------+ | Age | AVERAGE of Amount | +-----+-------------------+ | 16 | $27.13 | | 17 | $5.24 | | 18 | $20.15 | ... +-----+-------------------+ could be turned into a pivot table that looks like the one below by applying a histogram group rule with a HistogramRule.start of 25, an HistogramRule.interval of 20, and an HistogramRule.end of 65. +-------------+-------------------+ | Grouped Age | AVERAGE of Amount | +-------------+-------------------+ | < 25 | $19.34 | | 25-45 | $31.43 | | 45-65 | $35.87 | | > 65 | $27.55 | +-------------+-------------------+ | Grand Total | $29.12 | +-------------+-------------------+
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramRule {
    /// The maximum value at which items are placed into buckets of constant size. Values above end are lumped into a single bucket. This field is optional.
    
    pub end: Option<f64>,
    /// The size of the buckets that are created. Must be positive.
    
    pub interval: Option<f64>,
    /// The minimum value at which items are placed into buckets of constant size. Values below start are lumped into a single bucket. This field is optional.
    
    pub start: Option<f64>,
}

impl client::Part for HistogramRule {}


/// A histogram series containing the series color and data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramSeries {
    /// The color of the column representing this series in each bucket. This field is optional. Deprecated: Use bar_color_style.
    #[serde(rename="barColor")]
    
    pub bar_color: Option<Color>,
    /// The color of the column representing this series in each bucket. This field is optional. If bar_color is also set, this field takes precedence.
    #[serde(rename="barColorStyle")]
    
    pub bar_color_style: Option<ColorStyle>,
    /// The data for this histogram series.
    
    pub data: Option<ChartData>,
}

impl client::Part for HistogramSeries {}


/// Inserts rows or columns in a sheet at a particular index.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertDimensionRequest {
    /// Whether dimension properties should be extended from the dimensions before or after the newly inserted dimensions. True to inherit from the dimensions before (in which case the start index must be greater than 0), and false to inherit from the dimensions after. For example, if row index 0 has red background and row index 1 has a green background, then inserting 2 rows at index 1 can inherit either the green or red background. If `inheritFromBefore` is true, the two new rows will be red (because the row before the insertion point was red), whereas if `inheritFromBefore` is false, the two new rows will be green (because the row after the insertion point was green).
    #[serde(rename="inheritFromBefore")]
    
    pub inherit_from_before: Option<bool>,
    /// The dimensions to insert. Both the start and end indexes must be bounded.
    
    pub range: Option<DimensionRange>,
}

impl client::Part for InsertDimensionRequest {}


/// Inserts cells into a range, shifting the existing cells over or down.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertRangeRequest {
    /// The range to insert new cells into.
    
    pub range: Option<GridRange>,
    /// The dimension which will be shifted when inserting cells. If ROWS, existing cells will be shifted down. If COLUMNS, existing cells will be shifted right.
    #[serde(rename="shiftDimension")]
    
    pub shift_dimension: Option<String>,
}

impl client::Part for InsertRangeRequest {}


/// A single interpolation point on a gradient conditional format. These pin the gradient color scale according to the color, type and value chosen.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InterpolationPoint {
    /// The color this interpolation point should use. Deprecated: Use color_style.
    
    pub color: Option<Color>,
    /// The color this interpolation point should use. If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    
    pub color_style: Option<ColorStyle>,
    /// How the value should be interpreted.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The value this interpolation point uses. May be a formula. Unused if type is MIN or MAX.
    
    pub value: Option<String>,
}

impl client::Part for InterpolationPoint {}


/// Represents a time interval, encoded as a Timestamp start (inclusive) and a Timestamp end (exclusive). The start must be less than or equal to the end. When the start equals the end, the interval is empty (matches no time). When both start and end are unspecified, the interval matches any time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Interval {
    /// Optional. Exclusive end of the interval. If specified, a Timestamp matching this interval will have to be before the end.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Inclusive start of the interval. If specified, a Timestamp matching this interval will have to be the same or after the start.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for Interval {}


/// Settings to control how circular dependencies are resolved with iterative calculation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IterativeCalculationSettings {
    /// When iterative calculation is enabled and successive results differ by less than this threshold value, the calculation rounds stop.
    #[serde(rename="convergenceThreshold")]
    
    pub convergence_threshold: Option<f64>,
    /// When iterative calculation is enabled, the maximum number of calculation rounds to perform.
    #[serde(rename="maxIterations")]
    
    pub max_iterations: Option<i32>,
}

impl client::Part for IterativeCalculationSettings {}


/// Formatting options for key value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyValueFormat {
    /// Specifies the horizontal text positioning of key value. This field is optional. If not specified, default positioning is used.
    
    pub position: Option<TextPosition>,
    /// Text formatting options for key value. The link field is not supported.
    #[serde(rename="textFormat")]
    
    pub text_format: Option<TextFormat>,
}

impl client::Part for KeyValueFormat {}


/// Properties that describe the style of a line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineStyle {
    /// The dash type of the line.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The thickness of the line, in px.
    
    pub width: Option<i32>,
}

impl client::Part for LineStyle {}


/// An external or local reference.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// The link identifier.
    
    pub uri: Option<String>,
}

impl client::Part for Link {}


/// Allows you to manually organize the values in a source data column into buckets with names of your choosing. For example, a pivot table that aggregates population by state: +-------+-------------------+ | State | SUM of Population | +-------+-------------------+ | AK | 0.7 | | AL | 4.8 | | AR | 2.9 | ... +-------+-------------------+ could be turned into a pivot table that aggregates population by time zone by providing a list of groups (for example, groupName = 'Central', items = ['AL', 'AR', 'IA', ...]) to a manual group rule. Note that a similar effect could be achieved by adding a time zone column to the source data and adjusting the pivot table. +-----------+-------------------+ | Time Zone | SUM of Population | +-----------+-------------------+ | Central | 106.3 | | Eastern | 151.9 | | Mountain | 17.4 | ... +-----------+-------------------+
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualRule {
    /// The list of group names and the corresponding items from the source data that map to each group name.
    
    pub groups: Option<Vec<ManualRuleGroup>>,
}

impl client::Part for ManualRule {}


/// A group name and a list of items from the source data that should be placed in the group with this name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualRuleGroup {
    /// The group name, which must be a string. Each group in a given ManualRule must have a unique group name.
    #[serde(rename="groupName")]
    
    pub group_name: Option<ExtendedValue>,
    /// The items in the source data that should be placed into this group. Each item may be a string, number, or boolean. Items may appear in at most one group within a given ManualRule. Items that do not appear in any group will appear on their own.
    
    pub items: Option<Vec<ExtendedValue>>,
}

impl client::Part for ManualRuleGroup {}


/// A developer metadata entry and the data filters specified in the original request that matched it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatchedDeveloperMetadata {
    /// All filters matching the returned developer metadata.
    #[serde(rename="dataFilters")]
    
    pub data_filters: Option<Vec<DataFilter>>,
    /// The developer metadata matching the specified filters.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<DeveloperMetadata>,
}

impl client::Part for MatchedDeveloperMetadata {}


/// A value range that was matched by one or more data filers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatchedValueRange {
    /// The DataFilters from the request that matched the range of values.
    #[serde(rename="dataFilters")]
    
    pub data_filters: Option<Vec<DataFilter>>,
    /// The values matched by the DataFilter.
    #[serde(rename="valueRange")]
    
    pub value_range: Option<ValueRange>,
}

impl client::Part for MatchedValueRange {}


/// Merges all cells in the range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MergeCellsRequest {
    /// How the cells should be merged.
    #[serde(rename="mergeType")]
    
    pub merge_type: Option<String>,
    /// The range of cells to merge.
    
    pub range: Option<GridRange>,
}

impl client::Part for MergeCellsRequest {}


/// Moves one or more rows or columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveDimensionRequest {
    /// The zero-based start index of where to move the source data to, based on the coordinates *before* the source data is removed from the grid. Existing data will be shifted down or right (depending on the dimension) to make room for the moved dimensions. The source dimensions are removed from the grid, so the the data may end up in a different index than specified. For example, given `A1..A5` of `0, 1, 2, 3, 4` and wanting to move `"1"` and `"2"` to between `"3"` and `"4"`, the source would be `ROWS [1..3)`,and the destination index would be `"4"` (the zero-based index of row 5). The end result would be `A1..A5` of `0, 3, 1, 2, 4`.
    #[serde(rename="destinationIndex")]
    
    pub destination_index: Option<i32>,
    /// The source dimensions to move.
    
    pub source: Option<DimensionRange>,
}

impl client::Part for MoveDimensionRequest {}


/// A named range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedRange {
    /// The name of the named range.
    
    pub name: Option<String>,
    /// The ID of the named range.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
    /// The range this represents.
    
    pub range: Option<GridRange>,
}

impl client::Part for NamedRange {}


/// The number format of a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumberFormat {
    /// Pattern string used for formatting. If not set, a default pattern based on the user’s locale will be used if necessary for the given type. See the [Date and Number Formats guide](https://developers.google.com/sheets/api/guides/formats) for more information about the supported patterns.
    
    pub pattern: Option<String>,
    /// The type of the number format. When writing, this field must be set.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for NumberFormat {}


/// An org chart. Org charts require a unique set of labels in labels and may optionally include parent_labels and tooltips. parent_labels contain, for each node, the label identifying the parent node. tooltips contain, for each node, an optional tooltip. For example, to describe an OrgChart with Alice as the CEO, Bob as the President (reporting to Alice) and Cathy as VP of Sales (also reporting to Alice), have labels contain "Alice", "Bob", "Cathy", parent_labels contain "", "Alice", "Alice" and tooltips contain "CEO", "President", "VP Sales".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrgChartSpec {
    /// The data containing the labels for all the nodes in the chart. Labels must be unique.
    
    pub labels: Option<ChartData>,
    /// The color of the org chart nodes. Deprecated: Use node_color_style.
    #[serde(rename="nodeColor")]
    
    pub node_color: Option<Color>,
    /// The color of the org chart nodes. If node_color is also set, this field takes precedence.
    #[serde(rename="nodeColorStyle")]
    
    pub node_color_style: Option<ColorStyle>,
    /// The size of the org chart nodes.
    #[serde(rename="nodeSize")]
    
    pub node_size: Option<String>,
    /// The data containing the label of the parent for the corresponding node. A blank value indicates that the node has no parent and is a top-level node. This field is optional.
    #[serde(rename="parentLabels")]
    
    pub parent_labels: Option<ChartData>,
    /// The color of the selected org chart nodes. Deprecated: Use selected_node_color_style.
    #[serde(rename="selectedNodeColor")]
    
    pub selected_node_color: Option<Color>,
    /// The color of the selected org chart nodes. If selected_node_color is also set, this field takes precedence.
    #[serde(rename="selectedNodeColorStyle")]
    
    pub selected_node_color_style: Option<ColorStyle>,
    /// The data containing the tooltip for the corresponding node. A blank value results in no tooltip being displayed for the node. This field is optional.
    
    pub tooltips: Option<ChartData>,
}

impl client::Part for OrgChartSpec {}


/// The location an object is overlaid on top of a grid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OverlayPosition {
    /// The cell the object is anchored to.
    #[serde(rename="anchorCell")]
    
    pub anchor_cell: Option<GridCoordinate>,
    /// The height of the object, in pixels. Defaults to 371.
    #[serde(rename="heightPixels")]
    
    pub height_pixels: Option<i32>,
    /// The horizontal offset, in pixels, that the object is offset from the anchor cell.
    #[serde(rename="offsetXPixels")]
    
    pub offset_x_pixels: Option<i32>,
    /// The vertical offset, in pixels, that the object is offset from the anchor cell.
    #[serde(rename="offsetYPixels")]
    
    pub offset_y_pixels: Option<i32>,
    /// The width of the object, in pixels. Defaults to 600.
    #[serde(rename="widthPixels")]
    
    pub width_pixels: Option<i32>,
}

impl client::Part for OverlayPosition {}


/// The amount of padding around the cell, in pixels. When updating padding, every field must be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Padding {
    /// The bottom padding of the cell.
    
    pub bottom: Option<i32>,
    /// The left padding of the cell.
    
    pub left: Option<i32>,
    /// The right padding of the cell.
    
    pub right: Option<i32>,
    /// The top padding of the cell.
    
    pub top: Option<i32>,
}

impl client::Part for Padding {}


/// Inserts data into the spreadsheet starting at the specified coordinate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PasteDataRequest {
    /// The coordinate at which the data should start being inserted.
    
    pub coordinate: Option<GridCoordinate>,
    /// The data to insert.
    
    pub data: Option<String>,
    /// The delimiter in the data.
    
    pub delimiter: Option<String>,
    /// True if the data is HTML.
    
    pub html: Option<bool>,
    /// How the data should be pasted.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for PasteDataRequest {}


/// A pie chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PieChartSpec {
    /// The data that covers the domain of the pie chart.
    
    pub domain: Option<ChartData>,
    /// Where the legend of the pie chart should be drawn.
    #[serde(rename="legendPosition")]
    
    pub legend_position: Option<String>,
    /// The size of the hole in the pie chart.
    #[serde(rename="pieHole")]
    
    pub pie_hole: Option<f64>,
    /// The data that covers the one and only series of the pie chart.
    
    pub series: Option<ChartData>,
    /// True if the pie is three dimensional.
    #[serde(rename="threeDimensional")]
    
    pub three_dimensional: Option<bool>,
}

impl client::Part for PieChartSpec {}


/// Criteria for showing/hiding rows in a pivot table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotFilterCriteria {
    /// A condition that must be true for values to be shown. (`visibleValues` does not override this -- even if a value is listed there, it is still hidden if it does not meet the condition.) Condition values that refer to ranges in A1-notation are evaluated relative to the pivot table sheet. References are treated absolutely, so are not filled down the pivot table. For example, a condition value of `=A1` on "Pivot Table 1" is treated as `'Pivot Table 1'!$A$1`. The source data of the pivot table can be referenced by column header name. For example, if the source data has columns named "Revenue" and "Cost" and a condition is applied to the "Revenue" column with type `NUMBER_GREATER` and value `=Cost`, then only columns where "Revenue" > "Cost" are included.
    
    pub condition: Option<BooleanCondition>,
    /// Whether values are visible by default. If true, the visible_values are ignored, all values that meet condition (if specified) are shown. If false, values that are both in visible_values and meet condition are shown.
    #[serde(rename="visibleByDefault")]
    
    pub visible_by_default: Option<bool>,
    /// Values that should be included. Values not listed here are excluded.
    #[serde(rename="visibleValues")]
    
    pub visible_values: Option<Vec<String>>,
}

impl client::Part for PivotFilterCriteria {}


/// The pivot table filter criteria associated with a specific source column offset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotFilterSpec {
    /// The zero-based column offset of the source range.
    #[serde(rename="columnOffsetIndex")]
    
    pub column_offset_index: Option<i32>,
    /// The reference to the data source column.
    #[serde(rename="dataSourceColumnReference")]
    
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /// The criteria for the column.
    #[serde(rename="filterCriteria")]
    
    pub filter_criteria: Option<PivotFilterCriteria>,
}

impl client::Part for PivotFilterSpec {}


/// A single grouping (either row or column) in a pivot table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroup {
    /// The reference to the data source column this grouping is based on.
    #[serde(rename="dataSourceColumnReference")]
    
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /// The count limit on rows or columns to apply to this pivot group.
    #[serde(rename="groupLimit")]
    
    pub group_limit: Option<PivotGroupLimit>,
    /// The group rule to apply to this row/column group.
    #[serde(rename="groupRule")]
    
    pub group_rule: Option<PivotGroupRule>,
    /// The labels to use for the row/column groups which can be customized. For example, in the following pivot table, the row label is `Region` (which could be renamed to `State`) and the column label is `Product` (which could be renamed `Item`). Pivot tables created before December 2017 do not have header labels. If you'd like to add header labels to an existing pivot table, please delete the existing pivot table and then create a new pivot table with same parameters. +--------------+---------+-------+ | SUM of Units | Product | | | Region | Pen | Paper | +--------------+---------+-------+ | New York | 345 | 98 | | Oregon | 234 | 123 | | Tennessee | 531 | 415 | +--------------+---------+-------+ | Grand Total | 1110 | 636 | +--------------+---------+-------+
    
    pub label: Option<String>,
    /// True if the headings in this pivot group should be repeated. This is only valid for row groupings and is ignored by columns. By default, we minimize repetition of headings by not showing higher level headings where they are the same. For example, even though the third row below corresponds to "Q1 Mar", "Q1" is not shown because it is redundant with previous rows. Setting repeat_headings to true would cause "Q1" to be repeated for "Feb" and "Mar". +--------------+ | Q1 | Jan | | | Feb | | | Mar | +--------+-----+ | Q1 Total | +--------------+
    #[serde(rename="repeatHeadings")]
    
    pub repeat_headings: Option<bool>,
    /// True if the pivot table should include the totals for this grouping.
    #[serde(rename="showTotals")]
    
    pub show_totals: Option<bool>,
    /// The order the values in this group should be sorted.
    #[serde(rename="sortOrder")]
    
    pub sort_order: Option<String>,
    /// The column offset of the source range that this grouping is based on. For example, if the source was `C10:E15`, a `sourceColumnOffset` of `0` means this group refers to column `C`, whereas the offset `1` would refer to column `D`.
    #[serde(rename="sourceColumnOffset")]
    
    pub source_column_offset: Option<i32>,
    /// The bucket of the opposite pivot group to sort by. If not specified, sorting is alphabetical by this group's values.
    #[serde(rename="valueBucket")]
    
    pub value_bucket: Option<PivotGroupSortValueBucket>,
    /// Metadata about values in the grouping.
    #[serde(rename="valueMetadata")]
    
    pub value_metadata: Option<Vec<PivotGroupValueMetadata>>,
}

impl client::Part for PivotGroup {}


/// The count limit on rows or columns in the pivot group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroupLimit {
    /// The order in which the group limit is applied to the pivot table. Pivot group limits are applied from lower to higher order number. Order numbers are normalized to consecutive integers from 0. For write request, to fully customize the applying orders, all pivot group limits should have this field set with an unique number. Otherwise, the order is determined by the index in the PivotTable.rows list and then the PivotTable.columns list.
    #[serde(rename="applyOrder")]
    
    pub apply_order: Option<i32>,
    /// The count limit.
    #[serde(rename="countLimit")]
    
    pub count_limit: Option<i32>,
}

impl client::Part for PivotGroupLimit {}


/// An optional setting on a PivotGroup that defines buckets for the values in the source data column rather than breaking out each individual value. Only one PivotGroup with a group rule may be added for each column in the source data, though on any given column you may add both a PivotGroup that has a rule and a PivotGroup that does not.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroupRule {
    /// A DateTimeRule.
    #[serde(rename="dateTimeRule")]
    
    pub date_time_rule: Option<DateTimeRule>,
    /// A HistogramRule.
    #[serde(rename="histogramRule")]
    
    pub histogram_rule: Option<HistogramRule>,
    /// A ManualRule.
    #[serde(rename="manualRule")]
    
    pub manual_rule: Option<ManualRule>,
}

impl client::Part for PivotGroupRule {}


/// Information about which values in a pivot group should be used for sorting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroupSortValueBucket {
    /// Determines the bucket from which values are chosen to sort. For example, in a pivot table with one row group & two column groups, the row group can list up to two values. The first value corresponds to a value within the first column group, and the second value corresponds to a value in the second column group. If no values are listed, this would indicate that the row should be sorted according to the "Grand Total" over the column groups. If a single value is listed, this would correspond to using the "Total" of that bucket.
    
    pub buckets: Option<Vec<ExtendedValue>>,
    /// The offset in the PivotTable.values list which the values in this grouping should be sorted by.
    #[serde(rename="valuesIndex")]
    
    pub values_index: Option<i32>,
}

impl client::Part for PivotGroupSortValueBucket {}


/// Metadata about a value in a pivot grouping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroupValueMetadata {
    /// True if the data corresponding to the value is collapsed.
    
    pub collapsed: Option<bool>,
    /// The calculated value the metadata corresponds to. (Note that formulaValue is not valid, because the values will be calculated.)
    
    pub value: Option<ExtendedValue>,
}

impl client::Part for PivotGroupValueMetadata {}


/// A pivot table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotTable {
    /// Each column grouping in the pivot table.
    
    pub columns: Option<Vec<PivotGroup>>,
    /// An optional mapping of filters per source column offset. The filters are applied before aggregating data into the pivot table. The map's key is the column offset of the source range that you want to filter, and the value is the criteria for that column. For example, if the source was `C10:E15`, a key of `0` will have the filter for column `C`, whereas the key `1` is for column `D`. This field is deprecated in favor of filter_specs.
    
    pub criteria: Option<HashMap<String, PivotFilterCriteria>>,
    /// Output only. The data execution status for data source pivot tables.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// The ID of the data source the pivot table is reading data from.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// The filters applied to the source columns before aggregating data for the pivot table. Both criteria and filter_specs are populated in responses. If both fields are specified in an update request, this field takes precedence.
    #[serde(rename="filterSpecs")]
    
    pub filter_specs: Option<Vec<PivotFilterSpec>>,
    /// Each row grouping in the pivot table.
    
    pub rows: Option<Vec<PivotGroup>>,
    /// The range the pivot table is reading data from.
    
    pub source: Option<GridRange>,
    /// Whether values should be listed horizontally (as columns) or vertically (as rows).
    #[serde(rename="valueLayout")]
    
    pub value_layout: Option<String>,
    /// A list of values to include in the pivot table.
    
    pub values: Option<Vec<PivotValue>>,
}

impl client::Part for PivotTable {}


/// The definition of how a value in a pivot table should be calculated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotValue {
    /// If specified, indicates that pivot values should be displayed as the result of a calculation with another pivot value. For example, if calculated_display_type is specified as PERCENT_OF_GRAND_TOTAL, all the pivot values are displayed as the percentage of the grand total. In the Sheets editor, this is referred to as "Show As" in the value section of a pivot table.
    #[serde(rename="calculatedDisplayType")]
    
    pub calculated_display_type: Option<String>,
    /// The reference to the data source column that this value reads from.
    #[serde(rename="dataSourceColumnReference")]
    
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /// A custom formula to calculate the value. The formula must start with an `=` character.
    
    pub formula: Option<String>,
    /// A name to use for the value.
    
    pub name: Option<String>,
    /// The column offset of the source range that this value reads from. For example, if the source was `C10:E15`, a `sourceColumnOffset` of `0` means this value refers to column `C`, whereas the offset `1` would refer to column `D`.
    #[serde(rename="sourceColumnOffset")]
    
    pub source_column_offset: Option<i32>,
    /// A function to summarize the value. If formula is set, the only supported values are SUM and CUSTOM. If sourceColumnOffset is set, then `CUSTOM` is not supported.
    #[serde(rename="summarizeFunction")]
    
    pub summarize_function: Option<String>,
}

impl client::Part for PivotValue {}


/// The style of a point on the chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointStyle {
    /// The point shape. If empty or unspecified, a default shape is used.
    
    pub shape: Option<String>,
    /// The point size. If empty, a default size is used.
    
    pub size: Option<f64>,
}

impl client::Part for PointStyle {}


/// A protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProtectedRange {
    /// The description of this protected range.
    
    pub description: Option<String>,
    /// The users and groups with edit access to the protected range. This field is only visible to users with edit access to the protected range and the document. Editors are not supported with warning_only protection.
    
    pub editors: Option<Editors>,
    /// The named range this protected range is backed by, if any. When writing, only one of range or named_range_id may be set.
    #[serde(rename="namedRangeId")]
    
    pub named_range_id: Option<String>,
    /// The ID of the protected range. This field is read-only.
    #[serde(rename="protectedRangeId")]
    
    pub protected_range_id: Option<i32>,
    /// The range that is being protected. The range may be fully unbounded, in which case this is considered a protected sheet. When writing, only one of range or named_range_id may be set.
    
    pub range: Option<GridRange>,
    /// True if the user who requested this protected range can edit the protected area. This field is read-only.
    #[serde(rename="requestingUserCanEdit")]
    
    pub requesting_user_can_edit: Option<bool>,
    /// The list of unprotected ranges within a protected sheet. Unprotected ranges are only supported on protected sheets.
    #[serde(rename="unprotectedRanges")]
    
    pub unprotected_ranges: Option<Vec<GridRange>>,
    /// True if this protected range will show a warning when editing. Warning-based protection means that every user can edit data in the protected range, except editing will prompt a warning asking the user to confirm the edit. When writing: if this field is true, then editors are ignored. Additionally, if this field is changed from true to false and the `editors` field is not set (nor included in the field mask), then the editors will be set to all the editors in the document.
    #[serde(rename="warningOnly")]
    
    pub warning_only: Option<bool>,
}

impl client::Part for ProtectedRange {}


/// Randomizes the order of the rows in a range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RandomizeRangeRequest {
    /// The range to randomize.
    
    pub range: Option<GridRange>,
}

impl client::Part for RandomizeRangeRequest {}


/// The status of a refresh cancellation. You can send a cancel request to explicitly cancel one or multiple data source object refreshes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshCancellationStatus {
    /// The error code.
    #[serde(rename="errorCode")]
    
    pub error_code: Option<String>,
    /// The state of a call to cancel a refresh in Sheets.
    
    pub state: Option<String>,
}

impl client::Part for RefreshCancellationStatus {}


/// The execution status of refreshing one data source object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshDataSourceObjectExecutionStatus {
    /// The data execution status.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// Reference to a data source object being refreshed.
    
    pub reference: Option<DataSourceObjectReference>,
}

impl client::Part for RefreshDataSourceObjectExecutionStatus {}


/// Refreshes one or multiple data source objects in the spreadsheet by the specified references. The request requires an additional `bigquery.readonly` OAuth scope. If there are multiple refresh requests referencing the same data source objects in one batch, only the last refresh request is processed, and all those requests will have the same response accordingly.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshDataSourceRequest {
    /// Reference to a DataSource. If specified, refreshes all associated data source objects for the data source.
    #[serde(rename="dataSourceId")]
    
    pub data_source_id: Option<String>,
    /// Refreshes the data source objects regardless of the current state. If not set and a referenced data source object was in error state, the refresh will fail immediately.
    
    pub force: Option<bool>,
    /// Refreshes all existing data source objects in the spreadsheet.
    #[serde(rename="isAll")]
    
    pub is_all: Option<bool>,
    /// References to data source objects to refresh.
    
    pub references: Option<DataSourceObjectReferences>,
}

impl client::Part for RefreshDataSourceRequest {}


/// The response from refreshing one or multiple data source objects.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshDataSourceResponse {
    /// All the refresh status for the data source object references specified in the request. If is_all is specified, the field contains only those in failure status.
    
    pub statuses: Option<Vec<RefreshDataSourceObjectExecutionStatus>>,
}

impl client::Part for RefreshDataSourceResponse {}


/// Updates all cells in the range to the values in the given Cell object. Only the fields listed in the fields field are updated; others are unchanged. If writing a cell with a formula, the formula's ranges will automatically increment for each field in the range. For example, if writing a cell with formula `=A1` into range B2:C4, B2 would be `=A1`, B3 would be `=A2`, B4 would be `=A3`, C2 would be `=B1`, C3 would be `=B2`, C4 would be `=B3`. To keep the formula's ranges static, use the `$` indicator. For example, use the formula `=$A$1` to prevent both the row and the column from incrementing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepeatCellRequest {
    /// The data to write.
    
    pub cell: Option<CellData>,
    /// The fields that should be updated. At least one field must be specified. The root `cell` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The range to repeat the cell in.
    
    pub range: Option<GridRange>,
}

impl client::Part for RepeatCellRequest {}


/// A single kind of update to apply to a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// Adds a new banded range
    #[serde(rename="addBanding")]
    
    pub add_banding: Option<AddBandingRequest>,
    /// Adds a chart.
    #[serde(rename="addChart")]
    
    pub add_chart: Option<AddChartRequest>,
    /// Adds a new conditional format rule.
    #[serde(rename="addConditionalFormatRule")]
    
    pub add_conditional_format_rule: Option<AddConditionalFormatRuleRequest>,
    /// Adds a data source.
    #[serde(rename="addDataSource")]
    
    pub add_data_source: Option<AddDataSourceRequest>,
    /// Creates a group over the specified range.
    #[serde(rename="addDimensionGroup")]
    
    pub add_dimension_group: Option<AddDimensionGroupRequest>,
    /// Adds a filter view.
    #[serde(rename="addFilterView")]
    
    pub add_filter_view: Option<AddFilterViewRequest>,
    /// Adds a named range.
    #[serde(rename="addNamedRange")]
    
    pub add_named_range: Option<AddNamedRangeRequest>,
    /// Adds a protected range.
    #[serde(rename="addProtectedRange")]
    
    pub add_protected_range: Option<AddProtectedRangeRequest>,
    /// Adds a sheet.
    #[serde(rename="addSheet")]
    
    pub add_sheet: Option<AddSheetRequest>,
    /// Adds a slicer.
    #[serde(rename="addSlicer")]
    
    pub add_slicer: Option<AddSlicerRequest>,
    /// Appends cells after the last row with data in a sheet.
    #[serde(rename="appendCells")]
    
    pub append_cells: Option<AppendCellsRequest>,
    /// Appends dimensions to the end of a sheet.
    #[serde(rename="appendDimension")]
    
    pub append_dimension: Option<AppendDimensionRequest>,
    /// Automatically fills in more data based on existing data.
    #[serde(rename="autoFill")]
    
    pub auto_fill: Option<AutoFillRequest>,
    /// Automatically resizes one or more dimensions based on the contents of the cells in that dimension.
    #[serde(rename="autoResizeDimensions")]
    
    pub auto_resize_dimensions: Option<AutoResizeDimensionsRequest>,
    /// Cancels refreshes of one or multiple data sources and associated dbobjects.
    #[serde(rename="cancelDataSourceRefresh")]
    
    pub cancel_data_source_refresh: Option<CancelDataSourceRefreshRequest>,
    /// Clears the basic filter on a sheet.
    #[serde(rename="clearBasicFilter")]
    
    pub clear_basic_filter: Option<ClearBasicFilterRequest>,
    /// Copies data from one area and pastes it to another.
    #[serde(rename="copyPaste")]
    
    pub copy_paste: Option<CopyPasteRequest>,
    /// Creates new developer metadata
    #[serde(rename="createDeveloperMetadata")]
    
    pub create_developer_metadata: Option<CreateDeveloperMetadataRequest>,
    /// Cuts data from one area and pastes it to another.
    #[serde(rename="cutPaste")]
    
    pub cut_paste: Option<CutPasteRequest>,
    /// Removes a banded range
    #[serde(rename="deleteBanding")]
    
    pub delete_banding: Option<DeleteBandingRequest>,
    /// Deletes an existing conditional format rule.
    #[serde(rename="deleteConditionalFormatRule")]
    
    pub delete_conditional_format_rule: Option<DeleteConditionalFormatRuleRequest>,
    /// Deletes a data source.
    #[serde(rename="deleteDataSource")]
    
    pub delete_data_source: Option<DeleteDataSourceRequest>,
    /// Deletes developer metadata
    #[serde(rename="deleteDeveloperMetadata")]
    
    pub delete_developer_metadata: Option<DeleteDeveloperMetadataRequest>,
    /// Deletes rows or columns in a sheet.
    #[serde(rename="deleteDimension")]
    
    pub delete_dimension: Option<DeleteDimensionRequest>,
    /// Deletes a group over the specified range.
    #[serde(rename="deleteDimensionGroup")]
    
    pub delete_dimension_group: Option<DeleteDimensionGroupRequest>,
    /// Removes rows containing duplicate values in specified columns of a cell range.
    #[serde(rename="deleteDuplicates")]
    
    pub delete_duplicates: Option<DeleteDuplicatesRequest>,
    /// Deletes an embedded object (e.g, chart, image) in a sheet.
    #[serde(rename="deleteEmbeddedObject")]
    
    pub delete_embedded_object: Option<DeleteEmbeddedObjectRequest>,
    /// Deletes a filter view from a sheet.
    #[serde(rename="deleteFilterView")]
    
    pub delete_filter_view: Option<DeleteFilterViewRequest>,
    /// Deletes a named range.
    #[serde(rename="deleteNamedRange")]
    
    pub delete_named_range: Option<DeleteNamedRangeRequest>,
    /// Deletes a protected range.
    #[serde(rename="deleteProtectedRange")]
    
    pub delete_protected_range: Option<DeleteProtectedRangeRequest>,
    /// Deletes a range of cells from a sheet, shifting the remaining cells.
    #[serde(rename="deleteRange")]
    
    pub delete_range: Option<DeleteRangeRequest>,
    /// Deletes a sheet.
    #[serde(rename="deleteSheet")]
    
    pub delete_sheet: Option<DeleteSheetRequest>,
    /// Duplicates a filter view.
    #[serde(rename="duplicateFilterView")]
    
    pub duplicate_filter_view: Option<DuplicateFilterViewRequest>,
    /// Duplicates a sheet.
    #[serde(rename="duplicateSheet")]
    
    pub duplicate_sheet: Option<DuplicateSheetRequest>,
    /// Finds and replaces occurrences of some text with other text.
    #[serde(rename="findReplace")]
    
    pub find_replace: Option<FindReplaceRequest>,
    /// Inserts new rows or columns in a sheet.
    #[serde(rename="insertDimension")]
    
    pub insert_dimension: Option<InsertDimensionRequest>,
    /// Inserts new cells in a sheet, shifting the existing cells.
    #[serde(rename="insertRange")]
    
    pub insert_range: Option<InsertRangeRequest>,
    /// Merges cells together.
    #[serde(rename="mergeCells")]
    
    pub merge_cells: Option<MergeCellsRequest>,
    /// Moves rows or columns to another location in a sheet.
    #[serde(rename="moveDimension")]
    
    pub move_dimension: Option<MoveDimensionRequest>,
    /// Pastes data (HTML or delimited) into a sheet.
    #[serde(rename="pasteData")]
    
    pub paste_data: Option<PasteDataRequest>,
    /// Randomizes the order of the rows in a range.
    #[serde(rename="randomizeRange")]
    
    pub randomize_range: Option<RandomizeRangeRequest>,
    /// Refreshes one or multiple data sources and associated dbobjects.
    #[serde(rename="refreshDataSource")]
    
    pub refresh_data_source: Option<RefreshDataSourceRequest>,
    /// Repeats a single cell across a range.
    #[serde(rename="repeatCell")]
    
    pub repeat_cell: Option<RepeatCellRequest>,
    /// Sets the basic filter on a sheet.
    #[serde(rename="setBasicFilter")]
    
    pub set_basic_filter: Option<SetBasicFilterRequest>,
    /// Sets data validation for one or more cells.
    #[serde(rename="setDataValidation")]
    
    pub set_data_validation: Option<SetDataValidationRequest>,
    /// Sorts data in a range.
    #[serde(rename="sortRange")]
    
    pub sort_range: Option<SortRangeRequest>,
    /// Converts a column of text into many columns of text.
    #[serde(rename="textToColumns")]
    
    pub text_to_columns: Option<TextToColumnsRequest>,
    /// Trims cells of whitespace (such as spaces, tabs, or new lines).
    #[serde(rename="trimWhitespace")]
    
    pub trim_whitespace: Option<TrimWhitespaceRequest>,
    /// Unmerges merged cells.
    #[serde(rename="unmergeCells")]
    
    pub unmerge_cells: Option<UnmergeCellsRequest>,
    /// Updates a banded range
    #[serde(rename="updateBanding")]
    
    pub update_banding: Option<UpdateBandingRequest>,
    /// Updates the borders in a range of cells.
    #[serde(rename="updateBorders")]
    
    pub update_borders: Option<UpdateBordersRequest>,
    /// Updates many cells at once.
    #[serde(rename="updateCells")]
    
    pub update_cells: Option<UpdateCellsRequest>,
    /// Updates a chart's specifications.
    #[serde(rename="updateChartSpec")]
    
    pub update_chart_spec: Option<UpdateChartSpecRequest>,
    /// Updates an existing conditional format rule.
    #[serde(rename="updateConditionalFormatRule")]
    
    pub update_conditional_format_rule: Option<UpdateConditionalFormatRuleRequest>,
    /// Updates a data source.
    #[serde(rename="updateDataSource")]
    
    pub update_data_source: Option<UpdateDataSourceRequest>,
    /// Updates an existing developer metadata entry
    #[serde(rename="updateDeveloperMetadata")]
    
    pub update_developer_metadata: Option<UpdateDeveloperMetadataRequest>,
    /// Updates the state of the specified group.
    #[serde(rename="updateDimensionGroup")]
    
    pub update_dimension_group: Option<UpdateDimensionGroupRequest>,
    /// Updates dimensions' properties.
    #[serde(rename="updateDimensionProperties")]
    
    pub update_dimension_properties: Option<UpdateDimensionPropertiesRequest>,
    /// Updates an embedded object's border.
    #[serde(rename="updateEmbeddedObjectBorder")]
    
    pub update_embedded_object_border: Option<UpdateEmbeddedObjectBorderRequest>,
    /// Updates an embedded object's (e.g. chart, image) position.
    #[serde(rename="updateEmbeddedObjectPosition")]
    
    pub update_embedded_object_position: Option<UpdateEmbeddedObjectPositionRequest>,
    /// Updates the properties of a filter view.
    #[serde(rename="updateFilterView")]
    
    pub update_filter_view: Option<UpdateFilterViewRequest>,
    /// Updates a named range.
    #[serde(rename="updateNamedRange")]
    
    pub update_named_range: Option<UpdateNamedRangeRequest>,
    /// Updates a protected range.
    #[serde(rename="updateProtectedRange")]
    
    pub update_protected_range: Option<UpdateProtectedRangeRequest>,
    /// Updates a sheet's properties.
    #[serde(rename="updateSheetProperties")]
    
    pub update_sheet_properties: Option<UpdateSheetPropertiesRequest>,
    /// Updates a slicer's specifications.
    #[serde(rename="updateSlicerSpec")]
    
    pub update_slicer_spec: Option<UpdateSlicerSpecRequest>,
    /// Updates the spreadsheet's properties.
    #[serde(rename="updateSpreadsheetProperties")]
    
    pub update_spreadsheet_properties: Option<UpdateSpreadsheetPropertiesRequest>,
}

impl client::Part for Request {}


/// A single response from an update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// A reply from adding a banded range.
    #[serde(rename="addBanding")]
    
    pub add_banding: Option<AddBandingResponse>,
    /// A reply from adding a chart.
    #[serde(rename="addChart")]
    
    pub add_chart: Option<AddChartResponse>,
    /// A reply from adding a data source.
    #[serde(rename="addDataSource")]
    
    pub add_data_source: Option<AddDataSourceResponse>,
    /// A reply from adding a dimension group.
    #[serde(rename="addDimensionGroup")]
    
    pub add_dimension_group: Option<AddDimensionGroupResponse>,
    /// A reply from adding a filter view.
    #[serde(rename="addFilterView")]
    
    pub add_filter_view: Option<AddFilterViewResponse>,
    /// A reply from adding a named range.
    #[serde(rename="addNamedRange")]
    
    pub add_named_range: Option<AddNamedRangeResponse>,
    /// A reply from adding a protected range.
    #[serde(rename="addProtectedRange")]
    
    pub add_protected_range: Option<AddProtectedRangeResponse>,
    /// A reply from adding a sheet.
    #[serde(rename="addSheet")]
    
    pub add_sheet: Option<AddSheetResponse>,
    /// A reply from adding a slicer.
    #[serde(rename="addSlicer")]
    
    pub add_slicer: Option<AddSlicerResponse>,
    /// A reply from cancelling data source object refreshes.
    #[serde(rename="cancelDataSourceRefresh")]
    
    pub cancel_data_source_refresh: Option<CancelDataSourceRefreshResponse>,
    /// A reply from creating a developer metadata entry.
    #[serde(rename="createDeveloperMetadata")]
    
    pub create_developer_metadata: Option<CreateDeveloperMetadataResponse>,
    /// A reply from deleting a conditional format rule.
    #[serde(rename="deleteConditionalFormatRule")]
    
    pub delete_conditional_format_rule: Option<DeleteConditionalFormatRuleResponse>,
    /// A reply from deleting a developer metadata entry.
    #[serde(rename="deleteDeveloperMetadata")]
    
    pub delete_developer_metadata: Option<DeleteDeveloperMetadataResponse>,
    /// A reply from deleting a dimension group.
    #[serde(rename="deleteDimensionGroup")]
    
    pub delete_dimension_group: Option<DeleteDimensionGroupResponse>,
    /// A reply from removing rows containing duplicate values.
    #[serde(rename="deleteDuplicates")]
    
    pub delete_duplicates: Option<DeleteDuplicatesResponse>,
    /// A reply from duplicating a filter view.
    #[serde(rename="duplicateFilterView")]
    
    pub duplicate_filter_view: Option<DuplicateFilterViewResponse>,
    /// A reply from duplicating a sheet.
    #[serde(rename="duplicateSheet")]
    
    pub duplicate_sheet: Option<DuplicateSheetResponse>,
    /// A reply from doing a find/replace.
    #[serde(rename="findReplace")]
    
    pub find_replace: Option<FindReplaceResponse>,
    /// A reply from refreshing data source objects.
    #[serde(rename="refreshDataSource")]
    
    pub refresh_data_source: Option<RefreshDataSourceResponse>,
    /// A reply from trimming whitespace.
    #[serde(rename="trimWhitespace")]
    
    pub trim_whitespace: Option<TrimWhitespaceResponse>,
    /// A reply from updating a conditional format rule.
    #[serde(rename="updateConditionalFormatRule")]
    
    pub update_conditional_format_rule: Option<UpdateConditionalFormatRuleResponse>,
    /// A reply from updating a data source.
    #[serde(rename="updateDataSource")]
    
    pub update_data_source: Option<UpdateDataSourceResponse>,
    /// A reply from updating a developer metadata entry.
    #[serde(rename="updateDeveloperMetadata")]
    
    pub update_developer_metadata: Option<UpdateDeveloperMetadataResponse>,
    /// A reply from updating an embedded object's position.
    #[serde(rename="updateEmbeddedObjectPosition")]
    
    pub update_embedded_object_position: Option<UpdateEmbeddedObjectPositionResponse>,
}

impl client::Part for Response {}


/// Data about each cell in a row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RowData {
    /// The values in the row, one per column.
    
    pub values: Option<Vec<CellData>>,
}

impl client::Part for RowData {}


/// A scorecard chart. Scorecard charts are used to highlight key performance indicators, known as KPIs, on the spreadsheet. A scorecard chart can represent things like total sales, average cost, or a top selling item. You can specify a single data value, or aggregate over a range of data. Percentage or absolute difference from a baseline value can be highlighted, like changes over time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScorecardChartSpec {
    /// The aggregation type for key and baseline chart data in scorecard chart. This field is not supported for data source charts. Use the ChartData.aggregateType field of the key_value_data or baseline_value_data instead for data source charts. This field is optional.
    #[serde(rename="aggregateType")]
    
    pub aggregate_type: Option<String>,
    /// The data for scorecard baseline value. This field is optional.
    #[serde(rename="baselineValueData")]
    
    pub baseline_value_data: Option<ChartData>,
    /// Formatting options for baseline value. This field is needed only if baseline_value_data is specified.
    #[serde(rename="baselineValueFormat")]
    
    pub baseline_value_format: Option<BaselineValueFormat>,
    /// Custom formatting options for numeric key/baseline values in scorecard chart. This field is used only when number_format_source is set to CUSTOM. This field is optional.
    #[serde(rename="customFormatOptions")]
    
    pub custom_format_options: Option<ChartCustomNumberFormatOptions>,
    /// The data for scorecard key value.
    #[serde(rename="keyValueData")]
    
    pub key_value_data: Option<ChartData>,
    /// Formatting options for key value.
    #[serde(rename="keyValueFormat")]
    
    pub key_value_format: Option<KeyValueFormat>,
    /// The number format source used in the scorecard chart. This field is optional.
    #[serde(rename="numberFormatSource")]
    
    pub number_format_source: Option<String>,
    /// Value to scale scorecard key and baseline value. For example, a factor of 10 can be used to divide all values in the chart by 10. This field is optional.
    #[serde(rename="scaleFactor")]
    
    pub scale_factor: Option<f64>,
}

impl client::Part for ScorecardChartSpec {}


/// A request to retrieve all developer metadata matching the set of specified criteria.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developer metadata search spreadsheets](SpreadsheetDeveloperMetadataSearchCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchDeveloperMetadataRequest {
    /// The data filters describing the criteria used to determine which DeveloperMetadata entries to return. DeveloperMetadata matching any of the specified filters are included in the response.
    #[serde(rename="dataFilters")]
    
    pub data_filters: Option<Vec<DataFilter>>,
}

impl client::RequestValue for SearchDeveloperMetadataRequest {}


/// A reply to a developer metadata search request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developer metadata search spreadsheets](SpreadsheetDeveloperMetadataSearchCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchDeveloperMetadataResponse {
    /// The metadata matching the criteria of the search request.
    #[serde(rename="matchedDeveloperMetadata")]
    
    pub matched_developer_metadata: Option<Vec<MatchedDeveloperMetadata>>,
}

impl client::ResponseResult for SearchDeveloperMetadataResponse {}


/// Sets the basic filter associated with a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetBasicFilterRequest {
    /// The filter to set.
    
    pub filter: Option<BasicFilter>,
}

impl client::Part for SetBasicFilterRequest {}


/// Sets a data validation rule to every cell in the range. To clear validation in a range, call this with no rule specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetDataValidationRequest {
    /// The range the data validation rule should apply to.
    
    pub range: Option<GridRange>,
    /// The data validation rule to set on each cell in the range, or empty to clear the data validation in the range.
    
    pub rule: Option<DataValidationRule>,
}

impl client::Part for SetDataValidationRequest {}


/// A sheet in a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sheet {
    /// The banded (alternating colors) ranges on this sheet.
    #[serde(rename="bandedRanges")]
    
    pub banded_ranges: Option<Vec<BandedRange>>,
    /// The filter on this sheet, if any.
    #[serde(rename="basicFilter")]
    
    pub basic_filter: Option<BasicFilter>,
    /// The specifications of every chart on this sheet.
    
    pub charts: Option<Vec<EmbeddedChart>>,
    /// All column groups on this sheet, ordered by increasing range start index, then by group depth.
    #[serde(rename="columnGroups")]
    
    pub column_groups: Option<Vec<DimensionGroup>>,
    /// The conditional format rules in this sheet.
    #[serde(rename="conditionalFormats")]
    
    pub conditional_formats: Option<Vec<ConditionalFormatRule>>,
    /// Data in the grid, if this is a grid sheet. The number of GridData objects returned is dependent on the number of ranges requested on this sheet. For example, if this is representing `Sheet1`, and the spreadsheet was requested with ranges `Sheet1!A1:C10` and `Sheet1!D15:E20`, then the first GridData will have a startRow/startColumn of `0`, while the second one will have `startRow 14` (zero-based row 15), and `startColumn 3` (zero-based column D). For a DATA_SOURCE sheet, you can not request a specific range, the GridData contains all the values.
    
    pub data: Option<Vec<GridData>>,
    /// The developer metadata associated with a sheet.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// The filter views in this sheet.
    #[serde(rename="filterViews")]
    
    pub filter_views: Option<Vec<FilterView>>,
    /// The ranges that are merged together.
    
    pub merges: Option<Vec<GridRange>>,
    /// The properties of the sheet.
    
    pub properties: Option<SheetProperties>,
    /// The protected ranges in this sheet.
    #[serde(rename="protectedRanges")]
    
    pub protected_ranges: Option<Vec<ProtectedRange>>,
    /// All row groups on this sheet, ordered by increasing range start index, then by group depth.
    #[serde(rename="rowGroups")]
    
    pub row_groups: Option<Vec<DimensionGroup>>,
    /// The slicers on this sheet.
    
    pub slicers: Option<Vec<Slicer>>,
}

impl client::Part for Sheet {}


/// Properties of a sheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sheets copy to spreadsheets](SpreadsheetSheetCopyToCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetProperties {
    /// Output only. If present, the field contains DATA_SOURCE sheet specific properties.
    #[serde(rename="dataSourceSheetProperties")]
    
    pub data_source_sheet_properties: Option<DataSourceSheetProperties>,
    /// Additional properties of the sheet if this sheet is a grid. (If the sheet is an object sheet, containing a chart or image, then this field will be absent.) When writing it is an error to set any grid properties on non-grid sheets. If this sheet is a DATA_SOURCE sheet, this field is output only but contains the properties that reflect how a data source sheet is rendered in the UI, e.g. row_count.
    #[serde(rename="gridProperties")]
    
    pub grid_properties: Option<GridProperties>,
    /// True if the sheet is hidden in the UI, false if it's visible.
    
    pub hidden: Option<bool>,
    /// The index of the sheet within the spreadsheet. When adding or updating sheet properties, if this field is excluded then the sheet is added or moved to the end of the sheet list. When updating sheet indices or inserting sheets, movement is considered in "before the move" indexes. For example, if there were three sheets (S1, S2, S3) in order to move S1 ahead of S2 the index would have to be set to 2. A sheet index update request is ignored if the requested index is identical to the sheets current index or if the requested new index is equal to the current sheet index + 1.
    
    pub index: Option<i32>,
    /// True if the sheet is an RTL sheet instead of an LTR sheet.
    #[serde(rename="rightToLeft")]
    
    pub right_to_left: Option<bool>,
    /// The ID of the sheet. Must be non-negative. This field cannot be changed once set.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
    /// The type of sheet. Defaults to GRID. This field cannot be changed once set.
    #[serde(rename="sheetType")]
    
    pub sheet_type: Option<String>,
    /// The color of the tab in the UI. Deprecated: Use tab_color_style.
    #[serde(rename="tabColor")]
    
    pub tab_color: Option<Color>,
    /// The color of the tab in the UI. If tab_color is also set, this field takes precedence.
    #[serde(rename="tabColorStyle")]
    
    pub tab_color_style: Option<ColorStyle>,
    /// The name of the sheet.
    
    pub title: Option<String>,
}

impl client::ResponseResult for SheetProperties {}


/// A slicer in a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Slicer {
    /// The position of the slicer. Note that slicer can be positioned only on existing sheet. Also, width and height of slicer can be automatically adjusted to keep it within permitted limits.
    
    pub position: Option<EmbeddedObjectPosition>,
    /// The ID of the slicer.
    #[serde(rename="slicerId")]
    
    pub slicer_id: Option<i32>,
    /// The specification of the slicer.
    
    pub spec: Option<SlicerSpec>,
}

impl client::Part for Slicer {}


/// The specifications of a slicer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlicerSpec {
    /// True if the filter should apply to pivot tables. If not set, default to `True`.
    #[serde(rename="applyToPivotTables")]
    
    pub apply_to_pivot_tables: Option<bool>,
    /// The background color of the slicer. Deprecated: Use background_color_style.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<Color>,
    /// The background color of the slicer. If background_color is also set, this field takes precedence.
    #[serde(rename="backgroundColorStyle")]
    
    pub background_color_style: Option<ColorStyle>,
    /// The zero-based column index in the data table on which the filter is applied to.
    #[serde(rename="columnIndex")]
    
    pub column_index: Option<i32>,
    /// The data range of the slicer.
    #[serde(rename="dataRange")]
    
    pub data_range: Option<GridRange>,
    /// The filtering criteria of the slicer.
    #[serde(rename="filterCriteria")]
    
    pub filter_criteria: Option<FilterCriteria>,
    /// The horizontal alignment of title in the slicer. If unspecified, defaults to `LEFT`
    #[serde(rename="horizontalAlignment")]
    
    pub horizontal_alignment: Option<String>,
    /// The text format of title in the slicer. The link field is not supported.
    #[serde(rename="textFormat")]
    
    pub text_format: Option<TextFormat>,
    /// The title of the slicer.
    
    pub title: Option<String>,
}

impl client::Part for SlicerSpec {}


/// Sorts data in rows based on a sort order per column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SortRangeRequest {
    /// The range to sort.
    
    pub range: Option<GridRange>,
    /// The sort order per column. Later specifications are used when values are equal in the earlier specifications.
    #[serde(rename="sortSpecs")]
    
    pub sort_specs: Option<Vec<SortSpec>>,
}

impl client::Part for SortRangeRequest {}


/// A sort order associated with a specific column or row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SortSpec {
    /// The background fill color to sort by; cells with this fill color are sorted to the top. Mutually exclusive with foreground_color. Deprecated: Use background_color_style.
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<Color>,
    /// The background fill color to sort by; cells with this fill color are sorted to the top. Mutually exclusive with foreground_color, and must be an RGB-type color. If background_color is also set, this field takes precedence.
    #[serde(rename="backgroundColorStyle")]
    
    pub background_color_style: Option<ColorStyle>,
    /// Reference to a data source column.
    #[serde(rename="dataSourceColumnReference")]
    
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    /// The dimension the sort should be applied to.
    #[serde(rename="dimensionIndex")]
    
    pub dimension_index: Option<i32>,
    /// The foreground color to sort by; cells with this foreground color are sorted to the top. Mutually exclusive with background_color. Deprecated: Use foreground_color_style.
    #[serde(rename="foregroundColor")]
    
    pub foreground_color: Option<Color>,
    /// The foreground color to sort by; cells with this foreground color are sorted to the top. Mutually exclusive with background_color, and must be an RGB-type color. If foreground_color is also set, this field takes precedence.
    #[serde(rename="foregroundColorStyle")]
    
    pub foreground_color_style: Option<ColorStyle>,
    /// The order data should be sorted.
    #[serde(rename="sortOrder")]
    
    pub sort_order: Option<String>,
}

impl client::Part for SortSpec {}


/// A combination of a source range and how to extend that source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceAndDestination {
    /// The dimension that data should be filled into.
    
    pub dimension: Option<String>,
    /// The number of rows or columns that data should be filled into. Positive numbers expand beyond the last row or last column of the source. Negative numbers expand before the first row or first column of the source.
    #[serde(rename="fillLength")]
    
    pub fill_length: Option<i32>,
    /// The location of the data to use as the source of the autofill.
    
    pub source: Option<GridRange>,
}

impl client::Part for SourceAndDestination {}


/// Resource that represents a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developer metadata get spreadsheets](SpreadsheetDeveloperMetadataGetCall) (none)
/// * [developer metadata search spreadsheets](SpreadsheetDeveloperMetadataSearchCall) (none)
/// * [sheets copy to spreadsheets](SpreadsheetSheetCopyToCall) (none)
/// * [values append spreadsheets](SpreadsheetValueAppendCall) (none)
/// * [values batch clear spreadsheets](SpreadsheetValueBatchClearCall) (none)
/// * [values batch clear by data filter spreadsheets](SpreadsheetValueBatchClearByDataFilterCall) (none)
/// * [values batch get spreadsheets](SpreadsheetValueBatchGetCall) (none)
/// * [values batch get by data filter spreadsheets](SpreadsheetValueBatchGetByDataFilterCall) (none)
/// * [values batch update spreadsheets](SpreadsheetValueBatchUpdateCall) (none)
/// * [values batch update by data filter spreadsheets](SpreadsheetValueBatchUpdateByDataFilterCall) (none)
/// * [values clear spreadsheets](SpreadsheetValueClearCall) (none)
/// * [values get spreadsheets](SpreadsheetValueGetCall) (none)
/// * [values update spreadsheets](SpreadsheetValueUpdateCall) (none)
/// * [batch update spreadsheets](SpreadsheetBatchUpdateCall) (none)
/// * [create spreadsheets](SpreadsheetCreateCall) (request|response)
/// * [get spreadsheets](SpreadsheetGetCall) (response)
/// * [get by data filter spreadsheets](SpreadsheetGetByDataFilterCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Spreadsheet {
    /// Output only. A list of data source refresh schedules.
    #[serde(rename="dataSourceSchedules")]
    
    pub data_source_schedules: Option<Vec<DataSourceRefreshSchedule>>,
    /// A list of external data sources connected with the spreadsheet.
    #[serde(rename="dataSources")]
    
    pub data_sources: Option<Vec<DataSource>>,
    /// The developer metadata associated with a spreadsheet.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// The named ranges defined in a spreadsheet.
    #[serde(rename="namedRanges")]
    
    pub named_ranges: Option<Vec<NamedRange>>,
    /// Overall properties of a spreadsheet.
    
    pub properties: Option<SpreadsheetProperties>,
    /// The sheets that are part of a spreadsheet.
    
    pub sheets: Option<Vec<Sheet>>,
    /// The ID of the spreadsheet. This field is read-only.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The url of the spreadsheet. This field is read-only.
    #[serde(rename="spreadsheetUrl")]
    
    pub spreadsheet_url: Option<String>,
}

impl client::RequestValue for Spreadsheet {}
impl client::Resource for Spreadsheet {}
impl client::ResponseResult for Spreadsheet {}


/// Properties of a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpreadsheetProperties {
    /// The amount of time to wait before volatile functions are recalculated.
    #[serde(rename="autoRecalc")]
    
    pub auto_recalc: Option<String>,
    /// The default format of all cells in the spreadsheet. CellData.effectiveFormat will not be set if the cell's format is equal to this default format. This field is read-only.
    #[serde(rename="defaultFormat")]
    
    pub default_format: Option<CellFormat>,
    /// Whether to allow external URL access for image and import functions. Read only when true. When false, you can set to true.
    #[serde(rename="importFunctionsExternalUrlAccessAllowed")]
    
    pub import_functions_external_url_access_allowed: Option<bool>,
    /// Determines whether and how circular references are resolved with iterative calculation. Absence of this field means that circular references result in calculation errors.
    #[serde(rename="iterativeCalculationSettings")]
    
    pub iterative_calculation_settings: Option<IterativeCalculationSettings>,
    /// The locale of the spreadsheet in one of the following formats: * an ISO 639-1 language code such as `en` * an ISO 639-2 language code such as `fil`, if no 639-1 code exists * a combination of the ISO language code and country code, such as `en_US` Note: when updating this field, not all locales/languages are supported.
    
    pub locale: Option<String>,
    /// Theme applied to the spreadsheet.
    #[serde(rename="spreadsheetTheme")]
    
    pub spreadsheet_theme: Option<SpreadsheetTheme>,
    /// The time zone of the spreadsheet, in CLDR format such as `America/New_York`. If the time zone isn't recognized, this may be a custom time zone such as `GMT-07:00`.
    #[serde(rename="timeZone")]
    
    pub time_zone: Option<String>,
    /// The title of the spreadsheet.
    
    pub title: Option<String>,
}

impl client::Part for SpreadsheetProperties {}


/// Represents spreadsheet theme
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpreadsheetTheme {
    /// Name of the primary font family.
    #[serde(rename="primaryFontFamily")]
    
    pub primary_font_family: Option<String>,
    /// The spreadsheet theme color pairs. To update you must provide all theme color pairs.
    #[serde(rename="themeColors")]
    
    pub theme_colors: Option<Vec<ThemeColorPair>>,
}

impl client::Part for SpreadsheetTheme {}


/// The format of a run of text in a cell. Absent values indicate that the field isn't specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextFormat {
    /// True if the text is bold.
    
    pub bold: Option<bool>,
    /// The font family.
    #[serde(rename="fontFamily")]
    
    pub font_family: Option<String>,
    /// The size of the font.
    #[serde(rename="fontSize")]
    
    pub font_size: Option<i32>,
    /// The foreground color of the text. Deprecated: Use foreground_color_style.
    #[serde(rename="foregroundColor")]
    
    pub foreground_color: Option<Color>,
    /// The foreground color of the text. If foreground_color is also set, this field takes precedence.
    #[serde(rename="foregroundColorStyle")]
    
    pub foreground_color_style: Option<ColorStyle>,
    /// True if the text is italicized.
    
    pub italic: Option<bool>,
    /// The link destination of the text, if any. Setting the link field in a TextFormatRun will clear the cell's existing links or a cell-level link set in the same request. When a link is set, the text foreground color will be set to the default link color and the text will be underlined. If these fields are modified in the same request, those values will be used instead of the link defaults.
    
    pub link: Option<Link>,
    /// True if the text has a strikethrough.
    
    pub strikethrough: Option<bool>,
    /// True if the text is underlined.
    
    pub underline: Option<bool>,
}

impl client::Part for TextFormat {}


/// A run of a text format. The format of this run continues until the start index of the next run. When updating, all fields must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextFormatRun {
    /// The format of this run. Absent values inherit the cell's format.
    
    pub format: Option<TextFormat>,
    /// The zero-based character index where this run starts, in UTF-16 code units.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
}

impl client::Part for TextFormatRun {}


/// Position settings for text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextPosition {
    /// Horizontal alignment setting for the piece of text.
    #[serde(rename="horizontalAlignment")]
    
    pub horizontal_alignment: Option<String>,
}

impl client::Part for TextPosition {}


/// The rotation applied to text in a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextRotation {
    /// The angle between the standard orientation and the desired orientation. Measured in degrees. Valid values are between -90 and 90. Positive angles are angled upwards, negative are angled downwards. Note: For LTR text direction positive angles are in the counterclockwise direction, whereas for RTL they are in the clockwise direction
    
    pub angle: Option<i32>,
    /// If true, text reads top to bottom, but the orientation of individual characters is unchanged. For example: | V | | e | | r | | t | | i | | c | | a | | l |
    
    pub vertical: Option<bool>,
}

impl client::Part for TextRotation {}


/// Splits a column of text into multiple columns, based on a delimiter in each cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextToColumnsRequest {
    /// The delimiter to use. Used only if delimiterType is CUSTOM.
    
    pub delimiter: Option<String>,
    /// The delimiter type to use.
    #[serde(rename="delimiterType")]
    
    pub delimiter_type: Option<String>,
    /// The source data range. This must span exactly one column.
    
    pub source: Option<GridRange>,
}

impl client::Part for TextToColumnsRequest {}


/// A pair mapping a spreadsheet theme color type to the concrete color it represents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColorPair {
    /// The concrete color corresponding to the theme color type.
    
    pub color: Option<ColorStyle>,
    /// The type of the spreadsheet theme color.
    #[serde(rename="colorType")]
    
    pub color_type: Option<String>,
}

impl client::Part for ThemeColorPair {}


/// Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    
    pub hours: Option<i32>,
    /// Minutes of hour of day. Must be from 0 to 59.
    
    pub minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999.
    
    pub nanos: Option<i32>,
    /// Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds.
    
    pub seconds: Option<i32>,
}

impl client::Part for TimeOfDay {}


/// A color scale for a treemap chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TreemapChartColorScale {
    /// The background color for cells with a color value greater than or equal to maxValue. Defaults to #109618 if not specified. Deprecated: Use max_value_color_style.
    #[serde(rename="maxValueColor")]
    
    pub max_value_color: Option<Color>,
    /// The background color for cells with a color value greater than or equal to maxValue. Defaults to #109618 if not specified. If max_value_color is also set, this field takes precedence.
    #[serde(rename="maxValueColorStyle")]
    
    pub max_value_color_style: Option<ColorStyle>,
    /// The background color for cells with a color value at the midpoint between minValue and maxValue. Defaults to #efe6dc if not specified. Deprecated: Use mid_value_color_style.
    #[serde(rename="midValueColor")]
    
    pub mid_value_color: Option<Color>,
    /// The background color for cells with a color value at the midpoint between minValue and maxValue. Defaults to #efe6dc if not specified. If mid_value_color is also set, this field takes precedence.
    #[serde(rename="midValueColorStyle")]
    
    pub mid_value_color_style: Option<ColorStyle>,
    /// The background color for cells with a color value less than or equal to minValue. Defaults to #dc3912 if not specified. Deprecated: Use min_value_color_style.
    #[serde(rename="minValueColor")]
    
    pub min_value_color: Option<Color>,
    /// The background color for cells with a color value less than or equal to minValue. Defaults to #dc3912 if not specified. If min_value_color is also set, this field takes precedence.
    #[serde(rename="minValueColorStyle")]
    
    pub min_value_color_style: Option<ColorStyle>,
    /// The background color for cells that have no color data associated with them. Defaults to #000000 if not specified. Deprecated: Use no_data_color_style.
    #[serde(rename="noDataColor")]
    
    pub no_data_color: Option<Color>,
    /// The background color for cells that have no color data associated with them. Defaults to #000000 if not specified. If no_data_color is also set, this field takes precedence.
    #[serde(rename="noDataColorStyle")]
    
    pub no_data_color_style: Option<ColorStyle>,
}

impl client::Part for TreemapChartColorScale {}


/// A Treemap chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TreemapChartSpec {
    /// The data that determines the background color of each treemap data cell. This field is optional. If not specified, size_data is used to determine background colors. If specified, the data is expected to be numeric. color_scale will determine how the values in this data map to data cell background colors.
    #[serde(rename="colorData")]
    
    pub color_data: Option<ChartData>,
    /// The color scale for data cells in the treemap chart. Data cells are assigned colors based on their color values. These color values come from color_data, or from size_data if color_data is not specified. Cells with color values less than or equal to min_value will have minValueColor as their background color. Cells with color values greater than or equal to max_value will have maxValueColor as their background color. Cells with color values between min_value and max_value will have background colors on a gradient between minValueColor and maxValueColor, the midpoint of the gradient being midValueColor. Cells with missing or non-numeric color values will have noDataColor as their background color.
    #[serde(rename="colorScale")]
    
    pub color_scale: Option<TreemapChartColorScale>,
    /// The background color for header cells. Deprecated: Use header_color_style.
    #[serde(rename="headerColor")]
    
    pub header_color: Option<Color>,
    /// The background color for header cells. If header_color is also set, this field takes precedence.
    #[serde(rename="headerColorStyle")]
    
    pub header_color_style: Option<ColorStyle>,
    /// True to hide tooltips.
    #[serde(rename="hideTooltips")]
    
    pub hide_tooltips: Option<bool>,
    /// The number of additional data levels beyond the labeled levels to be shown on the treemap chart. These levels are not interactive and are shown without their labels. Defaults to 0 if not specified.
    #[serde(rename="hintedLevels")]
    
    pub hinted_levels: Option<i32>,
    /// The data that contains the treemap cell labels.
    
    pub labels: Option<ChartData>,
    /// The number of data levels to show on the treemap chart. These levels are interactive and are shown with their labels. Defaults to 2 if not specified.
    
    pub levels: Option<i32>,
    /// The maximum possible data value. Cells with values greater than this will have the same color as cells with this value. If not specified, defaults to the actual maximum value from color_data, or the maximum value from size_data if color_data is not specified.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<f64>,
    /// The minimum possible data value. Cells with values less than this will have the same color as cells with this value. If not specified, defaults to the actual minimum value from color_data, or the minimum value from size_data if color_data is not specified.
    #[serde(rename="minValue")]
    
    pub min_value: Option<f64>,
    /// The data the contains the treemap cells' parent labels.
    #[serde(rename="parentLabels")]
    
    pub parent_labels: Option<ChartData>,
    /// The data that determines the size of each treemap data cell. This data is expected to be numeric. The cells corresponding to non-numeric or missing data will not be rendered. If color_data is not specified, this data is used to determine data cell background colors as well.
    #[serde(rename="sizeData")]
    
    pub size_data: Option<ChartData>,
    /// The text format for all labels on the chart. The link field is not supported.
    #[serde(rename="textFormat")]
    
    pub text_format: Option<TextFormat>,
}

impl client::Part for TreemapChartSpec {}


/// Trims the whitespace (such as spaces, tabs, or new lines) in every cell in the specified range. This request removes all whitespace from the start and end of each cell's text, and reduces any subsequence of remaining whitespace characters to a single space. If the resulting trimmed text starts with a '+' or '=' character, the text remains as a string value and isn't interpreted as a formula.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrimWhitespaceRequest {
    /// The range whose cells to trim.
    
    pub range: Option<GridRange>,
}

impl client::Part for TrimWhitespaceRequest {}


/// The result of trimming whitespace in cells.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrimWhitespaceResponse {
    /// The number of cells that were trimmed of whitespace.
    #[serde(rename="cellsChangedCount")]
    
    pub cells_changed_count: Option<i32>,
}

impl client::Part for TrimWhitespaceResponse {}


/// Unmerges cells in the given range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnmergeCellsRequest {
    /// The range within which all cells should be unmerged. If the range spans multiple merges, all will be unmerged. The range must not partially span any merge.
    
    pub range: Option<GridRange>,
}

impl client::Part for UnmergeCellsRequest {}


/// Updates properties of the supplied banded range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateBandingRequest {
    /// The banded range to update with the new properties.
    #[serde(rename="bandedRange")]
    
    pub banded_range: Option<BandedRange>,
    /// The fields that should be updated. At least one field must be specified. The root `bandedRange` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
}

impl client::Part for UpdateBandingRequest {}


/// Updates the borders of a range. If a field is not set in the request, that means the border remains as-is. For example, with two subsequent UpdateBordersRequest: 1. range: A1:A5 `{ top: RED, bottom: WHITE }` 2. range: A1:A5 `{ left: BLUE }` That would result in A1:A5 having a borders of `{ top: RED, bottom: WHITE, left: BLUE }`. If you want to clear a border, explicitly set the style to NONE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateBordersRequest {
    /// The border to put at the bottom of the range.
    
    pub bottom: Option<Border>,
    /// The horizontal border to put within the range.
    #[serde(rename="innerHorizontal")]
    
    pub inner_horizontal: Option<Border>,
    /// The vertical border to put within the range.
    #[serde(rename="innerVertical")]
    
    pub inner_vertical: Option<Border>,
    /// The border to put at the left of the range.
    
    pub left: Option<Border>,
    /// The range whose borders should be updated.
    
    pub range: Option<GridRange>,
    /// The border to put at the right of the range.
    
    pub right: Option<Border>,
    /// The border to put at the top of the range.
    
    pub top: Option<Border>,
}

impl client::Part for UpdateBordersRequest {}


/// Updates all cells in a range with new data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCellsRequest {
    /// The fields of CellData that should be updated. At least one field must be specified. The root is the CellData; 'row.values.' should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The range to write data to. If the data in rows does not cover the entire requested range, the fields matching those set in fields will be cleared.
    
    pub range: Option<GridRange>,
    /// The data to write.
    
    pub rows: Option<Vec<RowData>>,
    /// The coordinate to start writing data at. Any number of rows and columns (including a different number of columns per row) may be written.
    
    pub start: Option<GridCoordinate>,
}

impl client::Part for UpdateCellsRequest {}


/// Updates a chart's specifications. (This does not move or resize a chart. To move or resize a chart, use UpdateEmbeddedObjectPositionRequest.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChartSpecRequest {
    /// The ID of the chart to update.
    #[serde(rename="chartId")]
    
    pub chart_id: Option<i32>,
    /// The specification to apply to the chart.
    
    pub spec: Option<ChartSpec>,
}

impl client::Part for UpdateChartSpecRequest {}


/// Updates a conditional format rule at the given index, or moves a conditional format rule to another index.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateConditionalFormatRuleRequest {
    /// The zero-based index of the rule that should be replaced or moved.
    
    pub index: Option<i32>,
    /// The zero-based new index the rule should end up at.
    #[serde(rename="newIndex")]
    
    pub new_index: Option<i32>,
    /// The rule that should replace the rule at the given index.
    
    pub rule: Option<ConditionalFormatRule>,
    /// The sheet of the rule to move. Required if new_index is set, unused otherwise.
    #[serde(rename="sheetId")]
    
    pub sheet_id: Option<i32>,
}

impl client::Part for UpdateConditionalFormatRuleRequest {}


/// The result of updating a conditional format rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateConditionalFormatRuleResponse {
    /// The index of the new rule.
    #[serde(rename="newIndex")]
    
    pub new_index: Option<i32>,
    /// The new rule that replaced the old rule (if replacing), or the rule that was moved (if moved)
    #[serde(rename="newRule")]
    
    pub new_rule: Option<ConditionalFormatRule>,
    /// The old index of the rule. Not set if a rule was replaced (because it is the same as new_index).
    #[serde(rename="oldIndex")]
    
    pub old_index: Option<i32>,
    /// The old (deleted) rule. Not set if a rule was moved (because it is the same as new_rule).
    #[serde(rename="oldRule")]
    
    pub old_rule: Option<ConditionalFormatRule>,
}

impl client::Part for UpdateConditionalFormatRuleResponse {}


/// Updates a data source. After the data source is updated successfully, an execution is triggered to refresh the associated DATA_SOURCE sheet to read data from the updated data source. The request requires an additional `bigquery.readonly` OAuth scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDataSourceRequest {
    /// The data source to update.
    #[serde(rename="dataSource")]
    
    pub data_source: Option<DataSource>,
    /// The fields that should be updated. At least one field must be specified. The root `dataSource` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
}

impl client::Part for UpdateDataSourceRequest {}


/// The response from updating data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDataSourceResponse {
    /// The data execution status.
    #[serde(rename="dataExecutionStatus")]
    
    pub data_execution_status: Option<DataExecutionStatus>,
    /// The updated data source.
    #[serde(rename="dataSource")]
    
    pub data_source: Option<DataSource>,
}

impl client::Part for UpdateDataSourceResponse {}


/// A request to update properties of developer metadata. Updates the properties of the developer metadata selected by the filters to the values provided in the DeveloperMetadata resource. Callers must specify the properties they wish to update in the fields parameter, as well as specify at least one DataFilter matching the metadata they wish to update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDeveloperMetadataRequest {
    /// The filters matching the developer metadata entries to update.
    #[serde(rename="dataFilters")]
    
    pub data_filters: Option<Vec<DataFilter>>,
    /// The value that all metadata matched by the data filters will be updated to.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<DeveloperMetadata>,
    /// The fields that should be updated. At least one field must be specified. The root `developerMetadata` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
}

impl client::Part for UpdateDeveloperMetadataRequest {}


/// The response from updating developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDeveloperMetadataResponse {
    /// The updated developer metadata.
    #[serde(rename="developerMetadata")]
    
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
}

impl client::Part for UpdateDeveloperMetadataResponse {}


/// Updates the state of the specified group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDimensionGroupRequest {
    /// The group whose state should be updated. The range and depth of the group should specify a valid group on the sheet, and all other fields updated.
    #[serde(rename="dimensionGroup")]
    
    pub dimension_group: Option<DimensionGroup>,
    /// The fields that should be updated. At least one field must be specified. The root `dimensionGroup` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
}

impl client::Part for UpdateDimensionGroupRequest {}


/// Updates properties of dimensions within the specified range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDimensionPropertiesRequest {
    /// The columns on a data source sheet to update.
    #[serde(rename="dataSourceSheetRange")]
    
    pub data_source_sheet_range: Option<DataSourceSheetDimensionRange>,
    /// The fields that should be updated. At least one field must be specified. The root `properties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// Properties to update.
    
    pub properties: Option<DimensionProperties>,
    /// The rows or columns to update.
    
    pub range: Option<DimensionRange>,
}

impl client::Part for UpdateDimensionPropertiesRequest {}


/// Updates an embedded object's border property.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEmbeddedObjectBorderRequest {
    /// The border that applies to the embedded object.
    
    pub border: Option<EmbeddedObjectBorder>,
    /// The fields that should be updated. At least one field must be specified. The root `border` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The ID of the embedded object to update.
    #[serde(rename="objectId")]
    
    pub object_id: Option<i32>,
}

impl client::Part for UpdateEmbeddedObjectBorderRequest {}


/// Update an embedded object's position (such as a moving or resizing a chart or image).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEmbeddedObjectPositionRequest {
    /// The fields of OverlayPosition that should be updated when setting a new position. Used only if newPosition.overlayPosition is set, in which case at least one field must be specified. The root `newPosition.overlayPosition` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// An explicit position to move the embedded object to. If newPosition.sheetId is set, a new sheet with that ID will be created. If newPosition.newSheet is set to true, a new sheet will be created with an ID that will be chosen for you.
    #[serde(rename="newPosition")]
    
    pub new_position: Option<EmbeddedObjectPosition>,
    /// The ID of the object to moved.
    #[serde(rename="objectId")]
    
    pub object_id: Option<i32>,
}

impl client::Part for UpdateEmbeddedObjectPositionRequest {}


/// The result of updating an embedded object's position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEmbeddedObjectPositionResponse {
    /// The new position of the embedded object.
    
    pub position: Option<EmbeddedObjectPosition>,
}

impl client::Part for UpdateEmbeddedObjectPositionResponse {}


/// Updates properties of the filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateFilterViewRequest {
    /// The fields that should be updated. At least one field must be specified. The root `filter` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The new properties of the filter view.
    
    pub filter: Option<FilterView>,
}

impl client::Part for UpdateFilterViewRequest {}


/// Updates properties of the named range with the specified namedRangeId.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateNamedRangeRequest {
    /// The fields that should be updated. At least one field must be specified. The root `namedRange` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The named range to update with the new properties.
    #[serde(rename="namedRange")]
    
    pub named_range: Option<NamedRange>,
}

impl client::Part for UpdateNamedRangeRequest {}


/// Updates an existing protected range with the specified protectedRangeId.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProtectedRangeRequest {
    /// The fields that should be updated. At least one field must be specified. The root `protectedRange` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The protected range to update with the new properties.
    #[serde(rename="protectedRange")]
    
    pub protected_range: Option<ProtectedRange>,
}

impl client::Part for UpdateProtectedRangeRequest {}


/// Updates properties of the sheet with the specified sheetId.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSheetPropertiesRequest {
    /// The fields that should be updated. At least one field must be specified. The root `properties` is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The properties to update.
    
    pub properties: Option<SheetProperties>,
}

impl client::Part for UpdateSheetPropertiesRequest {}


/// Updates a slicer's specifications. (This does not move or resize a slicer. To move or resize a slicer use UpdateEmbeddedObjectPositionRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSlicerSpecRequest {
    /// The fields that should be updated. At least one field must be specified. The root `SlicerSpec` is implied and should not be specified. A single "*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The id of the slicer to update.
    #[serde(rename="slicerId")]
    
    pub slicer_id: Option<i32>,
    /// The specification to apply to the slicer.
    
    pub spec: Option<SlicerSpec>,
}

impl client::Part for UpdateSlicerSpecRequest {}


/// Updates properties of a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSpreadsheetPropertiesRequest {
    /// The fields that should be updated. At least one field must be specified. The root 'properties' is implied and should not be specified. A single `"*"` can be used as short-hand for listing every field.
    
    pub fields: Option<client::FieldMask>,
    /// The properties to update.
    
    pub properties: Option<SpreadsheetProperties>,
}

impl client::Part for UpdateSpreadsheetPropertiesRequest {}


/// The response when updating a range of values by a data filter in a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateValuesByDataFilterResponse {
    /// The data filter that selected the range that was updated.
    #[serde(rename="dataFilter")]
    
    pub data_filter: Option<DataFilter>,
    /// The number of cells updated.
    #[serde(rename="updatedCells")]
    
    pub updated_cells: Option<i32>,
    /// The number of columns where at least one cell in the column was updated.
    #[serde(rename="updatedColumns")]
    
    pub updated_columns: Option<i32>,
    /// The values of the cells in the range matched by the dataFilter after all updates were applied. This is only included if the request's `includeValuesInResponse` field was `true`.
    #[serde(rename="updatedData")]
    
    pub updated_data: Option<ValueRange>,
    /// The range (in [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell)) that updates were applied to.
    #[serde(rename="updatedRange")]
    
    pub updated_range: Option<String>,
    /// The number of rows where at least one cell in the row was updated.
    #[serde(rename="updatedRows")]
    
    pub updated_rows: Option<i32>,
}

impl client::Part for UpdateValuesByDataFilterResponse {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values update spreadsheets](SpreadsheetValueUpdateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateValuesResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    
    pub spreadsheet_id: Option<String>,
    /// The number of cells updated.
    #[serde(rename="updatedCells")]
    
    pub updated_cells: Option<i32>,
    /// The number of columns where at least one cell in the column was updated.
    #[serde(rename="updatedColumns")]
    
    pub updated_columns: Option<i32>,
    /// The values of the cells after updates were applied. This is only included if the request's `includeValuesInResponse` field was `true`.
    #[serde(rename="updatedData")]
    
    pub updated_data: Option<ValueRange>,
    /// The range (in A1 notation) that updates were applied to.
    #[serde(rename="updatedRange")]
    
    pub updated_range: Option<String>,
    /// The number of rows where at least one cell in the row was updated.
    #[serde(rename="updatedRows")]
    
    pub updated_rows: Option<i32>,
}

impl client::ResponseResult for UpdateValuesResponse {}


/// Data within a range of the spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values append spreadsheets](SpreadsheetValueAppendCall) (request)
/// * [values get spreadsheets](SpreadsheetValueGetCall) (response)
/// * [values update spreadsheets](SpreadsheetValueUpdateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueRange {
    /// The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS.
    #[serde(rename="majorDimension")]
    
    pub major_dimension: Option<String>,
    /// The range the values cover, in [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell). For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended.
    
    pub range: Option<String>,
    /// The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell. For output, empty trailing rows and columns will not be included. For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string.
    
    pub values: Option<Vec<Vec<json::Value>>>,
}

impl client::RequestValue for ValueRange {}
impl client::ResponseResult for ValueRange {}


/// Styles for a waterfall chart column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartColumnStyle {
    /// The color of the column. Deprecated: Use color_style.
    
    pub color: Option<Color>,
    /// The color of the column. If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    
    pub color_style: Option<ColorStyle>,
    /// The label of the column's legend.
    
    pub label: Option<String>,
}

impl client::Part for WaterfallChartColumnStyle {}


/// A custom subtotal column for a waterfall chart series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartCustomSubtotal {
    /// True if the data point at subtotal_index is the subtotal. If false, the subtotal will be computed and appear after the data point.
    #[serde(rename="dataIsSubtotal")]
    
    pub data_is_subtotal: Option<bool>,
    /// A label for the subtotal column.
    
    pub label: Option<String>,
    /// The zero-based index of a data point within the series. If data_is_subtotal is true, the data point at this index is the subtotal. Otherwise, the subtotal appears after the data point with this index. A series can have multiple subtotals at arbitrary indices, but subtotals do not affect the indices of the data points. For example, if a series has three data points, their indices will always be 0, 1, and 2, regardless of how many subtotals exist on the series or what data points they are associated with.
    #[serde(rename="subtotalIndex")]
    
    pub subtotal_index: Option<i32>,
}

impl client::Part for WaterfallChartCustomSubtotal {}


/// The domain of a waterfall chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartDomain {
    /// The data of the WaterfallChartDomain.
    
    pub data: Option<ChartData>,
    /// True to reverse the order of the domain values (horizontal axis).
    
    pub reversed: Option<bool>,
}

impl client::Part for WaterfallChartDomain {}


/// A single series of data for a waterfall chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartSeries {
    /// Custom subtotal columns appearing in this series. The order in which subtotals are defined is not significant. Only one subtotal may be defined for each data point.
    #[serde(rename="customSubtotals")]
    
    pub custom_subtotals: Option<Vec<WaterfallChartCustomSubtotal>>,
    /// The data being visualized in this series.
    
    pub data: Option<ChartData>,
    /// Information about the data labels for this series.
    #[serde(rename="dataLabel")]
    
    pub data_label: Option<DataLabel>,
    /// True to hide the subtotal column from the end of the series. By default, a subtotal column will appear at the end of each series. Setting this field to true will hide that subtotal column for this series.
    #[serde(rename="hideTrailingSubtotal")]
    
    pub hide_trailing_subtotal: Option<bool>,
    /// Styles for all columns in this series with negative values.
    #[serde(rename="negativeColumnsStyle")]
    
    pub negative_columns_style: Option<WaterfallChartColumnStyle>,
    /// Styles for all columns in this series with positive values.
    #[serde(rename="positiveColumnsStyle")]
    
    pub positive_columns_style: Option<WaterfallChartColumnStyle>,
    /// Styles for all subtotal columns in this series.
    #[serde(rename="subtotalColumnsStyle")]
    
    pub subtotal_columns_style: Option<WaterfallChartColumnStyle>,
}

impl client::Part for WaterfallChartSeries {}


/// A waterfall chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartSpec {
    /// The line style for the connector lines.
    #[serde(rename="connectorLineStyle")]
    
    pub connector_line_style: Option<LineStyle>,
    /// The domain data (horizontal axis) for the waterfall chart.
    
    pub domain: Option<WaterfallChartDomain>,
    /// True to interpret the first value as a total.
    #[serde(rename="firstValueIsTotal")]
    
    pub first_value_is_total: Option<bool>,
    /// True to hide connector lines between columns.
    #[serde(rename="hideConnectorLines")]
    
    pub hide_connector_lines: Option<bool>,
    /// The data this waterfall chart is visualizing.
    
    pub series: Option<Vec<WaterfallChartSeries>>,
    /// The stacked type.
    #[serde(rename="stackedType")]
    
    pub stacked_type: Option<String>,
    /// Controls whether to display additional data labels on stacked charts which sum the total value of all stacked values at each value along the domain axis. stacked_type must be STACKED and neither CUSTOM nor placement can be set on the total_data_label.
    #[serde(rename="totalDataLabel")]
    
    pub total_data_label: Option<DataLabel>,
}

impl client::Part for WaterfallChartSpec {}



// ###################
// MethodBuilders ###
// #################

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
/// let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_update(...)`, `create(...)`, `developer_metadata_get(...)`, `developer_metadata_search(...)`, `get(...)`, `get_by_data_filter(...)`, `sheets_copy_to(...)`, `values_append(...)`, `values_batch_clear(...)`, `values_batch_clear_by_data_filter(...)`, `values_batch_get(...)`, `values_batch_get_by_data_filter(...)`, `values_batch_update(...)`, `values_batch_update_by_data_filter(...)`, `values_clear(...)`, `values_get(...)` and `values_update(...)`
/// // to build up your call.
/// let rb = hub.spreadsheets();
/// # }
/// ```
pub struct SpreadsheetMethods<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
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





// ###################
// CallBuilders   ###
// #################

/// Returns the developer metadata with the specified ID. The caller must specify the spreadsheet ID and the developer metadata's unique metadataId.
///
/// A builder for the *developerMetadata.get* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().developer_metadata_get("spreadsheetId", -50)
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetDeveloperMetadataGetCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _spreadsheet_id: String,
    _metadata_id: i32,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetDeveloperMetadataGetCall<'a, S> {}

impl<'a, S> SpreadsheetDeveloperMetadataGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DeveloperMetadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.developerMetadata.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "spreadsheetId", "metadataId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        params.push("metadataId", self._metadata_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/developerMetadata/{metadataId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{metadataId}", "metadataId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["metadataId", "spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the spreadsheet to retrieve metadata from.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetDeveloperMetadataGetCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The ID of the developer metadata to retrieve.
    ///
    /// Sets the *metadata id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metadata_id(mut self, new_value: i32) -> SpreadsheetDeveloperMetadataGetCall<'a, S> {
        self._metadata_id = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetDeveloperMetadataGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetDeveloperMetadataGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetDeveloperMetadataGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetDeveloperMetadataGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetDeveloperMetadataGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns all developer metadata matching the specified DataFilter. If the provided DataFilter represents a DeveloperMetadataLookup object, this will return all DeveloperMetadata entries selected by it. If the DataFilter represents a location in a spreadsheet, this will return all developer metadata associated with locations intersecting that region.
///
/// A builder for the *developerMetadata.search* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::SearchDeveloperMetadataRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SearchDeveloperMetadataRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().developer_metadata_search(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetDeveloperMetadataSearchCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: SearchDeveloperMetadataRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetDeveloperMetadataSearchCall<'a, S> {}

impl<'a, S> SpreadsheetDeveloperMetadataSearchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, SearchDeveloperMetadataResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.developerMetadata.search",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/developerMetadata:search";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: SearchDeveloperMetadataRequest) -> SpreadsheetDeveloperMetadataSearchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to retrieve metadata from.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetDeveloperMetadataSearchCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetDeveloperMetadataSearchCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetDeveloperMetadataSearchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetDeveloperMetadataSearchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetDeveloperMetadataSearchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetDeveloperMetadataSearchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.
///
/// A builder for the *sheets.copyTo* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::CopySheetToAnotherSpreadsheetRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CopySheetToAnotherSpreadsheetRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().sheets_copy_to(req, "spreadsheetId", -12)
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetSheetCopyToCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: CopySheetToAnotherSpreadsheetRequest,
    _spreadsheet_id: String,
    _sheet_id: i32,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetSheetCopyToCall<'a, S> {}

impl<'a, S> SpreadsheetSheetCopyToCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, SheetProperties)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.sheets.copyTo",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId", "sheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        params.push("sheetId", self._sheet_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/sheets/{sheetId}:copyTo";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{sheetId}", "sheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["sheetId", "spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CopySheetToAnotherSpreadsheetRequest) -> SpreadsheetSheetCopyToCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet containing the sheet to copy.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetSheetCopyToCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The ID of the sheet to copy.
    ///
    /// Sets the *sheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn sheet_id(mut self, new_value: i32) -> SpreadsheetSheetCopyToCall<'a, S> {
        self._sheet_id = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetSheetCopyToCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetSheetCopyToCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetSheetCopyToCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetSheetCopyToCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetSheetCopyToCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Appends values to a spreadsheet. The input range is used to search for existing data and find a “table” within that range. Values will be appended to the next row of the table, starting with the first column of the table. See the [guide](https://developers.google.com/sheets/api/guides/values#appending_values) and [sample code](https://developers.google.com/sheets/api/samples/writing#append_values) for specific details of how tables are detected and data is appended. The caller must specify the spreadsheet ID, range, and a valueInputOption. The `valueInputOption` only controls how the input data will be added to the sheet (column-wise or row-wise), it does not influence what cell the data starts being written to.
///
/// A builder for the *values.append* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::ValueRange;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ValueRange::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
///              .value_input_option("ipsum")
///              .response_value_render_option("ipsum")
///              .response_date_time_render_option("est")
///              .insert_data_option("gubergren")
///              .include_values_in_response(false)
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueAppendCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: ValueRange,
    _spreadsheet_id: String,
    _range: String,
    _value_input_option: Option<String>,
    _response_value_render_option: Option<String>,
    _response_date_time_render_option: Option<String>,
    _insert_data_option: Option<String>,
    _include_values_in_response: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueAppendCall<'a, S> {}

impl<'a, S> SpreadsheetValueAppendCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, AppendValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.append",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId", "range", "valueInputOption", "responseValueRenderOption", "responseDateTimeRenderOption", "insertDataOption", "includeValuesInResponse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        params.push("range", self._range);
        if let Some(value) = self._value_input_option.as_ref() {
            params.push("valueInputOption", value);
        }
        if let Some(value) = self._response_value_render_option.as_ref() {
            params.push("responseValueRenderOption", value);
        }
        if let Some(value) = self._response_date_time_render_option.as_ref() {
            params.push("responseDateTimeRenderOption", value);
        }
        if let Some(value) = self._insert_data_option.as_ref() {
            params.push("insertDataOption", value);
        }
        if let Some(value) = self._include_values_in_response.as_ref() {
            params.push("includeValuesInResponse", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}:append";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["range", "spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ValueRange) -> SpreadsheetValueAppendCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of a range to search for a logical table of data. Values are appended after the last row of the table.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, S> {
        self._range = new_value.to_string();
        self
    }
    /// How the input data should be interpreted.
    ///
    /// Sets the *value input option* query property to the given value.
    pub fn value_input_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, S> {
        self._value_input_option = Some(new_value.to_string());
        self
    }
    /// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
    ///
    /// Sets the *response value render option* query property to the given value.
    pub fn response_value_render_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, S> {
        self._response_value_render_option = Some(new_value.to_string());
        self
    }
    /// Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    ///
    /// Sets the *response date time render option* query property to the given value.
    pub fn response_date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, S> {
        self._response_date_time_render_option = Some(new_value.to_string());
        self
    }
    /// How the input data should be inserted.
    ///
    /// Sets the *insert data option* query property to the given value.
    pub fn insert_data_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, S> {
        self._insert_data_option = Some(new_value.to_string());
        self
    }
    /// Determines if the update response should include the values of the cells that were appended. By default, responses do not include the updated values.
    ///
    /// Sets the *include values in response* query property to the given value.
    pub fn include_values_in_response(mut self, new_value: bool) -> SpreadsheetValueAppendCall<'a, S> {
        self._include_values_in_response = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueAppendCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueAppendCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueAppendCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueAppendCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueAppendCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges. Only values are cleared -- all other properties of the cell (such as formatting and data validation) are kept.
///
/// A builder for the *values.batchClear* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::BatchClearValuesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchClearValuesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_clear(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueBatchClearCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: BatchClearValuesRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueBatchClearCall<'a, S> {}

impl<'a, S> SpreadsheetValueBatchClearCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchClearValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.batchClear",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchClear";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchClearValuesRequest) -> SpreadsheetValueBatchClearCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchClearCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueBatchClearCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchClearCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueBatchClearCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueBatchClearCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueBatchClearCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
///
/// A builder for the *values.batchClearByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::BatchClearValuesByDataFilterRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchClearValuesByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_clear_by_data_filter(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueBatchClearByDataFilterCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: BatchClearValuesByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueBatchClearByDataFilterCall<'a, S> {}

impl<'a, S> SpreadsheetValueBatchClearByDataFilterCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchClearValuesByDataFilterResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.batchClearByDataFilter",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchClearByDataFilter";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchClearValuesByDataFilterRequest) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueBatchClearByDataFilterCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges.
///
/// A builder for the *values.batchGet* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_get("spreadsheetId")
///              .value_render_option("sed")
///              .add_ranges("duo")
///              .major_dimension("sed")
///              .date_time_render_option("no")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueBatchGetCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _spreadsheet_id: String,
    _value_render_option: Option<String>,
    _ranges: Vec<String>,
    _major_dimension: Option<String>,
    _date_time_render_option: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueBatchGetCall<'a, S> {}

impl<'a, S> SpreadsheetValueBatchGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchGetValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.batchGet",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "spreadsheetId", "valueRenderOption", "ranges", "majorDimension", "dateTimeRenderOption"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        if let Some(value) = self._value_render_option.as_ref() {
            params.push("valueRenderOption", value);
        }
        if self._ranges.len() > 0 {
            for f in self._ranges.iter() {
                params.push("ranges", f);
            }
        }
        if let Some(value) = self._major_dimension.as_ref() {
            params.push("majorDimension", value);
        }
        if let Some(value) = self._date_time_render_option.as_ref() {
            params.push("dateTimeRenderOption", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchGet";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the spreadsheet to retrieve data from.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// How values should be represented in the output. The default render option is ValueRenderOption.FORMATTED_VALUE.
    ///
    /// Sets the *value render option* query property to the given value.
    pub fn value_render_option(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, S> {
        self._value_render_option = Some(new_value.to_string());
        self
    }
    /// The [A1 notation or R1C1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of the range to retrieve values from.
    ///
    /// Append the given value to the *ranges* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_ranges(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, S> {
        self._ranges.push(new_value.to_string());
        self
    }
    /// The major dimension that results should use. For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `ranges=["A1:B2"],majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `ranges=["A1:B2"],majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
    ///
    /// Sets the *major dimension* query property to the given value.
    pub fn major_dimension(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, S> {
        self._major_dimension = Some(new_value.to_string());
        self
    }
    /// How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    ///
    /// Sets the *date time render option* query property to the given value.
    pub fn date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, S> {
        self._date_time_render_option = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueBatchGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::DriveReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueBatchGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueBatchGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueBatchGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns one or more ranges of values that match the specified data filters. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges that match any of the data filters in the request will be returned.
///
/// A builder for the *values.batchGetByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::BatchGetValuesByDataFilterRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchGetValuesByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_get_by_data_filter(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueBatchGetByDataFilterCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: BatchGetValuesByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueBatchGetByDataFilterCall<'a, S> {}

impl<'a, S> SpreadsheetValueBatchGetByDataFilterCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchGetValuesByDataFilterResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.batchGetByDataFilter",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchGetByDataFilter";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchGetValuesByDataFilterRequest) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to retrieve data from.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueBatchGetByDataFilterCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more ValueRanges.
///
/// A builder for the *values.batchUpdate* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::BatchUpdateValuesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateValuesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_update(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueBatchUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: BatchUpdateValuesRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueBatchUpdateCall<'a, S> {}

impl<'a, S> SpreadsheetValueBatchUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchUpdateValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.batchUpdate",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchUpdate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchUpdateValuesRequest) -> SpreadsheetValueBatchUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchUpdateCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueBatchUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueBatchUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueBatchUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueBatchUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges.
///
/// A builder for the *values.batchUpdateByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::BatchUpdateValuesByDataFilterRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateValuesByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_update_by_data_filter(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueBatchUpdateByDataFilterCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: BatchUpdateValuesByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueBatchUpdateByDataFilterCall<'a, S> {}

impl<'a, S> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchUpdateValuesByDataFilterResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.batchUpdateByDataFilter",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchUpdateByDataFilter";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchUpdateValuesByDataFilterRequest) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Clears values from a spreadsheet. The caller must specify the spreadsheet ID and range. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
///
/// A builder for the *values.clear* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::ClearValuesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ClearValuesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_clear(req, "spreadsheetId", "range")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueClearCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: ClearValuesRequest,
    _spreadsheet_id: String,
    _range: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueClearCall<'a, S> {}

impl<'a, S> SpreadsheetValueClearCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ClearValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.clear",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId", "range"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        params.push("range", self._range);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}:clear";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["range", "spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ClearValuesRequest) -> SpreadsheetValueClearCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueClearCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The [A1 notation or R1C1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of the values to clear.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueClearCall<'a, S> {
        self._range = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueClearCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueClearCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueClearCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueClearCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueClearCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns a range of values from a spreadsheet. The caller must specify the spreadsheet ID and a range.
///
/// A builder for the *values.get* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_get("spreadsheetId", "range")
///              .value_render_option("erat")
///              .major_dimension("sed")
///              .date_time_render_option("duo")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueGetCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _spreadsheet_id: String,
    _range: String,
    _value_render_option: Option<String>,
    _major_dimension: Option<String>,
    _date_time_render_option: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueGetCall<'a, S> {}

impl<'a, S> SpreadsheetValueGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ValueRange)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "spreadsheetId", "range", "valueRenderOption", "majorDimension", "dateTimeRenderOption"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        params.push("range", self._range);
        if let Some(value) = self._value_render_option.as_ref() {
            params.push("valueRenderOption", value);
        }
        if let Some(value) = self._major_dimension.as_ref() {
            params.push("majorDimension", value);
        }
        if let Some(value) = self._date_time_render_option.as_ref() {
            params.push("dateTimeRenderOption", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["range", "spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The ID of the spreadsheet to retrieve data from.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The [A1 notation or R1C1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of the range to retrieve values from.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, S> {
        self._range = new_value.to_string();
        self
    }
    /// How values should be represented in the output. The default render option is FORMATTED_VALUE.
    ///
    /// Sets the *value render option* query property to the given value.
    pub fn value_render_option(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, S> {
        self._value_render_option = Some(new_value.to_string());
        self
    }
    /// The major dimension that results should use. For example, if the spreadsheet data in Sheet1 is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=Sheet1!A1:B2?majorDimension=ROWS` returns `[[1,2],[3,4]]`, whereas requesting `range=Sheet1!A1:B2?majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
    ///
    /// Sets the *major dimension* query property to the given value.
    pub fn major_dimension(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, S> {
        self._major_dimension = Some(new_value.to_string());
        self
    }
    /// How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    ///
    /// Sets the *date time render option* query property to the given value.
    pub fn date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, S> {
        self._date_time_render_option = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::DriveReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sets values in a range of a spreadsheet. The caller must specify the spreadsheet ID, range, and a valueInputOption.
///
/// A builder for the *values.update* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::ValueRange;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ValueRange::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_update(req, "spreadsheetId", "range")
///              .value_input_option("voluptua.")
///              .response_value_render_option("amet.")
///              .response_date_time_render_option("consetetur")
///              .include_values_in_response(false)
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetValueUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: ValueRange,
    _spreadsheet_id: String,
    _range: String,
    _value_input_option: Option<String>,
    _response_value_render_option: Option<String>,
    _response_date_time_render_option: Option<String>,
    _include_values_in_response: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetValueUpdateCall<'a, S> {}

impl<'a, S> SpreadsheetValueUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, UpdateValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.values.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "spreadsheetId", "range", "valueInputOption", "responseValueRenderOption", "responseDateTimeRenderOption", "includeValuesInResponse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        params.push("range", self._range);
        if let Some(value) = self._value_input_option.as_ref() {
            params.push("valueInputOption", value);
        }
        if let Some(value) = self._response_value_render_option.as_ref() {
            params.push("responseValueRenderOption", value);
        }
        if let Some(value) = self._response_date_time_render_option.as_ref() {
            params.push("responseDateTimeRenderOption", value);
        }
        if let Some(value) = self._include_values_in_response.as_ref() {
            params.push("includeValuesInResponse", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["range", "spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ValueRange) -> SpreadsheetValueUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell) of the values to update.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, S> {
        self._range = new_value.to_string();
        self
    }
    /// How the input data should be interpreted.
    ///
    /// Sets the *value input option* query property to the given value.
    pub fn value_input_option(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, S> {
        self._value_input_option = Some(new_value.to_string());
        self
    }
    /// Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE.
    ///
    /// Sets the *response value render option* query property to the given value.
    pub fn response_value_render_option(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, S> {
        self._response_value_render_option = Some(new_value.to_string());
        self
    }
    /// Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER.
    ///
    /// Sets the *response date time render option* query property to the given value.
    pub fn response_date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, S> {
        self._response_date_time_render_option = Some(new_value.to_string());
        self
    }
    /// Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns).
    ///
    /// Sets the *include values in response* query property to the given value.
    pub fn include_values_in_response(mut self, new_value: bool) -> SpreadsheetValueUpdateCall<'a, S> {
        self._include_values_in_response = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetValueUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetValueUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetValueUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetValueUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Applies one or more updates to the spreadsheet. Each request is validated before being applied. If any request is not valid then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. The replies will mirror the requests. For example, if you applied 4 updates and the 3rd one had a reply, then the response will have 2 empty replies, the actual reply, and another empty reply, in that order. Due to the collaborative nature of spreadsheets, it is not guaranteed that the spreadsheet will reflect exactly your changes after this completes, however it is guaranteed that the updates in the request will be applied together atomically. Your changes may be altered with respect to collaborator changes. If there are no collaborators, the spreadsheet should reflect your changes.
///
/// A builder for the *batchUpdate* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::BatchUpdateSpreadsheetRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateSpreadsheetRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().batch_update(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetBatchUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: BatchUpdateSpreadsheetRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetBatchUpdateCall<'a, S> {}

impl<'a, S> SpreadsheetBatchUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BatchUpdateSpreadsheetResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.batchUpdate",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}:batchUpdate";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchUpdateSpreadsheetRequest) -> SpreadsheetBatchUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The spreadsheet to apply the updates to.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetBatchUpdateCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetBatchUpdateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetBatchUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetBatchUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetBatchUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetBatchUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a spreadsheet, returning the newly created spreadsheet.
///
/// A builder for the *create* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::Spreadsheet;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Spreadsheet::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().create(req)
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetCreateCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: Spreadsheet,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetCreateCall<'a, S> {}

impl<'a, S> SpreadsheetCreateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Spreadsheet)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.create",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }


        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Spreadsheet) -> SpreadsheetCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetCreateCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetCreateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetCreateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetCreateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. By default, data within grids is not returned. You can include grid data in one of 2 ways: * Specify a [field mask](https://developers.google.com/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData URL parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, as a best practice, retrieve only the specific spreadsheet fields that you want. To retrieve only subsets of spreadsheet data, use the ranges URL parameter. Ranges are specified using [A1 notation](https://developers.google.com/sheets/api/guides/concepts#cell). You can define a single cell (for example, `A1`) or multiple cells (for example, `A1:D5`). You can also get cells from other sheets within the same spreadsheet (for example, `Sheet2!A1:C4`) or retrieve multiple ranges at once (for example, `?ranges=A1:D5&ranges=Sheet2!A1:C4`). Limiting the range returns only the portions of the spreadsheet that intersect the requested ranges.
///
/// A builder for the *get* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().get("spreadsheetId")
///              .add_ranges("et")
///              .include_grid_data(false)
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetGetCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _spreadsheet_id: String,
    _ranges: Vec<String>,
    _include_grid_data: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetGetCall<'a, S> {}

impl<'a, S> SpreadsheetGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Spreadsheet)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "spreadsheetId", "ranges", "includeGridData"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);
        if self._ranges.len() > 0 {
            for f in self._ranges.iter() {
                params.push("ranges", f);
            }
        }
        if let Some(value) = self._include_grid_data.as_ref() {
            params.push("includeGridData", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_LENGTH, 0_u64)
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The spreadsheet to request.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetGetCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The ranges to retrieve from the spreadsheet.
    ///
    /// Append the given value to the *ranges* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_ranges(mut self, new_value: &str) -> SpreadsheetGetCall<'a, S> {
        self._ranges.push(new_value.to_string());
        self
    }
    /// True if grid data should be returned. This parameter is ignored if a field mask was set in the request.
    ///
    /// Sets the *include grid data* query property to the given value.
    pub fn include_grid_data(mut self, new_value: bool) -> SpreadsheetGetCall<'a, S> {
        self._include_grid_data = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetGetCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::DriveReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. This method differs from GetSpreadsheet in that it allows selecting which subsets of spreadsheet data to return by specifying a dataFilters parameter. Multiple DataFilters can be specified. Specifying one or more data filters returns the portions of the spreadsheet that intersect ranges matched by any of the filters. By default, data within grids is not returned. You can include grid data one of 2 ways: * Specify a [field mask](https://developers.google.com/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData parameter to true. If a field mask is set, the `includeGridData` parameter is ignored For large spreadsheets, as a best practice, retrieve only the specific spreadsheet fields that you want.
///
/// A builder for the *getByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a [`SpreadsheetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::api::GetSpreadsheetByDataFilterRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use sheets4::{Sheets, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetSpreadsheetByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().get_by_data_filter(req, "spreadsheetId")
///              .doit().await;
/// # }
/// ```
pub struct SpreadsheetGetByDataFilterCall<'a, S>
    where S: 'a {

    hub: &'a Sheets<S>,
    _request: GetSpreadsheetByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for SpreadsheetGetByDataFilterCall<'a, S> {}

impl<'a, S> SpreadsheetGetByDataFilterCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Spreadsheet)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "sheets.spreadsheets.getByDataFilter",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("spreadsheetId", self._spreadsheet_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}:getByDataFilter";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Drive.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["spreadsheetId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: GetSpreadsheetByDataFilterRequest) -> SpreadsheetGetByDataFilterCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The spreadsheet to request.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetGetByDataFilterCall<'a, S> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> SpreadsheetGetByDataFilterCall<'a, S> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetGetByDataFilterCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Drive`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> SpreadsheetGetByDataFilterCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> SpreadsheetGetByDataFilterCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> SpreadsheetGetByDataFilterCall<'a, S> {
        self._scopes.clear();
        self
    }
}


