// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Sheets* crate version *1.0.13+20200402*, where *20200402* is the exact revision of the *sheets:v4* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.13*.
//! 
//! Everything else about the *Sheets* *v4* API can be found at the
//! [official documentation site](https://developers.google.com/sheets/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/sheets4).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Sheets.html) ... 
//! 
//! * [spreadsheets](struct.Spreadsheet.html)
//!  * [*batch update*](struct.SpreadsheetBatchUpdateCall.html), [*create*](struct.SpreadsheetCreateCall.html), [*developer metadata get*](struct.SpreadsheetDeveloperMetadataGetCall.html), [*developer metadata search*](struct.SpreadsheetDeveloperMetadataSearchCall.html), [*get*](struct.SpreadsheetGetCall.html), [*get by data filter*](struct.SpreadsheetGetByDataFilterCall.html), [*sheets copy to*](struct.SpreadsheetSheetCopyToCall.html), [*values append*](struct.SpreadsheetValueAppendCall.html), [*values batch clear*](struct.SpreadsheetValueBatchClearCall.html), [*values batch clear by data filter*](struct.SpreadsheetValueBatchClearByDataFilterCall.html), [*values batch get*](struct.SpreadsheetValueBatchGetCall.html), [*values batch get by data filter*](struct.SpreadsheetValueBatchGetByDataFilterCall.html), [*values batch update*](struct.SpreadsheetValueBatchUpdateCall.html), [*values batch update by data filter*](struct.SpreadsheetValueBatchUpdateByDataFilterCall.html), [*values clear*](struct.SpreadsheetValueClearCall.html), [*values get*](struct.SpreadsheetValueGetCall.html) and [*values update*](struct.SpreadsheetValueUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Sheets.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.spreadsheets().get_by_data_filter(...).doit()
//! let r = hub.spreadsheets().create(...).doit()
//! let r = hub.spreadsheets().values_batch_clear(...).doit()
//! let r = hub.spreadsheets().get(...).doit()
//! let r = hub.spreadsheets().values_batch_get(...).doit()
//! let r = hub.spreadsheets().values_append(...).doit()
//! let r = hub.spreadsheets().values_get(...).doit()
//! let r = hub.spreadsheets().sheets_copy_to(...).doit()
//! let r = hub.spreadsheets().values_clear(...).doit()
//! let r = hub.spreadsheets().values_update(...).doit()
//! let r = hub.spreadsheets().values_batch_update(...).doit()
//! let r = hub.spreadsheets().values_batch_get_by_data_filter(...).doit()
//! let r = hub.spreadsheets().values_batch_clear_by_data_filter(...).doit()
//! let r = hub.spreadsheets().values_batch_update_by_data_filter(...).doit()
//! let r = hub.spreadsheets().batch_update(...).doit()
//! let r = hub.spreadsheets().developer_metadata_search(...).doit()
//! let r = hub.spreadsheets().developer_metadata_get(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-sheets4 = "*"
//! # This project intentionally uses an old version of Hyper. See
//! # https://github.com/Byron/google-apis-rs/issues/173 for more
//! # information.
//! hyper = "^0.10"
//! hyper-rustls = "^0.6"
//! serde = "^1.0"
//! serde_json = "^1.0"
//! yup-oauth2 = "^1.0"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_sheets4 as sheets4;
//! use sheets4::ValueRange;
//! use sheets4::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use sheets4::Sheets;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = ValueRange::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
//!              .value_input_option("justo")
//!              .response_value_render_option("amet.")
//!              .response_date_time_render_option("erat")
//!              .insert_data_option("labore")
//!              .include_values_in_response(true)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [encodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::*;


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// See, edit, create, and delete all of your Google Drive files
    Drive,

    /// See and download all your Google Drive files
    DriveReadonly,

    /// View and manage Google Drive files and folders that you have opened or created with this app
    DriveFile,

    /// See, edit, create, and delete your spreadsheets in Google Drive
    Spreadsheet,

    /// View your Google Spreadsheets
    SpreadsheetReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::DriveReadonly => "https://www.googleapis.com/auth/drive.readonly",
            Scope::DriveFile => "https://www.googleapis.com/auth/drive.file",
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
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_sheets4 as sheets4;
/// use sheets4::ValueRange;
/// use sheets4::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use sheets4::Sheets;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ValueRange::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
///              .value_input_option("gubergren")
///              .response_value_render_option("sadipscing")
///              .response_date_time_render_option("aliquyam")
///              .insert_data_option("ea")
///              .include_values_in_response(false)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct Sheets<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Sheets<C, A> {}

impl<'a, C, A> Sheets<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Sheets<C, A> {
        Sheets {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.13".to_string(),
            _base_url: "https://sheets.googleapis.com/".to_string(),
            _root_url: "https://sheets.googleapis.com/".to_string(),
        }
    }

    pub fn spreadsheets(&'a self) -> SpreadsheetMethods<'a, C, A> {
        SpreadsheetMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.13`.
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
/// A color value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColorStyle {
    /// Theme color.
    #[serde(rename="themeColor")]
    pub theme_color: Option<String>,
    /// RGB color.
    #[serde(rename="rgbColor")]
    pub rgb_color: Option<Color>,
}

impl Part for ColorStyle {}


/// A group over an interval of rows or columns on a sheet, which can contain or
/// be contained within other groups. A group can be collapsed or expanded as a
/// unit on the sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionGroup {
    /// The range over which this group exists.
    pub range: Option<DimensionRange>,
    /// The depth of the group, representing how many groups have a range that
    /// wholly contains the range of this group.
    pub depth: Option<i32>,
    /// This field is true if this group is collapsed. A collapsed group remains
    /// collapsed if an overlapping group at a shallower depth is expanded.
    /// 
    /// A true value does not imply that all dimensions within the group are
    /// hidden, since a dimension's visibility can change independently from this
    /// group property. However, when this property is updated, all dimensions
    /// within it are set to hidden if this field is true, or set to visible if
    /// this field is false.
    pub collapsed: Option<bool>,
}

impl Part for DimensionGroup {}


/// Adds a new conditional format rule at the given index.
/// All subsequent rules' indexes are incremented.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddConditionalFormatRuleRequest {
    /// The zero-based index where the rule should be inserted.
    pub index: Option<i32>,
    /// The rule to add.
    pub rule: Option<ConditionalFormatRule>,
}

impl Part for AddConditionalFormatRuleRequest {}


/// Updates properties of the sheet with the specified
/// sheetId.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSheetPropertiesRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `properties` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The properties to update.
    pub properties: Option<SheetProperties>,
}

impl Part for UpdateSheetPropertiesRequest {}


/// The response from deleting developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDeveloperMetadataResponse {
    /// The metadata that was deleted.
    #[serde(rename="deletedDeveloperMetadata")]
    pub deleted_developer_metadata: Option<Vec<DeveloperMetadata>>,
}

impl Part for DeleteDeveloperMetadataResponse {}


/// The editors of a protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Editors {
    /// True if anyone in the document's domain has edit access to the protected
    /// range.  Domain protection is only supported on documents within a domain.
    #[serde(rename="domainUsersCanEdit")]
    pub domain_users_can_edit: Option<bool>,
    /// The email addresses of users with edit access to the protected range.
    pub users: Option<Vec<String>>,
    /// The email addresses of groups with edit access to the protected range.
    pub groups: Option<Vec<String>>,
}

impl Part for Editors {}


/// Allows you to organize the numeric values in a source data column into
/// buckets of a constant size. All values from HistogramRule.start to
/// HistogramRule.end are placed into groups of size
/// HistogramRule.interval. In addition, all values below
/// HistogramRule.start are placed in one group, and all values above
/// HistogramRule.end are placed in another. Only
/// HistogramRule.interval is required, though if HistogramRule.start
/// and HistogramRule.end are both provided, HistogramRule.start must
/// be less than HistogramRule.end. For example, a pivot table showing
/// average purchase amount by age that has 50+ rows:
/// 
/// ````text
/// +-----+-------------------+
/// | Age | AVERAGE of Amount |
/// +-----+-------------------+
/// | 16  |            $27.13 |
/// | 17  |             $5.24 |
/// | 18  |            $20.15 |
/// ...
/// +-----+-------------------+
/// ````
/// 
/// could be turned into a pivot table that looks like the one below by
/// applying a histogram group rule with a HistogramRule.start of 25,
/// an HistogramRule.interval of 20, and an HistogramRule.end
/// of 65.
/// 
/// ````text
/// +-------------+-------------------+
/// | Grouped Age | AVERAGE of Amount |
/// +-------------+-------------------+
/// | < 25        |            $19.34 |
/// | 25-45       |            $31.43 |
/// | 45-65       |            $35.87 |
/// | > 65        |            $27.55 |
/// +-------------+-------------------+
/// | Grand Total |            $29.12 |
/// +-------------+-------------------+
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramRule {
    /// The minimum value at which items are placed into buckets
    /// of constant size. Values below start are lumped into a single bucket.
    /// This field is optional.
    pub start: Option<f64>,
    /// The size of the buckets that are created. Must be positive.
    pub interval: Option<f64>,
    /// The maximum value at which items are placed into buckets
    /// of constant size. Values above end are lumped into a single bucket.
    /// This field is optional.
    pub end: Option<f64>,
}

impl Part for HistogramRule {}


/// The result of adding a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSheetResponse {
    /// The properties of the newly added sheet.
    pub properties: Option<SheetProperties>,
}

impl Part for AddSheetResponse {}


/// The position of an embedded object such as a chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedObjectPosition {
    /// If true, the embedded object is put on a new sheet whose ID
    /// is chosen for you. Used only when writing.
    #[serde(rename="newSheet")]
    pub new_sheet: Option<bool>,
    /// The sheet this is on. Set only if the embedded object
    /// is on its own sheet. Must be non-negative.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
    /// The position at which the object is overlaid on top of a grid.
    #[serde(rename="overlayPosition")]
    pub overlay_position: Option<OverlayPosition>,
}

impl Part for EmbeddedObjectPosition {}


/// The result of adding a filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddFilterViewResponse {
    /// The newly added filter view.
    pub filter: Option<FilterView>,
}

impl Part for AddFilterViewResponse {}


/// Inserts cells into a range, shifting the existing cells over or down.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertRangeRequest {
    /// The range to insert new cells into.
    pub range: Option<GridRange>,
    /// The dimension which will be shifted when inserting cells.
    /// If ROWS, existing cells will be shifted down.
    /// If COLUMNS, existing cells will be shifted right.
    #[serde(rename="shiftDimension")]
    pub shift_dimension: Option<String>,
}

impl Part for InsertRangeRequest {}


/// A sheet in a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sheet {
    /// The conditional format rules in this sheet.
    #[serde(rename="conditionalFormats")]
    pub conditional_formats: Option<Vec<ConditionalFormatRule>>,
    /// The banded (alternating colors) ranges on this sheet.
    #[serde(rename="bandedRanges")]
    pub banded_ranges: Option<Vec<BandedRange>>,
    /// The ranges that are merged together.
    pub merges: Option<Vec<GridRange>>,
    /// The filter on this sheet, if any.
    #[serde(rename="basicFilter")]
    pub basic_filter: Option<BasicFilter>,
    /// The developer metadata associated with a sheet.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// All column groups on this sheet, ordered by increasing range start index,
    /// then by group depth.
    #[serde(rename="columnGroups")]
    pub column_groups: Option<Vec<DimensionGroup>>,
    /// The specifications of every chart on this sheet.
    pub charts: Option<Vec<EmbeddedChart>>,
    /// The filter views in this sheet.
    #[serde(rename="filterViews")]
    pub filter_views: Option<Vec<FilterView>>,
    /// The slicers on this sheet.
    pub slicers: Option<Vec<Slicer>>,
    /// The properties of the sheet.
    pub properties: Option<SheetProperties>,
    /// The protected ranges in this sheet.
    #[serde(rename="protectedRanges")]
    pub protected_ranges: Option<Vec<ProtectedRange>>,
    /// Data in the grid, if this is a grid sheet.
    /// 
    /// The number of GridData objects returned is dependent on the number of
    /// ranges requested on this sheet. For example, if this is representing
    /// `Sheet1`, and the spreadsheet was requested with ranges
    /// `Sheet1!A1:C10` and `Sheet1!D15:E20`, then the first GridData will have a
    /// startRow/startColumn of `0`,
    /// while the second one will have `startRow 14` (zero-based row 15),
    /// and `startColumn 3` (zero-based column D).
    pub data: Option<Vec<GridData>>,
    /// All row groups on this sheet, ordered by increasing range start index, then
    /// by group depth.
    #[serde(rename="rowGroups")]
    pub row_groups: Option<Vec<DimensionGroup>>,
}

impl Part for Sheet {}


/// The result of removing duplicates in a range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDuplicatesResponse {
    /// The number of duplicate rows removed.
    #[serde(rename="duplicatesRemovedCount")]
    pub duplicates_removed_count: Option<i32>,
}

impl Part for DeleteDuplicatesResponse {}


/// The format of a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CellFormat {
    /// The background color of the cell.
    /// If background_color is also set, this field takes precedence.
    #[serde(rename="backgroundColorStyle")]
    pub background_color_style: Option<ColorStyle>,
    /// A format describing how number values should be represented to the user.
    #[serde(rename="numberFormat")]
    pub number_format: Option<NumberFormat>,
    /// The direction of the text in the cell.
    #[serde(rename="textDirection")]
    pub text_direction: Option<String>,
    /// The padding of the cell.
    pub padding: Option<Padding>,
    /// The horizontal alignment of the value in the cell.
    #[serde(rename="horizontalAlignment")]
    pub horizontal_alignment: Option<String>,
    /// The background color of the cell.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<Color>,
    /// The vertical alignment of the value in the cell.
    #[serde(rename="verticalAlignment")]
    pub vertical_alignment: Option<String>,
    /// The borders of the cell.
    pub borders: Option<Borders>,
    /// The rotation applied to text in a cell
    #[serde(rename="textRotation")]
    pub text_rotation: Option<TextRotation>,
    /// How a hyperlink, if it exists, should be displayed in the cell.
    #[serde(rename="hyperlinkDisplayType")]
    pub hyperlink_display_type: Option<String>,
    /// The format of the text in the cell (unless overridden by a format run).
    #[serde(rename="textFormat")]
    pub text_format: Option<TextFormat>,
    /// The wrap strategy for the value in the cell.
    #[serde(rename="wrapStrategy")]
    pub wrap_strategy: Option<String>,
}

impl Part for CellFormat {}


/// Inserts data into the spreadsheet starting at the specified coordinate.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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

impl Part for PasteDataRequest {}


/// The result of adding a slicer to a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSlicerResponse {
    /// The newly added slicer.
    pub slicer: Option<Slicer>,
}

impl Part for AddSlicerResponse {}


/// Criteria for showing/hiding rows in a filter or filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterCriteria {
    /// The background fill color to filter by; only cells with this fill color are
    /// shown. Mutually exclusive with visible_foreground_color.
    #[serde(rename="visibleBackgroundColor")]
    pub visible_background_color: Option<Color>,
    /// Values that should be hidden.
    #[serde(rename="hiddenValues")]
    pub hidden_values: Option<Vec<String>>,
    /// The background fill color to filter by; only cells with this fill color are
    /// shown. This field is mutually exclusive with visible_foreground_color,
    /// and must be set to an RGB-type color. If visible_background_color is
    /// also set, this field takes precedence.
    #[serde(rename="visibleBackgroundColorStyle")]
    pub visible_background_color_style: Option<ColorStyle>,
    /// The foreground color to filter by; only cells with this foreground color
    /// are shown. This field is mutually exclusive with
    /// visible_background_color, and must be set to an RGB-type color. If
    /// visible_foreground_color is also set, this field takes precedence.
    #[serde(rename="visibleForegroundColorStyle")]
    pub visible_foreground_color_style: Option<ColorStyle>,
    /// The foreground color to filter by; only cells with this foreground color
    /// are shown. Mutually exclusive with visible_background_color.
    #[serde(rename="visibleForegroundColor")]
    pub visible_foreground_color: Option<Color>,
    /// A condition that must be true for values to be shown.
    /// (This does not override hidden_values -- if a value is listed there,
    ///  it will still be hidden.)
    pub condition: Option<BooleanCondition>,
}

impl Part for FilterCriteria {}


/// The default filter associated with a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicFilter {
    /// The range the filter covers.
    pub range: Option<GridRange>,
    /// The sort order per column. Later specifications are used when values
    /// are equal in the earlier specifications.
    #[serde(rename="sortSpecs")]
    pub sort_specs: Option<Vec<SortSpec>>,
    /// The criteria for showing/hiding values per column.
    /// The map's key is the column index, and the value is the criteria for
    /// that column.
    pub criteria: Option<HashMap<String, FilterCriteria>>,
}

impl Part for BasicFilter {}


/// Adds a filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddFilterViewRequest {
    /// The filter to add. The filterViewId
    /// field is optional; if one is not set, an id will be randomly generated. (It
    /// is an error to specify the ID of a filter that already exists.)
    pub filter: Option<FilterView>,
}

impl Part for AddFilterViewRequest {}


/// The specification for a basic chart.  See BasicChartType for the list
/// of charts this supports.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartSpec {
    /// The stacked type for charts that support vertical stacking.
    /// Applies to Area, Bar, Column, Combo, and Stepped Area charts.
    #[serde(rename="stackedType")]
    pub stacked_type: Option<String>,
    /// The number of rows or columns in the data that are "headers".
    /// If not set, Google Sheets will guess how many rows are headers based
    /// on the data.
    /// 
    /// (Note that BasicChartAxis.title may override the axis title
    ///  inferred from the header values.)
    #[serde(rename="headerCount")]
    pub header_count: Option<i32>,
    /// The data this chart is visualizing.
    pub series: Option<Vec<BasicChartSeries>>,
    /// If some values in a series are missing, gaps may appear in the chart (e.g,
    /// segments of lines in a line chart will be missing).  To eliminate these
    /// gaps set this to true.
    /// Applies to Line, Area, and Combo charts.
    #[serde(rename="interpolateNulls")]
    pub interpolate_nulls: Option<bool>,
    /// The position of the chart legend.
    #[serde(rename="legendPosition")]
    pub legend_position: Option<String>,
    /// Gets whether all lines should be rendered smooth or straight by default.
    /// Applies to Line charts.
    #[serde(rename="lineSmoothing")]
    pub line_smoothing: Option<bool>,
    /// The behavior of tooltips and data highlighting when hovering on data and
    /// chart area.
    #[serde(rename="compareMode")]
    pub compare_mode: Option<String>,
    /// The domain of data this is charting.
    /// Only a single domain is supported.
    pub domains: Option<Vec<BasicChartDomain>>,
    /// The type of the chart.
    #[serde(rename="chartType")]
    pub chart_type: Option<String>,
    /// The axis on the chart.
    pub axis: Option<Vec<BasicChartAxis>>,
    /// True to make the chart 3D.
    /// Applies to Bar and Column charts.
    #[serde(rename="threeDimensional")]
    pub three_dimensional: Option<bool>,
}

impl Part for BasicChartSpec {}


/// Allows you to manually organize the values in a source data column into
/// buckets with names of your choosing. For example, a pivot table that
/// aggregates population by state:
/// 
/// ````text
/// +-------+-------------------+
/// | State | SUM of Population |
/// +-------+-------------------+
/// | AK    |               0.7 |
/// | AL    |               4.8 |
/// | AR    |               2.9 |
/// ...
/// +-------+-------------------+
/// ````
/// 
/// could be turned into a pivot table that aggregates population by time zone
/// by providing a list of groups (for example, groupName = 'Central',
/// items = ['AL', 'AR', 'IA', ...]) to a manual group rule.
/// Note that a similar effect could be achieved by adding a time zone column
/// to the source data and adjusting the pivot table.
/// 
/// ````text
/// +-----------+-------------------+
/// | Time Zone | SUM of Population |
/// +-----------+-------------------+
/// | Central   |             106.3 |
/// | Eastern   |             151.9 |
/// | Mountain  |              17.4 |
/// ...
/// +-----------+-------------------+
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualRule {
    /// The list of group names and the corresponding items from the source data
    /// that map to each group name.
    pub groups: Option<Vec<ManualRuleGroup>>,
}

impl Part for ManualRule {}


/// Duplicates a particular filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateFilterViewRequest {
    /// The ID of the filter being duplicated.
    #[serde(rename="filterId")]
    pub filter_id: Option<i32>,
}

impl Part for DuplicateFilterViewRequest {}


/// A waterfall chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartSpec {
    /// True to hide connector lines between columns.
    #[serde(rename="hideConnectorLines")]
    pub hide_connector_lines: Option<bool>,
    /// The stacked type.
    #[serde(rename="stackedType")]
    pub stacked_type: Option<String>,
    /// The domain data (horizontal axis) for the waterfall chart.
    pub domain: Option<WaterfallChartDomain>,
    /// The line style for the connector lines.
    #[serde(rename="connectorLineStyle")]
    pub connector_line_style: Option<LineStyle>,
    /// The data this waterfall chart is visualizing.
    pub series: Option<Vec<WaterfallChartSeries>>,
    /// True to interpret the first value as a total.
    #[serde(rename="firstValueIsTotal")]
    pub first_value_is_total: Option<bool>,
}

impl Part for WaterfallChartSpec {}


/// A group name and a list of items from the source data that should be placed
/// in the group with this name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ManualRuleGroup {
    /// The group name, which must be a string. Each group in a given
    /// ManualRule must have a unique group name.
    #[serde(rename="groupName")]
    pub group_name: Option<ExtendedValue>,
    /// The items in the source data that should be placed into this group. Each
    /// item may be a string, number, or boolean. Items may appear in at most one
    /// group within a given ManualRule. Items that do not appear in any
    /// group will appear on their own.
    pub items: Option<Vec<ExtendedValue>>,
}

impl Part for ManualRuleGroup {}


/// The request for clearing more than one range selected by a
/// DataFilter in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear by data filter spreadsheets](struct.SpreadsheetValueBatchClearByDataFilterCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesByDataFilterRequest {
    /// The DataFilters used to determine which ranges to clear.
    #[serde(rename="dataFilters")]
    pub data_filters: Option<Vec<DataFilter>>,
}

impl RequestValue for BatchClearValuesByDataFilterRequest {}


/// A request to create developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateDeveloperMetadataRequest {
    /// The developer metadata to create.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<DeveloperMetadata>,
}

impl Part for CreateDeveloperMetadataRequest {}


/// Selects DeveloperMetadata that matches all of the specified fields.  For
/// example, if only a metadata ID is specified this considers the
/// DeveloperMetadata with that particular unique ID. If a metadata key is
/// specified, this considers all developer metadata with that key.  If a
/// key, visibility, and location type are all specified, this considers all
/// developer metadata with that key and visibility that are associated with a
/// location of that type.  In general, this
/// selects all DeveloperMetadata that matches the intersection of all the
/// specified fields; any field or combination of fields may be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperMetadataLookup {
    /// Limits the selected developer metadata to those entries associated with
    /// the specified location.  This field either matches exact locations or all
    /// intersecting locations according the specified
    /// locationMatchingStrategy.
    #[serde(rename="metadataLocation")]
    pub metadata_location: Option<DeveloperMetadataLocation>,
    /// Limits the selected developer metadata to that which has a matching
    /// DeveloperMetadata.metadata_value.
    #[serde(rename="metadataValue")]
    pub metadata_value: Option<String>,
    /// Determines how this lookup matches the location.  If this field is
    /// specified as EXACT, only developer metadata associated on the exact
    /// location specified is matched.  If this field is specified to INTERSECTING,
    /// developer metadata associated on intersecting locations is also
    /// matched.  If left unspecified, this field assumes a default value of
    /// INTERSECTING.
    /// If this field is specified, a metadataLocation
    /// must also be specified.
    #[serde(rename="locationMatchingStrategy")]
    pub location_matching_strategy: Option<String>,
    /// Limits the selected developer metadata to those entries which are
    /// associated with locations of the specified type.  For example, when this
    /// field is specified as ROW this lookup
    /// only considers developer metadata associated on rows.  If the field is left
    /// unspecified, all location types are considered.  This field cannot be
    /// specified as SPREADSHEET when
    /// the locationMatchingStrategy
    /// is specified as INTERSECTING or when the
    /// metadataLocation is specified as a
    /// non-spreadsheet location: spreadsheet metadata cannot intersect any other
    /// developer metadata location.  This field also must be left unspecified when
    /// the locationMatchingStrategy
    /// is specified as EXACT.
    #[serde(rename="locationType")]
    pub location_type: Option<String>,
    /// Limits the selected developer metadata to that which has a matching
    /// DeveloperMetadata.metadata_id.
    #[serde(rename="metadataId")]
    pub metadata_id: Option<i32>,
    /// Limits the selected developer metadata to that which has a matching
    /// DeveloperMetadata.visibility.  If left unspecified, all developer
    /// metadata visibile to the requesting project is considered.
    pub visibility: Option<String>,
    /// Limits the selected developer metadata to that which has a matching
    /// DeveloperMetadata.metadata_key.
    #[serde(rename="metadataKey")]
    pub metadata_key: Option<String>,
}

impl Part for DeveloperMetadataLookup {}


/// A scorecard chart. Scorecard charts are used to highlight key performance
/// indicators, known as KPIs, on the spreadsheet. A scorecard chart can
/// represent things like total sales, average cost, or a top selling item. You
/// can specify a single data value, or aggregate over a range of data.
/// Percentage or absolute difference from a baseline value can be highlighted,
/// like changes over time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScorecardChartSpec {
    /// The number format source used in the scorecard chart.
    /// This field is optional.
    #[serde(rename="numberFormatSource")]
    pub number_format_source: Option<String>,
    /// The data for scorecard baseline value.
    /// This field is optional.
    #[serde(rename="baselineValueData")]
    pub baseline_value_data: Option<ChartData>,
    /// Formatting options for key value.
    #[serde(rename="keyValueFormat")]
    pub key_value_format: Option<KeyValueFormat>,
    /// The aggregation type for key and baseline chart data in scorecard chart.
    /// This field is optional.
    #[serde(rename="aggregateType")]
    pub aggregate_type: Option<String>,
    /// Formatting options for baseline value.
    /// This field is needed only if baseline_value_data is specified.
    #[serde(rename="baselineValueFormat")]
    pub baseline_value_format: Option<BaselineValueFormat>,
    /// The data for scorecard key value.
    #[serde(rename="keyValueData")]
    pub key_value_data: Option<ChartData>,
    /// Value to scale scorecard key and baseline value. For example, a factor of
    /// 10 can be used to divide all values in the chart by 10.
    /// This field is optional.
    #[serde(rename="scaleFactor")]
    pub scale_factor: Option<f64>,
    /// Custom formatting options for numeric key/baseline values in scorecard
    /// chart. This field is used only when number_format_source is set to
    /// CUSTOM. This field is optional.
    #[serde(rename="customFormatOptions")]
    pub custom_format_options: Option<ChartCustomNumberFormatOptions>,
}

impl Part for ScorecardChartSpec {}


/// Adds new cells after the last row with data in a sheet,
/// inserting new rows into the sheet if necessary.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppendCellsRequest {
    /// The fields of CellData that should be updated.
    /// At least one field must be specified.
    /// The root is the CellData; 'row.values.' should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The data to append.
    pub rows: Option<Vec<RowData>>,
    /// The sheet ID to append the data to.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for AppendCellsRequest {}


/// The specifications of a chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartSpec {
    /// The name of the font to use by default for all chart text (e.g. title,
    /// axis labels, legend).  If a font is specified for a specific part of the
    /// chart it will override this font name.
    #[serde(rename="fontName")]
    pub font_name: Option<String>,
    /// The alternative text that describes the chart.  This is often used
    /// for accessibility.
    #[serde(rename="altText")]
    pub alt_text: Option<String>,
    /// The subtitle of the chart.
    pub subtitle: Option<String>,
    /// The background color of the entire chart.
    /// Not applicable to Org charts.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<Color>,
    /// The title text format.
    /// Strikethrough and underline are not supported.
    #[serde(rename="titleTextFormat")]
    pub title_text_format: Option<TextFormat>,
    /// The title text position.
    /// This field is optional.
    #[serde(rename="titleTextPosition")]
    pub title_text_position: Option<TextPosition>,
    /// Determines how the charts will use hidden rows or columns.
    #[serde(rename="hiddenDimensionStrategy")]
    pub hidden_dimension_strategy: Option<String>,
    /// A pie chart specification.
    #[serde(rename="pieChart")]
    pub pie_chart: Option<PieChartSpec>,
    /// A histogram chart specification.
    #[serde(rename="histogramChart")]
    pub histogram_chart: Option<HistogramChartSpec>,
    /// A bubble chart specification.
    #[serde(rename="bubbleChart")]
    pub bubble_chart: Option<BubbleChartSpec>,
    /// A basic chart specification, can be one of many kinds of charts.
    /// See BasicChartType for the list of all
    /// charts this supports.
    #[serde(rename="basicChart")]
    pub basic_chart: Option<BasicChartSpec>,
    /// The title of the chart.
    pub title: Option<String>,
    /// A treemap chart specification.
    #[serde(rename="treemapChart")]
    pub treemap_chart: Option<TreemapChartSpec>,
    /// A scorecard chart specification.
    #[serde(rename="scorecardChart")]
    pub scorecard_chart: Option<ScorecardChartSpec>,
    /// The background color of the entire chart.
    /// Not applicable to Org charts.
    /// If background_color is also set, this field takes precedence.
    #[serde(rename="backgroundColorStyle")]
    pub background_color_style: Option<ColorStyle>,
    /// A candlestick chart specification.
    #[serde(rename="candlestickChart")]
    pub candlestick_chart: Option<CandlestickChartSpec>,
    /// The subtitle text position.
    /// This field is optional.
    #[serde(rename="subtitleTextPosition")]
    pub subtitle_text_position: Option<TextPosition>,
    /// The subtitle text format.
    /// Strikethrough and underline are not supported.
    #[serde(rename="subtitleTextFormat")]
    pub subtitle_text_format: Option<TextFormat>,
    /// True to make a chart fill the entire space in which it's rendered with
    /// minimum padding.  False to use the default padding.
    /// (Not applicable to Geo and Org charts.)
    pub maximized: Option<bool>,
    /// A waterfall chart specification.
    #[serde(rename="waterfallChart")]
    pub waterfall_chart: Option<WaterfallChartSpec>,
    /// An org chart specification.
    #[serde(rename="orgChart")]
    pub org_chart: Option<OrgChartSpec>,
}

impl Part for ChartSpec {}


/// A rule describing a conditional format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConditionalFormatRule {
    /// The ranges that are formatted if the condition is true.
    /// All the ranges must be on the same grid.
    pub ranges: Option<Vec<GridRange>>,
    /// The formatting is either "on" or "off" according to the rule.
    #[serde(rename="booleanRule")]
    pub boolean_rule: Option<BooleanRule>,
    /// The formatting will vary based on the gradients in the rule.
    #[serde(rename="gradientRule")]
    pub gradient_rule: Option<GradientRule>,
}

impl Part for ConditionalFormatRule {}


/// A protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProtectedRange {
    /// The list of unprotected ranges within a protected sheet.
    /// Unprotected ranges are only supported on protected sheets.
    #[serde(rename="unprotectedRanges")]
    pub unprotected_ranges: Option<Vec<GridRange>>,
    /// True if the user who requested this protected range can edit the
    /// protected area.
    /// This field is read-only.
    #[serde(rename="requestingUserCanEdit")]
    pub requesting_user_can_edit: Option<bool>,
    /// The description of this protected range.
    pub description: Option<String>,
    /// The named range this protected range is backed by, if any.
    /// 
    /// When writing, only one of range or named_range_id
    /// may be set.
    #[serde(rename="namedRangeId")]
    pub named_range_id: Option<String>,
    /// The users and groups with edit access to the protected range.
    /// This field is only visible to users with edit access to the protected
    /// range and the document.
    /// Editors are not supported with warning_only protection.
    pub editors: Option<Editors>,
    /// The range that is being protected.
    /// The range may be fully unbounded, in which case this is considered
    /// a protected sheet.
    /// 
    /// When writing, only one of range or named_range_id
    /// may be set.
    pub range: Option<GridRange>,
    /// The ID of the protected range.
    /// This field is read-only.
    #[serde(rename="protectedRangeId")]
    pub protected_range_id: Option<i32>,
    /// True if this protected range will show a warning when editing.
    /// Warning-based protection means that every user can edit data in the
    /// protected range, except editing will prompt a warning asking the user
    /// to confirm the edit.
    /// 
    /// When writing: if this field is true, then editors is ignored.
    /// Additionally, if this field is changed from true to false and the
    /// `editors` field is not set (nor included in the field mask), then
    /// the editors will be set to all the editors in the document.
    #[serde(rename="warningOnly")]
    pub warning_only: Option<bool>,
}

impl Part for ProtectedRange {}


/// The result of updating an embedded object's position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEmbeddedObjectPositionResponse {
    /// The new position of the embedded object.
    pub position: Option<EmbeddedObjectPosition>,
}

impl Part for UpdateEmbeddedObjectPositionResponse {}


/// Updates properties of a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSpreadsheetPropertiesRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root 'properties' is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The properties to update.
    pub properties: Option<SpreadsheetProperties>,
}

impl Part for UpdateSpreadsheetPropertiesRequest {}


/// Deletes the protected range with the given ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteProtectedRangeRequest {
    /// The ID of the protected range to delete.
    #[serde(rename="protectedRangeId")]
    pub protected_range_id: Option<i32>,
}

impl Part for DeleteProtectedRangeRequest {}


/// Copies data from the source to the destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopyPasteRequest {
    /// What kind of data to paste.
    #[serde(rename="pasteType")]
    pub paste_type: Option<String>,
    /// The source range to copy.
    pub source: Option<GridRange>,
    /// The location to paste to. If the range covers a span that's
    /// a multiple of the source's height or width, then the
    /// data will be repeated to fill in the destination range.
    /// If the range is smaller than the source range, the entire
    /// source data will still be copied (beyond the end of the destination range).
    pub destination: Option<GridRange>,
    /// How that data should be oriented when pasting.
    #[serde(rename="pasteOrientation")]
    pub paste_orientation: Option<String>,
}

impl Part for CopyPasteRequest {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update spreadsheets](struct.SpreadsheetValueBatchUpdateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// One UpdateValuesResponse per requested range, in the same order as
    /// the requests appeared.
    pub responses: Option<Vec<UpdateValuesResponse>>,
    /// The total number of cells updated.
    #[serde(rename="totalUpdatedCells")]
    pub total_updated_cells: Option<i32>,
    /// The total number of rows where at least one cell in the row was updated.
    #[serde(rename="totalUpdatedRows")]
    pub total_updated_rows: Option<i32>,
    /// The total number of sheets where at least one cell in the sheet was
    /// updated.
    #[serde(rename="totalUpdatedSheets")]
    pub total_updated_sheets: Option<i32>,
    /// The total number of columns where at least one cell in the column was
    /// updated.
    #[serde(rename="totalUpdatedColumns")]
    pub total_updated_columns: Option<i32>,
}

impl ResponseResult for BatchUpdateValuesResponse {}


/// The domain of a CandlestickChart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickDomain {
    /// True to reverse the order of the domain values (horizontal axis).
    pub reversed: Option<bool>,
    /// The data of the CandlestickDomain.
    pub data: Option<ChartData>,
}

impl Part for CandlestickDomain {}


/// The number format of a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NumberFormat {
    /// Pattern string used for formatting.  If not set, a default pattern based on
    /// the user's locale will be used if necessary for the given type.
    /// See the [Date and Number Formats guide](/sheets/api/guides/formats) for
    /// more information about the supported patterns.
    pub pattern: Option<String>,
    /// The type of the number format.
    /// When writing, this field must be set.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for NumberFormat {}


/// A slicer in a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Slicer {
    /// The position of the slicer. Note that slicer can be positioned only on
    /// existing sheet. Also, width and height of slicer can be automatically
    /// adjusted to keep it within permitted limits.
    pub position: Option<EmbeddedObjectPosition>,
    /// The specification of the slicer.
    pub spec: Option<SlicerSpec>,
    /// The ID of the slicer.
    #[serde(rename="slicerId")]
    pub slicer_id: Option<i32>,
}

impl Part for Slicer {}


/// Represents spreadsheet theme
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpreadsheetTheme {
    /// The spreadsheet theme color pairs. To update you must provide all theme
    /// color pairs.
    #[serde(rename="themeColors")]
    pub theme_colors: Option<Vec<ThemeColorPair>>,
    /// / Name of the primary font family.
    #[serde(rename="primaryFontFamily")]
    pub primary_font_family: Option<String>,
}

impl Part for SpreadsheetTheme {}


/// A sort order associated with a specific column or row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SortSpec {
    /// The background fill color to sort by; cells with this fill color are sorted
    /// to the top. Mutually exclusive with foreground_color, and must be an
    /// RGB-type color. If background_color is also set, this field takes
    /// precedence.
    #[serde(rename="backgroundColorStyle")]
    pub background_color_style: Option<ColorStyle>,
    /// The foreground color to sort by; cells with this foreground color are
    /// sorted to the top. Mutually exclusive with background_color.
    #[serde(rename="foregroundColor")]
    pub foreground_color: Option<Color>,
    /// The order data should be sorted.
    #[serde(rename="sortOrder")]
    pub sort_order: Option<String>,
    /// The background fill color to sort by; cells with this fill color are sorted
    /// to the top. Mutually exclusive with foreground_color.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<Color>,
    /// The foreground color to sort by; cells with this foreground color are
    /// sorted to the top. Mutually exclusive with background_color, and must
    /// be an RGB-type color. If foreground_color is also set, this field takes
    /// precedence.
    #[serde(rename="foregroundColorStyle")]
    pub foreground_color_style: Option<ColorStyle>,
    /// The dimension the sort should be applied to.
    #[serde(rename="dimensionIndex")]
    pub dimension_index: Option<i32>,
}

impl Part for SortSpec {}


/// Adds a new protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddProtectedRangeRequest {
    /// The protected range to be added. The
    /// protectedRangeId field is optional; if
    /// one is not set, an id will be randomly generated. (It is an error to
    /// specify the ID of a range that already exists.)
    #[serde(rename="protectedRange")]
    pub protected_range: Option<ProtectedRange>,
}

impl Part for AddProtectedRangeRequest {}


/// The result of deleting a conditional format rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteConditionalFormatRuleResponse {
    /// The rule that was deleted.
    pub rule: Option<ConditionalFormatRule>,
}

impl Part for DeleteConditionalFormatRuleResponse {}


/// Sets a data validation rule to every cell in the range.
/// To clear validation in a range, call this with no rule specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetDataValidationRequest {
    /// The range the data validation rule should apply to.
    pub range: Option<GridRange>,
    /// The data validation rule to set on each cell in the range,
    /// or empty to clear the data validation in the range.
    pub rule: Option<DataValidationRule>,
}

impl Part for SetDataValidationRequest {}


/// Updates an existing protected range with the specified
/// protectedRangeId.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProtectedRangeRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `protectedRange` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The protected range to update with the new properties.
    #[serde(rename="protectedRange")]
    pub protected_range: Option<ProtectedRange>,
}

impl Part for UpdateProtectedRangeRequest {}


/// Updates properties of the named range with the specified
/// namedRangeId.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateNamedRangeRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `namedRange` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The named range to update with the new properties.
    #[serde(rename="namedRange")]
    pub named_range: Option<NamedRange>,
}

impl Part for UpdateNamedRangeRequest {}


/// Updates properties of dimensions within the specified range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDimensionPropertiesRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `properties` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The rows or columns to update.
    pub range: Option<DimensionRange>,
    /// Properties to update.
    pub properties: Option<DimensionProperties>,
}

impl Part for UpdateDimensionPropertiesRequest {}


/// Moves data from the source to the destination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CutPasteRequest {
    /// What kind of data to paste.  All the source data will be cut, regardless
    /// of what is pasted.
    #[serde(rename="pasteType")]
    pub paste_type: Option<String>,
    /// The source data to cut.
    pub source: Option<GridRange>,
    /// The top-left coordinate where the data should be pasted.
    pub destination: Option<GridCoordinate>,
}

impl Part for CutPasteRequest {}


/// Updates a slicer's specifications.
/// (This does not move or resize a slicer. To move or resize a slicer use
/// UpdateEmbeddedObjectPositionRequest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSlicerSpecRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `SlicerSpec` is implied and should not be specified. A single "*"`
    /// can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The specification to apply to the slicer.
    pub spec: Option<SlicerSpec>,
    /// The id of the slicer to update.
    #[serde(rename="slicerId")]
    pub slicer_id: Option<i32>,
}

impl Part for UpdateSlicerSpecRequest {}


/// A <a href="/chart/interactive/docs/gallery/bubblechart">bubble chart</a>.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BubbleChartSpec {
    /// The opacity of the bubbles between 0 and 1.0.
    /// 0 is fully transparent and 1 is fully opaque.
    #[serde(rename="bubbleOpacity")]
    pub bubble_opacity: Option<f32>,
    /// The data containing the bubble x-values.  These values locate the bubbles
    /// in the chart horizontally.
    pub domain: Option<ChartData>,
    /// The format of the text inside the bubbles.
    /// Underline and Strikethrough are not supported.
    #[serde(rename="bubbleTextStyle")]
    pub bubble_text_style: Option<TextFormat>,
    /// The data contianing the bubble y-values.  These values locate the bubbles
    /// in the chart vertically.
    pub series: Option<ChartData>,
    /// The bubble border color.
    /// If bubble_border_color is also set, this field takes precedence.
    #[serde(rename="bubbleBorderColorStyle")]
    pub bubble_border_color_style: Option<ColorStyle>,
    /// Where the legend of the chart should be drawn.
    #[serde(rename="legendPosition")]
    pub legend_position: Option<String>,
    /// The max radius size of the bubbles, in pixels.
    /// If specified, the field must be a positive value.
    #[serde(rename="bubbleMaxRadiusSize")]
    pub bubble_max_radius_size: Option<i32>,
    /// The minimum radius size of the bubbles, in pixels.
    /// If specific, the field must be a positive value.
    #[serde(rename="bubbleMinRadiusSize")]
    pub bubble_min_radius_size: Option<i32>,
    /// The data containing the bubble group IDs. All bubbles with the same group
    /// ID are drawn in the same color. If bubble_sizes is specified then
    /// this field must also be specified but may contain blank values.
    /// This field is optional.
    #[serde(rename="groupIds")]
    pub group_ids: Option<ChartData>,
    /// The data contianing the bubble sizes.  Bubble sizes are used to draw
    /// the bubbles at different sizes relative to each other.
    /// If specified, group_ids must also be specified.  This field is
    /// optional.
    #[serde(rename="bubbleSizes")]
    pub bubble_sizes: Option<ChartData>,
    /// The bubble border color.
    #[serde(rename="bubbleBorderColor")]
    pub bubble_border_color: Option<Color>,
    /// The data containing the bubble labels.  These do not need to be unique.
    #[serde(rename="bubbleLabels")]
    pub bubble_labels: Option<ChartData>,
}

impl Part for BubbleChartSpec {}


/// A chart embedded in a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedChart {
    /// The ID of the chart.
    #[serde(rename="chartId")]
    pub chart_id: Option<i32>,
    /// The position of the chart.
    pub position: Option<EmbeddedObjectPosition>,
    /// The specification of the chart.
    pub spec: Option<ChartSpec>,
}

impl Part for EmbeddedChart {}


/// Resource that represents a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get by data filter spreadsheets](struct.SpreadsheetGetByDataFilterCall.html) (response)
/// * [create spreadsheets](struct.SpreadsheetCreateCall.html) (request|response)
/// * [values batch clear spreadsheets](struct.SpreadsheetValueBatchClearCall.html) (none)
/// * [get spreadsheets](struct.SpreadsheetGetCall.html) (response)
/// * [values batch get spreadsheets](struct.SpreadsheetValueBatchGetCall.html) (none)
/// * [values append spreadsheets](struct.SpreadsheetValueAppendCall.html) (none)
/// * [values get spreadsheets](struct.SpreadsheetValueGetCall.html) (none)
/// * [sheets copy to spreadsheets](struct.SpreadsheetSheetCopyToCall.html) (none)
/// * [values clear spreadsheets](struct.SpreadsheetValueClearCall.html) (none)
/// * [values update spreadsheets](struct.SpreadsheetValueUpdateCall.html) (none)
/// * [values batch update spreadsheets](struct.SpreadsheetValueBatchUpdateCall.html) (none)
/// * [values batch get by data filter spreadsheets](struct.SpreadsheetValueBatchGetByDataFilterCall.html) (none)
/// * [values batch clear by data filter spreadsheets](struct.SpreadsheetValueBatchClearByDataFilterCall.html) (none)
/// * [values batch update by data filter spreadsheets](struct.SpreadsheetValueBatchUpdateByDataFilterCall.html) (none)
/// * [batch update spreadsheets](struct.SpreadsheetBatchUpdateCall.html) (none)
/// * [developer metadata search spreadsheets](struct.SpreadsheetDeveloperMetadataSearchCall.html) (none)
/// * [developer metadata get spreadsheets](struct.SpreadsheetDeveloperMetadataGetCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Spreadsheet {
    /// The sheets that are part of a spreadsheet.
    pub sheets: Option<Vec<Sheet>>,
    /// The url of the spreadsheet.
    /// This field is read-only.
    #[serde(rename="spreadsheetUrl")]
    pub spreadsheet_url: Option<String>,
    /// The ID of the spreadsheet.
    /// This field is read-only.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The named ranges defined in a spreadsheet.
    #[serde(rename="namedRanges")]
    pub named_ranges: Option<Vec<NamedRange>>,
    /// The developer metadata associated with a spreadsheet.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// Overall properties of a spreadsheet.
    pub properties: Option<SpreadsheetProperties>,
}

impl RequestValue for Spreadsheet {}
impl Resource for Spreadsheet {}
impl ResponseResult for Spreadsheet {}


/// Fills in more data based on existing data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoFillRequest {
    /// True if we should generate data with the "alternate" series.
    /// This differs based on the type and amount of source data.
    #[serde(rename="useAlternateSeries")]
    pub use_alternate_series: Option<bool>,
    /// The range to autofill. This will examine the range and detect
    /// the location that has data and automatically fill that data
    /// in to the rest of the range.
    pub range: Option<GridRange>,
    /// The source and destination areas to autofill.
    /// This explicitly lists the source of the autofill and where to
    /// extend that data.
    #[serde(rename="sourceAndDestination")]
    pub source_and_destination: Option<SourceAndDestination>,
}

impl Part for AutoFillRequest {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values append spreadsheets](struct.SpreadsheetValueAppendCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppendValuesResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The range (in A1 notation) of the table that values are being appended to
    /// (before the values were appended).
    /// Empty if no table was found.
    #[serde(rename="tableRange")]
    pub table_range: Option<String>,
    /// Information about the updates that were applied.
    pub updates: Option<UpdateValuesResponse>,
}

impl ResponseResult for AppendValuesResponse {}


/// The result of updating a conditional format rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateConditionalFormatRuleResponse {
    /// The old index of the rule. Not set if a rule was replaced
    /// (because it is the same as new_index).
    #[serde(rename="oldIndex")]
    pub old_index: Option<i32>,
    /// The index of the new rule.
    #[serde(rename="newIndex")]
    pub new_index: Option<i32>,
    /// The old (deleted) rule. Not set if a rule was moved
    /// (because it is the same as new_rule).
    #[serde(rename="oldRule")]
    pub old_rule: Option<ConditionalFormatRule>,
    /// The new rule that replaced the old rule (if replacing),
    /// or the rule that was moved (if moved)
    #[serde(rename="newRule")]
    pub new_rule: Option<ConditionalFormatRule>,
}

impl Part for UpdateConditionalFormatRuleResponse {}


/// Adds a chart to a sheet in the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddChartRequest {
    /// The chart that should be added to the spreadsheet, including the position
    /// where it should be placed. The chartId
    /// field is optional; if one is not set, an id will be randomly generated. (It
    /// is an error to specify the ID of an embedded object that already exists.)
    pub chart: Option<EmbeddedChart>,
}

impl Part for AddChartRequest {}


/// The result of adding a named range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddNamedRangeResponse {
    /// The named range to add.
    #[serde(rename="namedRange")]
    pub named_range: Option<NamedRange>,
}

impl Part for AddNamedRangeResponse {}


/// The domain of a waterfall chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartDomain {
    /// True to reverse the order of the domain values (horizontal axis).
    pub reversed: Option<bool>,
    /// The data of the WaterfallChartDomain.
    pub data: Option<ChartData>,
}

impl Part for WaterfallChartDomain {}


/// The result of duplicating a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateSheetResponse {
    /// The properties of the duplicate sheet.
    pub properties: Option<SheetProperties>,
}

impl Part for DuplicateSheetResponse {}


/// The location an object is overlaid on top of a grid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OverlayPosition {
    /// The cell the object is anchored to.
    #[serde(rename="anchorCell")]
    pub anchor_cell: Option<GridCoordinate>,
    /// The vertical offset, in pixels, that the object is offset
    /// from the anchor cell.
    #[serde(rename="offsetYPixels")]
    pub offset_y_pixels: Option<i32>,
    /// The width of the object, in pixels. Defaults to 600.
    #[serde(rename="widthPixels")]
    pub width_pixels: Option<i32>,
    /// The horizontal offset, in pixels, that the object is offset
    /// from the anchor cell.
    #[serde(rename="offsetXPixels")]
    pub offset_x_pixels: Option<i32>,
    /// The height of the object, in pixels. Defaults to 371.
    #[serde(rename="heightPixels")]
    pub height_pixels: Option<i32>,
}

impl Part for OverlayPosition {}


/// The format of a run of text in a cell.
/// Absent values indicate that the field isn't specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextFormat {
    /// The foreground color of the text.
    #[serde(rename="foregroundColor")]
    pub foreground_color: Option<Color>,
    /// True if the text is bold.
    pub bold: Option<bool>,
    /// The foreground color of the text.
    /// If foreground_color is also set, this field takes precedence.
    #[serde(rename="foregroundColorStyle")]
    pub foreground_color_style: Option<ColorStyle>,
    /// True if the text has a strikethrough.
    pub strikethrough: Option<bool>,
    /// The font family.
    #[serde(rename="fontFamily")]
    pub font_family: Option<String>,
    /// The size of the font.
    #[serde(rename="fontSize")]
    pub font_size: Option<i32>,
    /// True if the text is italicized.
    pub italic: Option<bool>,
    /// True if the text is underlined.
    pub underline: Option<bool>,
}

impl Part for TextFormat {}


/// A single interpolation point on a gradient conditional format.
/// These pin the gradient color scale according to the color,
/// type and value chosen.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InterpolationPoint {
    /// The color this interpolation point should use.
    pub color: Option<Color>,
    /// The color this interpolation point should use.
    /// If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    pub color_style: Option<ColorStyle>,
    /// How the value should be interpreted.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The value this interpolation point uses.  May be a formula.
    /// Unused if type is MIN or
    /// MAX.
    pub value: Option<String>,
}

impl Part for InterpolationPoint {}


/// A location where metadata may be associated in a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperMetadataLocation {
    /// The type of location this object represents.  This field is read-only.
    #[serde(rename="locationType")]
    pub location_type: Option<String>,
    /// Represents the row or column when metadata is associated with
    /// a dimension. The specified DimensionRange must represent a single row
    /// or column; it cannot be unbounded or span multiple rows or columns.
    #[serde(rename="dimensionRange")]
    pub dimension_range: Option<DimensionRange>,
    /// The ID of the sheet when metadata is associated with an entire sheet.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
    /// True when metadata is associated with an entire spreadsheet.
    pub spreadsheet: Option<bool>,
}

impl Part for DeveloperMetadataLocation {}


/// Merges all cells in the range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MergeCellsRequest {
    /// The range of cells to merge.
    pub range: Option<GridRange>,
    /// How the cells should be merged.
    #[serde(rename="mergeType")]
    pub merge_type: Option<String>,
}

impl Part for MergeCellsRequest {}


/// Criteria for showing/hiding rows in a pivot table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotFilterCriteria {
    /// Values that should be included.  Values not listed here are excluded.
    #[serde(rename="visibleValues")]
    pub visible_values: Option<Vec<String>>,
}

impl Part for PivotFilterCriteria {}


/// Update an embedded object's position (such as a moving or resizing a
/// chart or image).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEmbeddedObjectPositionRequest {
    /// An explicit position to move the embedded object to.
    /// If newPosition.sheetId is set,
    /// a new sheet with that ID will be created.
    /// If newPosition.newSheet is set to true,
    /// a new sheet will be created with an ID that will be chosen for you.
    #[serde(rename="newPosition")]
    pub new_position: Option<EmbeddedObjectPosition>,
    /// The fields of OverlayPosition
    /// that should be updated when setting a new position. Used only if
    /// newPosition.overlayPosition
    /// is set, in which case at least one field must
    /// be specified.  The root `newPosition.overlayPosition` is implied and
    /// should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The ID of the object to moved.
    #[serde(rename="objectId")]
    pub object_id: Option<i32>,
}

impl Part for UpdateEmbeddedObjectPositionRequest {}


/// A reply to a developer metadata search request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developer metadata search spreadsheets](struct.SpreadsheetDeveloperMetadataSearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchDeveloperMetadataResponse {
    /// The metadata matching the criteria of the search request.
    #[serde(rename="matchedDeveloperMetadata")]
    pub matched_developer_metadata: Option<Vec<MatchedDeveloperMetadata>>,
}

impl ResponseResult for SearchDeveloperMetadataResponse {}


/// The request for clearing more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear spreadsheets](struct.SpreadsheetValueBatchClearCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesRequest {
    /// The ranges to clear, in A1 notation.
    pub ranges: Option<Vec<String>>,
}

impl RequestValue for BatchClearValuesRequest {}


/// Splits a column of text into multiple columns,
/// based on a delimiter in each cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextToColumnsRequest {
    /// The source data range.  This must span exactly one column.
    pub source: Option<GridRange>,
    /// The delimiter to use. Used only if delimiterType is
    /// CUSTOM.
    pub delimiter: Option<String>,
    /// The delimiter type to use.
    #[serde(rename="delimiterType")]
    pub delimiter_type: Option<String>,
}

impl Part for TextToColumnsRequest {}


/// A single grouping (either row or column) in a pivot table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroup {
    /// True if the pivot table should include the totals for this grouping.
    #[serde(rename="showTotals")]
    pub show_totals: Option<bool>,
    /// The column offset of the source range that this grouping is based on.
    /// 
    /// For example, if the source was `C10:E15`, a `sourceColumnOffset` of `0`
    /// means this group refers to column `C`, whereas the offset `1` would refer
    /// to column `D`.
    #[serde(rename="sourceColumnOffset")]
    pub source_column_offset: Option<i32>,
    /// The labels to use for the row/column groups which can be customized. For
    /// example, in the following pivot table, the row label is `Region` (which
    /// could be renamed to `State`) and the column label is `Product` (which
    /// could be renamed `Item`). Pivot tables created before December 2017 do
    /// not have header labels. If you'd like to add header labels to an existing
    /// pivot table, please delete the existing pivot table and then create a new
    /// pivot table with same parameters.
    /// 
    /// ````text
    /// +--------------+---------+-------+
    /// | SUM of Units | Product |       |
    /// | Region       | Pen     | Paper |
    /// +--------------+---------+-------+
    /// | New York     |     345 |    98 |
    /// | Oregon       |     234 |   123 |
    /// | Tennessee    |     531 |   415 |
    /// +--------------+---------+-------+
    /// | Grand Total  |    1110 |   636 |
    /// +--------------+---------+-------+````
    pub label: Option<String>,
    /// Metadata about values in the grouping.
    #[serde(rename="valueMetadata")]
    pub value_metadata: Option<Vec<PivotGroupValueMetadata>>,
    /// The bucket of the opposite pivot group to sort by.
    /// If not specified, sorting is alphabetical by this group's values.
    #[serde(rename="valueBucket")]
    pub value_bucket: Option<PivotGroupSortValueBucket>,
    /// The order the values in this group should be sorted.
    #[serde(rename="sortOrder")]
    pub sort_order: Option<String>,
    /// True if the headings in this pivot group should be repeated.
    /// This is only valid for row groupings and is ignored by columns.
    /// 
    /// By default, we minimize repitition of headings by not showing higher
    /// level headings where they are the same. For example, even though the
    /// third row below corresponds to "Q1 Mar", "Q1" is not shown because
    /// it is redundant with previous rows. Setting repeat_headings to true
    /// would cause "Q1" to be repeated for "Feb" and "Mar".
    /// 
    /// ````text
    /// +--------------+
    /// | Q1     | Jan |
    /// |        | Feb |
    /// |        | Mar |
    /// +--------+-----+
    /// | Q1 Total     |
    /// +--------------+````
    #[serde(rename="repeatHeadings")]
    pub repeat_headings: Option<bool>,
    /// The group rule to apply to this row/column group.
    #[serde(rename="groupRule")]
    pub group_rule: Option<PivotGroupRule>,
}

impl Part for PivotGroup {}


/// The data included in a domain or series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartData {
    /// The source ranges of the data.
    #[serde(rename="sourceRange")]
    pub source_range: Option<ChartSourceRange>,
}

impl Part for ChartData {}


/// Updates a conditional format rule at the given index,
/// or moves a conditional format rule to another index.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateConditionalFormatRuleRequest {
    /// The zero-based index of the rule that should be replaced or moved.
    pub index: Option<i32>,
    /// The zero-based new index the rule should end up at.
    #[serde(rename="newIndex")]
    pub new_index: Option<i32>,
    /// The rule that should replace the rule at the given index.
    pub rule: Option<ConditionalFormatRule>,
    /// The sheet of the rule to move.  Required if new_index is set,
    /// unused otherwise.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for UpdateConditionalFormatRuleRequest {}


/// The response from creating developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateDeveloperMetadataResponse {
    /// The developer metadata that was created.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<DeveloperMetadata>,
}

impl Part for CreateDeveloperMetadataResponse {}


/// A condition that can evaluate to true or false.
/// BooleanConditions are used by conditional formatting,
/// data validation, and the criteria in filters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooleanCondition {
    /// The type of condition.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The values of the condition. The number of supported values depends
    /// on the condition type.  Some support zero values,
    /// others one or two values,
    /// and ConditionType.ONE_OF_LIST supports an arbitrary number of values.
    pub values: Option<Vec<ConditionValue>>,
}

impl Part for BooleanCondition {}


/// A range on a sheet.
/// All indexes are zero-based.
/// Indexes are half open, e.g the start index is inclusive
/// and the end index is exclusive -- [start_index, end_index).
/// Missing indexes indicate the range is unbounded on that side.
/// 
/// For example, if `"Sheet1"` is sheet ID 0, then:
/// 
/// `Sheet1!A1:A1 == sheet_id: 0, start_row_index: 0, end_row_index: 1, start_column_index: 0, end_column_index: 1`
/// 
/// `Sheet1!A3:B4 == sheet_id: 0, start_row_index: 2, end_row_index: 4, start_column_index: 0, end_column_index: 2`
/// 
/// `Sheet1!A:B == sheet_id: 0, start_column_index: 0, end_column_index: 2`
/// 
/// `Sheet1!A5:B == sheet_id: 0, start_row_index: 4, start_column_index: 0, end_column_index: 2`
/// 
/// `Sheet1 == sheet_id:0`
/// 
/// The start index must always be less than or equal to the end index.
/// If the start index equals the end index, then the range is empty.
/// Empty ranges are typically not meaningful and are usually rendered in the
/// UI as `#REF!`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridRange {
    /// The end row (exclusive) of the range, or not set if unbounded.
    #[serde(rename="endRowIndex")]
    pub end_row_index: Option<i32>,
    /// The end column (exclusive) of the range, or not set if unbounded.
    #[serde(rename="endColumnIndex")]
    pub end_column_index: Option<i32>,
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

impl Part for GridRange {}


/// A <a href="/chart/interactive/docs/gallery/histogram">histogram chart</a>.
/// A histogram chart groups data items into bins, displaying each bin as a
/// column of stacked items.  Histograms are used to display the distribution
/// of a dataset.  Each column of items represents a range into which those
/// items fall.  The number of bins can be chosen automatically or specified
/// explicitly.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramChartSpec {
    /// The outlier percentile is used to ensure that outliers do not adversely
    /// affect the calculation of bucket sizes.  For example, setting an outlier
    /// percentile of 0.05 indicates that the top and bottom 5% of values when
    /// calculating buckets.  The values are still included in the chart, they will
    /// be added to the first or last buckets instead of their own buckets.
    /// Must be between 0.0 and 0.5.
    #[serde(rename="outlierPercentile")]
    pub outlier_percentile: Option<f64>,
    /// The series for a histogram may be either a single series of values to be
    /// bucketed or multiple series, each of the same length, containing the name
    /// of the series followed by the values to be bucketed for that series.
    pub series: Option<Vec<HistogramSeries>>,
    /// Whether horizontal divider lines should be displayed between items in each
    /// column.
    #[serde(rename="showItemDividers")]
    pub show_item_dividers: Option<bool>,
    /// The position of the chart legend.
    #[serde(rename="legendPosition")]
    pub legend_position: Option<String>,
    /// By default the bucket size (the range of values stacked in a single
    /// column) is chosen automatically, but it may be overridden here.
    /// E.g., A bucket size of 1.5 results in buckets from 0 - 1.5, 1.5 - 3.0, etc.
    /// Cannot be negative.
    /// This field is optional.
    #[serde(rename="bucketSize")]
    pub bucket_size: Option<f64>,
}

impl Part for HistogramChartSpec {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update by data filter spreadsheets](struct.SpreadsheetValueBatchUpdateByDataFilterCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesByDataFilterResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The response for each range updated.
    pub responses: Option<Vec<UpdateValuesByDataFilterResponse>>,
    /// The total number of cells updated.
    #[serde(rename="totalUpdatedCells")]
    pub total_updated_cells: Option<i32>,
    /// The total number of rows where at least one cell in the row was updated.
    #[serde(rename="totalUpdatedRows")]
    pub total_updated_rows: Option<i32>,
    /// The total number of sheets where at least one cell in the sheet was
    /// updated.
    #[serde(rename="totalUpdatedSheets")]
    pub total_updated_sheets: Option<i32>,
    /// The total number of columns where at least one cell in the column was
    /// updated.
    #[serde(rename="totalUpdatedColumns")]
    pub total_updated_columns: Option<i32>,
}

impl ResponseResult for BatchUpdateValuesByDataFilterResponse {}


/// The result of adding a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddDimensionGroupResponse {
    /// All groups of a dimension after adding a group to that dimension.
    #[serde(rename="dimensionGroups")]
    pub dimension_groups: Option<Vec<DimensionGroup>>,
}

impl Part for AddDimensionGroupResponse {}


/// Metadata about a value in a pivot grouping.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroupValueMetadata {
    /// True if the data corresponding to the value is collapsed.
    pub collapsed: Option<bool>,
    /// The calculated value the metadata corresponds to.
    /// (Note that formulaValue is not valid,
    ///  because the values will be calculated.)
    pub value: Option<ExtendedValue>,
}

impl Part for PivotGroupValueMetadata {}


/// Information about which values in a pivot group should be used for sorting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroupSortValueBucket {
    /// Determines the bucket from which values are chosen to sort.
    /// 
    /// For example, in a pivot table with one row group & two column groups,
    /// the row group can list up to two values. The first value corresponds
    /// to a value within the first column group, and the second value
    /// corresponds to a value in the second column group.  If no values
    /// are listed, this would indicate that the row should be sorted according
    /// to the "Grand Total" over the column groups. If a single value is listed,
    /// this would correspond to using the "Total" of that bucket.
    pub buckets: Option<Vec<ExtendedValue>>,
    /// The offset in the PivotTable.values list which the values in this
    /// grouping should be sorted by.
    #[serde(rename="valuesIndex")]
    pub values_index: Option<i32>,
}

impl Part for PivotGroupSortValueBucket {}


/// The result of the find/replace.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindReplaceResponse {
    /// The number of occurrences (possibly multiple within a cell) changed.
    /// For example, if replacing `"e"` with `"o"` in `"Google Sheets"`, this would
    /// be `"3"` because `"Google Sheets"` -> `"Googlo Shoots"`.
    #[serde(rename="occurrencesChanged")]
    pub occurrences_changed: Option<i32>,
    /// The number of sheets changed.
    #[serde(rename="sheetsChanged")]
    pub sheets_changed: Option<i32>,
    /// The number of rows changed.
    #[serde(rename="rowsChanged")]
    pub rows_changed: Option<i32>,
    /// The number of non-formula cells changed.
    #[serde(rename="valuesChanged")]
    pub values_changed: Option<i32>,
    /// The number of formula cells changed.
    #[serde(rename="formulasChanged")]
    pub formulas_changed: Option<i32>,
}

impl Part for FindReplaceResponse {}


/// Custom number formatting options for chart attributes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartCustomNumberFormatOptions {
    /// Custom prefix to be prepended to the chart attribute.
    /// This field is optional.
    pub prefix: Option<String>,
    /// Custom suffix to be appended to the chart attribute.
    /// This field is optional.
    pub suffix: Option<String>,
}

impl Part for ChartCustomNumberFormatOptions {}


/// The Candlestick chart data, each containing the low, open, close, and high
/// values for a series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickData {
    /// The range data (vertical axis) for the close/final value for each candle.
    /// This is the top of the candle body.  If greater than the open value the
    /// candle will be filled.  Otherwise the candle will be hollow.
    #[serde(rename="closeSeries")]
    pub close_series: Option<CandlestickSeries>,
    /// The range data (vertical axis) for the high/maximum value for each
    /// candle. This is the top of the candle's center line.
    #[serde(rename="highSeries")]
    pub high_series: Option<CandlestickSeries>,
    /// The range data (vertical axis) for the low/minimum value for each candle.
    /// This is the bottom of the candle's center line.
    #[serde(rename="lowSeries")]
    pub low_series: Option<CandlestickSeries>,
    /// The range data (vertical axis) for the open/initial value for each
    /// candle. This is the bottom of the candle body.  If less than the close
    /// value the candle will be filled.  Otherwise the candle will be hollow.
    #[serde(rename="openSeries")]
    pub open_series: Option<CandlestickSeries>,
}

impl Part for CandlestickData {}


/// Clears the basic filter, if any exists on the sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearBasicFilterRequest {
    /// The sheet ID on which the basic filter should be cleared.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for ClearBasicFilterRequest {}


/// The request for updating more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update spreadsheets](struct.SpreadsheetValueBatchUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesRequest {
    /// Determines how values in the response should be rendered.
    /// The default render option is ValueRenderOption.FORMATTED_VALUE.
    #[serde(rename="responseValueRenderOption")]
    pub response_value_render_option: Option<String>,
    /// The new values to apply to the spreadsheet.
    pub data: Option<Vec<ValueRange>>,
    /// Determines if the update response should include the values
    /// of the cells that were updated. By default, responses
    /// do not include the updated values. The `updatedData` field within
    /// each of the BatchUpdateValuesResponse.responses contains the updated
    /// values. If the range to write was larger than the range actually written,
    /// the response includes all values in the requested range (excluding trailing
    /// empty rows and columns).
    #[serde(rename="includeValuesInResponse")]
    pub include_values_in_response: Option<bool>,
    /// How the input data should be interpreted.
    #[serde(rename="valueInputOption")]
    pub value_input_option: Option<String>,
    /// Determines how dates, times, and durations in the response should be
    /// rendered. This is ignored if response_value_render_option is
    /// FORMATTED_VALUE.
    /// The default dateTime render option is
    /// DateTimeRenderOption.SERIAL_NUMBER.
    #[serde(rename="responseDateTimeRenderOption")]
    pub response_date_time_render_option: Option<String>,
}

impl RequestValue for BatchUpdateValuesRequest {}


/// Updates the borders of a range.
/// If a field is not set in the request, that means the border remains as-is.
/// For example, with two subsequent UpdateBordersRequest:
/// 
///  1. range: A1:A5 `{ top: RED, bottom: WHITE }`
///  2. range: A1:A5 `{ left: BLUE }`
/// 
/// That would result in A1:A5 having a borders of
/// `{ top: RED, bottom: WHITE, left: BLUE }`.
/// If you want to clear a border, explicitly set the style to
/// NONE.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateBordersRequest {
    /// The range whose borders should be updated.
    pub range: Option<GridRange>,
    /// The border to put at the right of the range.
    pub right: Option<Border>,
    /// The border to put at the left of the range.
    pub left: Option<Border>,
    /// The border to put at the bottom of the range.
    pub bottom: Option<Border>,
    /// The border to put at the top of the range.
    pub top: Option<Border>,
    /// The vertical border to put within the range.
    #[serde(rename="innerVertical")]
    pub inner_vertical: Option<Border>,
    /// The horizontal border to put within the range.
    #[serde(rename="innerHorizontal")]
    pub inner_horizontal: Option<Border>,
}

impl Part for UpdateBordersRequest {}


/// Deletes a group over the specified range by decrementing the depth of the
/// dimensions in the range.
/// 
/// For example, assume the sheet has a depth-1 group over B:E and a depth-2
/// group over C:D. Deleting a group over D:E leaves the sheet with a
/// depth-1 group over B:D and a depth-2 group over C:C.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDimensionGroupRequest {
    /// The range of the group to be deleted.
    pub range: Option<DimensionRange>,
}

impl Part for DeleteDimensionGroupRequest {}


/// A border along a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Border {
    /// The color of the border.
    pub color: Option<Color>,
    /// The width of the border, in pixels.
    /// Deprecated; the width is determined by the "style" field.
    pub width: Option<i32>,
    /// The color of the border.
    /// If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    pub color_style: Option<ColorStyle>,
    /// The style of the border.
    pub style: Option<String>,
}

impl Part for Border {}


/// Position settings for text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextPosition {
    /// Horizontal alignment setting for the piece of text.
    #[serde(rename="horizontalAlignment")]
    pub horizontal_alignment: Option<String>,
}

impl Part for TextPosition {}


/// The value of the condition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConditionValue {
    /// A relative date (based on the current date).
    /// Valid only if the type is
    /// DATE_BEFORE,
    /// DATE_AFTER,
    /// DATE_ON_OR_BEFORE or
    /// DATE_ON_OR_AFTER.
    /// 
    /// Relative dates are not supported in data validation.
    /// They are supported only in conditional formatting and
    /// conditional filters.
    #[serde(rename="relativeDate")]
    pub relative_date: Option<String>,
    /// A value the condition is based on.
    /// The value is parsed as if the user typed into a cell.
    /// Formulas are supported (and must begin with an `=` or a '+').
    #[serde(rename="userEnteredValue")]
    pub user_entered_value: Option<String>,
}

impl Part for ConditionValue {}


/// A value range that was matched by one or more data filers.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatchedValueRange {
    /// The values matched by the DataFilter.
    #[serde(rename="valueRange")]
    pub value_range: Option<ValueRange>,
    /// The DataFilters from the request that matched the range of
    /// values.
    #[serde(rename="dataFilters")]
    pub data_filters: Option<Vec<DataFilter>>,
}

impl Part for MatchedValueRange {}


/// Inserts rows or columns in a sheet at a particular index.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertDimensionRequest {
    /// Whether dimension properties should be extended from the dimensions
    /// before or after the newly inserted dimensions.
    /// True to inherit from the dimensions before (in which case the start
    /// index must be greater than 0), and false to inherit from the dimensions
    /// after.
    /// 
    /// For example, if row index 0 has red background and row index 1
    /// has a green background, then inserting 2 rows at index 1 can inherit
    /// either the green or red background.  If `inheritFromBefore` is true,
    /// the two new rows will be red (because the row before the insertion point
    /// was red), whereas if `inheritFromBefore` is false, the two new rows will
    /// be green (because the row after the insertion point was green).
    #[serde(rename="inheritFromBefore")]
    pub inherit_from_before: Option<bool>,
    /// The dimensions to insert.  Both the start and end indexes must be bounded.
    pub range: Option<DimensionRange>,
}

impl Part for InsertDimensionRequest {}


/// A request to delete developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDeveloperMetadataRequest {
    /// The data filter describing the criteria used to select which developer
    /// metadata entry to delete.
    #[serde(rename="dataFilter")]
    pub data_filter: Option<DataFilter>,
}

impl Part for DeleteDeveloperMetadataRequest {}


/// The reply for batch updating a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update spreadsheets](struct.SpreadsheetBatchUpdateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateSpreadsheetResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The spreadsheet after updates were applied. This is only set if
    /// [BatchUpdateSpreadsheetRequest.include_spreadsheet_in_response] is `true`.
    #[serde(rename="updatedSpreadsheet")]
    pub updated_spreadsheet: Option<Spreadsheet>,
    /// The reply of the updates.  This maps 1:1 with the updates, although
    /// replies to some requests may be empty.
    pub replies: Option<Vec<Response>>,
}

impl ResponseResult for BatchUpdateSpreadsheetResponse {}


/// The response when clearing a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear spreadsheets](struct.SpreadsheetValueBatchClearCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The ranges that were cleared, in A1 notation. If the requests are for an
    /// unbounded range or a ranger larger than the bounds of the sheet, this is
    /// the actual ranges that were cleared, bounded to the sheet's limits.
    #[serde(rename="clearedRanges")]
    pub cleared_ranges: Option<Vec<String>>,
}

impl ResponseResult for BatchClearValuesResponse {}


/// Properties about a dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionProperties {
    /// The height (if a row) or width (if a column) of the dimension in pixels.
    #[serde(rename="pixelSize")]
    pub pixel_size: Option<i32>,
    /// The developer metadata associated with a single row or column.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    /// True if this dimension is explicitly hidden.
    #[serde(rename="hiddenByUser")]
    pub hidden_by_user: Option<bool>,
    /// True if this dimension is being filtered.
    /// This field is read-only.
    #[serde(rename="hiddenByFilter")]
    pub hidden_by_filter: Option<bool>,
}

impl Part for DimensionProperties {}


/// The result of adding a chart to a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddChartResponse {
    /// The newly added chart.
    pub chart: Option<EmbeddedChart>,
}

impl Part for AddChartResponse {}


/// The rotation applied to text in a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextRotation {
    /// The angle between the standard orientation and the desired orientation.
    /// Measured in degrees. Valid values are between -90 and 90. Positive
    /// angles are angled upwards, negative are angled downwards.
    /// 
    /// Note: For LTR text direction positive angles are in the
    /// counterclockwise direction, whereas for RTL they are in the clockwise
    /// direction
    pub angle: Option<i32>,
    /// If true, text reads top to bottom, but the orientation of individual
    /// characters is unchanged.
    /// For example:
    /// 
    /// ````text
    /// | V |
    /// | e |
    /// | r |
    /// | t |
    /// | i |
    /// | c |
    /// | a |
    /// | l |````
    pub vertical: Option<bool>,
}

impl Part for TextRotation {}


/// Updates properties of the supplied banded range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateBandingRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `bandedRange` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The banded range to update with the new properties.
    #[serde(rename="bandedRange")]
    pub banded_range: Option<BandedRange>,
}

impl Part for UpdateBandingRequest {}


/// The request for retrieving a range of values in a spreadsheet selected by a
/// set of DataFilters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch get by data filter spreadsheets](struct.SpreadsheetValueBatchGetByDataFilterCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetValuesByDataFilterRequest {
    /// How dates, times, and durations should be represented in the output.
    /// This is ignored if value_render_option is
    /// FORMATTED_VALUE.
    /// The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER].
    #[serde(rename="dateTimeRenderOption")]
    pub date_time_render_option: Option<String>,
    /// How values should be represented in the output.
    /// The default render option is ValueRenderOption.FORMATTED_VALUE.
    #[serde(rename="valueRenderOption")]
    pub value_render_option: Option<String>,
    /// The data filters used to match the ranges of values to retrieve. Ranges
    /// that match any of the specified data filters are included in the response.
    #[serde(rename="dataFilters")]
    pub data_filters: Option<Vec<DataFilter>>,
    /// The major dimension that results should use.
    /// 
    /// For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,
    /// then a request that selects that range and sets `majorDimension=ROWS`
    /// returns `[[1,2],[3,4]]`, whereas a request that sets
    /// `majorDimension=COLUMNS` returns `[[1,3],[2,4]]`.
    #[serde(rename="majorDimension")]
    pub major_dimension: Option<String>,
}

impl RequestValue for BatchGetValuesByDataFilterRequest {}


/// The response when updating a range of values by a data filter in a
/// spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateValuesByDataFilterResponse {
    /// The number of columns where at least one cell in the column was updated.
    #[serde(rename="updatedColumns")]
    pub updated_columns: Option<i32>,
    /// The range (in A1 notation) that updates were applied to.
    #[serde(rename="updatedRange")]
    pub updated_range: Option<String>,
    /// The data filter that selected the range that was updated.
    #[serde(rename="dataFilter")]
    pub data_filter: Option<DataFilter>,
    /// The number of rows where at least one cell in the row was updated.
    #[serde(rename="updatedRows")]
    pub updated_rows: Option<i32>,
    /// The values of the cells in the range matched by the dataFilter after all
    /// updates were applied. This is only included if the request's
    /// `includeValuesInResponse` field was `true`.
    #[serde(rename="updatedData")]
    pub updated_data: Option<ValueRange>,
    /// The number of cells updated.
    #[serde(rename="updatedCells")]
    pub updated_cells: Option<i32>,
}

impl Part for UpdateValuesByDataFilterResponse {}


/// The borders of the cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Borders {
    /// The top border of the cell.
    pub top: Option<Border>,
    /// The right border of the cell.
    pub right: Option<Border>,
    /// The bottom border of the cell.
    pub bottom: Option<Border>,
    /// The left border of the cell.
    pub left: Option<Border>,
}

impl Part for Borders {}


/// A request to update properties of developer metadata.
/// Updates the properties of the developer metadata selected by the filters to
/// the values provided in the DeveloperMetadata resource.  Callers must
/// specify the properties they wish to update in the fields parameter, as well
/// as specify at least one DataFilter matching the metadata they wish to
/// update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDeveloperMetadataRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `developerMetadata` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The filters matching the developer metadata entries to update.
    #[serde(rename="dataFilters")]
    pub data_filters: Option<Vec<DataFilter>>,
    /// The value that all metadata matched by the data filters will be updated to.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<DeveloperMetadata>,
}

impl Part for UpdateDeveloperMetadataRequest {}


/// Adds a new banded range to the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddBandingRequest {
    /// The banded range to add. The bandedRangeId
    /// field is optional; if one is not set, an id will be randomly generated. (It
    /// is an error to specify the ID of a range that already exists.)
    #[serde(rename="bandedRange")]
    pub banded_range: Option<BandedRange>,
}

impl Part for AddBandingRequest {}


/// A custom subtotal column for a waterfall chart series.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartCustomSubtotal {
    /// True if the data point at subtotal_index is the subtotal. If false,
    /// the subtotal will be computed and appear after the data point.
    #[serde(rename="dataIsSubtotal")]
    pub data_is_subtotal: Option<bool>,
    /// The 0-based index of a data point within the series. If
    /// data_is_subtotal is true, the data point at this index is the
    /// subtotal. Otherwise, the subtotal appears after the data point with
    /// this index. A series can have multiple subtotals at arbitrary indices,
    /// but subtotals do not affect the indices of the data points. For
    /// example, if a series has three data points, their indices will always
    /// be 0, 1, and 2, regardless of how many subtotals exist on the series or
    /// what data points they are associated with.
    #[serde(rename="subtotalIndex")]
    pub subtotal_index: Option<i32>,
    /// A label for the subtotal column.
    pub label: Option<String>,
}

impl Part for WaterfallChartCustomSubtotal {}


/// Properties of a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SpreadsheetProperties {
    /// The amount of time to wait before volatile functions are recalculated.
    #[serde(rename="autoRecalc")]
    pub auto_recalc: Option<String>,
    /// Theme applied to the spreadsheet.
    #[serde(rename="spreadsheetTheme")]
    pub spreadsheet_theme: Option<SpreadsheetTheme>,
    /// The default format of all cells in the spreadsheet.
    /// CellData.effectiveFormat will not be set if
    /// the cell's format is equal to this default format. This field is read-only.
    #[serde(rename="defaultFormat")]
    pub default_format: Option<CellFormat>,
    /// The title of the spreadsheet.
    pub title: Option<String>,
    /// The locale of the spreadsheet in one of the following formats:
    /// 
    /// * an ISO 639-1 language code such as `en`
    /// 
    /// * an ISO 639-2 language code such as `fil`, if no 639-1 code exists
    /// 
    /// * a combination of the ISO language code and country code, such as `en_US`
    /// 
    /// Note: when updating this field, not all locales/languages are supported.
    pub locale: Option<String>,
    /// The time zone of the spreadsheet, in CLDR format such as
    /// `America/New_York`. If the time zone isn't recognized, this may
    /// be a custom time zone such as `GMT-07:00`.
    #[serde(rename="timeZone")]
    pub time_zone: Option<String>,
    /// Determines whether and how circular references are resolved with iterative
    /// calculation.  Absence of this field means that circular references result
    /// in calculation errors.
    #[serde(rename="iterativeCalculationSettings")]
    pub iterative_calculation_settings: Option<IterativeCalculationSettings>,
}

impl Part for SpreadsheetProperties {}


/// Deletes the requested sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteSheetRequest {
    /// The ID of the sheet to delete.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for DeleteSheetRequest {}


/// Data about a specific cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CellData {
    /// A pivot table anchored at this cell. The size of pivot table itself
    /// is computed dynamically based on its data, grouping, filters, values,
    /// etc. Only the top-left cell of the pivot table contains the pivot table
    /// definition. The other cells will contain the calculated values of the
    /// results of the pivot in their effective_value fields.
    #[serde(rename="pivotTable")]
    pub pivot_table: Option<PivotTable>,
    /// A hyperlink this cell points to, if any.
    /// This field is read-only.  (To set it, use a `=HYPERLINK` formula
    /// in the userEnteredValue.formulaValue
    /// field.)
    pub hyperlink: Option<String>,
    /// The effective value of the cell. For cells with formulas, this is
    /// the calculated value.  For cells with literals, this is
    /// the same as the user_entered_value.
    /// This field is read-only.
    #[serde(rename="effectiveValue")]
    pub effective_value: Option<ExtendedValue>,
    /// The formatted value of the cell.
    /// This is the value as it's shown to the user.
    /// This field is read-only.
    #[serde(rename="formattedValue")]
    pub formatted_value: Option<String>,
    /// The value the user entered in the cell. e.g, `1234`, `'Hello'`, or `=NOW()`
    /// Note: Dates, Times and DateTimes are represented as doubles in
    /// serial number format.
    #[serde(rename="userEnteredValue")]
    pub user_entered_value: Option<ExtendedValue>,
    /// Any note on the cell.
    pub note: Option<String>,
    /// The effective format being used by the cell.
    /// This includes the results of applying any conditional formatting and,
    /// if the cell contains a formula, the computed number format.
    /// If the effective format is the default format, effective format will
    /// not be written.
    /// This field is read-only.
    #[serde(rename="effectiveFormat")]
    pub effective_format: Option<CellFormat>,
    /// The format the user entered for the cell.
    /// 
    /// When writing, the new format will be merged with the existing format.
    #[serde(rename="userEnteredFormat")]
    pub user_entered_format: Option<CellFormat>,
    /// A data validation rule on the cell, if any.
    /// 
    /// When writing, the new data validation rule will overwrite any prior rule.
    #[serde(rename="dataValidation")]
    pub data_validation: Option<DataValidationRule>,
    /// Runs of rich text applied to subsections of the cell.  Runs are only valid
    /// on user entered strings, not formulas, bools, or numbers.
    /// Runs start at specific indexes in the text and continue until the next
    /// run. Properties of a run will continue unless explicitly changed
    /// in a subsequent run (and properties of the first run will continue
    /// the properties of the cell unless explicitly changed).
    /// 
    /// When writing, the new runs will overwrite any prior runs.  When writing a
    /// new user_entered_value, previous runs are erased.
    #[serde(rename="textFormatRuns")]
    pub text_format_runs: Option<Vec<TextFormatRun>>,
}

impl Part for CellData {}


/// Properties of a sheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sheets copy to spreadsheets](struct.SpreadsheetSheetCopyToCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SheetProperties {
    /// The type of sheet. Defaults to GRID.
    /// This field cannot be changed once set.
    #[serde(rename="sheetType")]
    pub sheet_type: Option<String>,
    /// The index of the sheet within the spreadsheet.
    /// When adding or updating sheet properties, if this field
    /// is excluded then the sheet is added or moved to the end
    /// of the sheet list. When updating sheet indices or inserting
    /// sheets, movement is considered in "before the move" indexes.
    /// For example, if there were 3 sheets (S1, S2, S3) in order to
    /// move S1 ahead of S2 the index would have to be set to 2. A sheet
    /// index update request is ignored if the requested index is
    /// identical to the sheets current index or if the requested new
    /// index is equal to the current sheet index + 1.
    pub index: Option<i32>,
    /// The name of the sheet.
    pub title: Option<String>,
    /// Additional properties of the sheet if this sheet is a grid.
    /// (If the sheet is an object sheet, containing a chart or image, then
    /// this field will be absent.)
    /// When writing it is an error to set any grid properties on non-grid sheets.
    #[serde(rename="gridProperties")]
    pub grid_properties: Option<GridProperties>,
    /// True if the sheet is an RTL sheet instead of an LTR sheet.
    #[serde(rename="rightToLeft")]
    pub right_to_left: Option<bool>,
    /// The color of the tab in the UI.
    #[serde(rename="tabColor")]
    pub tab_color: Option<Color>,
    /// True if the sheet is hidden in the UI, false if it's visible.
    pub hidden: Option<bool>,
    /// The color of the tab in the UI.
    /// If tab_color is also set, this field takes precedence.
    #[serde(rename="tabColorStyle")]
    pub tab_color_style: Option<ColorStyle>,
    /// The ID of the sheet. Must be non-negative.
    /// This field cannot be changed once set.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl ResponseResult for SheetProperties {}


/// The response when retrieving more than one range of values in a spreadsheet
/// selected by DataFilters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch get by data filter spreadsheets](struct.SpreadsheetValueBatchGetByDataFilterCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetValuesByDataFilterResponse {
    /// The ID of the spreadsheet the data was retrieved from.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The requested values with the list of data filters that matched them.
    #[serde(rename="valueRanges")]
    pub value_ranges: Option<Vec<MatchedValueRange>>,
}

impl ResponseResult for BatchGetValuesByDataFilterResponse {}


/// The request for retrieving a Spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get by data filter spreadsheets](struct.SpreadsheetGetByDataFilterCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetSpreadsheetByDataFilterRequest {
    /// The DataFilters used to select which ranges to retrieve from
    /// the spreadsheet.
    #[serde(rename="dataFilters")]
    pub data_filters: Option<Vec<DataFilter>>,
    /// True if grid data should be returned.
    /// This parameter is ignored if a field mask was set in the request.
    #[serde(rename="includeGridData")]
    pub include_grid_data: Option<bool>,
}

impl RequestValue for GetSpreadsheetByDataFilterRequest {}


/// The amount of padding around the cell, in pixels.
/// When updating padding, every field must be specified.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Padding {
    /// The top padding of the cell.
    pub top: Option<i32>,
    /// The right padding of the cell.
    pub right: Option<i32>,
    /// The bottom padding of the cell.
    pub bottom: Option<i32>,
    /// The left padding of the cell.
    pub left: Option<i32>,
}

impl Part for Padding {}


/// A data validation rule.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataValidationRule {
    /// True if the UI should be customized based on the kind of condition.
    /// If true, "List" conditions will show a dropdown.
    #[serde(rename="showCustomUi")]
    pub show_custom_ui: Option<bool>,
    /// True if invalid data should be rejected.
    pub strict: Option<bool>,
    /// A message to show the user when adding data to the cell.
    #[serde(rename="inputMessage")]
    pub input_message: Option<String>,
    /// The condition that data in the cell must match.
    pub condition: Option<BooleanCondition>,
}

impl Part for DataValidationRule {}


/// The request for clearing a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values clear spreadsheets](struct.SpreadsheetValueClearCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearValuesRequest { _never_set: Option<bool> }

impl RequestValue for ClearValuesRequest {}


/// The kinds of value that a cell in a spreadsheet can have.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedValue {
    /// Represents a double value.
    /// Note: Dates, Times and DateTimes are represented as doubles in
    /// "serial number" format.
    #[serde(rename="numberValue")]
    pub number_value: Option<f64>,
    /// Represents a boolean value.
    #[serde(rename="boolValue")]
    pub bool_value: Option<bool>,
    /// Represents an error.
    /// This field is read-only.
    #[serde(rename="errorValue")]
    pub error_value: Option<ErrorValue>,
    /// Represents a formula.
    #[serde(rename="formulaValue")]
    pub formula_value: Option<String>,
    /// Represents a string value.
    /// Leading single quotes are not included. For example, if the user typed
    /// `'123` into the UI, this would be represented as a `stringValue` of
    /// `"123"`.
    #[serde(rename="stringValue")]
    pub string_value: Option<String>,
}

impl Part for ExtendedValue {}


/// A rule that may or may not match, depending on the condition.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BooleanRule {
    /// The condition of the rule. If the condition evaluates to true,
    /// the format is applied.
    pub condition: Option<BooleanCondition>,
    /// The format to apply.
    /// Conditional formatting can only apply a subset of formatting:
    /// bold, italic,
    /// strikethrough,
    /// foreground color &
    /// background color.
    pub format: Option<CellFormat>,
}

impl Part for BooleanRule {}


/// Adds a named range to the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddNamedRangeRequest {
    /// The named range to add. The namedRangeId
    /// field is optional; if one is not set, an id will be randomly generated. (It
    /// is an error to specify the ID of a range that already exists.)
    #[serde(rename="namedRange")]
    pub named_range: Option<NamedRange>,
}

impl Part for AddNamedRangeRequest {}


/// Deletes a conditional format rule at the given index.
/// All subsequent rules' indexes are decremented.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteConditionalFormatRuleRequest {
    /// The zero-based index of the rule to be deleted.
    pub index: Option<i32>,
    /// The sheet the rule is being deleted from.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for DeleteConditionalFormatRuleRequest {}


/// Unmerges cells in the given range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnmergeCellsRequest {
    /// The range within which all cells should be unmerged.
    /// If the range spans multiple merges, all will be unmerged.
    /// The range must not partially span any merge.
    pub range: Option<GridRange>,
}

impl Part for UnmergeCellsRequest {}


/// Formatting options for key value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyValueFormat {
    /// Specifies the horizontal text positioning of key value.
    /// This field is optional. If not specified, default positioning is used.
    pub position: Option<TextPosition>,
    /// Text formatting options for key value.
    #[serde(rename="textFormat")]
    pub text_format: Option<TextFormat>,
}

impl Part for KeyValueFormat {}


/// A rule that applies a gradient color scale format, based on
/// the interpolation points listed. The format of a cell will vary
/// based on its contents as compared to the values of the interpolation
/// points.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GradientRule {
    /// The final interpolation point.
    pub maxpoint: Option<InterpolationPoint>,
    /// An optional midway interpolation point.
    pub midpoint: Option<InterpolationPoint>,
    /// The starting interpolation point.
    pub minpoint: Option<InterpolationPoint>,
}

impl Part for GradientRule {}


/// A <a href="/chart/interactive/docs/gallery/treemap">Treemap chart</a>.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TreemapChartSpec {
    /// The data the contains the treemap cells' parent labels.
    #[serde(rename="parentLabels")]
    pub parent_labels: Option<ChartData>,
    /// The background color for header cells.
    #[serde(rename="headerColor")]
    pub header_color: Option<Color>,
    /// True to hide tooltips.
    #[serde(rename="hideTooltips")]
    pub hide_tooltips: Option<bool>,
    /// The color scale for data cells in the treemap chart. Data cells are
    /// assigned colors based on their color values. These color values come from
    /// color_data, or from size_data if color_data is not specified.
    /// Cells with color values less than or equal to min_value will
    /// have minValueColor as their
    /// background color. Cells with color values greater than or equal to
    /// max_value will have
    /// maxValueColor as their background
    /// color. Cells with color values between min_value and max_value will
    /// have background colors on a gradient between
    /// minValueColor and
    /// maxValueColor, the midpoint of
    /// the gradient being midValueColor.
    /// Cells with missing or non-numeric color values will have
    /// noDataColor as their background
    /// color.
    #[serde(rename="colorScale")]
    pub color_scale: Option<TreemapChartColorScale>,
    /// The data that contains the treemap cell labels.
    pub labels: Option<ChartData>,
    /// The data that determines the background color of each treemap data cell.
    /// This field is optional. If not specified, size_data is used to
    /// determine background colors. If specified, the data is expected to be
    /// numeric. color_scale will determine how the values in this data map to
    /// data cell background colors.
    #[serde(rename="colorData")]
    pub color_data: Option<ChartData>,
    /// The maximum possible data value. Cells with values greater than this will
    /// have the same color as cells with this value. If not specified, defaults
    /// to the actual maximum value from color_data, or the maximum value from
    /// size_data if color_data is not specified.
    #[serde(rename="maxValue")]
    pub max_value: Option<f64>,
    /// The number of data levels to show on the treemap chart. These levels are
    /// interactive and are shown with their labels. Defaults to 2 if not
    /// specified.
    pub levels: Option<i32>,
    /// The background color for header cells.
    /// If header_color is also set, this field takes precedence.
    #[serde(rename="headerColorStyle")]
    pub header_color_style: Option<ColorStyle>,
    /// The data that determines the size of each treemap data cell. This data is
    /// expected to be numeric. The cells corresponding to non-numeric or missing
    /// data will not be rendered. If color_data is not specified, this data
    /// is used to determine data cell background colors as well.
    #[serde(rename="sizeData")]
    pub size_data: Option<ChartData>,
    /// The number of additional data levels beyond the labeled levels to be shown
    /// on the treemap chart. These levels are not interactive and are shown
    /// without their labels. Defaults to 0 if not specified.
    #[serde(rename="hintedLevels")]
    pub hinted_levels: Option<i32>,
    /// The text format for all labels on the chart.
    #[serde(rename="textFormat")]
    pub text_format: Option<TextFormat>,
    /// The minimum possible data value. Cells with values less than this will
    /// have the same color as cells with this value. If not specified, defaults
    /// to the actual minimum value from color_data, or the minimum value from
    /// size_data if color_data is not specified.
    #[serde(rename="minValue")]
    pub min_value: Option<f64>,
}

impl Part for TreemapChartSpec {}


/// The response when retrieving more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch get spreadsheets](struct.SpreadsheetValueBatchGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchGetValuesResponse {
    /// The ID of the spreadsheet the data was retrieved from.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The requested values. The order of the ValueRanges is the same as the
    /// order of the requested ranges.
    #[serde(rename="valueRanges")]
    pub value_ranges: Option<Vec<ValueRange>>,
}

impl ResponseResult for BatchGetValuesResponse {}


/// The result of adding a new protected range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddProtectedRangeResponse {
    /// The newly added protected range.
    #[serde(rename="protectedRange")]
    pub protected_range: Option<ProtectedRange>,
}

impl Part for AddProtectedRangeResponse {}


/// Allows you to organize the date-time values in a source data column into
/// buckets based on selected parts of their date or time values. For example,
/// consider a pivot table showing sales transactions by date:
/// 
/// ````text
/// +----------+--------------+
/// | Date     | SUM of Sales |
/// +----------+--------------+
/// | 1/1/2017 |      $621.14 |
/// | 2/3/2017 |      $708.84 |
/// | 5/8/2017 |      $326.84 |
/// ...
/// +----------+--------------+
/// ````
/// 
/// Applying a date-time group rule with a DateTimeRuleType of YEAR_MONTH
/// results in the following pivot table.
/// 
/// ````text
/// +--------------+--------------+
/// | Grouped Date | SUM of Sales |
/// +--------------+--------------+
/// | 2017-Jan     |   $53,731.78 |
/// | 2017-Feb     |   $83,475.32 |
/// | 2017-Mar     |   $94,385.05 |
/// ...
/// +--------------+--------------+
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateTimeRule {
    /// The type of date-time grouping to apply.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for DateTimeRule {}


/// The response when updating a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values update spreadsheets](struct.SpreadsheetValueUpdateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateValuesResponse {
    /// The number of columns where at least one cell in the column was updated.
    #[serde(rename="updatedColumns")]
    pub updated_columns: Option<i32>,
    /// The range (in A1 notation) that updates were applied to.
    #[serde(rename="updatedRange")]
    pub updated_range: Option<String>,
    /// The number of rows where at least one cell in the row was updated.
    #[serde(rename="updatedRows")]
    pub updated_rows: Option<i32>,
    /// The values of the cells after updates were applied.
    /// This is only included if the request's `includeValuesInResponse` field
    /// was `true`.
    #[serde(rename="updatedData")]
    pub updated_data: Option<ValueRange>,
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The number of cells updated.
    #[serde(rename="updatedCells")]
    pub updated_cells: Option<i32>,
}

impl ResponseResult for UpdateValuesResponse {}


/// The series of a CandlestickData.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickSeries {
    /// The data of the CandlestickSeries.
    pub data: Option<ChartData>,
}

impl Part for CandlestickSeries {}


/// A named range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NamedRange {
    /// The ID of the named range.
    #[serde(rename="namedRangeId")]
    pub named_range_id: Option<String>,
    /// The range this represents.
    pub range: Option<GridRange>,
    /// The name of the named range.
    pub name: Option<String>,
}

impl Part for NamedRange {}


/// A combination of a source range and how to extend that source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceAndDestination {
    /// The number of rows or columns that data should be filled into.
    /// Positive numbers expand beyond the last row or last column
    /// of the source.  Negative numbers expand before the first row
    /// or first column of the source.
    #[serde(rename="fillLength")]
    pub fill_length: Option<i32>,
    /// The dimension that data should be filled into.
    pub dimension: Option<String>,
    /// The location of the data to use as the source of the autofill.
    pub source: Option<GridRange>,
}

impl Part for SourceAndDestination {}


/// The response when clearing a range of values selected with
/// DataFilters in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch clear by data filter spreadsheets](struct.SpreadsheetValueBatchClearByDataFilterCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchClearValuesByDataFilterResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The ranges that were cleared, in A1 notation. If the requests are for an
    /// unbounded range or a ranger larger than the bounds of the sheet, this is
    /// the actual ranges that were cleared, bounded to the sheet's limits.
    #[serde(rename="clearedRanges")]
    pub cleared_ranges: Option<Vec<String>>,
}

impl ResponseResult for BatchClearValuesByDataFilterResponse {}


/// A banded (alternating colors) range in a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BandedRange {
    /// The range over which these properties are applied.
    pub range: Option<GridRange>,
    /// Properties for column bands. These properties are applied on a column-
    /// by-column basis throughout all the columns in the range. At least one of
    /// row_properties or column_properties must be specified.
    #[serde(rename="columnProperties")]
    pub column_properties: Option<BandingProperties>,
    /// Properties for row bands. These properties are applied on a row-by-row
    /// basis throughout all the rows in the range. At least one of
    /// row_properties or column_properties must be specified.
    #[serde(rename="rowProperties")]
    pub row_properties: Option<BandingProperties>,
    /// The id of the banded range.
    #[serde(rename="bandedRangeId")]
    pub banded_range_id: Option<i32>,
}

impl Part for BandedRange {}


/// The response from updating developer metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDeveloperMetadataResponse {
    /// The updated developer metadata.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
}

impl Part for UpdateDeveloperMetadataResponse {}


/// Properties of a grid.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridProperties {
    /// The number of columns in the grid.
    #[serde(rename="columnCount")]
    pub column_count: Option<i32>,
    /// The number of rows in the grid.
    #[serde(rename="rowCount")]
    pub row_count: Option<i32>,
    /// True if the row grouping control toggle is shown after the group.
    #[serde(rename="rowGroupControlAfter")]
    pub row_group_control_after: Option<bool>,
    /// True if the column grouping control toggle is shown after the group.
    #[serde(rename="columnGroupControlAfter")]
    pub column_group_control_after: Option<bool>,
    /// True if the grid isn't showing gridlines in the UI.
    #[serde(rename="hideGridlines")]
    pub hide_gridlines: Option<bool>,
    /// The number of rows that are frozen in the grid.
    #[serde(rename="frozenRowCount")]
    pub frozen_row_count: Option<i32>,
    /// The number of columns that are frozen in the grid.
    #[serde(rename="frozenColumnCount")]
    pub frozen_column_count: Option<i32>,
}

impl Part for GridProperties {}


/// A single response from an update.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// A reply from duplicating a filter view.
    #[serde(rename="duplicateFilterView")]
    pub duplicate_filter_view: Option<DuplicateFilterViewResponse>,
    /// A reply from updating an embedded object's position.
    #[serde(rename="updateEmbeddedObjectPosition")]
    pub update_embedded_object_position: Option<UpdateEmbeddedObjectPositionResponse>,
    /// A reply from deleting a dimension group.
    #[serde(rename="deleteDimensionGroup")]
    pub delete_dimension_group: Option<DeleteDimensionGroupResponse>,
    /// A reply from updating a conditional format rule.
    #[serde(rename="updateConditionalFormatRule")]
    pub update_conditional_format_rule: Option<UpdateConditionalFormatRuleResponse>,
    /// A reply from adding a named range.
    #[serde(rename="addNamedRange")]
    pub add_named_range: Option<AddNamedRangeResponse>,
    /// A reply from adding a protected range.
    #[serde(rename="addProtectedRange")]
    pub add_protected_range: Option<AddProtectedRangeResponse>,
    /// A reply from updating a developer metadata entry.
    #[serde(rename="updateDeveloperMetadata")]
    pub update_developer_metadata: Option<UpdateDeveloperMetadataResponse>,
    /// A reply from adding a dimension group.
    #[serde(rename="addDimensionGroup")]
    pub add_dimension_group: Option<AddDimensionGroupResponse>,
    /// A reply from adding a banded range.
    #[serde(rename="addBanding")]
    pub add_banding: Option<AddBandingResponse>,
    /// A reply from adding a filter view.
    #[serde(rename="addFilterView")]
    pub add_filter_view: Option<AddFilterViewResponse>,
    /// A reply from adding a slicer.
    #[serde(rename="addSlicer")]
    pub add_slicer: Option<AddSlicerResponse>,
    /// A reply from duplicating a sheet.
    #[serde(rename="duplicateSheet")]
    pub duplicate_sheet: Option<DuplicateSheetResponse>,
    /// A reply from deleting a developer metadata entry.
    #[serde(rename="deleteDeveloperMetadata")]
    pub delete_developer_metadata: Option<DeleteDeveloperMetadataResponse>,
    /// A reply from adding a chart.
    #[serde(rename="addChart")]
    pub add_chart: Option<AddChartResponse>,
    /// A reply from creating a developer metadata entry.
    #[serde(rename="createDeveloperMetadata")]
    pub create_developer_metadata: Option<CreateDeveloperMetadataResponse>,
    /// A reply from adding a sheet.
    #[serde(rename="addSheet")]
    pub add_sheet: Option<AddSheetResponse>,
    /// A reply from doing a find/replace.
    #[serde(rename="findReplace")]
    pub find_replace: Option<FindReplaceResponse>,
    /// A reply from trimming whitespace.
    #[serde(rename="trimWhitespace")]
    pub trim_whitespace: Option<TrimWhitespaceResponse>,
    /// A reply from deleting a conditional format rule.
    #[serde(rename="deleteConditionalFormatRule")]
    pub delete_conditional_format_rule: Option<DeleteConditionalFormatRuleResponse>,
    /// A reply from removing rows containing duplicate values.
    #[serde(rename="deleteDuplicates")]
    pub delete_duplicates: Option<DeleteDuplicatesResponse>,
}

impl Part for Response {}


/// The result of trimming whitespace in cells.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrimWhitespaceResponse {
    /// The number of cells that were trimmed of whitespace.
    #[serde(rename="cellsChangedCount")]
    pub cells_changed_count: Option<i32>,
}

impl Part for TrimWhitespaceResponse {}


/// A single series of data in a chart.
/// For example, if charting stock prices over time, multiple series may exist,
/// one for the "Open Price", "High Price", "Low Price" and "Close Price".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartSeries {
    /// The line style of this series. Valid only if the
    /// chartType is AREA,
    /// LINE, or SCATTER.
    /// COMBO charts are also supported if the
    /// series chart type is
    /// AREA or LINE.
    #[serde(rename="lineStyle")]
    pub line_style: Option<LineStyle>,
    /// The color for elements (such as bars, lines, and points) associated with
    /// this series.  If empty, a default color is used.
    /// If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    pub color_style: Option<ColorStyle>,
    /// The color for elements (such as bars, lines, and points) associated with
    /// this series.  If empty, a default color is used.
    pub color: Option<Color>,
    /// The data being visualized in this chart series.
    pub series: Option<ChartData>,
    /// The type of this series. Valid only if the
    /// chartType is
    /// COMBO.
    /// Different types will change the way the series is visualized.
    /// Only LINE, AREA,
    /// and COLUMN are supported.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// The minor axis that will specify the range of values for this series.
    /// For example, if charting stocks over time, the "Volume" series
    /// may want to be pinned to the right with the prices pinned to the left,
    /// because the scale of trading volume is different than the scale of
    /// prices.
    /// It is an error to specify an axis that isn't a valid minor axis
    /// for the chart's type.
    #[serde(rename="targetAxis")]
    pub target_axis: Option<String>,
}

impl Part for BasicChartSeries {}


/// The request for updating any aspect of a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch update spreadsheets](struct.SpreadsheetBatchUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateSpreadsheetRequest {
    /// Limits the ranges included in the response spreadsheet.
    /// Meaningful only if include_spreadsheet_in_response is 'true'.
    #[serde(rename="responseRanges")]
    pub response_ranges: Option<Vec<String>>,
    /// Determines if the update response should include the spreadsheet
    /// resource.
    #[serde(rename="includeSpreadsheetInResponse")]
    pub include_spreadsheet_in_response: Option<bool>,
    /// True if grid data should be returned. Meaningful only if
    /// include_spreadsheet_in_response is 'true'.
    /// This parameter is ignored if a field mask was set in the request.
    #[serde(rename="responseIncludeGridData")]
    pub response_include_grid_data: Option<bool>,
    /// A list of updates to apply to the spreadsheet.
    /// Requests will be applied in the order they are specified.
    /// If any request is not valid, no requests will be applied.
    pub requests: Option<Vec<Request>>,
}

impl RequestValue for BatchUpdateSpreadsheetRequest {}


/// The options that define a "view window" for a chart (such as the visible
/// values in an axis).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartAxisViewWindowOptions {
    /// The minimum numeric value to be shown in this view window. If unset, will
    /// automatically determine a minimum value that looks good for the data.
    #[serde(rename="viewWindowMin")]
    pub view_window_min: Option<f64>,
    /// The maximum numeric value to be shown in this view window. If unset, will
    /// automatically determine a maximum value that looks good for the data.
    #[serde(rename="viewWindowMax")]
    pub view_window_max: Option<f64>,
    /// The view window's mode.
    #[serde(rename="viewWindowMode")]
    pub view_window_mode: Option<String>,
}

impl Part for ChartAxisViewWindowOptions {}


/// Updates the state of the specified group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDimensionGroupRequest {
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `dimensionGroup` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The group whose state should be updated. The range and depth of the group
    /// should specify a valid group on the sheet, and all other fields updated.
    #[serde(rename="dimensionGroup")]
    pub dimension_group: Option<DimensionGroup>,
}

impl Part for UpdateDimensionGroupRequest {}


/// The request for updating more than one range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values batch update by data filter spreadsheets](struct.SpreadsheetValueBatchUpdateByDataFilterCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchUpdateValuesByDataFilterRequest {
    /// Determines how values in the response should be rendered.
    /// The default render option is ValueRenderOption.FORMATTED_VALUE.
    #[serde(rename="responseValueRenderOption")]
    pub response_value_render_option: Option<String>,
    /// The new values to apply to the spreadsheet.  If more than one range is
    /// matched by the specified DataFilter the specified values are applied to
    /// all of those ranges.
    pub data: Option<Vec<DataFilterValueRange>>,
    /// Determines if the update response should include the values
    /// of the cells that were updated. By default, responses
    /// do not include the updated values. The `updatedData` field within
    /// each of the BatchUpdateValuesResponse.responses contains the updated
    /// values. If the range to write was larger than the range actually written,
    /// the response includes all values in the requested range (excluding trailing
    /// empty rows and columns).
    #[serde(rename="includeValuesInResponse")]
    pub include_values_in_response: Option<bool>,
    /// How the input data should be interpreted.
    #[serde(rename="valueInputOption")]
    pub value_input_option: Option<String>,
    /// Determines how dates, times, and durations in the response should be
    /// rendered. This is ignored if response_value_render_option is
    /// FORMATTED_VALUE.
    /// The default dateTime render option is
    /// DateTimeRenderOption.SERIAL_NUMBER.
    #[serde(rename="responseDateTimeRenderOption")]
    pub response_date_time_render_option: Option<String>,
}

impl RequestValue for BatchUpdateValuesByDataFilterRequest {}


/// Automatically resizes one or more dimensions based on the contents
/// of the cells in that dimension.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoResizeDimensionsRequest {
    /// The dimensions to automatically resize.
    pub dimensions: Option<DimensionRange>,
}

impl Part for AutoResizeDimensionsRequest {}


/// Data in the grid, as well as metadata about the dimensions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridData {
    /// Metadata about the requested rows in the grid, starting with the row
    /// in start_row.
    #[serde(rename="rowMetadata")]
    pub row_metadata: Option<Vec<DimensionProperties>>,
    /// The first column this GridData refers to, zero-based.
    #[serde(rename="startColumn")]
    pub start_column: Option<i32>,
    /// Metadata about the requested columns in the grid, starting with the column
    /// in start_column.
    #[serde(rename="columnMetadata")]
    pub column_metadata: Option<Vec<DimensionProperties>>,
    /// The first row this GridData refers to, zero-based.
    #[serde(rename="startRow")]
    pub start_row: Option<i32>,
    /// The data in the grid, one entry per row,
    /// starting with the row in startRow.
    /// The values in RowData will correspond to columns starting
    /// at start_column.
    #[serde(rename="rowData")]
    pub row_data: Option<Vec<RowData>>,
}

impl Part for GridData {}


/// The result of a filter view being duplicated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateFilterViewResponse {
    /// The newly created filter.
    pub filter: Option<FilterView>,
}

impl Part for DuplicateFilterViewResponse {}


/// Filter that describes what data should be selected or returned from a
/// request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataFilter {
    /// Selects data associated with the developer metadata matching the criteria
    /// described by this DeveloperMetadataLookup.
    #[serde(rename="developerMetadataLookup")]
    pub developer_metadata_lookup: Option<DeveloperMetadataLookup>,
    /// Selects data that matches the specified A1 range.
    #[serde(rename="a1Range")]
    pub a1_range: Option<String>,
    /// Selects data that matches the range described by the GridRange.
    #[serde(rename="gridRange")]
    pub grid_range: Option<GridRange>,
}

impl Part for DataFilter {}


/// Removes the named range with the given ID from the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteNamedRangeRequest {
    /// The ID of the named range to delete.
    #[serde(rename="namedRangeId")]
    pub named_range_id: Option<String>,
}

impl Part for DeleteNamedRangeRequest {}


/// Removes rows within this range that contain values in the specified columns
/// that are duplicates of values in any previous row. Rows with identical values
/// but different letter cases, formatting, or formulas are considered to be
/// duplicates.
/// 
/// This request also removes duplicate rows hidden from view (for example, due
/// to a filter). When removing duplicates, the first instance of each duplicate
/// row scanning from the top downwards is kept in the resulting range. Content
/// outside of the specified range isn't removed, and rows considered duplicates
/// do not have to be adjacent to each other in the range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDuplicatesRequest {
    /// The columns in the range to analyze for duplicate values. If no columns are
    /// selected then all columns are analyzed for duplicates.
    #[serde(rename="comparisonColumns")]
    pub comparison_columns: Option<Vec<DimensionRange>>,
    /// The range to remove duplicates rows from.
    pub range: Option<GridRange>,
}

impl Part for DeleteDuplicatesRequest {}


/// Developer metadata associated with a location or object in a spreadsheet.
/// Developer metadata may be used to associate arbitrary data with various
/// parts of a spreadsheet and will remain associated at those locations as they
/// move around and the spreadsheet is edited.  For example, if developer
/// metadata is associated with row 5 and another row is then subsequently
/// inserted above row 5, that original metadata will still be associated with
/// the row it was first associated with (what is now row 6). If the associated
/// object is deleted its metadata is deleted too.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developer metadata get spreadsheets](struct.SpreadsheetDeveloperMetadataGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeveloperMetadata {
    /// The spreadsheet-scoped unique ID that identifies the metadata. IDs may be
    /// specified when metadata is created, otherwise one will be randomly
    /// generated and assigned. Must be positive.
    #[serde(rename="metadataId")]
    pub metadata_id: Option<i32>,
    /// Data associated with the metadata's key.
    #[serde(rename="metadataValue")]
    pub metadata_value: Option<String>,
    /// The location where the metadata is associated.
    pub location: Option<DeveloperMetadataLocation>,
    /// The metadata visibility.  Developer metadata must always have a visibility
    /// specified.
    pub visibility: Option<String>,
    /// The metadata key. There may be multiple metadata in a spreadsheet with the
    /// same key.  Developer metadata must always have a key specified.
    #[serde(rename="metadataKey")]
    pub metadata_key: Option<String>,
}

impl ResponseResult for DeveloperMetadata {}


/// A developer metadata entry and the data filters specified in the original
/// request that matched it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MatchedDeveloperMetadata {
    /// All filters matching the returned developer metadata.
    #[serde(rename="dataFilters")]
    pub data_filters: Option<Vec<DataFilter>>,
    /// The developer metadata matching the specified filters.
    #[serde(rename="developerMetadata")]
    pub developer_metadata: Option<DeveloperMetadata>,
}

impl Part for MatchedDeveloperMetadata {}


/// Updates all cells in a range with new data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCellsRequest {
    /// The fields of CellData that should be updated.
    /// At least one field must be specified.
    /// The root is the CellData; 'row.values.' should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The range to write data to.
    /// 
    /// If the data in rows does not cover the entire requested range,
    /// the fields matching those set in fields will be cleared.
    pub range: Option<GridRange>,
    /// The data to write.
    pub rows: Option<Vec<RowData>>,
    /// The coordinate to start writing data at.
    /// Any number of rows and columns (including a different number of
    /// columns per row) may be written.
    pub start: Option<GridCoordinate>,
}

impl Part for UpdateCellsRequest {}


/// Source ranges for a chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ChartSourceRange {
    /// The ranges of data for a series or domain.
    /// Exactly one dimension must have a length of 1,
    /// and all sources in the list must have the same dimension
    /// with length 1.
    /// The domain (if it exists) & all series must have the same number
    /// of source ranges. If using more than one source range, then the source
    /// range at a given offset must be in order and contiguous across the domain
    /// and series.
    /// 
    /// For example, these are valid configurations:
    /// 
    /// ````text
    /// domain sources: A1:A5
    /// series1 sources: B1:B5
    /// series2 sources: D6:D10
    /// 
    /// domain sources: A1:A5, C10:C12
    /// series1 sources: B1:B5, D10:D12
    /// series2 sources: C1:C5, E10:E12````
    pub sources: Option<Vec<GridRange>>,
}

impl Part for ChartSourceRange {}


/// Deletes the dimensions from the sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDimensionRequest {
    /// The dimensions to delete from the sheet.
    pub range: Option<DimensionRange>,
}

impl Part for DeleteDimensionRequest {}


/// Properties that describe the style of a line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineStyle {
    /// The thickness of the line, in px.
    pub width: Option<i32>,
    /// The dash type of the line.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for LineStyle {}


/// Trims the whitespace (such as spaces, tabs, or new lines) in every cell in
/// the specified range. This request removes all whitespace from the start and
/// end of each cell's text, and reduces any subsequence of remaining whitespace
/// characters to a single space. If the resulting trimmed text starts with a '+'
/// or '=' character, the text remains as a string value and isn't interpreted
/// as a formula.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrimWhitespaceRequest {
    /// The range whose cells to trim.
    pub range: Option<GridRange>,
}

impl Part for TrimWhitespaceRequest {}


/// Properties referring a single dimension (either row or column). If both
/// BandedRange.row_properties and BandedRange.column_properties are
/// set, the fill colors are applied to cells according to the following rules:
/// 
/// * header_color and footer_color take priority over band colors.
/// * first_band_color takes priority over second_band_color.
/// * row_properties takes priority over column_properties.
/// 
/// For example, the first row color takes priority over the first column
/// color, but the first column color takes priority over the second row color.
/// Similarly, the row header takes priority over the column header in the
/// top left cell, but the column header takes priority over the first row
/// color if the row header is not set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BandingProperties {
    /// The second color that is alternating. (Required)
    #[serde(rename="secondBandColor")]
    pub second_band_color: Option<Color>,
    /// The color of the first row or column. If this field is set, the first row
    /// or column is filled with this color and the colors alternate between
    /// first_band_color and second_band_color starting from the second
    /// row or column. Otherwise, the first row or column is filled with
    /// first_band_color and the colors proceed to alternate as they normally
    /// would.
    #[serde(rename="headerColor")]
    pub header_color: Option<Color>,
    /// The color of the last row or column. If this field is not set, the last
    /// row or column is filled with either first_band_color or
    /// second_band_color, depending on the color of the previous row or
    /// column.
    /// If footer_color is also set, this field takes precedence.
    #[serde(rename="footerColorStyle")]
    pub footer_color_style: Option<ColorStyle>,
    /// The color of the first row or column. If this field is set, the first row
    /// or column is filled with this color and the colors alternate between
    /// first_band_color and second_band_color starting from the second
    /// row or column. Otherwise, the first row or column is filled with
    /// first_band_color and the colors proceed to alternate as they normally
    /// would. If header_color is also set, this field takes precedence.
    #[serde(rename="headerColorStyle")]
    pub header_color_style: Option<ColorStyle>,
    /// The second color that is alternating. (Required)
    /// If second_band_color is also set, this field takes precedence.
    #[serde(rename="secondBandColorStyle")]
    pub second_band_color_style: Option<ColorStyle>,
    /// The first color that is alternating. (Required)
    #[serde(rename="firstBandColor")]
    pub first_band_color: Option<Color>,
    /// The first color that is alternating. (Required)
    /// If first_band_color is also set, this field takes precedence.
    #[serde(rename="firstBandColorStyle")]
    pub first_band_color_style: Option<ColorStyle>,
    /// The color of the last row or column. If this field is not set, the last
    /// row or column is filled with either first_band_color or
    /// second_band_color, depending on the color of the previous row or
    /// column.
    #[serde(rename="footerColor")]
    pub footer_color: Option<Color>,
}

impl Part for BandingProperties {}


/// A histogram series containing the series color and data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistogramSeries {
    /// The color of the column representing this series in each bucket.
    /// This field is optional.
    #[serde(rename="barColor")]
    pub bar_color: Option<Color>,
    /// The color of the column representing this series in each bucket.
    /// This field is optional.
    /// If bar_color is also set, this field takes precedence.
    #[serde(rename="barColorStyle")]
    pub bar_color_style: Option<ColorStyle>,
    /// The data for this histogram series.
    pub data: Option<ChartData>,
}

impl Part for HistogramSeries {}


/// The domain of a chart.
/// For example, if charting stock prices over time, this would be the date.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartDomain {
    /// The data of the domain. For example, if charting stock prices over time,
    /// this is the data representing the dates.
    pub domain: Option<ChartData>,
    /// True to reverse the order of the domain values (horizontal axis).
    pub reversed: Option<bool>,
}

impl Part for BasicChartDomain {}


/// The result of deleting a group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteDimensionGroupResponse {
    /// All groups of a dimension after deleting a group from that dimension.
    #[serde(rename="dimensionGroups")]
    pub dimension_groups: Option<Vec<DimensionGroup>>,
}

impl Part for DeleteDimensionGroupResponse {}


/// Styles for a waterfall chart column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartColumnStyle {
    /// The color of the column.
    pub color: Option<Color>,
    /// The color of the column.
    /// If color is also set, this field takes precedence.
    #[serde(rename="colorStyle")]
    pub color_style: Option<ColorStyle>,
    /// The label of the column's legend.
    pub label: Option<String>,
}

impl Part for WaterfallChartColumnStyle {}


/// An optional setting on a PivotGroup that defines buckets for the values
/// in the source data column rather than breaking out each individual value.
/// Only one PivotGroup with a group rule may be added for each column in
/// the source data, though on any given column you may add both a
/// PivotGroup that has a rule and a PivotGroup that does not.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotGroupRule {
    /// A HistogramRule.
    #[serde(rename="histogramRule")]
    pub histogram_rule: Option<HistogramRule>,
    /// A ManualRule.
    #[serde(rename="manualRule")]
    pub manual_rule: Option<ManualRule>,
    /// A DateTimeRule.
    #[serde(rename="dateTimeRule")]
    pub date_time_rule: Option<DateTimeRule>,
}

impl Part for PivotGroupRule {}


/// Sorts data in rows based on a sort order per column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SortRangeRequest {
    /// The range to sort.
    pub range: Option<GridRange>,
    /// The sort order per column. Later specifications are used when values
    /// are equal in the earlier specifications.
    #[serde(rename="sortSpecs")]
    pub sort_specs: Option<Vec<SortSpec>>,
}

impl Part for SortRangeRequest {}


/// The specifications of a slicer.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SlicerSpec {
    /// True if the filter should apply to pivot tables.
    /// If not set, default to `True`.
    #[serde(rename="applyToPivotTables")]
    pub apply_to_pivot_tables: Option<bool>,
    /// The data range of the slicer.
    #[serde(rename="dataRange")]
    pub data_range: Option<GridRange>,
    /// The title of the slicer.
    pub title: Option<String>,
    /// The background color of the slicer.
    /// If background_color is also set, this field takes precedence.
    #[serde(rename="backgroundColorStyle")]
    pub background_color_style: Option<ColorStyle>,
    /// The column index in the data table on which the filter is applied to.
    #[serde(rename="columnIndex")]
    pub column_index: Option<i32>,
    /// The horizontal alignment of title in the slicer.
    /// If unspecified, defaults to `LEFT`
    #[serde(rename="horizontalAlignment")]
    pub horizontal_alignment: Option<String>,
    /// The background color of the slicer.
    #[serde(rename="backgroundColor")]
    pub background_color: Option<Color>,
    /// The text format of title in the slicer.
    #[serde(rename="textFormat")]
    pub text_format: Option<TextFormat>,
    /// The filtering criteria of the slicer.
    #[serde(rename="filterCriteria")]
    pub filter_criteria: Option<FilterCriteria>,
}

impl Part for SlicerSpec {}


/// The definition of how a value in a pivot table should be calculated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotValue {
    /// A custom formula to calculate the value.  The formula must start
    /// with an `=` character.
    pub formula: Option<String>,
    /// A function to summarize the value.
    /// If formula is set, the only supported values are
    /// SUM and
    /// CUSTOM.
    /// If sourceColumnOffset is set, then `CUSTOM`
    /// is not supported.
    #[serde(rename="summarizeFunction")]
    pub summarize_function: Option<String>,
    /// A name to use for the value.
    pub name: Option<String>,
    /// The column offset of the source range that this value reads from.
    /// 
    /// For example, if the source was `C10:E15`, a `sourceColumnOffset` of `0`
    /// means this value refers to column `C`, whereas the offset `1` would
    /// refer to column `D`.
    #[serde(rename="sourceColumnOffset")]
    pub source_column_offset: Option<i32>,
    /// If specified, indicates that pivot values should be displayed as
    /// the result of a calculation with another pivot value. For example, if
    /// calculated_display_type is specified as PERCENT_OF_GRAND_TOTAL, all the
    /// pivot values are displayed as the percentage of the grand total. In
    /// the Sheets UI, this is referred to as "Show As" in the value section of a
    /// pivot table.
    #[serde(rename="calculatedDisplayType")]
    pub calculated_display_type: Option<String>,
}

impl Part for PivotValue {}


/// Adds a new sheet.
/// When a sheet is added at a given index,
/// all subsequent sheets' indexes are incremented.
/// To add an object sheet, use AddChartRequest instead and specify
/// EmbeddedObjectPosition.sheetId or
/// EmbeddedObjectPosition.newSheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSheetRequest {
    /// The properties the new sheet should have.
    /// All properties are optional.
    /// The sheetId field is optional; if one is not
    /// set, an id will be randomly generated. (It is an error to specify the ID
    /// of a sheet that already exists.)
    pub properties: Option<SheetProperties>,
}

impl Part for AddSheetRequest {}


/// Appends rows or columns to the end of a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppendDimensionRequest {
    /// The number of rows or columns to append.
    pub length: Option<i32>,
    /// Whether rows or columns should be appended.
    pub dimension: Option<String>,
    /// The sheet to append rows or columns to.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for AppendDimensionRequest {}


/// Randomizes the order of the rows in a range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RandomizeRangeRequest {
    /// The range to randomize.
    pub range: Option<GridRange>,
}

impl Part for RandomizeRangeRequest {}


/// Creates a group over the specified range.
/// 
/// If the requested range is a superset of the range of an existing group G,
/// then the depth of G is incremented and this new group G' has the
/// depth of that group. For example, a group [C:D, depth 1] + [B:E] results in
/// groups [B:E, depth 1] and [C:D, depth 2].
/// If the requested range is a subset of the range of an existing group G,
/// then the depth of the new group G' becomes one greater than the depth of G.
/// For example, a group [B:E, depth 1] + [C:D] results in groups [B:E, depth 1]
/// and [C:D, depth 2].
/// If the requested range starts before and ends within, or starts within and
/// ends after, the range of an existing group G, then the range of the existing
/// group G becomes the union of the ranges, and the new group G' has
/// depth one greater than the depth of G and range as the intersection of the
/// ranges. For example, a group [B:D, depth 1] + [C:E] results in groups [B:E,
/// depth 1] and [C:D, depth 2].
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddDimensionGroupRequest {
    /// The range over which to create a group.
    pub range: Option<DimensionRange>,
}

impl Part for AddDimensionGroupRequest {}


/// A range of values whose location is specified by a DataFilter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataFilterValueRange {
    /// The data filter describing the location of the values in the spreadsheet.
    #[serde(rename="dataFilter")]
    pub data_filter: Option<DataFilter>,
    /// The data to be written.  If the provided values exceed any of the ranges
    /// matched by the data filter then the request fails.  If the provided values
    /// are less than the matched ranges only the specified values are written,
    /// existing values in the matched ranges remain unaffected.
    pub values: Option<Vec<Vec<String>>>,
    /// The major dimension of the values.
    #[serde(rename="majorDimension")]
    pub major_dimension: Option<String>,
}

impl Part for DataFilterValueRange {}


/// Updates a chart's specifications.
/// (This does not move or resize a chart. To move or resize a chart, use
///  UpdateEmbeddedObjectPositionRequest.)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChartSpecRequest {
    /// The ID of the chart to update.
    #[serde(rename="chartId")]
    pub chart_id: Option<i32>,
    /// The specification to apply to the chart.
    pub spec: Option<ChartSpec>,
}

impl Part for UpdateChartSpecRequest {}


/// An error in a cell.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorValue {
    /// A message with more information about the error
    /// (in the spreadsheet's locale).
    pub message: Option<String>,
    /// The type of error.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for ErrorValue {}


/// Deletes the embedded object with the given ID.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteEmbeddedObjectRequest {
    /// The ID of the embedded object to delete.
    #[serde(rename="objectId")]
    pub object_id: Option<i32>,
}

impl Part for DeleteEmbeddedObjectRequest {}


/// The request to copy a sheet across spreadsheets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sheets copy to spreadsheets](struct.SpreadsheetSheetCopyToCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CopySheetToAnotherSpreadsheetRequest {
    /// The ID of the spreadsheet to copy the sheet to.
    #[serde(rename="destinationSpreadsheetId")]
    pub destination_spreadsheet_id: Option<String>,
}

impl RequestValue for CopySheetToAnotherSpreadsheetRequest {}


/// A pivot table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PivotTable {
    /// The range the pivot table is reading data from.
    pub source: Option<GridRange>,
    /// Whether values should be listed horizontally (as columns)
    /// or vertically (as rows).
    #[serde(rename="valueLayout")]
    pub value_layout: Option<String>,
    /// Each row grouping in the pivot table.
    pub rows: Option<Vec<PivotGroup>>,
    /// A list of values to include in the pivot table.
    pub values: Option<Vec<PivotValue>>,
    /// An optional mapping of filters per source column offset.
    /// 
    /// The filters are applied before aggregating data into the pivot table.
    /// The map's key is the column offset of the source range that you want to
    /// filter, and the value is the criteria for that column.
    /// 
    /// For example, if the source was `C10:E15`, a key of `0` will have the filter
    /// for column `C`, whereas the key `1` is for column `D`.
    pub criteria: Option<HashMap<String, PivotFilterCriteria>>,
    /// Each column grouping in the pivot table.
    pub columns: Option<Vec<PivotGroup>>,
}

impl Part for PivotTable {}


/// The response when clearing a range of values in a spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values clear spreadsheets](struct.SpreadsheetValueClearCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClearValuesResponse {
    /// The spreadsheet the updates were applied to.
    #[serde(rename="spreadsheetId")]
    pub spreadsheet_id: Option<String>,
    /// The range (in A1 notation) that was cleared.
    /// (If the request was for an unbounded range or a ranger larger
    ///  than the bounds of the sheet, this will be the actual range
    ///  that was cleared, bounded to the sheet's limits.)
    #[serde(rename="clearedRange")]
    pub cleared_range: Option<String>,
}

impl ResponseResult for ClearValuesResponse {}


/// Data about each cell in a row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RowData {
    /// The values in the row, one per column.
    pub values: Option<Vec<CellData>>,
}

impl Part for RowData {}


/// A coordinate in a sheet.
/// All indexes are zero-based.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GridCoordinate {
    /// The row index of the coordinate.
    #[serde(rename="rowIndex")]
    pub row_index: Option<i32>,
    /// The column index of the coordinate.
    #[serde(rename="columnIndex")]
    pub column_index: Option<i32>,
    /// The sheet this coordinate is on.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for GridCoordinate {}


/// Duplicates the contents of a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DuplicateSheetRequest {
    /// The name of the new sheet.  If empty, a new name is chosen for you.
    #[serde(rename="newSheetName")]
    pub new_sheet_name: Option<String>,
    /// The zero-based index where the new sheet should be inserted.
    /// The index of all sheets after this are incremented.
    #[serde(rename="insertSheetIndex")]
    pub insert_sheet_index: Option<i32>,
    /// The sheet to duplicate.
    #[serde(rename="sourceSheetId")]
    pub source_sheet_id: Option<i32>,
    /// If set, the ID of the new sheet. If not set, an ID is chosen.
    /// If set, the ID must not conflict with any existing sheet ID.
    /// If set, it must be non-negative.
    #[serde(rename="newSheetId")]
    pub new_sheet_id: Option<i32>,
}

impl Part for DuplicateSheetRequest {}


/// A single kind of update to apply to a spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// Duplicates a filter view.
    #[serde(rename="duplicateFilterView")]
    pub duplicate_filter_view: Option<DuplicateFilterViewRequest>,
    /// Sorts data in a range.
    #[serde(rename="sortRange")]
    pub sort_range: Option<SortRangeRequest>,
    /// Updates an embedded object's (e.g. chart, image) position.
    #[serde(rename="updateEmbeddedObjectPosition")]
    pub update_embedded_object_position: Option<UpdateEmbeddedObjectPositionRequest>,
    /// Updates an existing conditional format rule.
    #[serde(rename="updateConditionalFormatRule")]
    pub update_conditional_format_rule: Option<UpdateConditionalFormatRuleRequest>,
    /// Updates the state of the specified group.
    #[serde(rename="updateDimensionGroup")]
    pub update_dimension_group: Option<UpdateDimensionGroupRequest>,
    /// Deletes rows or columns in a sheet.
    #[serde(rename="deleteDimension")]
    pub delete_dimension: Option<DeleteDimensionRequest>,
    /// Adds a protected range.
    #[serde(rename="addProtectedRange")]
    pub add_protected_range: Option<AddProtectedRangeRequest>,
    /// Deletes an embedded object (e.g, chart, image) in a sheet.
    #[serde(rename="deleteEmbeddedObject")]
    pub delete_embedded_object: Option<DeleteEmbeddedObjectRequest>,
    /// Pastes data (HTML or delimited) into a sheet.
    #[serde(rename="pasteData")]
    pub paste_data: Option<PasteDataRequest>,
    /// Appends cells after the last row with data in a sheet.
    #[serde(rename="appendCells")]
    pub append_cells: Option<AppendCellsRequest>,
    /// Adds a slicer.
    #[serde(rename="addSlicer")]
    pub add_slicer: Option<AddSlicerRequest>,
    /// Duplicates a sheet.
    #[serde(rename="duplicateSheet")]
    pub duplicate_sheet: Option<DuplicateSheetRequest>,
    /// Updates a sheet's properties.
    #[serde(rename="updateSheetProperties")]
    pub update_sheet_properties: Option<UpdateSheetPropertiesRequest>,
    /// Adds a chart.
    #[serde(rename="addChart")]
    pub add_chart: Option<AddChartRequest>,
    /// Adds a new conditional format rule.
    #[serde(rename="addConditionalFormatRule")]
    pub add_conditional_format_rule: Option<AddConditionalFormatRuleRequest>,
    /// Removes a banded range
    #[serde(rename="deleteBanding")]
    pub delete_banding: Option<DeleteBandingRequest>,
    /// Repeats a single cell across a range.
    #[serde(rename="repeatCell")]
    pub repeat_cell: Option<RepeatCellRequest>,
    /// Finds and replaces occurrences of some text with other text.
    #[serde(rename="findReplace")]
    pub find_replace: Option<FindReplaceRequest>,
    /// Trims cells of whitespace (such as spaces, tabs, or new lines).
    #[serde(rename="trimWhitespace")]
    pub trim_whitespace: Option<TrimWhitespaceRequest>,
    /// Sets the basic filter on a sheet.
    #[serde(rename="setBasicFilter")]
    pub set_basic_filter: Option<SetBasicFilterRequest>,
    /// Updates the spreadsheet's properties.
    #[serde(rename="updateSpreadsheetProperties")]
    pub update_spreadsheet_properties: Option<UpdateSpreadsheetPropertiesRequest>,
    /// Updates many cells at once.
    #[serde(rename="updateCells")]
    pub update_cells: Option<UpdateCellsRequest>,
    /// Randomizes the order of the rows in a range.
    #[serde(rename="randomizeRange")]
    pub randomize_range: Option<RandomizeRangeRequest>,
    /// Appends dimensions to the end of a sheet.
    #[serde(rename="appendDimension")]
    pub append_dimension: Option<AppendDimensionRequest>,
    /// Updates a banded range
    #[serde(rename="updateBanding")]
    pub update_banding: Option<UpdateBandingRequest>,
    /// Automatically resizes one or more dimensions based on the contents
    /// of the cells in that dimension.
    #[serde(rename="autoResizeDimensions")]
    pub auto_resize_dimensions: Option<AutoResizeDimensionsRequest>,
    /// Removes rows containing duplicate values in specified columns of a cell
    /// range.
    #[serde(rename="deleteDuplicates")]
    pub delete_duplicates: Option<DeleteDuplicatesRequest>,
    /// Updates dimensions' properties.
    #[serde(rename="updateDimensionProperties")]
    pub update_dimension_properties: Option<UpdateDimensionPropertiesRequest>,
    /// Adds a new banded range
    #[serde(rename="addBanding")]
    pub add_banding: Option<AddBandingRequest>,
    /// Unmerges merged cells.
    #[serde(rename="unmergeCells")]
    pub unmerge_cells: Option<UnmergeCellsRequest>,
    /// Sets data validation for one or more cells.
    #[serde(rename="setDataValidation")]
    pub set_data_validation: Option<SetDataValidationRequest>,
    /// Deletes a range of cells from a sheet, shifting the remaining cells.
    #[serde(rename="deleteRange")]
    pub delete_range: Option<DeleteRangeRequest>,
    /// Clears the basic filter on a sheet.
    #[serde(rename="clearBasicFilter")]
    pub clear_basic_filter: Option<ClearBasicFilterRequest>,
    /// Converts a column of text into many columns of text.
    #[serde(rename="textToColumns")]
    pub text_to_columns: Option<TextToColumnsRequest>,
    /// Automatically fills in more data based on existing data.
    #[serde(rename="autoFill")]
    pub auto_fill: Option<AutoFillRequest>,
    /// Inserts new cells in a sheet, shifting the existing cells.
    #[serde(rename="insertRange")]
    pub insert_range: Option<InsertRangeRequest>,
    /// Updates an existing developer metadata entry
    #[serde(rename="updateDeveloperMetadata")]
    pub update_developer_metadata: Option<UpdateDeveloperMetadataRequest>,
    /// Moves rows or columns to another location in a sheet.
    #[serde(rename="moveDimension")]
    pub move_dimension: Option<MoveDimensionRequest>,
    /// Updates a protected range.
    #[serde(rename="updateProtectedRange")]
    pub update_protected_range: Option<UpdateProtectedRangeRequest>,
    /// Creates new developer metadata
    #[serde(rename="createDeveloperMetadata")]
    pub create_developer_metadata: Option<CreateDeveloperMetadataRequest>,
    /// Deletes a group over the specified range.
    #[serde(rename="deleteDimensionGroup")]
    pub delete_dimension_group: Option<DeleteDimensionGroupRequest>,
    /// Merges cells together.
    #[serde(rename="mergeCells")]
    pub merge_cells: Option<MergeCellsRequest>,
    /// Updates a slicer's specifications.
    #[serde(rename="updateSlicerSpec")]
    pub update_slicer_spec: Option<UpdateSlicerSpecRequest>,
    /// Updates a chart's specifications.
    #[serde(rename="updateChartSpec")]
    pub update_chart_spec: Option<UpdateChartSpecRequest>,
    /// Deletes a protected range.
    #[serde(rename="deleteProtectedRange")]
    pub delete_protected_range: Option<DeleteProtectedRangeRequest>,
    /// Adds a filter view.
    #[serde(rename="addFilterView")]
    pub add_filter_view: Option<AddFilterViewRequest>,
    /// Deletes developer metadata
    #[serde(rename="deleteDeveloperMetadata")]
    pub delete_developer_metadata: Option<DeleteDeveloperMetadataRequest>,
    /// Deletes a sheet.
    #[serde(rename="deleteSheet")]
    pub delete_sheet: Option<DeleteSheetRequest>,
    /// Updates the borders in a range of cells.
    #[serde(rename="updateBorders")]
    pub update_borders: Option<UpdateBordersRequest>,
    /// Cuts data from one area and pastes it to another.
    #[serde(rename="cutPaste")]
    pub cut_paste: Option<CutPasteRequest>,
    /// Copies data from one area and pastes it to another.
    #[serde(rename="copyPaste")]
    pub copy_paste: Option<CopyPasteRequest>,
    /// Adds a sheet.
    #[serde(rename="addSheet")]
    pub add_sheet: Option<AddSheetRequest>,
    /// Deletes a named range.
    #[serde(rename="deleteNamedRange")]
    pub delete_named_range: Option<DeleteNamedRangeRequest>,
    /// Deletes a filter view from a sheet.
    #[serde(rename="deleteFilterView")]
    pub delete_filter_view: Option<DeleteFilterViewRequest>,
    /// Updates a named range.
    #[serde(rename="updateNamedRange")]
    pub update_named_range: Option<UpdateNamedRangeRequest>,
    /// Inserts new rows or columns in a sheet.
    #[serde(rename="insertDimension")]
    pub insert_dimension: Option<InsertDimensionRequest>,
    /// Updates the properties of a filter view.
    #[serde(rename="updateFilterView")]
    pub update_filter_view: Option<UpdateFilterViewRequest>,
    /// Deletes an existing conditional format rule.
    #[serde(rename="deleteConditionalFormatRule")]
    pub delete_conditional_format_rule: Option<DeleteConditionalFormatRuleRequest>,
    /// Adds a named range.
    #[serde(rename="addNamedRange")]
    pub add_named_range: Option<AddNamedRangeRequest>,
    /// Creates a group over the specified range.
    #[serde(rename="addDimensionGroup")]
    pub add_dimension_group: Option<AddDimensionGroupRequest>,
}

impl Part for Request {}


/// Settings to control how circular dependencies are resolved with iterative
/// calculation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IterativeCalculationSettings {
    /// When iterative calculation is enabled and successive results differ by
    /// less than this threshold value, the calculation rounds stop.
    #[serde(rename="convergenceThreshold")]
    pub convergence_threshold: Option<f64>,
    /// When iterative calculation is enabled, the maximum number of calculation
    /// rounds to perform.
    #[serde(rename="maxIterations")]
    pub max_iterations: Option<i32>,
}

impl Part for IterativeCalculationSettings {}


/// A single series of data for a waterfall chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WaterfallChartSeries {
    /// Styles for all columns in this series with negative values.
    #[serde(rename="negativeColumnsStyle")]
    pub negative_columns_style: Option<WaterfallChartColumnStyle>,
    /// True to hide the subtotal column from the end of the series. By default,
    /// a subtotal column will appear at the end of each series. Setting this
    /// field to true will hide that subtotal column for this series.
    #[serde(rename="hideTrailingSubtotal")]
    pub hide_trailing_subtotal: Option<bool>,
    /// Styles for all columns in this series with positive values.
    #[serde(rename="positiveColumnsStyle")]
    pub positive_columns_style: Option<WaterfallChartColumnStyle>,
    /// The data being visualized in this series.
    pub data: Option<ChartData>,
    /// Custom subtotal columns appearing in this series. The order in which
    /// subtotals are defined is not significant. Only one subtotal may be
    /// defined for each data point.
    #[serde(rename="customSubtotals")]
    pub custom_subtotals: Option<Vec<WaterfallChartCustomSubtotal>>,
    /// Styles for all subtotal columns in this series.
    #[serde(rename="subtotalColumnsStyle")]
    pub subtotal_columns_style: Option<WaterfallChartColumnStyle>,
}

impl Part for WaterfallChartSeries {}


/// A request to retrieve all developer metadata matching the set of specified
/// criteria.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [developer metadata search spreadsheets](struct.SpreadsheetDeveloperMetadataSearchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchDeveloperMetadataRequest {
    /// The data filters describing the criteria used to determine which
    /// DeveloperMetadata entries to return.  DeveloperMetadata matching any of the
    /// specified filters are included in the response.
    #[serde(rename="dataFilters")]
    pub data_filters: Option<Vec<DataFilter>>,
}

impl RequestValue for SearchDeveloperMetadataRequest {}


/// Updates all cells in the range to the values in the given Cell object.
/// Only the fields listed in the fields field are updated; others are
/// unchanged.
/// 
/// If writing a cell with a formula, the formula's ranges will automatically
/// increment for each field in the range.
/// For example, if writing a cell with formula `=A1` into range B2:C4,
/// B2 would be `=A1`, B3 would be `=A2`, B4 would be `=A3`,
/// C2 would be `=B1`, C3 would be `=B2`, C4 would be `=B3`.
/// 
/// To keep the formula's ranges static, use the `$` indicator.
/// For example, use the formula `=$A$1` to prevent both the row and the
/// column from incrementing.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepeatCellRequest {
    /// The data to write.
    pub cell: Option<CellData>,
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `cell` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
    /// The range to repeat the cell in.
    pub range: Option<GridRange>,
}

impl Part for RepeatCellRequest {}


/// Deletes a particular filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteFilterViewRequest {
    /// The ID of the filter to delete.
    #[serde(rename="filterId")]
    pub filter_id: Option<i32>,
}

impl Part for DeleteFilterViewRequest {}


/// A range along a single dimension on a sheet.
/// All indexes are zero-based.
/// Indexes are half open: the start index is inclusive
/// and the end index is exclusive.
/// Missing indexes indicate the range is unbounded on that side.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionRange {
    /// The end (exclusive) of the span, or not set if unbounded.
    #[serde(rename="endIndex")]
    pub end_index: Option<i32>,
    /// The start (inclusive) of the span, or not set if unbounded.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// The dimension of the span.
    pub dimension: Option<String>,
    /// The sheet this span is on.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
}

impl Part for DimensionRange {}


/// Formatting options for baseline value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BaselineValueFormat {
    /// Description which is appended after the baseline value.
    /// This field is optional.
    pub description: Option<String>,
    /// Color to be used, in case baseline value represents a negative change for
    /// key value. This field is optional.
    #[serde(rename="negativeColor")]
    pub negative_color: Option<Color>,
    /// The comparison type of key value with baseline value.
    #[serde(rename="comparisonType")]
    pub comparison_type: Option<String>,
    /// Color to be used, in case baseline value represents a positive change for
    /// key value. This field is optional.
    /// If positive_color is also set, this field takes precedence.
    #[serde(rename="positiveColorStyle")]
    pub positive_color_style: Option<ColorStyle>,
    /// Color to be used, in case baseline value represents a negative change for
    /// key value. This field is optional.
    /// If negative_color is also set, this field takes precedence.
    #[serde(rename="negativeColorStyle")]
    pub negative_color_style: Option<ColorStyle>,
    /// Specifies the horizontal text positioning of baseline value.
    /// This field is optional. If not specified, default positioning is used.
    pub position: Option<TextPosition>,
    /// Text formatting options for baseline value.
    #[serde(rename="textFormat")]
    pub text_format: Option<TextFormat>,
    /// Color to be used, in case baseline value represents a positive change for
    /// key value. This field is optional.
    #[serde(rename="positiveColor")]
    pub positive_color: Option<Color>,
}

impl Part for BaselineValueFormat {}


/// Deletes a range of cells, shifting other cells into the deleted area.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteRangeRequest {
    /// The range of cells to delete.
    pub range: Option<GridRange>,
    /// The dimension from which deleted cells will be replaced with.
    /// If ROWS, existing cells will be shifted upward to
    /// replace the deleted cells. If COLUMNS, existing cells
    /// will be shifted left to replace the deleted cells.
    #[serde(rename="shiftDimension")]
    pub shift_dimension: Option<String>,
}

impl Part for DeleteRangeRequest {}


/// Moves one or more rows or columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MoveDimensionRequest {
    /// The source dimensions to move.
    pub source: Option<DimensionRange>,
    /// The zero-based start index of where to move the source data to,
    /// based on the coordinates *before* the source data is removed
    /// from the grid.  Existing data will be shifted down or right
    /// (depending on the dimension) to make room for the moved dimensions.
    /// The source dimensions are removed from the grid, so the
    /// the data may end up in a different index than specified.
    /// 
    /// For example, given `A1..A5` of `0, 1, 2, 3, 4` and wanting to move
    /// `"1"` and `"2"` to between `"3"` and `"4"`, the source would be
    /// `ROWS [1..3)`,and the destination index would be `"4"`
    /// (the zero-based index of row 5).
    /// The end result would be `A1..A5` of `0, 3, 1, 2, 4`.
    #[serde(rename="destinationIndex")]
    pub destination_index: Option<i32>,
}

impl Part for MoveDimensionRequest {}


/// A run of a text format. The format of this run continues until the start
/// index of the next run.
/// When updating, all fields must be set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextFormatRun {
    /// The character index where this run starts.
    #[serde(rename="startIndex")]
    pub start_index: Option<i32>,
    /// The format of this run.  Absent values inherit the cell's format.
    pub format: Option<TextFormat>,
}

impl Part for TextFormatRun {}


/// A filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterView {
    /// The range this filter view covers.
    /// 
    /// When writing, only one of range or named_range_id
    /// may be set.
    pub range: Option<GridRange>,
    /// The sort order per column. Later specifications are used when values
    /// are equal in the earlier specifications.
    #[serde(rename="sortSpecs")]
    pub sort_specs: Option<Vec<SortSpec>>,
    /// The name of the filter view.
    pub title: Option<String>,
    /// The named range this filter view is backed by, if any.
    /// 
    /// When writing, only one of range or named_range_id
    /// may be set.
    #[serde(rename="namedRangeId")]
    pub named_range_id: Option<String>,
    /// The criteria for showing/hiding values per column.
    /// The map's key is the column index, and the value is the criteria for
    /// that column.
    pub criteria: Option<HashMap<String, FilterCriteria>>,
    /// The ID of the filter view.
    #[serde(rename="filterViewId")]
    pub filter_view_id: Option<i32>,
}

impl Part for FilterView {}


/// Sets the basic filter associated with a sheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetBasicFilterRequest {
    /// The filter to set.
    pub filter: Option<BasicFilter>,
}

impl Part for SetBasicFilterRequest {}


/// An <a href="/chart/interactive/docs/gallery/orgchart">org chart</a>.
/// Org charts require a unique set of labels in labels and may
/// optionally include parent_labels and tooltips.
/// parent_labels contain, for each node, the label identifying the parent
/// node.  tooltips contain, for each node, an optional tooltip.
/// 
/// For example, to describe an OrgChart with Alice as the CEO, Bob as the
/// President (reporting to Alice) and Cathy as VP of Sales (also reporting to
/// Alice), have labels contain "Alice", "Bob", "Cathy",
/// parent_labels contain "", "Alice", "Alice" and tooltips contain
/// "CEO", "President", "VP Sales".
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OrgChartSpec {
    /// The data containing the tooltip for the corresponding node.  A blank value
    /// results in no tooltip being displayed for the node.
    /// This field is optional.
    pub tooltips: Option<ChartData>,
    /// The data containing the label of the parent for the corresponding node.
    /// A blank value indicates that the node has no parent and is a top-level
    /// node.
    /// This field is optional.
    #[serde(rename="parentLabels")]
    pub parent_labels: Option<ChartData>,
    /// The color of the org chart nodes.
    #[serde(rename="nodeColor")]
    pub node_color: Option<Color>,
    /// The data containing the labels for all the nodes in the chart.  Labels
    /// must be unique.
    pub labels: Option<ChartData>,
    /// The color of the selected org chart nodes.
    #[serde(rename="selectedNodeColor")]
    pub selected_node_color: Option<Color>,
    /// The color of the org chart nodes.
    /// If node_color is also set, this field takes precedence.
    #[serde(rename="nodeColorStyle")]
    pub node_color_style: Option<ColorStyle>,
    /// The color of the selected org chart nodes.
    /// If selected_node_color is also set, this field takes precedence.
    #[serde(rename="selectedNodeColorStyle")]
    pub selected_node_color_style: Option<ColorStyle>,
    /// The size of the org chart nodes.
    #[serde(rename="nodeSize")]
    pub node_size: Option<String>,
}

impl Part for OrgChartSpec {}


/// The result of adding a banded range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddBandingResponse {
    /// The banded range that was added.
    #[serde(rename="bandedRange")]
    pub banded_range: Option<BandedRange>,
}

impl Part for AddBandingResponse {}


/// Represents a color in the RGBA color space. This representation is designed
/// for simplicity of conversion to/from color representations in various
/// languages over compactness; for example, the fields of this representation
/// can be trivially provided to the constructor of "java.awt.Color" in Java; it
/// can also be trivially provided to UIColor's "+colorWithRed:green:blue:alpha"
/// method in iOS; and, with just a little work, it can be easily formatted into
/// a CSS "rgba()" string in JavaScript, as well.
/// 
/// Note: this proto does not carry information about the absolute color space
/// that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB,
/// DCI-P3, BT.2020, etc.). By default, applications SHOULD assume the sRGB color
/// space.
/// 
/// Example (Java):
/// 
/// ````text
///  import com.google.type.Color;
/// 
///  // ...
///  public static java.awt.Color fromProto(Color protocolor) {
///    float alpha = protocolor.hasAlpha()
///        ? protocolor.getAlpha().getValue()
///        : 1.0;
/// 
///    return new java.awt.Color(
///        protocolor.getRed(),
///        protocolor.getGreen(),
///        protocolor.getBlue(),
///        alpha);
///  }
/// 
///  public static Color toProto(java.awt.Color color) {
///    float red = (float) color.getRed();
///    float green = (float) color.getGreen();
///    float blue = (float) color.getBlue();
///    float denominator = 255.0;
///    Color.Builder resultBuilder =
///        Color
///            .newBuilder()
///            .setRed(red / denominator)
///            .setGreen(green / denominator)
///            .setBlue(blue / denominator);
///    int alpha = color.getAlpha();
///    if (alpha != 255) {
///      result.setAlpha(
///          FloatValue
///              .newBuilder()
///              .setValue(((float) alpha) / denominator)
///              .build());
///    }
///    return resultBuilder.build();
///  }
///  // ...
/// ````
/// 
/// Example (iOS / Obj-C):
/// 
/// ````text
///  // ...
///  static UIColor* fromProto(Color* protocolor) {
///     float red = [protocolor red];
///     float green = [protocolor green];
///     float blue = [protocolor blue];
///     FloatValue* alpha_wrapper = [protocolor alpha];
///     float alpha = 1.0;
///     if (alpha_wrapper != nil) {
///       alpha = [alpha_wrapper value];
///     }
///     return [UIColor colorWithRed:red green:green blue:blue alpha:alpha];
///  }
/// 
///  static Color* toProto(UIColor* color) {
///      CGFloat red, green, blue, alpha;
///      if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) {
///        return nil;
///      }
///      Color* result = [[Color alloc] init];
///      [result setRed:red];
///      [result setGreen:green];
///      [result setBlue:blue];
///      if (alpha <= 0.9999) {
///        [result setAlpha:floatWrapperWithValue(alpha)];
///      }
///      [result autorelease];
///      return result;
/// }
/// // ...
/// ````
/// 
/// Example (JavaScript):
/// 
/// ````text
/// // ...
/// 
/// var protoToCssColor = function(rgb_color) {
///    var redFrac = rgb_color.red || 0.0;
///    var greenFrac = rgb_color.green || 0.0;
///    var blueFrac = rgb_color.blue || 0.0;
///    var red = Math.floor(redFrac * 255);
///    var green = Math.floor(greenFrac * 255);
///    var blue = Math.floor(blueFrac * 255);
/// 
///    if (!('alpha' in rgb_color)) {
///       return rgbToCssColor_(red, green, blue);
///    }
/// 
///    var alphaFrac = rgb_color.alpha.value || 0.0;
///    var rgbParams = [red, green, blue].join(',');
///    return ['rgba(', rgbParams, ',', alphaFrac, ')'].join('');
/// };
/// 
/// var rgbToCssColor_ = function(red, green, blue) {
///   var rgbNumber = new Number((red << 16) | (green << 8) | blue);
///   var hexString = rgbNumber.toString(16);
///   var missingZeros = 6 - hexString.length;
///   var resultBuilder = ['#'];
///   for (var i = 0; i < missingZeros; i++) {
///      resultBuilder.push('0');
///   }
///   resultBuilder.push(hexString);
///   return resultBuilder.join('');
/// };
/// 
/// // ...
/// ````
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// The amount of blue in the color as a value in the interval [0, 1].
    pub blue: Option<f32>,
    /// The fraction of this color that should be applied to the pixel. That is,
    /// the final pixel color is defined by the equation:
    /// 
    ///   pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
    /// 
    /// This means that a value of 1.0 corresponds to a solid color, whereas
    /// a value of 0.0 corresponds to a completely transparent color. This
    /// uses a wrapper message rather than a simple float scalar so that it is
    /// possible to distinguish between a default value and the value being unset.
    /// If omitted, this color object is to be rendered as a solid color
    /// (as if the alpha value had been explicitly given with a value of 1.0).
    pub alpha: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    pub red: Option<f32>,
}

impl Part for Color {}


/// A <a href="/chart/interactive/docs/gallery/candlestickchart">candlestick
/// chart</a>.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CandlestickChartSpec {
    /// The domain data (horizontal axis) for the candlestick chart.  String data
    /// will be treated as discrete labels, other data will be treated as
    /// continuous values.
    pub domain: Option<CandlestickDomain>,
    /// The Candlestick chart data.
    /// Only one CandlestickData is supported.
    pub data: Option<Vec<CandlestickData>>,
}

impl Part for CandlestickChartSpec {}


/// Adds a slicer to a sheet in the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddSlicerRequest {
    /// The slicer that should be added to the spreadsheet, including
    /// the position where it should be placed. The slicerId field is optional; if one is not set, an id
    /// will be randomly generated. (It is an error to specify the ID
    /// of a slicer that already exists.)
    pub slicer: Option<Slicer>,
}

impl Part for AddSlicerRequest {}


/// Updates properties of the filter view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateFilterViewRequest {
    /// The new properties of the filter view.
    pub filter: Option<FilterView>,
    /// The fields that should be updated.  At least one field must be specified.
    /// The root `filter` is implied and should not be specified.
    /// A single `"*"` can be used as short-hand for listing every field.
    pub fields: Option<String>,
}

impl Part for UpdateFilterViewRequest {}


/// Data within a range of the spreadsheet.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [values append spreadsheets](struct.SpreadsheetValueAppendCall.html) (request)
/// * [values get spreadsheets](struct.SpreadsheetValueGetCall.html) (response)
/// * [values update spreadsheets](struct.SpreadsheetValueUpdateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ValueRange {
    /// The range the values cover, in A1 notation.
    /// For output, this range indicates the entire requested range,
    /// even though the values will exclude trailing rows and columns.
    /// When appending values, this field represents the range to search for a
    /// table, after which values will be appended.
    pub range: Option<String>,
    /// The data that was read or to be written.  This is an array of arrays,
    /// the outer array representing all the data and each inner array
    /// representing a major dimension. Each item in the inner array
    /// corresponds with one cell.
    /// 
    /// For output, empty trailing rows and columns will not be included.
    /// 
    /// For input, supported value types are: bool, string, and double.
    /// Null values will be skipped.
    /// To set a cell to an empty value, set the string value to an empty string.
    pub values: Option<Vec<Vec<String>>>,
    /// The major dimension of the values.
    /// 
    /// For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,
    /// then requesting `range=A1:B2,majorDimension=ROWS` will return
    /// `[[1,2],[3,4]]`,
    /// whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return
    /// `[[1,3],[2,4]]`.
    /// 
    /// For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]`
    /// will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS`
    /// then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`.
    /// 
    /// When writing, if this field is not set, it defaults to ROWS.
    #[serde(rename="majorDimension")]
    pub major_dimension: Option<String>,
}

impl RequestValue for ValueRange {}
impl ResponseResult for ValueRange {}


/// Removes the banded range with the given ID from the spreadsheet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeleteBandingRequest {
    /// The ID of the banded range to delete.
    #[serde(rename="bandedRangeId")]
    pub banded_range_id: Option<i32>,
}

impl Part for DeleteBandingRequest {}


/// Finds and replaces data in cells over a range, sheet, or all sheets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FindReplaceRequest {
    /// True if the search should include cells with formulas.
    /// False to skip cells with formulas.
    #[serde(rename="includeFormulas")]
    pub include_formulas: Option<bool>,
    /// True if the find value should match the entire cell.
    #[serde(rename="matchEntireCell")]
    pub match_entire_cell: Option<bool>,
    /// True to find/replace over all sheets.
    #[serde(rename="allSheets")]
    pub all_sheets: Option<bool>,
    /// True if the search is case sensitive.
    #[serde(rename="matchCase")]
    pub match_case: Option<bool>,
    /// The value to search.
    pub find: Option<String>,
    /// The range to find/replace over.
    pub range: Option<GridRange>,
    /// True if the find value is a regex.
    /// The regular expression and replacement should follow Java regex rules
    /// at https://docs.oracle.com/javase/8/docs/api/java/util/regex/Pattern.html.
    /// The replacement string is allowed to refer to capturing groups.
    /// For example, if one cell has the contents `"Google Sheets"` and another
    /// has `"Google Docs"`, then searching for `"o.* (.*)"` with a replacement of
    /// `"$1 Rocks"` would change the contents of the cells to
    /// `"GSheets Rocks"` and `"GDocs Rocks"` respectively.
    #[serde(rename="searchByRegex")]
    pub search_by_regex: Option<bool>,
    /// The sheet to find/replace over.
    #[serde(rename="sheetId")]
    pub sheet_id: Option<i32>,
    /// The value to use as the replacement.
    pub replacement: Option<String>,
}

impl Part for FindReplaceRequest {}


/// A pair mapping a spreadsheet theme color type to the concrete color it
/// represents.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColorPair {
    /// The concrete color corresponding to the theme color type.
    pub color: Option<ColorStyle>,
    /// The type of the spreadsheet theme color.
    #[serde(rename="colorType")]
    pub color_type: Option<String>,
}

impl Part for ThemeColorPair {}


/// An axis of the chart.
/// A chart may not have more than one axis per
/// axis position.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BasicChartAxis {
    /// The format of the title.
    /// Only valid if the axis is not associated with the domain.
    pub format: Option<TextFormat>,
    /// The position of this axis.
    pub position: Option<String>,
    /// The view window options for this axis.
    #[serde(rename="viewWindowOptions")]
    pub view_window_options: Option<ChartAxisViewWindowOptions>,
    /// The title of this axis. If set, this overrides any title inferred
    /// from headers of the data.
    pub title: Option<String>,
    /// The axis title text position.
    #[serde(rename="titleTextPosition")]
    pub title_text_position: Option<TextPosition>,
}

impl Part for BasicChartAxis {}


/// A color scale for a treemap chart.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TreemapChartColorScale {
    /// The background color for cells with a color value greater than or equal
    /// to maxValue. Defaults to #109618 if not
    /// specified.
    #[serde(rename="maxValueColor")]
    pub max_value_color: Option<Color>,
    /// The background color for cells that have no color data associated with
    /// them. Defaults to #000000 if not specified.
    #[serde(rename="noDataColor")]
    pub no_data_color: Option<Color>,
    /// The background color for cells with a color value at the midpoint between
    /// minValue and
    /// maxValue. Defaults to #efe6dc if not
    /// specified.
    /// If mid_value_color is also set, this field takes precedence.
    #[serde(rename="midValueColorStyle")]
    pub mid_value_color_style: Option<ColorStyle>,
    /// The background color for cells with a color value greater than or equal
    /// to maxValue. Defaults to #109618 if not
    /// specified.
    /// If max_value_color is also set, this field takes precedence.
    #[serde(rename="maxValueColorStyle")]
    pub max_value_color_style: Option<ColorStyle>,
    /// The background color for cells with a color value less than or equal to
    /// minValue. Defaults to #dc3912 if not
    /// specified.
    #[serde(rename="minValueColor")]
    pub min_value_color: Option<Color>,
    /// The background color for cells that have no color data associated with
    /// them. Defaults to #000000 if not specified.
    /// If no_data_color is also set, this field takes precedence.
    #[serde(rename="noDataColorStyle")]
    pub no_data_color_style: Option<ColorStyle>,
    /// The background color for cells with a color value at the midpoint between
    /// minValue and
    /// maxValue. Defaults to #efe6dc if not
    /// specified.
    #[serde(rename="midValueColor")]
    pub mid_value_color: Option<Color>,
    /// The background color for cells with a color value less than or equal to
    /// minValue. Defaults to #dc3912 if not
    /// specified.
    /// If min_value_color is also set, this field takes precedence.
    #[serde(rename="minValueColorStyle")]
    pub min_value_color_style: Option<ColorStyle>,
}

impl Part for TreemapChartColorScale {}


/// A <a href="/chart/interactive/docs/gallery/piechart">pie chart</a>.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PieChartSpec {
    /// The data that covers the one and only series of the pie chart.
    pub series: Option<ChartData>,
    /// The data that covers the domain of the pie chart.
    pub domain: Option<ChartData>,
    /// True if the pie is three dimensional.
    #[serde(rename="threeDimensional")]
    pub three_dimensional: Option<bool>,
    /// Where the legend of the pie chart should be drawn.
    #[serde(rename="legendPosition")]
    pub legend_position: Option<String>,
    /// The size of the hole in the pie chart.
    #[serde(rename="pieHole")]
    pub pie_hole: Option<f64>,
}

impl Part for PieChartSpec {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *spreadsheet* resources.
/// It is not used directly, but through the `Sheets` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_sheets4 as sheets4;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use sheets4::Sheets;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_update(...)`, `create(...)`, `developer_metadata_get(...)`, `developer_metadata_search(...)`, `get(...)`, `get_by_data_filter(...)`, `sheets_copy_to(...)`, `values_append(...)`, `values_batch_clear(...)`, `values_batch_clear_by_data_filter(...)`, `values_batch_get(...)`, `values_batch_get_by_data_filter(...)`, `values_batch_update(...)`, `values_batch_update_by_data_filter(...)`, `values_clear(...)`, `values_get(...)` and `values_update(...)`
/// // to build up your call.
/// let rb = hub.spreadsheets();
/// # }
/// ```
pub struct SpreadsheetMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
}

impl<'a, C, A> MethodsBuilder for SpreadsheetMethods<'a, C, A> {}

impl<'a, C, A> SpreadsheetMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the spreadsheet at the given ID.
    /// The caller must specify the spreadsheet ID.
    /// 
    /// This method differs from GetSpreadsheet in that it allows selecting
    /// which subsets of spreadsheet data to return by specifying a
    /// dataFilters parameter.
    /// Multiple DataFilters can be specified.  Specifying one or
    /// more data filters will return the portions of the spreadsheet that
    /// intersect ranges matched by any of the filters.
    /// 
    /// By default, data within grids will not be returned.
    /// You can include grid data one of two ways:
    /// 
    /// * Specify a field mask listing your desired fields using the `fields` URL
    /// parameter in HTTP
    /// 
    /// * Set the includeGridData
    /// parameter to true.  If a field mask is set, the `includeGridData`
    /// parameter is ignored
    /// 
    /// For large spreadsheets, it is recommended to retrieve only the specific
    /// fields of the spreadsheet that you want.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The spreadsheet to request.
    pub fn get_by_data_filter(&self, request: GetSpreadsheetByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetGetByDataFilterCall<'a, C, A> {
        SpreadsheetGetByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a spreadsheet, returning the newly created spreadsheet.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Spreadsheet) -> SpreadsheetCreateCall<'a, C, A> {
        SpreadsheetCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears one or more ranges of values from a spreadsheet.
    /// The caller must specify the spreadsheet ID and one or more ranges.
    /// Only values are cleared -- all other properties of the cell (such as
    /// formatting, data validation, etc..) are kept.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_clear(&self, request: BatchClearValuesRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchClearCall<'a, C, A> {
        SpreadsheetValueBatchClearCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the spreadsheet at the given ID.
    /// The caller must specify the spreadsheet ID.
    /// 
    /// By default, data within grids will not be returned.
    /// You can include grid data one of two ways:
    /// 
    /// * Specify a field mask listing your desired fields using the `fields` URL
    /// parameter in HTTP
    /// 
    /// * Set the includeGridData
    /// URL parameter to true.  If a field mask is set, the `includeGridData`
    /// parameter is ignored
    /// 
    /// For large spreadsheets, it is recommended to retrieve only the specific
    /// fields of the spreadsheet that you want.
    /// 
    /// To retrieve only subsets of the spreadsheet, use the
    /// ranges URL parameter.
    /// Multiple ranges can be specified.  Limiting the range will
    /// return only the portions of the spreadsheet that intersect the requested
    /// ranges. Ranges are specified using A1 notation.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The spreadsheet to request.
    pub fn get(&self, spreadsheet_id: &str) -> SpreadsheetGetCall<'a, C, A> {
        SpreadsheetGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _ranges: Default::default(),
            _include_grid_data: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns one or more ranges of values from a spreadsheet.
    /// The caller must specify the spreadsheet ID and one or more ranges.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve data from.
    pub fn values_batch_get(&self, spreadsheet_id: &str) -> SpreadsheetValueBatchGetCall<'a, C, A> {
        SpreadsheetValueBatchGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _value_render_option: Default::default(),
            _ranges: Default::default(),
            _major_dimension: Default::default(),
            _date_time_render_option: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Appends values to a spreadsheet. The input range is used to search for
    /// existing data and find a "table" within that range. Values will be
    /// appended to the next row of the table, starting with the first column of
    /// the table. See the
    /// [guide](/sheets/api/guides/values#appending_values)
    /// and
    /// [sample code](/sheets/api/samples/writing#append_values)
    /// for specific details of how tables are detected and data is appended.
    /// 
    /// The caller must specify the spreadsheet ID, range, and
    /// a valueInputOption.  The `valueInputOption` only
    /// controls how the input data will be added to the sheet (column-wise or
    /// row-wise), it does not influence what cell the data starts being written
    /// to.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    /// * `range` - The A1 notation of a range to search for a logical table of data.
    ///             Values are appended after the last row of the table.
    pub fn values_append(&self, request: ValueRange, spreadsheet_id: &str, range: &str) -> SpreadsheetValueAppendCall<'a, C, A> {
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
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a range of values from a spreadsheet.
    /// The caller must specify the spreadsheet ID and a range.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve data from.
    /// * `range` - The A1 notation of the values to retrieve.
    pub fn values_get(&self, spreadsheet_id: &str, range: &str) -> SpreadsheetValueGetCall<'a, C, A> {
        SpreadsheetValueGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _range: range.to_string(),
            _value_render_option: Default::default(),
            _major_dimension: Default::default(),
            _date_time_render_option: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies a single sheet from a spreadsheet to another spreadsheet.
    /// Returns the properties of the newly created sheet.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet containing the sheet to copy.
    /// * `sheetId` - The ID of the sheet to copy.
    pub fn sheets_copy_to(&self, request: CopySheetToAnotherSpreadsheetRequest, spreadsheet_id: &str, sheet_id: i32) -> SpreadsheetSheetCopyToCall<'a, C, A> {
        SpreadsheetSheetCopyToCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _sheet_id: sheet_id,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears values from a spreadsheet.
    /// The caller must specify the spreadsheet ID and range.
    /// Only values are cleared -- all other properties of the cell (such as
    /// formatting, data validation, etc..) are kept.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    /// * `range` - The A1 notation of the values to clear.
    pub fn values_clear(&self, request: ClearValuesRequest, spreadsheet_id: &str, range: &str) -> SpreadsheetValueClearCall<'a, C, A> {
        SpreadsheetValueClearCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _range: range.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets values in a range of a spreadsheet.
    /// The caller must specify the spreadsheet ID, range, and
    /// a valueInputOption.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    /// * `range` - The A1 notation of the values to update.
    pub fn values_update(&self, request: ValueRange, spreadsheet_id: &str, range: &str) -> SpreadsheetValueUpdateCall<'a, C, A> {
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
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets values in one or more ranges of a spreadsheet.
    /// The caller must specify the spreadsheet ID,
    /// a valueInputOption, and one or more
    /// ValueRanges.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_update(&self, request: BatchUpdateValuesRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchUpdateCall<'a, C, A> {
        SpreadsheetValueBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns one or more ranges of values that match the specified data filters.
    /// The caller must specify the spreadsheet ID and one or more
    /// DataFilters.  Ranges that match any of the data filters in
    /// the request will be returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve data from.
    pub fn values_batch_get_by_data_filter(&self, request: BatchGetValuesByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchGetByDataFilterCall<'a, C, A> {
        SpreadsheetValueBatchGetByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears one or more ranges of values from a spreadsheet.
    /// The caller must specify the spreadsheet ID and one or more
    /// DataFilters. Ranges matching any of the specified data
    /// filters will be cleared.  Only values are cleared -- all other properties
    /// of the cell (such as formatting, data validation, etc..) are kept.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_clear_by_data_filter(&self, request: BatchClearValuesByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchClearByDataFilterCall<'a, C, A> {
        SpreadsheetValueBatchClearByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets values in one or more ranges of a spreadsheet.
    /// The caller must specify the spreadsheet ID,
    /// a valueInputOption, and one or more
    /// DataFilterValueRanges.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to update.
    pub fn values_batch_update_by_data_filter(&self, request: BatchUpdateValuesByDataFilterRequest, spreadsheet_id: &str) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A> {
        SpreadsheetValueBatchUpdateByDataFilterCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Applies one or more updates to the spreadsheet.
    /// 
    /// Each request is validated before
    /// being applied. If any request is not valid then the entire request will
    /// fail and nothing will be applied.
    /// 
    /// Some requests have replies to
    /// give you some information about how
    /// they are applied. The replies will mirror the requests.  For example,
    /// if you applied 4 updates and the 3rd one had a reply, then the
    /// response will have 2 empty replies, the actual reply, and another empty
    /// reply, in that order.
    /// 
    /// Due to the collaborative nature of spreadsheets, it is not guaranteed that
    /// the spreadsheet will reflect exactly your changes after this completes,
    /// however it is guaranteed that the updates in the request will be
    /// applied together atomically. Your changes may be altered with respect to
    /// collaborator changes. If there are no collaborators, the spreadsheet
    /// should reflect your changes.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The spreadsheet to apply the updates to.
    pub fn batch_update(&self, request: BatchUpdateSpreadsheetRequest, spreadsheet_id: &str) -> SpreadsheetBatchUpdateCall<'a, C, A> {
        SpreadsheetBatchUpdateCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns all developer metadata matching the specified DataFilter.
    /// If the provided DataFilter represents a DeveloperMetadataLookup object,
    /// this will return all DeveloperMetadata entries selected by it. If the
    /// DataFilter represents a location in a spreadsheet, this will return all
    /// developer metadata associated with locations intersecting that region.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve metadata from.
    pub fn developer_metadata_search(&self, request: SearchDeveloperMetadataRequest, spreadsheet_id: &str) -> SpreadsheetDeveloperMetadataSearchCall<'a, C, A> {
        SpreadsheetDeveloperMetadataSearchCall {
            hub: self.hub,
            _request: request,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the developer metadata with the specified ID.
    /// The caller must specify the spreadsheet ID and the developer metadata's
    /// unique metadataId.
    /// 
    /// # Arguments
    ///
    /// * `spreadsheetId` - The ID of the spreadsheet to retrieve metadata from.
    /// * `metadataId` - The ID of the developer metadata to retrieve.
    pub fn developer_metadata_get(&self, spreadsheet_id: &str, metadata_id: i32) -> SpreadsheetDeveloperMetadataGetCall<'a, C, A> {
        SpreadsheetDeveloperMetadataGetCall {
            hub: self.hub,
            _spreadsheet_id: spreadsheet_id.to_string(),
            _metadata_id: metadata_id,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns the spreadsheet at the given ID.
/// The caller must specify the spreadsheet ID.
/// 
/// This method differs from GetSpreadsheet in that it allows selecting
/// which subsets of spreadsheet data to return by specifying a
/// dataFilters parameter.
/// Multiple DataFilters can be specified.  Specifying one or
/// more data filters will return the portions of the spreadsheet that
/// intersect ranges matched by any of the filters.
/// 
/// By default, data within grids will not be returned.
/// You can include grid data one of two ways:
/// 
/// * Specify a field mask listing your desired fields using the `fields` URL
/// parameter in HTTP
/// 
/// * Set the includeGridData
/// parameter to true.  If a field mask is set, the `includeGridData`
/// parameter is ignored
/// 
/// For large spreadsheets, it is recommended to retrieve only the specific
/// fields of the spreadsheet that you want.
///
/// A builder for the *getByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::GetSpreadsheetByDataFilterRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetSpreadsheetByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().get_by_data_filter(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetGetByDataFilterCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: GetSpreadsheetByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetGetByDataFilterCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetGetByDataFilterCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Spreadsheet)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.getByDataFilter",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}:getByDataFilter";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: GetSpreadsheetByDataFilterRequest) -> SpreadsheetGetByDataFilterCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The spreadsheet to request.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetGetByDataFilterCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetGetByDataFilterCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetGetByDataFilterCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetGetByDataFilterCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a spreadsheet, returning the newly created spreadsheet.
///
/// A builder for the *create* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::Spreadsheet;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Spreadsheet::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().create(req)
///              .doit();
/// # }
/// ```
pub struct SpreadsheetCreateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: Spreadsheet,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetCreateCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetCreateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Spreadsheet)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.create",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(3 + self._additional_params.len());
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }


        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: Spreadsheet) -> SpreadsheetCreateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetCreateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetCreateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetCreateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Clears one or more ranges of values from a spreadsheet.
/// The caller must specify the spreadsheet ID and one or more ranges.
/// Only values are cleared -- all other properties of the cell (such as
/// formatting, data validation, etc..) are kept.
///
/// A builder for the *values.batchClear* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::BatchClearValuesRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchClearValuesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_clear(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueBatchClearCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: BatchClearValuesRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueBatchClearCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueBatchClearCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchClearValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.batchClear",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchClear";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: BatchClearValuesRequest) -> SpreadsheetValueBatchClearCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchClearCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueBatchClearCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchClearCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueBatchClearCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the spreadsheet at the given ID.
/// The caller must specify the spreadsheet ID.
/// 
/// By default, data within grids will not be returned.
/// You can include grid data one of two ways:
/// 
/// * Specify a field mask listing your desired fields using the `fields` URL
/// parameter in HTTP
/// 
/// * Set the includeGridData
/// URL parameter to true.  If a field mask is set, the `includeGridData`
/// parameter is ignored
/// 
/// For large spreadsheets, it is recommended to retrieve only the specific
/// fields of the spreadsheet that you want.
/// 
/// To retrieve only subsets of the spreadsheet, use the
/// ranges URL parameter.
/// Multiple ranges can be specified.  Limiting the range will
/// return only the portions of the spreadsheet that intersect the requested
/// ranges. Ranges are specified using A1 notation.
///
/// A builder for the *get* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().get("spreadsheetId")
///              .add_ranges("et")
///              .include_grid_data(true)
///              .doit();
/// # }
/// ```
pub struct SpreadsheetGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _spreadsheet_id: String,
    _ranges: Vec<String>,
    _include_grid_data: Option<bool>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetGetCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Spreadsheet)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        if self._ranges.len() > 0 {
            for f in self._ranges.iter() {
                params.push(("ranges", f.to_string()));
            }
        }
        if let Some(value) = self._include_grid_data {
            params.push(("includeGridData", value.to_string()));
        }
        for &field in ["alt", "spreadsheetId", "ranges", "includeGridData"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetGetCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The ranges to retrieve from the spreadsheet.
    ///
    /// Append the given value to the *ranges* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_ranges(mut self, new_value: &str) -> SpreadsheetGetCall<'a, C, A> {
        self._ranges.push(new_value.to_string());
        self
    }
    /// True if grid data should be returned.
    /// This parameter is ignored if a field mask was set in the request.
    ///
    /// Sets the *include grid data* query property to the given value.
    pub fn include_grid_data(mut self, new_value: bool) -> SpreadsheetGetCall<'a, C, A> {
        self._include_grid_data = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns one or more ranges of values from a spreadsheet.
/// The caller must specify the spreadsheet ID and one or more ranges.
///
/// A builder for the *values.batchGet* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_get("spreadsheetId")
///              .value_render_option("Lorem")
///              .add_ranges("et")
///              .major_dimension("duo")
///              .date_time_render_option("aliquyam")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueBatchGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _spreadsheet_id: String,
    _value_render_option: Option<String>,
    _ranges: Vec<String>,
    _major_dimension: Option<String>,
    _date_time_render_option: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueBatchGetCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueBatchGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchGetValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.batchGet",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        if let Some(value) = self._value_render_option {
            params.push(("valueRenderOption", value.to_string()));
        }
        if self._ranges.len() > 0 {
            for f in self._ranges.iter() {
                params.push(("ranges", f.to_string()));
            }
        }
        if let Some(value) = self._major_dimension {
            params.push(("majorDimension", value.to_string()));
        }
        if let Some(value) = self._date_time_render_option {
            params.push(("dateTimeRenderOption", value.to_string()));
        }
        for &field in ["alt", "spreadsheetId", "valueRenderOption", "ranges", "majorDimension", "dateTimeRenderOption"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchGet";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// How values should be represented in the output.
    /// The default render option is ValueRenderOption.FORMATTED_VALUE.
    ///
    /// Sets the *value render option* query property to the given value.
    pub fn value_render_option(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, C, A> {
        self._value_render_option = Some(new_value.to_string());
        self
    }
    /// The A1 notation of the values to retrieve.
    ///
    /// Append the given value to the *ranges* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_ranges(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, C, A> {
        self._ranges.push(new_value.to_string());
        self
    }
    /// The major dimension that results should use.
    /// 
    /// For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`,
    /// then requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`,
    /// whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns
    /// `[[1,3],[2,4]]`.
    ///
    /// Sets the *major dimension* query property to the given value.
    pub fn major_dimension(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, C, A> {
        self._major_dimension = Some(new_value.to_string());
        self
    }
    /// How dates, times, and durations should be represented in the output.
    /// This is ignored if value_render_option is
    /// FORMATTED_VALUE.
    /// The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER].
    ///
    /// Sets the *date time render option* query property to the given value.
    pub fn date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueBatchGetCall<'a, C, A> {
        self._date_time_render_option = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueBatchGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueBatchGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Appends values to a spreadsheet. The input range is used to search for
/// existing data and find a "table" within that range. Values will be
/// appended to the next row of the table, starting with the first column of
/// the table. See the
/// [guide](/sheets/api/guides/values#appending_values)
/// and
/// [sample code](/sheets/api/samples/writing#append_values)
/// for specific details of how tables are detected and data is appended.
/// 
/// The caller must specify the spreadsheet ID, range, and
/// a valueInputOption.  The `valueInputOption` only
/// controls how the input data will be added to the sheet (column-wise or
/// row-wise), it does not influence what cell the data starts being written
/// to.
///
/// A builder for the *values.append* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::ValueRange;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ValueRange::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
///              .value_input_option("eos")
///              .response_value_render_option("erat")
///              .response_date_time_render_option("sadipscing")
///              .insert_data_option("dolor")
///              .include_values_in_response(true)
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueAppendCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: ValueRange,
    _spreadsheet_id: String,
    _range: String,
    _value_input_option: Option<String>,
    _response_value_render_option: Option<String>,
    _response_date_time_render_option: Option<String>,
    _insert_data_option: Option<String>,
    _include_values_in_response: Option<bool>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueAppendCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueAppendCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, AppendValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.append",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        params.push(("range", self._range.to_string()));
        if let Some(value) = self._value_input_option {
            params.push(("valueInputOption", value.to_string()));
        }
        if let Some(value) = self._response_value_render_option {
            params.push(("responseValueRenderOption", value.to_string()));
        }
        if let Some(value) = self._response_date_time_render_option {
            params.push(("responseDateTimeRenderOption", value.to_string()));
        }
        if let Some(value) = self._insert_data_option {
            params.push(("insertDataOption", value.to_string()));
        }
        if let Some(value) = self._include_values_in_response {
            params.push(("includeValuesInResponse", value.to_string()));
        }
        for &field in ["alt", "spreadsheetId", "range", "valueInputOption", "responseValueRenderOption", "responseDateTimeRenderOption", "insertDataOption", "includeValuesInResponse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}:append";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["range", "spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: ValueRange) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The A1 notation of a range to search for a logical table of data.
    /// Values are appended after the last row of the table.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._range = new_value.to_string();
        self
    }
    /// How the input data should be interpreted.
    ///
    /// Sets the *value input option* query property to the given value.
    pub fn value_input_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._value_input_option = Some(new_value.to_string());
        self
    }
    /// Determines how values in the response should be rendered.
    /// The default render option is ValueRenderOption.FORMATTED_VALUE.
    ///
    /// Sets the *response value render option* query property to the given value.
    pub fn response_value_render_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._response_value_render_option = Some(new_value.to_string());
        self
    }
    /// Determines how dates, times, and durations in the response should be
    /// rendered. This is ignored if response_value_render_option is
    /// FORMATTED_VALUE.
    /// The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER].
    ///
    /// Sets the *response date time render option* query property to the given value.
    pub fn response_date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._response_date_time_render_option = Some(new_value.to_string());
        self
    }
    /// How the input data should be inserted.
    ///
    /// Sets the *insert data option* query property to the given value.
    pub fn insert_data_option(mut self, new_value: &str) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._insert_data_option = Some(new_value.to_string());
        self
    }
    /// Determines if the update response should include the values
    /// of the cells that were appended. By default, responses
    /// do not include the updated values.
    ///
    /// Sets the *include values in response* query property to the given value.
    pub fn include_values_in_response(mut self, new_value: bool) -> SpreadsheetValueAppendCall<'a, C, A> {
        self._include_values_in_response = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueAppendCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueAppendCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueAppendCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns a range of values from a spreadsheet.
/// The caller must specify the spreadsheet ID and a range.
///
/// A builder for the *values.get* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_get("spreadsheetId", "range")
///              .value_render_option("no")
///              .major_dimension("labore")
///              .date_time_render_option("eirmod")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _spreadsheet_id: String,
    _range: String,
    _value_render_option: Option<String>,
    _major_dimension: Option<String>,
    _date_time_render_option: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueGetCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ValueRange)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        params.push(("range", self._range.to_string()));
        if let Some(value) = self._value_render_option {
            params.push(("valueRenderOption", value.to_string()));
        }
        if let Some(value) = self._major_dimension {
            params.push(("majorDimension", value.to_string()));
        }
        if let Some(value) = self._date_time_render_option {
            params.push(("dateTimeRenderOption", value.to_string()));
        }
        for &field in ["alt", "spreadsheetId", "range", "valueRenderOption", "majorDimension", "dateTimeRenderOption"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::DriveReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["range", "spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The A1 notation of the values to retrieve.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, C, A> {
        self._range = new_value.to_string();
        self
    }
    /// How values should be represented in the output.
    /// The default render option is ValueRenderOption.FORMATTED_VALUE.
    ///
    /// Sets the *value render option* query property to the given value.
    pub fn value_render_option(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, C, A> {
        self._value_render_option = Some(new_value.to_string());
        self
    }
    /// The major dimension that results should use.
    /// 
    /// For example, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then
    /// requesting `range=A1:B2,majorDimension=ROWS` returns `[[1,2],[3,4]]`,
    /// whereas requesting `range=A1:B2,majorDimension=COLUMNS` returns
    /// `[[1,3],[2,4]]`.
    ///
    /// Sets the *major dimension* query property to the given value.
    pub fn major_dimension(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, C, A> {
        self._major_dimension = Some(new_value.to_string());
        self
    }
    /// How dates, times, and durations should be represented in the output.
    /// This is ignored if value_render_option is
    /// FORMATTED_VALUE.
    /// The default dateTime render option is [DateTimeRenderOption.SERIAL_NUMBER].
    ///
    /// Sets the *date time render option* query property to the given value.
    pub fn date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueGetCall<'a, C, A> {
        self._date_time_render_option = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::DriveReadonly`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Copies a single sheet from a spreadsheet to another spreadsheet.
/// Returns the properties of the newly created sheet.
///
/// A builder for the *sheets.copyTo* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::CopySheetToAnotherSpreadsheetRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CopySheetToAnotherSpreadsheetRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().sheets_copy_to(req, "spreadsheetId", -37)
///              .doit();
/// # }
/// ```
pub struct SpreadsheetSheetCopyToCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: CopySheetToAnotherSpreadsheetRequest,
    _spreadsheet_id: String,
    _sheet_id: i32,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetSheetCopyToCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetSheetCopyToCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SheetProperties)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.sheets.copyTo",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        params.push(("sheetId", self._sheet_id.to_string()));
        for &field in ["alt", "spreadsheetId", "sheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/sheets/{sheetId}:copyTo";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{sheetId}", "sheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["sheetId", "spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: CopySheetToAnotherSpreadsheetRequest) -> SpreadsheetSheetCopyToCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet containing the sheet to copy.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetSheetCopyToCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The ID of the sheet to copy.
    ///
    /// Sets the *sheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn sheet_id(mut self, new_value: i32) -> SpreadsheetSheetCopyToCall<'a, C, A> {
        self._sheet_id = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetSheetCopyToCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetSheetCopyToCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetSheetCopyToCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Clears values from a spreadsheet.
/// The caller must specify the spreadsheet ID and range.
/// Only values are cleared -- all other properties of the cell (such as
/// formatting, data validation, etc..) are kept.
///
/// A builder for the *values.clear* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::ClearValuesRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ClearValuesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_clear(req, "spreadsheetId", "range")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueClearCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: ClearValuesRequest,
    _spreadsheet_id: String,
    _range: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueClearCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueClearCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ClearValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.clear",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        params.push(("range", self._range.to_string()));
        for &field in ["alt", "spreadsheetId", "range"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}:clear";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["range", "spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: ClearValuesRequest) -> SpreadsheetValueClearCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueClearCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The A1 notation of the values to clear.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueClearCall<'a, C, A> {
        self._range = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueClearCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueClearCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueClearCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets values in a range of a spreadsheet.
/// The caller must specify the spreadsheet ID, range, and
/// a valueInputOption.
///
/// A builder for the *values.update* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::ValueRange;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ValueRange::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_update(req, "spreadsheetId", "range")
///              .value_input_option("et")
///              .response_value_render_option("duo")
///              .response_date_time_render_option("et")
///              .include_values_in_response(true)
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: ValueRange,
    _spreadsheet_id: String,
    _range: String,
    _value_input_option: Option<String>,
    _response_value_render_option: Option<String>,
    _response_date_time_render_option: Option<String>,
    _include_values_in_response: Option<bool>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueUpdateCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, UpdateValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        params.push(("range", self._range.to_string()));
        if let Some(value) = self._value_input_option {
            params.push(("valueInputOption", value.to_string()));
        }
        if let Some(value) = self._response_value_render_option {
            params.push(("responseValueRenderOption", value.to_string()));
        }
        if let Some(value) = self._response_date_time_render_option {
            params.push(("responseDateTimeRenderOption", value.to_string()));
        }
        if let Some(value) = self._include_values_in_response {
            params.push(("includeValuesInResponse", value.to_string()));
        }
        for &field in ["alt", "spreadsheetId", "range", "valueInputOption", "responseValueRenderOption", "responseDateTimeRenderOption", "includeValuesInResponse"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values/{range}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{range}", "range")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["range", "spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: ValueRange) -> SpreadsheetValueUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The A1 notation of the values to update.
    ///
    /// Sets the *range* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn range(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, C, A> {
        self._range = new_value.to_string();
        self
    }
    /// How the input data should be interpreted.
    ///
    /// Sets the *value input option* query property to the given value.
    pub fn value_input_option(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, C, A> {
        self._value_input_option = Some(new_value.to_string());
        self
    }
    /// Determines how values in the response should be rendered.
    /// The default render option is ValueRenderOption.FORMATTED_VALUE.
    ///
    /// Sets the *response value render option* query property to the given value.
    pub fn response_value_render_option(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, C, A> {
        self._response_value_render_option = Some(new_value.to_string());
        self
    }
    /// Determines how dates, times, and durations in the response should be
    /// rendered. This is ignored if response_value_render_option is
    /// FORMATTED_VALUE.
    /// The default dateTime render option is
    /// DateTimeRenderOption.SERIAL_NUMBER.
    ///
    /// Sets the *response date time render option* query property to the given value.
    pub fn response_date_time_render_option(mut self, new_value: &str) -> SpreadsheetValueUpdateCall<'a, C, A> {
        self._response_date_time_render_option = Some(new_value.to_string());
        self
    }
    /// Determines if the update response should include the values
    /// of the cells that were updated. By default, responses
    /// do not include the updated values.
    /// If the range to write was larger than the range actually written, the
    /// response includes all values in the requested range (excluding trailing
    /// empty rows and columns).
    ///
    /// Sets the *include values in response* query property to the given value.
    pub fn include_values_in_response(mut self, new_value: bool) -> SpreadsheetValueUpdateCall<'a, C, A> {
        self._include_values_in_response = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueUpdateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets values in one or more ranges of a spreadsheet.
/// The caller must specify the spreadsheet ID,
/// a valueInputOption, and one or more
/// ValueRanges.
///
/// A builder for the *values.batchUpdate* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::BatchUpdateValuesRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateValuesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_update(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueBatchUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: BatchUpdateValuesRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueBatchUpdateCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueBatchUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchUpdateValuesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.batchUpdate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchUpdate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: BatchUpdateValuesRequest) -> SpreadsheetValueBatchUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchUpdateCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueBatchUpdateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueBatchUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns one or more ranges of values that match the specified data filters.
/// The caller must specify the spreadsheet ID and one or more
/// DataFilters.  Ranges that match any of the data filters in
/// the request will be returned.
///
/// A builder for the *values.batchGetByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::BatchGetValuesByDataFilterRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchGetValuesByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_get_by_data_filter(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueBatchGetByDataFilterCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: BatchGetValuesByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueBatchGetByDataFilterCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueBatchGetByDataFilterCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchGetValuesByDataFilterResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.batchGetByDataFilter",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchGetByDataFilter";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: BatchGetValuesByDataFilterRequest) -> SpreadsheetValueBatchGetByDataFilterCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to retrieve data from.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchGetByDataFilterCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueBatchGetByDataFilterCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchGetByDataFilterCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueBatchGetByDataFilterCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Clears one or more ranges of values from a spreadsheet.
/// The caller must specify the spreadsheet ID and one or more
/// DataFilters. Ranges matching any of the specified data
/// filters will be cleared.  Only values are cleared -- all other properties
/// of the cell (such as formatting, data validation, etc..) are kept.
///
/// A builder for the *values.batchClearByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::BatchClearValuesByDataFilterRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchClearValuesByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_clear_by_data_filter(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueBatchClearByDataFilterCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: BatchClearValuesByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueBatchClearByDataFilterCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueBatchClearByDataFilterCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchClearValuesByDataFilterResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.batchClearByDataFilter",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchClearByDataFilter";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: BatchClearValuesByDataFilterRequest) -> SpreadsheetValueBatchClearByDataFilterCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchClearByDataFilterCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueBatchClearByDataFilterCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchClearByDataFilterCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueBatchClearByDataFilterCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets values in one or more ranges of a spreadsheet.
/// The caller must specify the spreadsheet ID,
/// a valueInputOption, and one or more
/// DataFilterValueRanges.
///
/// A builder for the *values.batchUpdateByDataFilter* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::BatchUpdateValuesByDataFilterRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateValuesByDataFilterRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().values_batch_update_by_data_filter(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: BatchUpdateValuesByDataFilterRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchUpdateValuesByDataFilterResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.values.batchUpdateByDataFilter",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/values:batchUpdateByDataFilter";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: BatchUpdateValuesByDataFilterRequest) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to update.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetValueBatchUpdateByDataFilterCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Applies one or more updates to the spreadsheet.
/// 
/// Each request is validated before
/// being applied. If any request is not valid then the entire request will
/// fail and nothing will be applied.
/// 
/// Some requests have replies to
/// give you some information about how
/// they are applied. The replies will mirror the requests.  For example,
/// if you applied 4 updates and the 3rd one had a reply, then the
/// response will have 2 empty replies, the actual reply, and another empty
/// reply, in that order.
/// 
/// Due to the collaborative nature of spreadsheets, it is not guaranteed that
/// the spreadsheet will reflect exactly your changes after this completes,
/// however it is guaranteed that the updates in the request will be
/// applied together atomically. Your changes may be altered with respect to
/// collaborator changes. If there are no collaborators, the spreadsheet
/// should reflect your changes.
///
/// A builder for the *batchUpdate* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::BatchUpdateSpreadsheetRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchUpdateSpreadsheetRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().batch_update(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetBatchUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: BatchUpdateSpreadsheetRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetBatchUpdateCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetBatchUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchUpdateSpreadsheetResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.batchUpdate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}:batchUpdate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: BatchUpdateSpreadsheetRequest) -> SpreadsheetBatchUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The spreadsheet to apply the updates to.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetBatchUpdateCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetBatchUpdateCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetBatchUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetBatchUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns all developer metadata matching the specified DataFilter.
/// If the provided DataFilter represents a DeveloperMetadataLookup object,
/// this will return all DeveloperMetadata entries selected by it. If the
/// DataFilter represents a location in a spreadsheet, this will return all
/// developer metadata associated with locations intersecting that region.
///
/// A builder for the *developerMetadata.search* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// use sheets4::SearchDeveloperMetadataRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SearchDeveloperMetadataRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().developer_metadata_search(req, "spreadsheetId")
///              .doit();
/// # }
/// ```
pub struct SpreadsheetDeveloperMetadataSearchCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _request: SearchDeveloperMetadataRequest,
    _spreadsheet_id: String,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetDeveloperMetadataSearchCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetDeveloperMetadataSearchCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, SearchDeveloperMetadataResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.developerMetadata.search",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        for &field in ["alt", "spreadsheetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/developerMetadata:search";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn request(mut self, new_value: SearchDeveloperMetadataRequest) -> SpreadsheetDeveloperMetadataSearchCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The ID of the spreadsheet to retrieve metadata from.
    ///
    /// Sets the *spreadsheet id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetDeveloperMetadataSearchCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetDeveloperMetadataSearchCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetDeveloperMetadataSearchCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetDeveloperMetadataSearchCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the developer metadata with the specified ID.
/// The caller must specify the spreadsheet ID and the developer metadata's
/// unique metadataId.
///
/// A builder for the *developerMetadata.get* method supported by a *spreadsheet* resource.
/// It is not used directly, but through a `SpreadsheetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_sheets4 as sheets4;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use sheets4::Sheets;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Sheets::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.spreadsheets().developer_metadata_get("spreadsheetId", -80)
///              .doit();
/// # }
/// ```
pub struct SpreadsheetDeveloperMetadataGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Sheets<C, A>,
    _spreadsheet_id: String,
    _metadata_id: i32,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for SpreadsheetDeveloperMetadataGetCall<'a, C, A> {}

impl<'a, C, A> SpreadsheetDeveloperMetadataGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, DeveloperMetadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut dyn Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "sheets.spreadsheets.developerMetadata.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("spreadsheetId", self._spreadsheet_id.to_string()));
        params.push(("metadataId", self._metadata_id.to_string()));
        for &field in ["alt", "spreadsheetId", "metadataId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v4/spreadsheets/{spreadsheetId}/developerMetadata/{metadataId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Drive.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{spreadsheetId}", "spreadsheetId"), ("{metadataId}", "metadataId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["metadataId", "spreadsheetId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = hyper::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, url.clone())
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();

                        let json_server_error = json::from_str::<JsonServerError>(&json_err).ok();
                        let server_error = json::from_str::<ServerError>(&json_err)
                            .or_else(|_| json::from_str::<ErrorResponse>(&json_err).map(|r| r.error))
                            .ok();

                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
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
    pub fn spreadsheet_id(mut self, new_value: &str) -> SpreadsheetDeveloperMetadataGetCall<'a, C, A> {
        self._spreadsheet_id = new_value.to_string();
        self
    }
    /// The ID of the developer metadata to retrieve.
    ///
    /// Sets the *metadata id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn metadata_id(mut self, new_value: i32) -> SpreadsheetDeveloperMetadataGetCall<'a, C, A> {
        self._metadata_id = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn Delegate) -> SpreadsheetDeveloperMetadataGetCall<'a, C, A> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> SpreadsheetDeveloperMetadataGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Drive`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> SpreadsheetDeveloperMetadataGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


