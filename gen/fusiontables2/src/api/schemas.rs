use super::*;
/// Specifies the minimum and maximum values, the color, opacity, icon and weight of a bucket within a StyleSetting.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// Color of line or the interior of a polygon in #RRGGBB format.
    
    pub color: Option<String>,
    /// Icon name used for a point.
    
    pub icon: Option<String>,
    /// Maximum value in the selected column for a row to be styled according to the bucket color, opacity, icon, or weight.
    
    pub max: Option<f64>,
    /// Minimum value in the selected column for a row to be styled according to the bucket color, opacity, icon, or weight.
    
    pub min: Option<f64>,
    /// Opacity of the color: 0.0 (transparent) to 1.0 (opaque).
    
    pub opacity: Option<f64>,
    /// Width of a line (in pixels).
    
    pub weight: Option<i32>,
}

impl client::Part for Bucket {}


/// Specifies the details of a column in a table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get column](ColumnGetCall) (response)
/// * [insert column](ColumnInsertCall) (request|response)
/// * [patch column](ColumnPatchCall) (request|response)
/// * [update column](ColumnUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Column {
    /// Identifier of the base column. If present, this column is derived from the specified base column.
    #[serde(rename="baseColumn")]
    
    pub base_column: Option<ColumnBaseColumn>,
    /// Identifier for the column.
    #[serde(rename="columnId")]
    
    pub column_id: Option<i32>,
    /// JSON schema for interpreting JSON in this column.
    #[serde(rename="columnJsonSchema")]
    
    pub column_json_schema: Option<String>,
    /// JSON object containing custom column properties.
    #[serde(rename="columnPropertiesJson")]
    
    pub column_properties_json: Option<String>,
    /// Column description.
    
    pub description: Option<String>,
    /// Format pattern.
    /// Acceptable values are DT_DATE_MEDIUMe.g Dec 24, 2008 DT_DATE_SHORTfor example 12/24/08 DT_DATE_TIME_MEDIUMfor example Dec 24, 2008 8:30:45 PM DT_DATE_TIME_SHORTfor example 12/24/08 8:30 PM DT_DAY_MONTH_2_DIGIT_YEARfor example 24/12/08 DT_DAY_MONTH_2_DIGIT_YEAR_TIMEfor example 24/12/08 20:30 DT_DAY_MONTH_2_DIGIT_YEAR_TIME_MERIDIANfor example 24/12/08 8:30 PM DT_DAY_MONTH_4_DIGIT_YEARfor example 24/12/2008 DT_DAY_MONTH_4_DIGIT_YEAR_TIMEfor example 24/12/2008 20:30 DT_DAY_MONTH_4_DIGIT_YEAR_TIME_MERIDIANfor example 24/12/2008 8:30 PM DT_ISO_YEAR_MONTH_DAYfor example 2008-12-24 DT_ISO_YEAR_MONTH_DAY_TIMEfor example 2008-12-24 20:30:45 DT_MONTH_DAY_4_DIGIT_YEARfor example 12/24/2008 DT_TIME_LONGfor example 8:30:45 PM UTC-6 DT_TIME_MEDIUMfor example 8:30:45 PM DT_TIME_SHORTfor example 8:30 PM DT_YEAR_ONLYfor example 2008 HIGHLIGHT_UNTYPED_CELLSHighlight cell data that does not match the data type NONENo formatting (default) NUMBER_CURRENCYfor example $1234.56 NUMBER_DEFAULTfor example 1,234.56 NUMBER_INTEGERfor example 1235 NUMBER_NO_SEPARATORfor example 1234.56 NUMBER_PERCENTfor example 123,456% NUMBER_SCIENTIFICfor example 1E3 STRING_EIGHT_LINE_IMAGEDisplays thumbnail images as tall as eight lines of text STRING_FOUR_LINE_IMAGEDisplays thumbnail images as tall as four lines of text STRING_JSON_TEXTAllows editing of text as JSON in UI STRING_JSON_LISTAllows editing of text as a JSON list in UI STRING_LINKTreats cell as a link (must start with http:// or https://) STRING_ONE_LINE_IMAGEDisplays thumbnail images as tall as one line of text STRING_VIDEO_OR_MAPDisplay a video or map thumbnail
    #[serde(rename="formatPattern")]
    
    pub format_pattern: Option<String>,
    /// Column graph predicate.
    /// Used to map table to graph data model (subject,predicate,object)
    /// See W3C Graph-based Data Model.
    #[serde(rename="graphPredicate")]
    
    pub graph_predicate: Option<String>,
    /// The kind of item this is. For a column, this is always fusiontables#column.
    
    pub kind: Option<String>,
    /// Name of the column.
    
    pub name: Option<String>,
    /// Type of the column.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// List of valid values used to validate data and supply a drop-down list of values in the web application.
    #[serde(rename="validValues")]
    
    pub valid_values: Option<Vec<String>>,
    /// If true, data entered via the web application is validated.
    #[serde(rename="validateData")]
    
    pub validate_data: Option<bool>,
}

impl client::RequestValue for Column {}
impl client::ResponseResult for Column {}


/// Represents a list of columns in a table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list column](ColumnListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnList {
    /// List of all requested columns.
    
    pub items: Option<Vec<Column>>,
    /// The kind of item this is. For a column list, this is always fusiontables#columnList.
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. No token is displayed if there are no more pages left.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of columns for the table.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for ColumnList {}


/// Represents an import request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [import rows table](TableImportRowCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Import {
    /// The kind of item this is. For an import, this is always fusiontables#import.
    
    pub kind: Option<String>,
    /// The number of rows received from the import request.
    #[serde(rename="numRowsReceived")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_rows_received: Option<i64>,
}

impl client::ResponseResult for Import {}


/// Represents a LineStyle within a StyleSetting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LineStyle {
    /// Color of the line in #RRGGBB format.
    #[serde(rename="strokeColor")]
    
    pub stroke_color: Option<String>,
    /// Column-value, gradient or buckets styler that is used to determine the line color and opacity.
    #[serde(rename="strokeColorStyler")]
    
    pub stroke_color_styler: Option<StyleFunction>,
    /// Opacity of the line : 0.0 (transparent) to 1.0 (opaque).
    #[serde(rename="strokeOpacity")]
    
    pub stroke_opacity: Option<f64>,
    /// Width of the line in pixels.
    #[serde(rename="strokeWeight")]
    
    pub stroke_weight: Option<i32>,
    /// Column-value or bucket styler that is used to determine the width of the line.
    #[serde(rename="strokeWeightStyler")]
    
    pub stroke_weight_styler: Option<StyleFunction>,
}

impl client::Part for LineStyle {}


/// Represents a PointStyle within a StyleSetting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PointStyle {
    /// Name of the icon. Use values defined in http://www.google.com/fusiontables/DataSource?dsrcid=308519
    #[serde(rename="iconName")]
    
    pub icon_name: Option<String>,
    /// Column or a bucket value from which the icon name is to be determined.
    #[serde(rename="iconStyler")]
    
    pub icon_styler: Option<StyleFunction>,
}

impl client::Part for PointStyle {}


/// Represents a PolygonStyle within a StyleSetting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolygonStyle {
    /// Color of the interior of the polygon in #RRGGBB format.
    #[serde(rename="fillColor")]
    
    pub fill_color: Option<String>,
    /// Column-value, gradient, or bucket styler that is used to determine the interior color and opacity of the polygon.
    #[serde(rename="fillColorStyler")]
    
    pub fill_color_styler: Option<StyleFunction>,
    /// Opacity of the interior of the polygon: 0.0 (transparent) to 1.0 (opaque).
    #[serde(rename="fillOpacity")]
    
    pub fill_opacity: Option<f64>,
    /// Color of the polygon border in #RRGGBB format.
    #[serde(rename="strokeColor")]
    
    pub stroke_color: Option<String>,
    /// Column-value, gradient or buckets styler that is used to determine the border color and opacity.
    #[serde(rename="strokeColorStyler")]
    
    pub stroke_color_styler: Option<StyleFunction>,
    /// Opacity of the polygon border: 0.0 (transparent) to 1.0 (opaque).
    #[serde(rename="strokeOpacity")]
    
    pub stroke_opacity: Option<f64>,
    /// Width of the polyon border in pixels.
    #[serde(rename="strokeWeight")]
    
    pub stroke_weight: Option<i32>,
    /// Column-value or bucket styler that is used to determine the width of the polygon border.
    #[serde(rename="strokeWeightStyler")]
    
    pub stroke_weight_styler: Option<StyleFunction>,
}

impl client::Part for PolygonStyle {}


/// Represents a response to a SQL statement.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [sql query](QuerySqlCall) (response)
/// * [sql get query](QuerySqlGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sqlresponse {
    /// Columns in the table.
    
    pub columns: Option<Vec<String>>,
    /// The kind of item this is. For responses to SQL queries, this is always fusiontables#sqlresponse.
    
    pub kind: Option<String>,
    /// The rows in the table. For each cell we print out whatever cell value (e.g., numeric, string) exists. Thus it is important that each cell contains only one value.
    
    pub rows: Option<Vec<Vec<json::Value>>>,
}

impl client::ResponseResult for Sqlresponse {}


/// Represents a StyleFunction within a StyleSetting
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StyleFunction {
    /// Bucket function that assigns a style based on the range a column value falls into.
    
    pub buckets: Option<Vec<Bucket>>,
    /// Name of the column whose value is used in the style.
    #[serde(rename="columnName")]
    
    pub column_name: Option<String>,
    /// Gradient function that interpolates a range of colors based on column value.
    
    pub gradient: Option<StyleFunctionGradient>,
    /// Stylers can be one of three kinds: "fusiontables#fromColumn if the column value is to be used as is, i.e., the column values can have colors in #RRGGBBAA format or integer line widths or icon names; fusiontables#gradient if the styling of the row is to be based on applying the gradient function on the column value; or fusiontables#buckets if the styling is to based on the bucket into which the the column value falls.
    
    pub kind: Option<String>,
}

impl client::Part for StyleFunction {}


/// Represents a complete StyleSettings object. The primary key is a combination of the tableId and a styleId.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get style](StyleGetCall) (response)
/// * [insert style](StyleInsertCall) (request|response)
/// * [patch style](StylePatchCall) (request|response)
/// * [update style](StyleUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StyleSetting {
    /// The kind of item this is. A StyleSetting contains the style definitions for points, lines, and polygons in a table. Since a table can have any one or all of them, a style definition can have point, line and polygon style definitions.
    
    pub kind: Option<String>,
    /// Style definition for points in the table.
    #[serde(rename="markerOptions")]
    
    pub marker_options: Option<PointStyle>,
    /// Optional name for the style setting.
    
    pub name: Option<String>,
    /// Style definition for polygons in the table.
    #[serde(rename="polygonOptions")]
    
    pub polygon_options: Option<PolygonStyle>,
    /// Style definition for lines in the table.
    #[serde(rename="polylineOptions")]
    
    pub polyline_options: Option<LineStyle>,
    /// Identifier for the style setting (unique only within tables).
    #[serde(rename="styleId")]
    
    pub style_id: Option<i32>,
    /// Identifier for the table.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::RequestValue for StyleSetting {}
impl client::ResponseResult for StyleSetting {}


/// Represents a list of styles for a given table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list style](StyleListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StyleSettingList {
    /// All requested style settings.
    
    pub items: Option<Vec<StyleSetting>>,
    /// The kind of item this is. For a style list, this is always fusiontables#styleSettingList .
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. No token is displayed if there are no more styles left.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of styles for the table.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for StyleSettingList {}


/// Represents a table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [copy table](TableCopyCall) (response)
/// * [get table](TableGetCall) (response)
/// * [import table table](TableImportTableCall) (response)
/// * [insert table](TableInsertCall) (request|response)
/// * [patch table](TablePatchCall) (request|response)
/// * [update table](TableUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Attribution assigned to the table.
    
    pub attribution: Option<String>,
    /// Optional link for attribution.
    #[serde(rename="attributionLink")]
    
    pub attribution_link: Option<String>,
    /// Base table identifier if this table is a view or merged table.
    #[serde(rename="baseTableIds")]
    
    pub base_table_ids: Option<Vec<String>>,
    /// Default JSON schema for validating all JSON column properties.
    #[serde(rename="columnPropertiesJsonSchema")]
    
    pub column_properties_json_schema: Option<String>,
    /// Columns in the table.
    
    pub columns: Option<Vec<Column>>,
    /// Description assigned to the table.
    
    pub description: Option<String>,
    /// Variable for whether table is exportable.
    #[serde(rename="isExportable")]
    
    pub is_exportable: Option<bool>,
    /// The kind of item this is. For a table, this is always fusiontables#table.
    
    pub kind: Option<String>,
    /// Name assigned to a table.
    
    pub name: Option<String>,
    /// SQL that encodes the table definition for derived tables.
    
    pub sql: Option<String>,
    /// Encrypted unique alphanumeric identifier for the table.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
    /// JSON object containing custom table properties.
    #[serde(rename="tablePropertiesJson")]
    
    pub table_properties_json: Option<String>,
    /// JSON schema for validating the JSON table properties.
    #[serde(rename="tablePropertiesJsonSchema")]
    
    pub table_properties_json_schema: Option<String>,
}

impl client::RequestValue for Table {}
impl client::ResponseResult for Table {}


/// Represents a list of tables.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list table](TableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableList {
    /// List of all requested tables.
    
    pub items: Option<Vec<Table>>,
    /// The kind of item this is. For table list, this is always fusiontables#tableList.
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. No token is displayed if there are no more pages left.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for TableList {}


/// A background task on a table, initiated for time- or resource-consuming operations such as changing column types or deleting all rows.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [refetch sheet table](TableRefetchSheetCall) (response)
/// * [replace rows table](TableReplaceRowCall) (response)
/// * [get task](TaskGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    /// Type of the resource. This is always "fusiontables#task".
    
    pub kind: Option<String>,
    /// Task percentage completion.
    
    pub progress: Option<String>,
    /// false while the table is busy with some other task. true if this background task is currently running.
    
    pub started: Option<bool>,
    /// Identifier for the task.
    #[serde(rename="taskId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub task_id: Option<i64>,
    /// Type of background task.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::ResponseResult for Task {}


/// Represents a list of tasks for a table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list task](TaskListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TaskList {
    /// List of all requested tasks.
    
    pub items: Option<Vec<Task>>,
    /// Type of the resource. This is always "fusiontables#taskList".
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. No token is displayed if there are no more pages left.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of tasks for the table.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for TaskList {}


/// Represents the contents of InfoWindow templates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get template](TemplateGetCall) (response)
/// * [insert template](TemplateInsertCall) (request|response)
/// * [patch template](TemplatePatchCall) (request|response)
/// * [update template](TemplateUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    /// List of columns from which the template is to be automatically constructed. Only one of body or automaticColumns can be specified.
    #[serde(rename="automaticColumnNames")]
    
    pub automatic_column_names: Option<Vec<String>>,
    /// Body of the template. It contains HTML with {column_name} to insert values from a particular column. The body is sanitized to remove certain tags, e.g., script. Only one of body or automaticColumns can be specified.
    
    pub body: Option<String>,
    /// The kind of item this is. For a template, this is always fusiontables#template.
    
    pub kind: Option<String>,
    /// Optional name assigned to a template.
    
    pub name: Option<String>,
    /// Identifier for the table for which the template is defined.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
    /// Identifier for the template, unique within the context of a particular table.
    #[serde(rename="templateId")]
    
    pub template_id: Option<i32>,
}

impl client::RequestValue for Template {}
impl client::ResponseResult for Template {}


/// Represents a list of templates for a given table.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list template](TemplateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TemplateList {
    /// List of all requested templates.
    
    pub items: Option<Vec<Template>>,
    /// The kind of item this is. For a template list, this is always fusiontables#templateList .
    
    pub kind: Option<String>,
    /// Token used to access the next page of this result. No token is displayed if there are no more pages left.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Total number of templates for the table.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for TemplateList {}


/// Identifier of the base column. If present, this column is derived from the specified base column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColumnBaseColumn {
    /// The id of the column in the base table from which this column is derived.
    #[serde(rename="columnId")]
    
    pub column_id: Option<i32>,
    /// Offset to the entry in the list of base tables in the table definition.
    #[serde(rename="tableIndex")]
    
    pub table_index: Option<i32>,
}

impl client::NestedType for ColumnBaseColumn {}
impl client::Part for ColumnBaseColumn {}


/// Gradient function that interpolates a range of colors based on column value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StyleFunctionGradient {
    /// Array with two or more colors.
    
    pub colors: Option<Vec<StyleFunctionGradientColors>>,
    /// Higher-end of the interpolation range: rows with this value will be assigned to colors[n-1].
    
    pub max: Option<f64>,
    /// Lower-end of the interpolation range: rows with this value will be assigned to colors[0].
    
    pub min: Option<f64>,
}

impl client::NestedType for StyleFunctionGradient {}
impl client::Part for StyleFunctionGradient {}


/// Array with two or more colors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StyleFunctionGradientColors {
    /// Color in #RRGGBB format.
    
    pub color: Option<String>,
    /// Opacity of the color: 0.0 (transparent) to 1.0 (opaque).
    
    pub opacity: Option<f64>,
}

impl client::NestedType for StyleFunctionGradientColors {}
impl client::Part for StyleFunctionGradientColors {}


