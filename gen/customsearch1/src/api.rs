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




// ########
// HUB ###
// ######

/// Central instance to access all CustomSearchAPI related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_customsearch1 as customsearch1;
/// use customsearch1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().siterestrict_list()
///              .start(81)
///              .sort("vero")
///              .snippet_length(-76)
///              .site_search_filter("invidunt")
///              .site_search("Stet")
///              .search_type("vero")
///              .safe("elitr")
///              .rights("Lorem")
///              .related_site("diam")
///              .q("no")
///              .or_terms("ipsum")
///              .num(-23)
///              .lr("takimata")
///              .low_range("consetetur")
///              .link_site("voluptua.")
///              .img_type("et")
///              .img_size("erat")
///              .img_dominant_color("consetetur")
///              .img_color_type("amet.")
///              .hq("sed")
///              .hl("takimata")
///              .high_range("dolores")
///              .googlehost("gubergren")
///              .gl("et")
///              .filter("accusam")
///              .file_type("voluptua.")
///              .exclude_terms("dolore")
///              .exact_terms("dolore")
///              .date_restrict("dolore")
///              .cx("voluptua.")
///              .cr("amet.")
///              .c2coff("ea")
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
pub struct CustomSearchAPI<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for CustomSearchAPI<S> {}

impl<'a, S> CustomSearchAPI<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> CustomSearchAPI<S> {
        CustomSearchAPI {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://customsearch.googleapis.com/".to_string(),
            _root_url: "https://customsearch.googleapis.com/".to_string(),
        }
    }

    pub fn cse(&'a self) -> CseMethods<'a, S> {
        CseMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://customsearch.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://customsearch.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Promotion result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Promotion {
    /// An array of block objects for this promotion.
    #[serde(rename="bodyLines")]
    
    pub body_lines: Option<Vec<PromotionBodyLines>>,
    /// An abridged version of this search's result URL, e.g. www.example.com.
    #[serde(rename="displayLink")]
    
    pub display_link: Option<String>,
    /// The title of the promotion, in HTML.
    #[serde(rename="htmlTitle")]
    
    pub html_title: Option<String>,
    /// Image belonging to a promotion.
    
    pub image: Option<PromotionImage>,
    /// The URL of the promotion.
    
    pub link: Option<String>,
    /// The title of the promotion.
    
    pub title: Option<String>,
}

impl client::Part for Promotion {}


/// A custom search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Result {
    /// Indicates the ID of Google's cached version of the search result.
    #[serde(rename="cacheId")]
    
    pub cache_id: Option<String>,
    /// An abridged version of this search result’s URL, e.g. www.example.com.
    #[serde(rename="displayLink")]
    
    pub display_link: Option<String>,
    /// The file format of the search result.
    #[serde(rename="fileFormat")]
    
    pub file_format: Option<String>,
    /// The URL displayed after the snippet for each search result.
    #[serde(rename="formattedUrl")]
    
    pub formatted_url: Option<String>,
    /// The HTML-formatted URL displayed after the snippet for each search result.
    #[serde(rename="htmlFormattedUrl")]
    
    pub html_formatted_url: Option<String>,
    /// The snippet of the search result, in HTML.
    #[serde(rename="htmlSnippet")]
    
    pub html_snippet: Option<String>,
    /// The title of the search result, in HTML.
    #[serde(rename="htmlTitle")]
    
    pub html_title: Option<String>,
    /// Image belonging to a custom search result.
    
    pub image: Option<ResultImage>,
    /// A unique identifier for the type of current object. For this API, it is `customsearch#result.`
    
    pub kind: Option<String>,
    /// Encapsulates all information about refinement labels.
    
    pub labels: Option<Vec<ResultLabels>>,
    /// The full URL to which the search result is pointing, e.g. http://www.example.com/foo/bar.
    
    pub link: Option<String>,
    /// The MIME type of the search result.
    
    pub mime: Option<String>,
    /// Contains [PageMap](https://developers.google.com/custom-search/docs/structured_data#pagemaps) information for this search result.
    
    pub pagemap: Option<HashMap<String, json::Value>>,
    /// The snippet of the search result, in plain text.
    
    pub snippet: Option<String>,
    /// The title of the search result, in plain text.
    
    pub title: Option<String>,
}

impl client::Part for Result {}


/// Response to a custom search request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [siterestrict list cse](CseSiterestrictListCall) (response)
/// * [list cse](CseListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Search {
    /// Metadata and refinements associated with the given search engine, including: * The name of the search engine that was used for the query. * A set of [facet objects](https://developers.google.com/custom-search/docs/refinements#create) (refinements) you can use for refining a search.
    
    pub context: Option<HashMap<String, json::Value>>,
    /// The current set of custom search results.
    
    pub items: Option<Vec<Result>>,
    /// Unique identifier for the type of current object. For this API, it is customsearch#search.
    
    pub kind: Option<String>,
    /// The set of [promotions](https://developers.google.com/custom-search/docs/promotions). Present only if the custom search engine's configuration files define any promotions for the given query.
    
    pub promotions: Option<Vec<Promotion>>,
    /// Query metadata for the previous, current, and next pages of results.
    
    pub queries: Option<SearchQueries>,
    /// Metadata about a search operation.
    #[serde(rename="searchInformation")]
    
    pub search_information: Option<SearchSearchInformation>,
    /// Spell correction information for a query.
    
    pub spelling: Option<SearchSpelling>,
    /// OpenSearch template and URL.
    
    pub url: Option<SearchUrl>,
}

impl client::ResponseResult for Search {}


/// Block object belonging to a promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotionBodyLines {
    /// The block object's text in HTML, if it has text.
    #[serde(rename="htmlTitle")]
    
    pub html_title: Option<String>,
    /// The anchor text of the block object's link, if it has a link.
    
    pub link: Option<String>,
    /// The block object's text, if it has text.
    
    pub title: Option<String>,
    /// The URL of the block object's link, if it has one.
    
    pub url: Option<String>,
}

impl client::NestedType for PromotionBodyLines {}
impl client::Part for PromotionBodyLines {}


/// Image belonging to a promotion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PromotionImage {
    /// Image height in pixels.
    
    pub height: Option<i32>,
    /// URL of the image for this promotion link.
    
    pub source: Option<String>,
    /// Image width in pixels.
    
    pub width: Option<i32>,
}

impl client::NestedType for PromotionImage {}
impl client::Part for PromotionImage {}


/// Image belonging to a custom search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultImage {
    /// The size of the image, in bytes.
    #[serde(rename="byteSize")]
    
    pub byte_size: Option<i32>,
    /// A URL pointing to the webpage hosting the image.
    #[serde(rename="contextLink")]
    
    pub context_link: Option<String>,
    /// The height of the image, in pixels.
    
    pub height: Option<i32>,
    /// The height of the thumbnail image, in pixels.
    #[serde(rename="thumbnailHeight")]
    
    pub thumbnail_height: Option<i32>,
    /// A URL to the thumbnail image.
    #[serde(rename="thumbnailLink")]
    
    pub thumbnail_link: Option<String>,
    /// The width of the thumbnail image, in pixels.
    #[serde(rename="thumbnailWidth")]
    
    pub thumbnail_width: Option<i32>,
    /// The width of the image, in pixels.
    
    pub width: Option<i32>,
}

impl client::NestedType for ResultImage {}
impl client::Part for ResultImage {}


/// Refinement label associated with a custom search result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultLabels {
    /// The display name of a refinement label. This is the name you should display in your user interface.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Refinement label and the associated refinement operation.
    
    pub label_with_op: Option<String>,
    /// The name of a refinement label, which you can use to refine searches. Don't display this in your user interface; instead, use displayName.
    
    pub name: Option<String>,
}

impl client::NestedType for ResultLabels {}
impl client::Part for ResultLabels {}


/// Query metadata for the previous, current, and next pages of results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueries {
    /// Metadata representing the next page of results, if applicable.
    #[serde(rename="nextPage")]
    
    pub next_page: Option<Vec<SearchQueriesNextPage>>,
    /// Metadata representing the previous page of results, if applicable.
    #[serde(rename="previousPage")]
    
    pub previous_page: Option<Vec<SearchQueriesPreviousPage>>,
    /// Metadata representing the current request.
    
    pub request: Option<Vec<SearchQueriesRequest>>,
}

impl client::NestedType for SearchQueries {}
impl client::Part for SearchQueries {}


/// Custom search request metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueriesNextPage {
    /// Number of search results returned in this set.
    
    pub count: Option<i32>,
    /// Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) for a list of valid values for this parameter.
    
    pub cr: Option<String>,
    /// The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**.
    
    pub cx: Option<String>,
    /// Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years.
    #[serde(rename="dateRestrict")]
    
    pub date_restrict: Option<String>,
    /// Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled
    #[serde(rename="disableCnTwTranslation")]
    
    pub disable_cn_tw_translation: Option<String>,
    /// Identifies a phrase that all documents in the search results must contain.
    #[serde(rename="exactTerms")]
    
    pub exact_terms: Option<String>,
    /// Identifies a word or phrase that should not appear in any documents in the search results.
    #[serde(rename="excludeTerms")]
    
    pub exclude_terms: Option<String>,
    /// Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287).
    #[serde(rename="fileType")]
    
    pub file_type: Option<String>,
    /// Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results.
    
    pub filter: Option<String>,
    /// Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States.
    
    pub gl: Option<String>,
    /// Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited.
    #[serde(rename="googleHost")]
    
    pub google_host: Option<String>,
    /// Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query.
    #[serde(rename="highRange")]
    
    pub high_range: Option<String>,
    /// Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages.
    
    pub hl: Option<String>,
    /// Appends the specified query terms to the query, as if they were combined with a logical `AND` operator.
    
    pub hq: Option<String>,
    /// Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)
    #[serde(rename="imgColorType")]
    
    pub img_color_type: Option<String>,
    /// Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`
    #[serde(rename="imgDominantColor")]
    
    pub img_dominant_color: Option<String>,
    /// Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)
    #[serde(rename="imgSize")]
    
    pub img_size: Option<String>,
    /// Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)
    #[serde(rename="imgType")]
    
    pub img_type: Option<String>,
    /// The character encoding supported for search requests.
    #[serde(rename="inputEncoding")]
    
    pub input_encoding: Option<String>,
    /// The language of the search results.
    
    pub language: Option<String>,
    /// Specifies that all results should contain a link to a specific URL.
    #[serde(rename="linkSite")]
    
    pub link_site: Option<String>,
    /// Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query.
    #[serde(rename="lowRange")]
    
    pub low_range: Option<String>,
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) query term for this type of query.
    #[serde(rename="orTerms")]
    
    pub or_terms: Option<String>,
    /// The character encoding supported for search results.
    #[serde(rename="outputEncoding")]
    
    pub output_encoding: Option<String>,
    /// Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL.
    #[serde(rename="relatedSite")]
    
    pub related_site: Option<String>,
    /// Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`
    
    pub rights: Option<String>,
    /// Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/json_api_reference#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `"off"`: Disable SafeSearch * `"active"`: Enable SafeSearch
    
    pub safe: Option<String>,
    /// The search terms entered by the user.
    #[serde(rename="searchTerms")]
    
    pub search_terms: Option<String>,
    /// Allowed values are `web` or `image`. If unspecified, results are limited to webpages.
    #[serde(rename="searchType")]
    
    pub search_type: Option<String>,
    /// Restricts results to URLs from a specified site.
    #[serde(rename="siteSearch")]
    
    pub site_search: Option<String>,
    /// Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site
    #[serde(rename="siteSearchFilter")]
    
    pub site_search_filter: Option<String>,
    /// Specifies that results should be sorted according to the specified expression. For example, sort by date.
    
    pub sort: Option<String>,
    /// The index of the current set of search results into the total set of results, where the index of the first result is 1.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The page number of this set of results, where the page length is set by the `count` property.
    #[serde(rename="startPage")]
    
    pub start_page: Option<i32>,
    /// A description of the query.
    
    pub title: Option<String>,
    /// Estimated number of total search results. May not be accurate.
    #[serde(rename="totalResults")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_results: Option<i64>,
}

impl client::NestedType for SearchQueriesNextPage {}
impl client::Part for SearchQueriesNextPage {}


/// Custom search request metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueriesPreviousPage {
    /// Number of search results returned in this set.
    
    pub count: Option<i32>,
    /// Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) for a list of valid values for this parameter.
    
    pub cr: Option<String>,
    /// The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**.
    
    pub cx: Option<String>,
    /// Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years.
    #[serde(rename="dateRestrict")]
    
    pub date_restrict: Option<String>,
    /// Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled
    #[serde(rename="disableCnTwTranslation")]
    
    pub disable_cn_tw_translation: Option<String>,
    /// Identifies a phrase that all documents in the search results must contain.
    #[serde(rename="exactTerms")]
    
    pub exact_terms: Option<String>,
    /// Identifies a word or phrase that should not appear in any documents in the search results.
    #[serde(rename="excludeTerms")]
    
    pub exclude_terms: Option<String>,
    /// Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287).
    #[serde(rename="fileType")]
    
    pub file_type: Option<String>,
    /// Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results.
    
    pub filter: Option<String>,
    /// Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States.
    
    pub gl: Option<String>,
    /// Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited.
    #[serde(rename="googleHost")]
    
    pub google_host: Option<String>,
    /// Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query.
    #[serde(rename="highRange")]
    
    pub high_range: Option<String>,
    /// Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages.
    
    pub hl: Option<String>,
    /// Appends the specified query terms to the query, as if they were combined with a logical `AND` operator.
    
    pub hq: Option<String>,
    /// Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)
    #[serde(rename="imgColorType")]
    
    pub img_color_type: Option<String>,
    /// Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`
    #[serde(rename="imgDominantColor")]
    
    pub img_dominant_color: Option<String>,
    /// Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)
    #[serde(rename="imgSize")]
    
    pub img_size: Option<String>,
    /// Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)
    #[serde(rename="imgType")]
    
    pub img_type: Option<String>,
    /// The character encoding supported for search requests.
    #[serde(rename="inputEncoding")]
    
    pub input_encoding: Option<String>,
    /// The language of the search results.
    
    pub language: Option<String>,
    /// Specifies that all results should contain a link to a specific URL.
    #[serde(rename="linkSite")]
    
    pub link_site: Option<String>,
    /// Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query.
    #[serde(rename="lowRange")]
    
    pub low_range: Option<String>,
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) query term for this type of query.
    #[serde(rename="orTerms")]
    
    pub or_terms: Option<String>,
    /// The character encoding supported for search results.
    #[serde(rename="outputEncoding")]
    
    pub output_encoding: Option<String>,
    /// Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL.
    #[serde(rename="relatedSite")]
    
    pub related_site: Option<String>,
    /// Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`
    
    pub rights: Option<String>,
    /// Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/json_api_reference#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `"off"`: Disable SafeSearch * `"active"`: Enable SafeSearch
    
    pub safe: Option<String>,
    /// The search terms entered by the user.
    #[serde(rename="searchTerms")]
    
    pub search_terms: Option<String>,
    /// Allowed values are `web` or `image`. If unspecified, results are limited to webpages.
    #[serde(rename="searchType")]
    
    pub search_type: Option<String>,
    /// Restricts results to URLs from a specified site.
    #[serde(rename="siteSearch")]
    
    pub site_search: Option<String>,
    /// Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site
    #[serde(rename="siteSearchFilter")]
    
    pub site_search_filter: Option<String>,
    /// Specifies that results should be sorted according to the specified expression. For example, sort by date.
    
    pub sort: Option<String>,
    /// The index of the current set of search results into the total set of results, where the index of the first result is 1.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The page number of this set of results, where the page length is set by the `count` property.
    #[serde(rename="startPage")]
    
    pub start_page: Option<i32>,
    /// A description of the query.
    
    pub title: Option<String>,
    /// Estimated number of total search results. May not be accurate.
    #[serde(rename="totalResults")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_results: Option<i64>,
}

impl client::NestedType for SearchQueriesPreviousPage {}
impl client::Part for SearchQueriesPreviousPage {}


/// Custom search request metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchQueriesRequest {
    /// Number of search results returned in this set.
    
    pub count: Option<i32>,
    /// Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) in the `cr` parameter's value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document's URL. * The geographic location of the web server's IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) for a list of valid values for this parameter.
    
    pub cr: Option<String>,
    /// The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**.
    
    pub cx: Option<String>,
    /// Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years.
    #[serde(rename="dateRestrict")]
    
    pub date_restrict: Option<String>,
    /// Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled
    #[serde(rename="disableCnTwTranslation")]
    
    pub disable_cn_tw_translation: Option<String>,
    /// Identifies a phrase that all documents in the search results must contain.
    #[serde(rename="exactTerms")]
    
    pub exact_terms: Option<String>,
    /// Identifies a word or phrase that should not appear in any documents in the search results.
    #[serde(rename="excludeTerms")]
    
    pub exclude_terms: Option<String>,
    /// Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google's [file type FAQ](https://support.google.com/webmasters/answer/35287).
    #[serde(rename="fileType")]
    
    pub file_type: Option<String>,
    /// Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google's search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results.
    
    pub filter: Option<String>,
    /// Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States.
    
    pub gl: Option<String>,
    /// Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited.
    #[serde(rename="googleHost")]
    
    pub google_host: Option<String>,
    /// Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query.
    #[serde(rename="highRange")]
    
    pub high_range: Option<String>,
    /// Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages.
    
    pub hl: Option<String>,
    /// Appends the specified query terms to the query, as if they were combined with a logical `AND` operator.
    
    pub hq: Option<String>,
    /// Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)
    #[serde(rename="imgColorType")]
    
    pub img_color_type: Option<String>,
    /// Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`
    #[serde(rename="imgDominantColor")]
    
    pub img_dominant_color: Option<String>,
    /// Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)
    #[serde(rename="imgSize")]
    
    pub img_size: Option<String>,
    /// Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)
    #[serde(rename="imgType")]
    
    pub img_type: Option<String>,
    /// The character encoding supported for search requests.
    #[serde(rename="inputEncoding")]
    
    pub input_encoding: Option<String>,
    /// The language of the search results.
    
    pub language: Option<String>,
    /// Specifies that all results should contain a link to a specific URL.
    #[serde(rename="linkSite")]
    
    pub link_site: Option<String>,
    /// Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query.
    #[serde(rename="lowRange")]
    
    pub low_range: Option<String>,
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) query term for this type of query.
    #[serde(rename="orTerms")]
    
    pub or_terms: Option<String>,
    /// The character encoding supported for search results.
    #[serde(rename="outputEncoding")]
    
    pub output_encoding: Option<String>,
    /// Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL.
    #[serde(rename="relatedSite")]
    
    pub related_site: Option<String>,
    /// Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`
    
    pub rights: Option<String>,
    /// Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/json_api_reference#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `"off"`: Disable SafeSearch * `"active"`: Enable SafeSearch
    
    pub safe: Option<String>,
    /// The search terms entered by the user.
    #[serde(rename="searchTerms")]
    
    pub search_terms: Option<String>,
    /// Allowed values are `web` or `image`. If unspecified, results are limited to webpages.
    #[serde(rename="searchType")]
    
    pub search_type: Option<String>,
    /// Restricts results to URLs from a specified site.
    #[serde(rename="siteSearch")]
    
    pub site_search: Option<String>,
    /// Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site
    #[serde(rename="siteSearchFilter")]
    
    pub site_search_filter: Option<String>,
    /// Specifies that results should be sorted according to the specified expression. For example, sort by date.
    
    pub sort: Option<String>,
    /// The index of the current set of search results into the total set of results, where the index of the first result is 1.
    #[serde(rename="startIndex")]
    
    pub start_index: Option<i32>,
    /// The page number of this set of results, where the page length is set by the `count` property.
    #[serde(rename="startPage")]
    
    pub start_page: Option<i32>,
    /// A description of the query.
    
    pub title: Option<String>,
    /// Estimated number of total search results. May not be accurate.
    #[serde(rename="totalResults")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_results: Option<i64>,
}

impl client::NestedType for SearchQueriesRequest {}
impl client::Part for SearchQueriesRequest {}


/// Metadata about a search operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchSearchInformation {
    /// The time taken for the server to return search results, formatted according to locale style.
    #[serde(rename="formattedSearchTime")]
    
    pub formatted_search_time: Option<String>,
    /// The total number of search results, formatted according to locale style.
    #[serde(rename="formattedTotalResults")]
    
    pub formatted_total_results: Option<String>,
    /// The time taken for the server to return search results.
    #[serde(rename="searchTime")]
    
    pub search_time: Option<f64>,
    /// The total number of search results returned by the query.
    #[serde(rename="totalResults")]
    
    pub total_results: Option<String>,
}

impl client::NestedType for SearchSearchInformation {}
impl client::Part for SearchSearchInformation {}


/// Spell correction information for a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchSpelling {
    /// The corrected query.
    #[serde(rename="correctedQuery")]
    
    pub corrected_query: Option<String>,
    /// The corrected query, formatted in HTML.
    #[serde(rename="htmlCorrectedQuery")]
    
    pub html_corrected_query: Option<String>,
}

impl client::NestedType for SearchSpelling {}
impl client::Part for SearchSpelling {}


/// OpenSearch template and URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchUrl {
    /// The actual [OpenSearch template](http://www.opensearch.org/specifications/opensearch/1.1#opensearch_url_template_syntax) for this API.
    
    pub template: Option<String>,
    /// The MIME type of the OpenSearch URL template for the Custom Search JSON API.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for SearchUrl {}
impl client::Part for SearchUrl {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *cse* resources.
/// It is not used directly, but through the [`CustomSearchAPI`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_customsearch1 as customsearch1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `siterestrict_list(...)`
/// // to build up your call.
/// let rb = hub.cse();
/// # }
/// ```
pub struct CseMethods<'a, S>
    where S: 'a {

    hub: &'a CustomSearchAPI<S>,
}

impl<'a, S> client::MethodsBuilder for CseMethods<'a, S> {}

impl<'a, S> CseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata about the search performed, metadata about the engine used for the search, and the search results. Uses a small set of url patterns.
    pub fn siterestrict_list(&self) -> CseSiterestrictListCall<'a, S> {
        CseSiterestrictListCall {
            hub: self.hub,
            _start: Default::default(),
            _sort: Default::default(),
            _snippet_length: Default::default(),
            _site_search_filter: Default::default(),
            _site_search: Default::default(),
            _search_type: Default::default(),
            _safe: Default::default(),
            _rights: Default::default(),
            _related_site: Default::default(),
            _q: Default::default(),
            _or_terms: Default::default(),
            _num: Default::default(),
            _lr: Default::default(),
            _low_range: Default::default(),
            _link_site: Default::default(),
            _img_type: Default::default(),
            _img_size: Default::default(),
            _img_dominant_color: Default::default(),
            _img_color_type: Default::default(),
            _hq: Default::default(),
            _hl: Default::default(),
            _high_range: Default::default(),
            _googlehost: Default::default(),
            _gl: Default::default(),
            _filter: Default::default(),
            _file_type: Default::default(),
            _exclude_terms: Default::default(),
            _exact_terms: Default::default(),
            _date_restrict: Default::default(),
            _cx: Default::default(),
            _cr: Default::default(),
            _c2coff: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata about the search performed, metadata about the engine used for the search, and the search results.
    pub fn list(&self) -> CseListCall<'a, S> {
        CseListCall {
            hub: self.hub,
            _start: Default::default(),
            _sort: Default::default(),
            _snippet_length: Default::default(),
            _site_search_filter: Default::default(),
            _site_search: Default::default(),
            _search_type: Default::default(),
            _safe: Default::default(),
            _rights: Default::default(),
            _related_site: Default::default(),
            _q: Default::default(),
            _or_terms: Default::default(),
            _num: Default::default(),
            _lr: Default::default(),
            _low_range: Default::default(),
            _link_site: Default::default(),
            _img_type: Default::default(),
            _img_size: Default::default(),
            _img_dominant_color: Default::default(),
            _img_color_type: Default::default(),
            _hq: Default::default(),
            _hl: Default::default(),
            _high_range: Default::default(),
            _googlehost: Default::default(),
            _gl: Default::default(),
            _filter: Default::default(),
            _file_type: Default::default(),
            _exclude_terms: Default::default(),
            _exact_terms: Default::default(),
            _date_restrict: Default::default(),
            _cx: Default::default(),
            _cr: Default::default(),
            _c2coff: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Returns metadata about the search performed, metadata about the engine used for the search, and the search results. Uses a small set of url patterns.
///
/// A builder for the *siterestrict.list* method supported by a *cse* resource.
/// It is not used directly, but through a [`CseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_customsearch1 as customsearch1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().siterestrict_list()
///              .start(6)
///              .sort("Lorem")
///              .snippet_length(-38)
///              .site_search_filter("no")
///              .site_search("est")
///              .search_type("At")
///              .safe("sed")
///              .rights("sit")
///              .related_site("et")
///              .q("tempor")
///              .or_terms("aliquyam")
///              .num(-5)
///              .lr("et")
///              .low_range("sanctus")
///              .link_site("Lorem")
///              .img_type("est")
///              .img_size("sed")
///              .img_dominant_color("diam")
///              .img_color_type("dolores")
///              .hq("dolores")
///              .hl("et")
///              .high_range("sed")
///              .googlehost("no")
///              .gl("et")
///              .filter("elitr")
///              .file_type("sed")
///              .exclude_terms("no")
///              .exact_terms("nonumy")
///              .date_restrict("At")
///              .cx("sadipscing")
///              .cr("aliquyam")
///              .c2coff("dolores")
///              .doit().await;
/// # }
/// ```
pub struct CseSiterestrictListCall<'a, S>
    where S: 'a {

    hub: &'a CustomSearchAPI<S>,
    _start: Option<u32>,
    _sort: Option<String>,
    _snippet_length: Option<i32>,
    _site_search_filter: Option<String>,
    _site_search: Option<String>,
    _search_type: Option<String>,
    _safe: Option<String>,
    _rights: Option<String>,
    _related_site: Option<String>,
    _q: Option<String>,
    _or_terms: Option<String>,
    _num: Option<i32>,
    _lr: Option<String>,
    _low_range: Option<String>,
    _link_site: Option<String>,
    _img_type: Option<String>,
    _img_size: Option<String>,
    _img_dominant_color: Option<String>,
    _img_color_type: Option<String>,
    _hq: Option<String>,
    _hl: Option<String>,
    _high_range: Option<String>,
    _googlehost: Option<String>,
    _gl: Option<String>,
    _filter: Option<String>,
    _file_type: Option<String>,
    _exclude_terms: Option<String>,
    _exact_terms: Option<String>,
    _date_restrict: Option<String>,
    _cx: Option<String>,
    _cr: Option<String>,
    _c2coff: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CseSiterestrictListCall<'a, S> {}

impl<'a, S> CseSiterestrictListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "search.cse.siterestrict.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "start", "sort", "snippetLength", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "q", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(34 + self._additional_params.len());
        if let Some(value) = self._start.as_ref() {
            params.push("start", value.to_string());
        }
        if let Some(value) = self._sort.as_ref() {
            params.push("sort", value);
        }
        if let Some(value) = self._snippet_length.as_ref() {
            params.push("snippetLength", value.to_string());
        }
        if let Some(value) = self._site_search_filter.as_ref() {
            params.push("siteSearchFilter", value);
        }
        if let Some(value) = self._site_search.as_ref() {
            params.push("siteSearch", value);
        }
        if let Some(value) = self._search_type.as_ref() {
            params.push("searchType", value);
        }
        if let Some(value) = self._safe.as_ref() {
            params.push("safe", value);
        }
        if let Some(value) = self._rights.as_ref() {
            params.push("rights", value);
        }
        if let Some(value) = self._related_site.as_ref() {
            params.push("relatedSite", value);
        }
        if let Some(value) = self._q.as_ref() {
            params.push("q", value);
        }
        if let Some(value) = self._or_terms.as_ref() {
            params.push("orTerms", value);
        }
        if let Some(value) = self._num.as_ref() {
            params.push("num", value.to_string());
        }
        if let Some(value) = self._lr.as_ref() {
            params.push("lr", value);
        }
        if let Some(value) = self._low_range.as_ref() {
            params.push("lowRange", value);
        }
        if let Some(value) = self._link_site.as_ref() {
            params.push("linkSite", value);
        }
        if let Some(value) = self._img_type.as_ref() {
            params.push("imgType", value);
        }
        if let Some(value) = self._img_size.as_ref() {
            params.push("imgSize", value);
        }
        if let Some(value) = self._img_dominant_color.as_ref() {
            params.push("imgDominantColor", value);
        }
        if let Some(value) = self._img_color_type.as_ref() {
            params.push("imgColorType", value);
        }
        if let Some(value) = self._hq.as_ref() {
            params.push("hq", value);
        }
        if let Some(value) = self._hl.as_ref() {
            params.push("hl", value);
        }
        if let Some(value) = self._high_range.as_ref() {
            params.push("highRange", value);
        }
        if let Some(value) = self._googlehost.as_ref() {
            params.push("googlehost", value);
        }
        if let Some(value) = self._gl.as_ref() {
            params.push("gl", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._file_type.as_ref() {
            params.push("fileType", value);
        }
        if let Some(value) = self._exclude_terms.as_ref() {
            params.push("excludeTerms", value);
        }
        if let Some(value) = self._exact_terms.as_ref() {
            params.push("exactTerms", value);
        }
        if let Some(value) = self._date_restrict.as_ref() {
            params.push("dateRestrict", value);
        }
        if let Some(value) = self._cx.as_ref() {
            params.push("cx", value);
        }
        if let Some(value) = self._cr.as_ref() {
            params.push("cr", value);
        }
        if let Some(value) = self._c2coff.as_ref() {
            params.push("c2coff", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "customsearch/v1/siterestrict";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = params.parse_with_url(&url);



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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


    /// The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10.
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseSiterestrictListCall<'a, S> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute).
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Optional. Maximum length of snippet text, in characters, to be returned with results. Note: this feature is limited to specific engines. * Valid values are integers between 161 and 1000, inclusive.
    ///
    /// Sets the *snippet length* query property to the given value.
    pub fn snippet_length(mut self, new_value: i32) -> CseSiterestrictListCall<'a, S> {
        self._snippet_length = Some(new_value);
        self
    }
    /// Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `"e"`: exclude * `"i"`: include
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below).
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `"image"`: custom image search.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level. Acceptable values are: * `"active"`: Enables SafeSearch filtering. * `"off"`: Disables SafeSearch filtering. (default)
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration).
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Deprecated.
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Query
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms.
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return. * Valid values are integers between 1 and 10, inclusive.
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: i32) -> CseSiterestrictListCall<'a, S> {
        self._num = Some(new_value);
        self
    }
    /// Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `"lang_ar"`: Arabic * `"lang_bg"`: Bulgarian * `"lang_ca"`: Catalan * `"lang_cs"`: Czech * `"lang_da"`: Danish * `"lang_de"`: German * `"lang_el"`: Greek * `"lang_en"`: English * `"lang_es"`: Spanish * `"lang_et"`: Estonian * `"lang_fi"`: Finnish * `"lang_fr"`: French * `"lang_hr"`: Croatian * `"lang_hu"`: Hungarian * `"lang_id"`: Indonesian * `"lang_is"`: Icelandic * `"lang_it"`: Italian * `"lang_iw"`: Hebrew * `"lang_ja"`: Japanese * `"lang_ko"`: Korean * `"lang_lt"`: Lithuanian * `"lang_lv"`: Latvian * `"lang_nl"`: Dutch * `"lang_no"`: Norwegian * `"lang_pl"`: Polish * `"lang_pt"`: Portuguese * `"lang_ro"`: Romanian * `"lang_ru"`: Russian * `"lang_sk"`: Slovak * `"lang_sl"`: Slovenian * `"lang_sr"`: Serbian * `"lang_sv"`: Swedish * `"lang_tr"`: Turkish * `"lang_zh-CN"`: Chinese (Simplified) * `"lang_zh-TW"`: Chinese (Traditional)
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular URL.
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type. Acceptable values are: * `"clipart"` * `"face"` * `"lineart"` * `"stock"` * `"photo"` * `"animated"`
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size. Acceptable values are: * `"huge"` * `"icon"` * `"large"` * `"medium"` * `"small"` * `"xlarge"` * `"xxlarge"`
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color. Acceptable values are: * `"black"` * `"blue"` * `"brown"` * `"gray"` * `"green"` * `"orange"` * `"pink"` * `"purple"` * `"red"` * `"teal"` * `"white"` * `"yellow"`
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `"color"` * `"gray"` * `"mono"`: black and white * `"trans"`: transparent background
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the specified query terms to the query, as if they were combined with a logical AND operator.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// **Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google's search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287).
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the search results.
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain.
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years.
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The Programmable Search Engine ID to use for this request.
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#booleanOperators) in the cr parameter's value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document's URL * the geographic location of the Web server's IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) page for a list of valid values for this parameter.
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)
    ///
    /// Sets the *c2coff* query property to the given value.
    pub fn c2coff(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._c2coff = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CseSiterestrictListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseSiterestrictListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns metadata about the search performed, metadata about the engine used for the search, and the search results.
///
/// A builder for the *list* method supported by a *cse* resource.
/// It is not used directly, but through a [`CseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_customsearch1 as customsearch1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().list()
///              .start(6)
///              .sort("erat")
///              .snippet_length(-82)
///              .site_search_filter("amet")
///              .site_search("est")
///              .search_type("et")
///              .safe("sea")
///              .rights("consetetur")
///              .related_site("consetetur")
///              .q("Stet")
///              .or_terms("est")
///              .num(-82)
///              .lr("elitr")
///              .low_range("duo")
///              .link_site("diam")
///              .img_type("est")
///              .img_size("sit")
///              .img_dominant_color("sed")
///              .img_color_type("eos")
///              .hq("Lorem")
///              .hl("ea")
///              .high_range("Stet")
///              .googlehost("dolores")
///              .gl("eos")
///              .filter("et")
///              .file_type("sea")
///              .exclude_terms("et")
///              .exact_terms("At")
///              .date_restrict("dolore")
///              .cx("eirmod")
///              .cr("Lorem")
///              .c2coff("accusam")
///              .doit().await;
/// # }
/// ```
pub struct CseListCall<'a, S>
    where S: 'a {

    hub: &'a CustomSearchAPI<S>,
    _start: Option<u32>,
    _sort: Option<String>,
    _snippet_length: Option<i32>,
    _site_search_filter: Option<String>,
    _site_search: Option<String>,
    _search_type: Option<String>,
    _safe: Option<String>,
    _rights: Option<String>,
    _related_site: Option<String>,
    _q: Option<String>,
    _or_terms: Option<String>,
    _num: Option<i32>,
    _lr: Option<String>,
    _low_range: Option<String>,
    _link_site: Option<String>,
    _img_type: Option<String>,
    _img_size: Option<String>,
    _img_dominant_color: Option<String>,
    _img_color_type: Option<String>,
    _hq: Option<String>,
    _hl: Option<String>,
    _high_range: Option<String>,
    _googlehost: Option<String>,
    _gl: Option<String>,
    _filter: Option<String>,
    _file_type: Option<String>,
    _exclude_terms: Option<String>,
    _exact_terms: Option<String>,
    _date_restrict: Option<String>,
    _cx: Option<String>,
    _cr: Option<String>,
    _c2coff: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CseListCall<'a, S> {}

impl<'a, S> CseListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "search.cse.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "start", "sort", "snippetLength", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "q", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(34 + self._additional_params.len());
        if let Some(value) = self._start.as_ref() {
            params.push("start", value.to_string());
        }
        if let Some(value) = self._sort.as_ref() {
            params.push("sort", value);
        }
        if let Some(value) = self._snippet_length.as_ref() {
            params.push("snippetLength", value.to_string());
        }
        if let Some(value) = self._site_search_filter.as_ref() {
            params.push("siteSearchFilter", value);
        }
        if let Some(value) = self._site_search.as_ref() {
            params.push("siteSearch", value);
        }
        if let Some(value) = self._search_type.as_ref() {
            params.push("searchType", value);
        }
        if let Some(value) = self._safe.as_ref() {
            params.push("safe", value);
        }
        if let Some(value) = self._rights.as_ref() {
            params.push("rights", value);
        }
        if let Some(value) = self._related_site.as_ref() {
            params.push("relatedSite", value);
        }
        if let Some(value) = self._q.as_ref() {
            params.push("q", value);
        }
        if let Some(value) = self._or_terms.as_ref() {
            params.push("orTerms", value);
        }
        if let Some(value) = self._num.as_ref() {
            params.push("num", value.to_string());
        }
        if let Some(value) = self._lr.as_ref() {
            params.push("lr", value);
        }
        if let Some(value) = self._low_range.as_ref() {
            params.push("lowRange", value);
        }
        if let Some(value) = self._link_site.as_ref() {
            params.push("linkSite", value);
        }
        if let Some(value) = self._img_type.as_ref() {
            params.push("imgType", value);
        }
        if let Some(value) = self._img_size.as_ref() {
            params.push("imgSize", value);
        }
        if let Some(value) = self._img_dominant_color.as_ref() {
            params.push("imgDominantColor", value);
        }
        if let Some(value) = self._img_color_type.as_ref() {
            params.push("imgColorType", value);
        }
        if let Some(value) = self._hq.as_ref() {
            params.push("hq", value);
        }
        if let Some(value) = self._hl.as_ref() {
            params.push("hl", value);
        }
        if let Some(value) = self._high_range.as_ref() {
            params.push("highRange", value);
        }
        if let Some(value) = self._googlehost.as_ref() {
            params.push("googlehost", value);
        }
        if let Some(value) = self._gl.as_ref() {
            params.push("gl", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._file_type.as_ref() {
            params.push("fileType", value);
        }
        if let Some(value) = self._exclude_terms.as_ref() {
            params.push("excludeTerms", value);
        }
        if let Some(value) = self._exact_terms.as_ref() {
            params.push("exactTerms", value);
        }
        if let Some(value) = self._date_restrict.as_ref() {
            params.push("dateRestrict", value);
        }
        if let Some(value) = self._cx.as_ref() {
            params.push("cx", value);
        }
        if let Some(value) = self._cr.as_ref() {
            params.push("cr", value);
        }
        if let Some(value) = self._c2coff.as_ref() {
            params.push("c2coff", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "customsearch/v1";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }


        let url = params.parse_with_url(&url);



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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


    /// The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10.
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseListCall<'a, S> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute).
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Optional. Maximum length of snippet text, in characters, to be returned with results. Note: this feature is limited to specific engines. * Valid values are integers between 161 and 1000, inclusive.
    ///
    /// Sets the *snippet length* query property to the given value.
    pub fn snippet_length(mut self, new_value: i32) -> CseListCall<'a, S> {
        self._snippet_length = Some(new_value);
        self
    }
    /// Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `"e"`: exclude * `"i"`: include
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below).
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `"image"`: custom image search.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level. Acceptable values are: * `"active"`: Enables SafeSearch filtering. * `"off"`: Disables SafeSearch filtering. (default)
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration).
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Deprecated.
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Query
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms.
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return. * Valid values are integers between 1 and 10, inclusive.
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: i32) -> CseListCall<'a, S> {
        self._num = Some(new_value);
        self
    }
    /// Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `"lang_ar"`: Arabic * `"lang_bg"`: Bulgarian * `"lang_ca"`: Catalan * `"lang_cs"`: Czech * `"lang_da"`: Danish * `"lang_de"`: German * `"lang_el"`: Greek * `"lang_en"`: English * `"lang_es"`: Spanish * `"lang_et"`: Estonian * `"lang_fi"`: Finnish * `"lang_fr"`: French * `"lang_hr"`: Croatian * `"lang_hu"`: Hungarian * `"lang_id"`: Indonesian * `"lang_is"`: Icelandic * `"lang_it"`: Italian * `"lang_iw"`: Hebrew * `"lang_ja"`: Japanese * `"lang_ko"`: Korean * `"lang_lt"`: Lithuanian * `"lang_lv"`: Latvian * `"lang_nl"`: Dutch * `"lang_no"`: Norwegian * `"lang_pl"`: Polish * `"lang_pt"`: Portuguese * `"lang_ro"`: Romanian * `"lang_ru"`: Russian * `"lang_sk"`: Slovak * `"lang_sl"`: Slovenian * `"lang_sr"`: Serbian * `"lang_sv"`: Swedish * `"lang_tr"`: Turkish * `"lang_zh-CN"`: Chinese (Simplified) * `"lang_zh-TW"`: Chinese (Traditional)
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular URL.
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type. Acceptable values are: * `"clipart"` * `"face"` * `"lineart"` * `"stock"` * `"photo"` * `"animated"`
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size. Acceptable values are: * `"huge"` * `"icon"` * `"large"` * `"medium"` * `"small"` * `"xlarge"` * `"xxlarge"`
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color. Acceptable values are: * `"black"` * `"blue"` * `"brown"` * `"gray"` * `"green"` * `"orange"` * `"pink"` * `"purple"` * `"red"` * `"teal"` * `"white"` * `"yellow"`
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `"color"` * `"gray"` * `"mono"`: black and white * `"trans"`: transparent background
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the specified query terms to the query, as if they were combined with a logical AND operator.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// **Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google's search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287).
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the search results.
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain.
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years.
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The Programmable Search Engine ID to use for this request.
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#booleanOperators) in the cr parameter's value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document's URL * the geographic location of the Web server's IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) page for a list of valid values for this parameter.
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)
    ///
    /// Sets the *c2coff* query property to the given value.
    pub fn c2coff(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._c2coff = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CseListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


