use super::*;
/// Promotion result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResultImage {
    /// The size of the image, in pixels.
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


