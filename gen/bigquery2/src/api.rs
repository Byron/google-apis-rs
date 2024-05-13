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
    /// View and manage your data in Google BigQuery and see the email address for your Google Account
    Full,

    /// Insert data into Google BigQuery
    Insertdata,

    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,

    /// Manage your data and permissions in Cloud Storage and see the email address for your Google Account
    DevstorageFullControl,

    /// View your data in Google Cloud Storage
    DevstorageReadOnly,

    /// Manage your data in Cloud Storage and see the email address of your Google Account
    DevstorageReadWrite,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/bigquery",
            Scope::Insertdata => "https://www.googleapis.com/auth/bigquery.insertdata",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
            Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Bigquery related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// use bigquery2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().get("projectId", "datasetId", "tableId")
///              .view("gubergren")
///              .selected_fields("Lorem")
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
pub struct Bigquery<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for Bigquery<S> {}

impl<'a, S> Bigquery<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> Bigquery<S> {
        Bigquery {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.3".to_string(),
            _base_url: "https://bigquery.googleapis.com/bigquery/v2/".to_string(),
            _root_url: "https://bigquery.googleapis.com/".to_string(),
        }
    }

    pub fn datasets(&'a self) -> DatasetMethods<'a, S> {
        DatasetMethods { hub: &self }
    }
    pub fn jobs(&'a self) -> JobMethods<'a, S> {
        JobMethods { hub: &self }
    }
    pub fn models(&'a self) -> ModelMethods<'a, S> {
        ModelMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, S> {
        ProjectMethods { hub: &self }
    }
    pub fn routines(&'a self) -> RoutineMethods<'a, S> {
        RoutineMethods { hub: &self }
    }
    pub fn row_access_policies(&'a self) -> RowAccessPolicyMethods<'a, S> {
        RowAccessPolicyMethods { hub: &self }
    }
    pub fn tabledata(&'a self) -> TabledataMethods<'a, S> {
        TabledataMethods { hub: &self }
    }
    pub fn tables(&'a self) -> TableMethods<'a, S> {
        TableMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.3`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://bigquery.googleapis.com/bigquery/v2/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://bigquery.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Aggregate metrics for classification/classifier models. For multi-class models, the metrics are either macro-averaged or micro-averaged. When macro-averaged, the metrics are calculated for each label and then an unweighted average is taken of those values. When micro-averaged, the metric is calculated globally by counting the total number of correctly predicted rows.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregateClassificationMetrics {
    /// Accuracy is the fraction of predictions given the correct label. For multiclass this is a micro-averaged metric.
    
    pub accuracy: Option<f64>,
    /// The F1 score is an average of recall and precision. For multiclass this is a macro-averaged metric.
    #[serde(rename="f1Score")]
    
    pub f1_score: Option<f64>,
    /// Logarithmic Loss. For multiclass this is a macro-averaged metric.
    #[serde(rename="logLoss")]
    
    pub log_loss: Option<f64>,
    /// Precision is the fraction of actual positive predictions that had positive actual labels. For multiclass this is a macro-averaged metric treating each class as a binary classifier.
    
    pub precision: Option<f64>,
    /// Recall is the fraction of actual positive labels that were given a positive prediction. For multiclass this is a macro-averaged metric.
    
    pub recall: Option<f64>,
    /// Area Under a ROC Curve. For multiclass this is a macro-averaged metric.
    #[serde(rename="rocAuc")]
    
    pub roc_auc: Option<f64>,
    /// Threshold at which the metrics are computed. For binary classification models this is the positive class threshold. For multi-class classfication models this is the confidence threshold.
    
    pub threshold: Option<f64>,
}

impl client::Part for AggregateClassificationMetrics {}


/// Represents privacy policy associated with "aggregation threshold" method.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AggregationThresholdPolicy {
    /// Optional. The privacy unit column(s) associated with this policy. For now, only one column per data source object (table, view) is allowed as a privacy unit column. Representing as a repeated field in metadata for extensibility to multiple columns in future. Duplicates and Repeated struct fields are not allowed. For nested fields, use dot notation ("outer.inner")
    #[serde(rename="privacyUnitColumns")]
    
    pub privacy_unit_columns: Option<Vec<String>>,
    /// Optional. The threshold for the "aggregation threshold" policy.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub threshold: Option<i64>,
}

impl client::Part for AggregationThresholdPolicy {}


/// Input/output argument of a function or a stored procedure.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Argument {
    /// Optional. Defaults to FIXED_TYPE.
    #[serde(rename="argumentKind")]
    
    pub argument_kind: Option<String>,
    /// Required unless argument_kind = ANY_TYPE.
    #[serde(rename="dataType")]
    
    pub data_type: Option<StandardSqlDataType>,
    /// Optional. Whether the argument is an aggregate function parameter. Must be Unset for routine types other than AGGREGATE_FUNCTION. For AGGREGATE_FUNCTION, if set to false, it is equivalent to adding "NOT AGGREGATE" clause in DDL; Otherwise, it is equivalent to omitting "NOT AGGREGATE" clause in DDL.
    #[serde(rename="isAggregate")]
    
    pub is_aggregate: Option<bool>,
    /// Optional. Specifies whether the argument is input or output. Can be set for procedures only.
    
    pub mode: Option<String>,
    /// Optional. The name of this argument. Can be absent for function return argument.
    
    pub name: Option<String>,
}

impl client::Part for Argument {}


/// Arima coefficients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArimaCoefficients {
    /// Auto-regressive coefficients, an array of double.
    #[serde(rename="autoRegressiveCoefficients")]
    
    pub auto_regressive_coefficients: Option<Vec<f64>>,
    /// Intercept coefficient, just a double not an array.
    #[serde(rename="interceptCoefficient")]
    
    pub intercept_coefficient: Option<f64>,
    /// Moving-average coefficients, an array of double.
    #[serde(rename="movingAverageCoefficients")]
    
    pub moving_average_coefficients: Option<Vec<f64>>,
}

impl client::Part for ArimaCoefficients {}


/// ARIMA model fitting metrics.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArimaFittingMetrics {
    /// AIC.
    
    pub aic: Option<f64>,
    /// Log-likelihood.
    #[serde(rename="logLikelihood")]
    
    pub log_likelihood: Option<f64>,
    /// Variance.
    
    pub variance: Option<f64>,
}

impl client::Part for ArimaFittingMetrics {}


/// Model evaluation metrics for ARIMA forecasting models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArimaForecastingMetrics {
    /// Arima model fitting metrics.
    #[serde(rename="arimaFittingMetrics")]
    
    pub arima_fitting_metrics: Option<Vec<ArimaFittingMetrics>>,
    /// Repeated as there can be many metric sets (one for each model) in auto-arima and the large-scale case.
    #[serde(rename="arimaSingleModelForecastingMetrics")]
    
    pub arima_single_model_forecasting_metrics: Option<Vec<ArimaSingleModelForecastingMetrics>>,
    /// Whether Arima model fitted with drift or not. It is always false when d is not 1.
    #[serde(rename="hasDrift")]
    
    pub has_drift: Option<Vec<bool>>,
    /// Non-seasonal order.
    #[serde(rename="nonSeasonalOrder")]
    
    pub non_seasonal_order: Option<Vec<ArimaOrder>>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    #[serde(rename="seasonalPeriods")]
    
    pub seasonal_periods: Option<Vec<String>>,
    /// Id to differentiate different time series for the large-scale case.
    #[serde(rename="timeSeriesId")]
    
    pub time_series_id: Option<Vec<String>>,
}

impl client::Part for ArimaForecastingMetrics {}


/// Arima model information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArimaModelInfo {
    /// Arima coefficients.
    #[serde(rename="arimaCoefficients")]
    
    pub arima_coefficients: Option<ArimaCoefficients>,
    /// Arima fitting metrics.
    #[serde(rename="arimaFittingMetrics")]
    
    pub arima_fitting_metrics: Option<ArimaFittingMetrics>,
    /// Whether Arima model fitted with drift or not. It is always false when d is not 1.
    #[serde(rename="hasDrift")]
    
    pub has_drift: Option<bool>,
    /// If true, holiday_effect is a part of time series decomposition result.
    #[serde(rename="hasHolidayEffect")]
    
    pub has_holiday_effect: Option<bool>,
    /// If true, spikes_and_dips is a part of time series decomposition result.
    #[serde(rename="hasSpikesAndDips")]
    
    pub has_spikes_and_dips: Option<bool>,
    /// If true, step_changes is a part of time series decomposition result.
    #[serde(rename="hasStepChanges")]
    
    pub has_step_changes: Option<bool>,
    /// Non-seasonal order.
    #[serde(rename="nonSeasonalOrder")]
    
    pub non_seasonal_order: Option<ArimaOrder>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    #[serde(rename="seasonalPeriods")]
    
    pub seasonal_periods: Option<Vec<String>>,
    /// The time_series_id value for this time series. It will be one of the unique values from the time_series_id_column specified during ARIMA model training. Only present when time_series_id_column training option was used.
    #[serde(rename="timeSeriesId")]
    
    pub time_series_id: Option<String>,
    /// The tuple of time_series_ids identifying this time series. It will be one of the unique tuples of values present in the time_series_id_columns specified during ARIMA model training. Only present when time_series_id_columns training option was used and the order of values here are same as the order of time_series_id_columns.
    #[serde(rename="timeSeriesIds")]
    
    pub time_series_ids: Option<Vec<String>>,
}

impl client::Part for ArimaModelInfo {}


/// Arima order, can be used for both non-seasonal and seasonal parts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArimaOrder {
    /// Order of the differencing part.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub d: Option<i64>,
    /// Order of the autoregressive part.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub p: Option<i64>,
    /// Order of the moving-average part.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub q: Option<i64>,
}

impl client::Part for ArimaOrder {}


/// (Auto-)arima fitting result. Wrap everything in ArimaResult for easier refactoring if we want to use model-specific iteration results.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArimaResult {
    /// This message is repeated because there are multiple arima models fitted in auto-arima. For non-auto-arima model, its size is one.
    #[serde(rename="arimaModelInfo")]
    
    pub arima_model_info: Option<Vec<ArimaModelInfo>>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    #[serde(rename="seasonalPeriods")]
    
    pub seasonal_periods: Option<Vec<String>>,
}

impl client::Part for ArimaResult {}


/// Model evaluation metrics for a single ARIMA forecasting model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArimaSingleModelForecastingMetrics {
    /// Arima fitting metrics.
    #[serde(rename="arimaFittingMetrics")]
    
    pub arima_fitting_metrics: Option<ArimaFittingMetrics>,
    /// Is arima model fitted with drift or not. It is always false when d is not 1.
    #[serde(rename="hasDrift")]
    
    pub has_drift: Option<bool>,
    /// If true, holiday_effect is a part of time series decomposition result.
    #[serde(rename="hasHolidayEffect")]
    
    pub has_holiday_effect: Option<bool>,
    /// If true, spikes_and_dips is a part of time series decomposition result.
    #[serde(rename="hasSpikesAndDips")]
    
    pub has_spikes_and_dips: Option<bool>,
    /// If true, step_changes is a part of time series decomposition result.
    #[serde(rename="hasStepChanges")]
    
    pub has_step_changes: Option<bool>,
    /// Non-seasonal order.
    #[serde(rename="nonSeasonalOrder")]
    
    pub non_seasonal_order: Option<ArimaOrder>,
    /// Seasonal periods. Repeated because multiple periods are supported for one time series.
    #[serde(rename="seasonalPeriods")]
    
    pub seasonal_periods: Option<Vec<String>>,
    /// The time_series_id value for this time series. It will be one of the unique values from the time_series_id_column specified during ARIMA model training. Only present when time_series_id_column training option was used.
    #[serde(rename="timeSeriesId")]
    
    pub time_series_id: Option<String>,
    /// The tuple of time_series_ids identifying this time series. It will be one of the unique tuples of values present in the time_series_id_columns specified during ARIMA model training. Only present when time_series_id_columns training option was used and the order of values here are same as the order of time_series_id_columns.
    #[serde(rename="timeSeriesIds")]
    
    pub time_series_ids: Option<Vec<String>>,
}

impl client::Part for ArimaSingleModelForecastingMetrics {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for AuditLogConfig {}


/// Options for external data sources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AvroOptions {
    /// Optional. If sourceFormat is set to "AVRO", indicates whether to interpret logical types as the corresponding BigQuery data type (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER).
    #[serde(rename="useAvroLogicalTypes")]
    
    pub use_avro_logical_types: Option<bool>,
}

impl client::Part for AvroOptions {}


/// Reason why BI Engine didn't accelerate the query (or sub-query).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BiEngineReason {
    /// Output only. High-level BI Engine reason for partial or disabled acceleration
    
    pub code: Option<String>,
    /// Output only. Free form human-readable reason for partial or disabled acceleration.
    
    pub message: Option<String>,
}

impl client::Part for BiEngineReason {}


/// Statistics for a BI Engine specific query. Populated as part of JobStatistics2
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BiEngineStatistics {
    /// Output only. Specifies which mode of BI Engine acceleration was performed (if any).
    #[serde(rename="accelerationMode")]
    
    pub acceleration_mode: Option<String>,
    /// Output only. Specifies which mode of BI Engine acceleration was performed (if any).
    #[serde(rename="biEngineMode")]
    
    pub bi_engine_mode: Option<String>,
    /// In case of DISABLED or PARTIAL bi_engine_mode, these contain the explanatory reasons as to why BI Engine could not accelerate. In case the full query was accelerated, this field is not populated.
    #[serde(rename="biEngineReasons")]
    
    pub bi_engine_reasons: Option<Vec<BiEngineReason>>,
}

impl client::Part for BiEngineStatistics {}


/// Configuration for BigLake managed tables.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigLakeConfiguration {
    /// Required. The connection specifying the credentials to be used to read and write to external storage, such as Cloud Storage. The connection_id can have the form "<project\_id>.<location\_id>.<connection\_id>" or "projects/<project\_id>/locations/<location\_id>/connections/<connection\_id>".
    #[serde(rename="connectionId")]
    
    pub connection_id: Option<String>,
    /// Required. The file format the table data is stored in.
    #[serde(rename="fileFormat")]
    
    pub file_format: Option<String>,
    /// Required. The fully qualified location prefix of the external folder where table data is stored. The '*' wildcard character is not allowed. The URI should be in the format "gs://bucket/path_to_table/"
    #[serde(rename="storageUri")]
    
    pub storage_uri: Option<String>,
    /// Required. The table format the metadata only snapshots are stored in.
    #[serde(rename="tableFormat")]
    
    pub table_format: Option<String>,
}

impl client::Part for BigLakeConfiguration {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigQueryModelTraining {
    /// Deprecated.
    #[serde(rename="currentIteration")]
    
    pub current_iteration: Option<i32>,
    /// Deprecated.
    #[serde(rename="expectedTotalIterations")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expected_total_iterations: Option<i64>,
}

impl client::Part for BigQueryModelTraining {}


/// Information related to a Bigtable column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigtableColumn {
    /// Optional. The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. 'encoding' can also be set at the column family level. However, the setting at this level takes precedence if 'encoding' is set at both levels.
    
    pub encoding: Option<String>,
    /// Optional. If the qualifier is not a valid BigQuery field identifier i.e. does not match a-zA-Z*, a valid identifier must be provided as the column field name and is used as field name in queries.
    #[serde(rename="fieldName")]
    
    pub field_name: Option<String>,
    /// Optional. If this is set, only the latest version of value in this column are exposed. 'onlyReadLatest' can also be set at the column family level. However, the setting at this level takes precedence if 'onlyReadLatest' is set at both levels.
    #[serde(rename="onlyReadLatest")]
    
    pub only_read_latest: Option<bool>,
    /// [Required] Qualifier of the column. Columns in the parent column family that has this exact qualifier are exposed as . field. If the qualifier is valid UTF-8 string, it can be specified in the qualifier_string field. Otherwise, a base-64 encoded value must be set to qualifier_encoded. The column field name is the same as the column qualifier. However, if the qualifier is not a valid BigQuery field identifier i.e. does not match a-zA-Z*, a valid identifier must be provided as field_name.
    #[serde(rename="qualifierEncoded")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub qualifier_encoded: Option<Vec<u8>>,
    /// Qualifier string.
    #[serde(rename="qualifierString")]
    
    pub qualifier_string: Option<String>,
    /// Optional. The type to convert the value in cells of this column. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): * BYTES * STRING * INTEGER * FLOAT * BOOLEAN * JSON Default type is BYTES. 'type' can also be set at the column family level. However, the setting at this level takes precedence if 'type' is set at both levels.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for BigtableColumn {}


/// Information related to a Bigtable column family.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigtableColumnFamily {
    /// Optional. Lists of columns that should be exposed as individual fields as opposed to a list of (column name, value) pairs. All columns whose qualifier matches a qualifier in this list can be accessed as .. Other columns can be accessed as a list through .Column field.
    
    pub columns: Option<Vec<BigtableColumn>>,
    /// Optional. The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it.
    
    pub encoding: Option<String>,
    /// Identifier of the column family.
    #[serde(rename="familyId")]
    
    pub family_id: Option<String>,
    /// Optional. If this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column.
    #[serde(rename="onlyReadLatest")]
    
    pub only_read_latest: Option<bool>,
    /// Optional. The type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive): * BYTES * STRING * INTEGER * FLOAT * BOOLEAN * JSON Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for BigtableColumnFamily {}


/// Options specific to Google Cloud Bigtable data sources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BigtableOptions {
    /// Optional. List of column families to expose in the table schema along with their types. This list restricts the column families that can be referenced in queries and specifies their value types. You can use this list to do type conversions - see the 'type' field for more details. If you leave this list empty, all column families are present in the table schema and their values are read as BYTES. During a query only the column families referenced in that query are read from Bigtable.
    #[serde(rename="columnFamilies")]
    
    pub column_families: Option<Vec<BigtableColumnFamily>>,
    /// Optional. If field is true, then the column families that are not specified in columnFamilies list are not exposed in the table schema. Otherwise, they are read with BYTES type values. The default value is false.
    #[serde(rename="ignoreUnspecifiedColumnFamilies")]
    
    pub ignore_unspecified_column_families: Option<bool>,
    /// Optional. If field is true, then each column family will be read as a single JSON column. Otherwise they are read as a repeated cell structure containing timestamp/value tuples. The default value is false.
    #[serde(rename="outputColumnFamiliesAsJson")]
    
    pub output_column_families_as_json: Option<bool>,
    /// Optional. If field is true, then the rowkey column families will be read and converted to string. Otherwise they are read with BYTES type values and users need to manually cast them with CAST if necessary. The default value is false.
    #[serde(rename="readRowkeyAsString")]
    
    pub read_rowkey_as_string: Option<bool>,
}

impl client::Part for BigtableOptions {}


/// Evaluation metrics for binary classification/classifier models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BinaryClassificationMetrics {
    /// Aggregate classification metrics.
    #[serde(rename="aggregateClassificationMetrics")]
    
    pub aggregate_classification_metrics: Option<AggregateClassificationMetrics>,
    /// Binary confusion matrix at multiple thresholds.
    #[serde(rename="binaryConfusionMatrixList")]
    
    pub binary_confusion_matrix_list: Option<Vec<BinaryConfusionMatrix>>,
    /// Label representing the negative class.
    #[serde(rename="negativeLabel")]
    
    pub negative_label: Option<String>,
    /// Label representing the positive class.
    #[serde(rename="positiveLabel")]
    
    pub positive_label: Option<String>,
}

impl client::Part for BinaryClassificationMetrics {}


/// Confusion matrix for binary classification models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BinaryConfusionMatrix {
    /// The fraction of predictions given the correct label.
    
    pub accuracy: Option<f64>,
    /// The equally weighted average of recall and precision.
    #[serde(rename="f1Score")]
    
    pub f1_score: Option<f64>,
    /// Number of false samples predicted as false.
    #[serde(rename="falseNegatives")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub false_negatives: Option<i64>,
    /// Number of false samples predicted as true.
    #[serde(rename="falsePositives")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub false_positives: Option<i64>,
    /// Threshold value used when computing each of the following metric.
    #[serde(rename="positiveClassThreshold")]
    
    pub positive_class_threshold: Option<f64>,
    /// The fraction of actual positive predictions that had positive actual labels.
    
    pub precision: Option<f64>,
    /// The fraction of actual positive labels that were given a positive prediction.
    
    pub recall: Option<f64>,
    /// Number of true samples predicted as false.
    #[serde(rename="trueNegatives")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub true_negatives: Option<i64>,
    /// Number of true samples predicted as true.
    #[serde(rename="truePositives")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub true_positives: Option<i64>,
}

impl client::Part for BinaryConfusionMatrix {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. * `principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workforce identity pool. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{group_id}`: All workforce identities in a group. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All workforce identities with a specific attribute value. * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*`: All identities in a workforce identity pool. * `principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}`: A single identity in a workload identity pool. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/group/{group_id}`: A workload identity pool group. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}`: All identities in a workload identity pool with a certain attribute. * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*`: All identities in a workload identity pool. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`: Deleted single identity in a workforce identity pool. For example, `deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-id/subject/my-subject-attribute-value`.
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`. For an overview of the IAM roles and permissions, see the [IAM documentation](https://cloud.google.com/iam/docs/roles-overview). For a list of the available pre-defined roles, see [here](https://cloud.google.com/iam/docs/understanding-roles).
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BqmlIterationResult {
    /// Deprecated.
    #[serde(rename="durationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_ms: Option<i64>,
    /// Deprecated.
    #[serde(rename="evalLoss")]
    
    pub eval_loss: Option<f64>,
    /// Deprecated.
    
    pub index: Option<i32>,
    /// Deprecated.
    #[serde(rename="learnRate")]
    
    pub learn_rate: Option<f64>,
    /// Deprecated.
    #[serde(rename="trainingLoss")]
    
    pub training_loss: Option<f64>,
}

impl client::Part for BqmlIterationResult {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BqmlTrainingRun {
    /// Deprecated.
    #[serde(rename="iterationResults")]
    
    pub iteration_results: Option<Vec<BqmlIterationResult>>,
    /// Deprecated.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Deprecated.
    
    pub state: Option<String>,
    /// Deprecated.
    #[serde(rename="trainingOptions")]
    
    pub training_options: Option<BqmlTrainingRunTrainingOptions>,
}

impl client::Part for BqmlTrainingRun {}


/// Representative value of a categorical feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalValue {
    /// Counts of all categories for the categorical feature. If there are more than ten categories, we return top ten (by count) and return one more CategoryCount with category "_OTHER_" and count as aggregate counts of remaining categories.
    #[serde(rename="categoryCounts")]
    
    pub category_counts: Option<Vec<CategoryCount>>,
}

impl client::Part for CategoricalValue {}


/// Represents the count of a single category within the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CategoryCount {
    /// The name of category.
    
    pub category: Option<String>,
    /// The count of training samples matching the category within the cluster.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
}

impl client::Part for CategoryCount {}


/// Information about base table and clone time of a table clone.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloneDefinition {
    /// Required. Reference describing the ID of the table that was cloned.
    #[serde(rename="baseTableReference")]
    
    pub base_table_reference: Option<TableReference>,
    /// Required. The time at which the base table was cloned. This value is reported in the JSON response using RFC3339 format.
    #[serde(rename="cloneTime")]
    
    pub clone_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for CloneDefinition {}


/// Message containing the information about one cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cluster {
    /// Centroid id.
    #[serde(rename="centroidId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub centroid_id: Option<i64>,
    /// Count of training data rows that were assigned to this cluster.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Values of highly variant features for this cluster.
    #[serde(rename="featureValues")]
    
    pub feature_values: Option<Vec<FeatureValue>>,
}

impl client::Part for Cluster {}


/// Information about a single cluster for clustering model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusterInfo {
    /// Centroid id.
    #[serde(rename="centroidId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub centroid_id: Option<i64>,
    /// Cluster radius, the average distance from centroid to each point assigned to the cluster.
    #[serde(rename="clusterRadius")]
    
    pub cluster_radius: Option<f64>,
    /// Cluster size, the total number of points assigned to the cluster.
    #[serde(rename="clusterSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub cluster_size: Option<i64>,
}

impl client::Part for ClusterInfo {}


/// Configures table clustering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Clustering {
    /// One or more fields on which data should be clustered. Only top-level, non-repeated, simple-type fields are supported. The ordering of the clustering fields should be prioritized from most to least important for filtering purposes. Additional information on limitations can be found here: https://cloud.google.com/bigquery/docs/creating-clustered-tables#limitations
    
    pub fields: Option<Vec<String>>,
}

impl client::Part for Clustering {}


/// Evaluation metrics for clustering models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClusteringMetrics {
    /// Information for all clusters.
    
    pub clusters: Option<Vec<Cluster>>,
    /// Davies-Bouldin index.
    #[serde(rename="daviesBouldinIndex")]
    
    pub davies_bouldin_index: Option<f64>,
    /// Mean of squared distances between each sample to its cluster centroid.
    #[serde(rename="meanSquaredDistance")]
    
    pub mean_squared_distance: Option<f64>,
}

impl client::Part for ClusteringMetrics {}


/// Confusion matrix for multi-class classification models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfusionMatrix {
    /// Confidence threshold used when computing the entries of the confusion matrix.
    #[serde(rename="confidenceThreshold")]
    
    pub confidence_threshold: Option<f64>,
    /// One row per actual label.
    
    pub rows: Option<Vec<Row>>,
}

impl client::Part for ConfusionMatrix {}


/// A connection-level property to customize query behavior. Under JDBC, these correspond directly to connection properties passed to the DriverManager. Under ODBC, these correspond to properties in the connection string. Currently supported connection properties: * **dataset_project_id**: represents the default project for datasets that are used in the query. Setting the system variable `@@dataset_project_id` achieves the same behavior. For more information about system variables, see: https://cloud.google.com/bigquery/docs/reference/system-variables * **time_zone**: represents the default timezone used to run the query. * **session_id**: associates the query with a given session. * **query_label**: associates the query with a given job label. If set, all subsequent queries in a script or session will have this label. For the format in which a you can specify a query label, see labels in the JobConfiguration resource type: https://cloud.google.com/bigquery/docs/reference/rest/v2/Job#jobconfiguration Additional properties are allowed, but ignored. Specifying multiple connection properties with the same key returns an error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionProperty {
    /// The key of the property to set.
    
    pub key: Option<String>,
    /// The value of the property to set.
    
    pub value: Option<String>,
}

impl client::Part for ConnectionProperty {}


/// Information related to a CSV data source.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CsvOptions {
    /// Optional. Indicates if BigQuery should accept rows that are missing trailing optional columns. If true, BigQuery treats missing trailing columns as null values. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false.
    #[serde(rename="allowJaggedRows")]
    
    pub allow_jagged_rows: Option<bool>,
    /// Optional. Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
    #[serde(rename="allowQuotedNewlines")]
    
    pub allow_quoted_newlines: Option<bool>,
    /// Optional. The character encoding of the data. The supported values are UTF-8, ISO-8859-1, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
    
    pub encoding: Option<String>,
    /// Optional. The separator character for fields in a CSV file. The separator is interpreted as a single byte. For files encoded in ISO-8859-1, any single character can be used as a separator. For files encoded in UTF-8, characters represented in decimal range 1-127 (U+0001-U+007F) can be used without any modification. UTF-8 characters encoded with multiple bytes (i.e. U+0080 and above) will have only the first byte used for separating fields. The remaining bytes will be treated as a part of the field. BigQuery also supports the escape sequence "\t" (U+0009) to specify a tab separator. The default value is comma (",", U+002C).
    #[serde(rename="fieldDelimiter")]
    
    pub field_delimiter: Option<String>,
    /// [Optional] A custom string that will represent a NULL value in CSV import data.
    #[serde(rename="nullMarker")]
    
    pub null_marker: Option<String>,
    /// Optional. Indicates if the embedded ASCII control characters (the first 32 characters in the ASCII-table, from '\x00' to '\x1F') are preserved.
    #[serde(rename="preserveAsciiControlCharacters")]
    
    pub preserve_ascii_control_characters: Option<bool>,
    /// Optional. The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ("). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true. To include the specific quote character within a quoted value, precede it with an additional matching quote character. For example, if you want to escape the default character ' " ', use ' "" '.
    
    pub quote: Option<String>,
    /// Optional. The number of rows at the top of a CSV file that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped. When autodetect is on, the behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema.
    #[serde(rename="skipLeadingRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub skip_leading_rows: Option<i64>,
}

impl client::Part for CsvOptions {}


/// Options for data format adjustments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataFormatOptions {
    /// Optional. Output timestamp as usec int64. Default is false.
    #[serde(rename="useInt64Timestamp")]
    
    pub use_int64_timestamp: Option<bool>,
}

impl client::Part for DataFormatOptions {}


/// Statistics for data-masking.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataMaskingStatistics {
    /// Whether any accessed data was protected by the data masking.
    #[serde(rename="dataMaskingApplied")]
    
    pub data_masking_applied: Option<bool>,
}

impl client::Part for DataMaskingStatistics {}


/// Data split result. This contains references to the training and evaluation data tables that were used to train the model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DataSplitResult {
    /// Table reference of the evaluation data after split.
    #[serde(rename="evaluationTable")]
    
    pub evaluation_table: Option<TableReference>,
    /// Table reference of the test data after split.
    #[serde(rename="testTable")]
    
    pub test_table: Option<TableReference>,
    /// Table reference of the training data after split.
    #[serde(rename="trainingTable")]
    
    pub training_table: Option<TableReference>,
}

impl client::Part for DataSplitResult {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete datasets](DatasetDeleteCall) (none)
/// * [get datasets](DatasetGetCall) (response)
/// * [insert datasets](DatasetInsertCall) (request|response)
/// * [list datasets](DatasetListCall) (none)
/// * [patch datasets](DatasetPatchCall) (request|response)
/// * [undelete datasets](DatasetUndeleteCall) (response)
/// * [update datasets](DatasetUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Dataset {
    /// Optional. An array of objects that define dataset access for one or more entities. You can set this property when inserting or updating a dataset in order to control who is allowed to access the data. If unspecified at dataset creation time, BigQuery adds default dataset access for the following entities: access.specialGroup: projectReaders; access.role: READER; access.specialGroup: projectWriters; access.role: WRITER; access.specialGroup: projectOwners; access.role: OWNER; access.userByEmail: [dataset creator email]; access.role: OWNER;
    
    pub access: Option<Vec<DatasetAccess>>,
    /// Output only. The time when this dataset was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// Required. A reference that identifies the dataset.
    #[serde(rename="datasetReference")]
    
    pub dataset_reference: Option<DatasetReference>,
    /// Optional. Defines the default collation specification of future tables created in the dataset. If a table is created in this dataset without table-level default collation, then the table inherits the dataset default collation, which is applied to the string fields that do not have explicit collation specified. A change to this field affects only tables created afterwards, and does not alter the existing tables. The following values are supported: * 'und:ci': undetermined locale, case insensitive. * '': empty string. Default to case-sensitive behavior.
    #[serde(rename="defaultCollation")]
    
    pub default_collation: Option<String>,
    /// The default encryption key for all tables in the dataset. Once this property is set, all newly-created partitioned tables in the dataset will have encryption key set to this value, unless table creation request (or query) overrides the key.
    #[serde(rename="defaultEncryptionConfiguration")]
    
    pub default_encryption_configuration: Option<EncryptionConfiguration>,
    /// This default partition expiration, expressed in milliseconds. When new time-partitioned tables are created in a dataset where this property is set, the table will inherit this value, propagated as the `TimePartitioning.expirationMs` property on the new table. If you set `TimePartitioning.expirationMs` explicitly when creating a table, the `defaultPartitionExpirationMs` of the containing dataset is ignored. When creating a partitioned table, if `defaultPartitionExpirationMs` is set, the `defaultTableExpirationMs` value is ignored and the table will not be inherit a table expiration deadline.
    #[serde(rename="defaultPartitionExpirationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_partition_expiration_ms: Option<i64>,
    /// Optional. Defines the default rounding mode specification of new tables created within this dataset. During table creation, if this field is specified, the table within this dataset will inherit the default rounding mode of the dataset. Setting the default rounding mode on a table overrides this option. Existing tables in the dataset are unaffected. If columns are defined during that table creation, they will immediately inherit the table's default rounding mode, unless otherwise specified.
    #[serde(rename="defaultRoundingMode")]
    
    pub default_rounding_mode: Option<String>,
    /// Optional. The default lifetime of all tables in the dataset, in milliseconds. The minimum lifetime value is 3600000 milliseconds (one hour). To clear an existing default expiration with a PATCH request, set to 0. Once this property is set, all newly-created tables in the dataset will have an expirationTime property set to the creation time plus the value in this property, and changing the value will only affect new tables, not existing ones. When the expirationTime for a given table is reached, that table will be deleted automatically. If a table's expirationTime is modified or removed before the table expires, or if you provide an explicit expirationTime when creating a table, that value takes precedence over the default expiration time indicated by this property.
    #[serde(rename="defaultTableExpirationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_table_expiration_ms: Option<i64>,
    /// Optional. A user-friendly description of the dataset.
    
    pub description: Option<String>,
    /// Output only. A hash of the resource.
    
    pub etag: Option<String>,
    /// Optional. Reference to a read-only external dataset defined in data catalogs outside of BigQuery. Filled out when the dataset type is EXTERNAL.
    #[serde(rename="externalDatasetReference")]
    
    pub external_dataset_reference: Option<ExternalDatasetReference>,
    /// Optional. A descriptive name for the dataset.
    #[serde(rename="friendlyName")]
    
    pub friendly_name: Option<String>,
    /// Output only. The fully-qualified unique name of the dataset in the format projectId:datasetId. The dataset name without the project name is given in the datasetId field. When creating a new dataset, leave this field blank, and instead specify the datasetId field.
    
    pub id: Option<String>,
    /// Optional. TRUE if the dataset and its table names are case-insensitive, otherwise FALSE. By default, this is FALSE, which means the dataset and its table names are case-sensitive. This field does not affect routine references.
    #[serde(rename="isCaseInsensitive")]
    
    pub is_case_insensitive: Option<bool>,
    /// Output only. The resource type.
    
    pub kind: Option<String>,
    /// The labels associated with this dataset. You can use these to organize and group your datasets. You can set this property when inserting or updating a dataset. See Creating and Updating Dataset Labels for more information.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The date when this dataset was last modified, in milliseconds since the epoch.
    #[serde(rename="lastModifiedTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_time: Option<i64>,
    /// Optional. The source dataset reference when the dataset is of type LINKED. For all other dataset types it is not set. This field cannot be updated once it is set. Any attempt to update this field using Update and Patch API Operations will be ignored.
    #[serde(rename="linkedDatasetSource")]
    
    pub linked_dataset_source: Option<LinkedDatasetSource>,
    /// The geographic location where the dataset should reside. See https://cloud.google.com/bigquery/docs/locations for supported locations.
    
    pub location: Option<String>,
    /// Optional. Defines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days). The default value is 168 hours if this is not set.
    #[serde(rename="maxTimeTravelHours")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_time_travel_hours: Option<i64>,
    /// Output only. Reserved for future use.
    #[serde(rename="satisfiesPzi")]
    
    pub satisfies_pzi: Option<bool>,
    /// Output only. Reserved for future use.
    #[serde(rename="satisfiesPzs")]
    
    pub satisfies_pzs: Option<bool>,
    /// Output only. A URL that can be used to access the resource again. You can use this URL in Get or Update requests to the resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Optional. Updates storage_billing_model for the dataset.
    #[serde(rename="storageBillingModel")]
    
    pub storage_billing_model: Option<String>,
    /// Output only. Tags for the Dataset.
    
    pub tags: Option<Vec<DatasetTags>>,
    /// Output only. Same as `type` in `ListFormatDataset`. The type of the dataset, one of: * DEFAULT - only accessible by owner and authorized accounts, * PUBLIC - accessible by everyone, * LINKED - linked dataset, * EXTERNAL - dataset with definition in external metadata catalog. -- *BIGLAKE_METASTORE - dataset that references a database created in BigLakeMetastore service. --
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Dataset {}
impl client::Resource for Dataset {}
impl client::ResponseResult for Dataset {}


/// Grants all resources of particular types in a particular dataset read access to the current dataset. Similar to how individually authorized views work, updates to any resource granted through its dataset (including creation of new resources) requires read permission to referenced resources, plus write permission to the authorizing dataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetAccessEntry {
    /// The dataset this entry applies to
    
    pub dataset: Option<DatasetReference>,
    /// Which resources in the dataset this entry applies to. Currently, only views are supported, but additional target types may be added in the future.
    #[serde(rename="targetTypes")]
    
    pub target_types: Option<Vec<String>>,
}

impl client::Part for DatasetAccessEntry {}


/// Response format for a page of results when listing datasets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list datasets](DatasetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetList {
    /// An array of the dataset resources in the project. Each resource contains basic information. For full information about a particular dataset resource, use the Datasets: get method. This property is omitted when there are no datasets in the project.
    
    pub datasets: Option<Vec<DatasetListDatasets>>,
    /// Output only. A hash value of the results page. You can use this property to determine if the page has changed since the last request.
    
    pub etag: Option<String>,
    /// Output only. The resource type. This property always returns the value "bigquery#datasetList"
    
    pub kind: Option<String>,
    /// A token that can be used to request the next results page. This property is omitted on the final results page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of skipped locations that were unreachable. For more information about BigQuery locations, see: https://cloud.google.com/bigquery/docs/locations. Example: "europe-west5"
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for DatasetList {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetReference {
    /// Required. A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// Optional. The ID of the project containing this dataset.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for DatasetReference {}


/// Properties for the destination table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DestinationTableProperties {
    /// Optional. The description for the destination table. This will only be used if the destination table is newly created. If the table already exists and a value different than the current description is provided, the job will fail.
    
    pub description: Option<String>,
    /// Internal use only.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. Friendly name for the destination table. If the table already exists, it should be same as the existing friendly name.
    #[serde(rename="friendlyName")]
    
    pub friendly_name: Option<String>,
    /// Optional. The labels associated with this table. You can use these to organize and group your tables. This will only be used if the destination table is newly created. If the table already exists and labels are different than the current labels are provided, the job will fail.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for DestinationTableProperties {}


/// Model evaluation metrics for dimensionality reduction models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DimensionalityReductionMetrics {
    /// Total percentage of variance explained by the selected principal components.
    #[serde(rename="totalExplainedVarianceRatio")]
    
    pub total_explained_variance_ratio: Option<f64>,
}

impl client::Part for DimensionalityReductionMetrics {}


/// Detailed statistics for DML statements
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DmlStatistics {
    /// Output only. Number of deleted Rows. populated by DML DELETE, MERGE and TRUNCATE statements.
    #[serde(rename="deletedRowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub deleted_row_count: Option<i64>,
    /// Output only. Number of inserted Rows. Populated by DML INSERT and MERGE statements
    #[serde(rename="insertedRowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub inserted_row_count: Option<i64>,
    /// Output only. Number of updated Rows. Populated by DML UPDATE and MERGE statements.
    #[serde(rename="updatedRowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub updated_row_count: Option<i64>,
}

impl client::Part for DmlStatistics {}


/// Discrete candidates of a double hyperparameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleCandidates {
    /// Candidates for the double parameter in increasing order.
    
    pub candidates: Option<Vec<f64>>,
}

impl client::Part for DoubleCandidates {}


/// Search space for a double hyperparameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleHparamSearchSpace {
    /// Candidates of the double hyperparameter.
    
    pub candidates: Option<DoubleCandidates>,
    /// Range of the double hyperparameter.
    
    pub range: Option<DoubleRange>,
}

impl client::Part for DoubleHparamSearchSpace {}


/// Range of a double hyperparameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DoubleRange {
    /// Max value of the double parameter.
    
    pub max: Option<f64>,
    /// Min value of the double parameter.
    
    pub min: Option<f64>,
}

impl client::Part for DoubleRange {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// Optional. Describes the Cloud KMS encryption key that will be used to protect destination BigQuery table. The BigQuery Service Account associated with your project requires access to this encryption key.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for EncryptionConfiguration {}


/// A single entry in the confusion matrix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Entry {
    /// Number of items being predicted as this label.
    #[serde(rename="itemCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub item_count: Option<i64>,
    /// The predicted label. For confidence_threshold > 0, we will also add an entry indicating the number of items under the confidence threshold.
    #[serde(rename="predictedLabel")]
    
    pub predicted_label: Option<String>,
}

impl client::Part for Entry {}


/// Error details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ErrorProto {
    /// Debugging information. This property is internal to Google and should not be used.
    #[serde(rename="debugInfo")]
    
    pub debug_info: Option<String>,
    /// Specifies where the error occurred, if present.
    
    pub location: Option<String>,
    /// A human-readable description of the error.
    
    pub message: Option<String>,
    /// A short error code that summarizes the error.
    
    pub reason: Option<String>,
}

impl client::Part for ErrorProto {}


/// Evaluation metrics of a model. These are either computed on all training data or just the eval data based on whether eval data was used during training. These are not present for imported models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    /// Populated for ARIMA models.
    #[serde(rename="arimaForecastingMetrics")]
    
    pub arima_forecasting_metrics: Option<ArimaForecastingMetrics>,
    /// Populated for binary classification/classifier models.
    #[serde(rename="binaryClassificationMetrics")]
    
    pub binary_classification_metrics: Option<BinaryClassificationMetrics>,
    /// Populated for clustering models.
    #[serde(rename="clusteringMetrics")]
    
    pub clustering_metrics: Option<ClusteringMetrics>,
    /// Evaluation metrics when the model is a dimensionality reduction model, which currently includes PCA.
    #[serde(rename="dimensionalityReductionMetrics")]
    
    pub dimensionality_reduction_metrics: Option<DimensionalityReductionMetrics>,
    /// Populated for multi-class classification/classifier models.
    #[serde(rename="multiClassClassificationMetrics")]
    
    pub multi_class_classification_metrics: Option<MultiClassClassificationMetrics>,
    /// Populated for implicit feedback type matrix factorization models.
    #[serde(rename="rankingMetrics")]
    
    pub ranking_metrics: Option<RankingMetrics>,
    /// Populated for regression models and explicit feedback type matrix factorization models.
    #[serde(rename="regressionMetrics")]
    
    pub regression_metrics: Option<RegressionMetrics>,
}

impl client::Part for EvaluationMetrics {}


/// A single stage of query execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExplainQueryStage {
    /// Number of parallel input segments completed.
    #[serde(rename="completedParallelInputs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub completed_parallel_inputs: Option<i64>,
    /// Output only. Compute mode for this stage.
    #[serde(rename="computeMode")]
    
    pub compute_mode: Option<String>,
    /// Milliseconds the average shard spent on CPU-bound tasks.
    #[serde(rename="computeMsAvg")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub compute_ms_avg: Option<i64>,
    /// Milliseconds the slowest shard spent on CPU-bound tasks.
    #[serde(rename="computeMsMax")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub compute_ms_max: Option<i64>,
    /// Relative amount of time the average shard spent on CPU-bound tasks.
    #[serde(rename="computeRatioAvg")]
    
    pub compute_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent on CPU-bound tasks.
    #[serde(rename="computeRatioMax")]
    
    pub compute_ratio_max: Option<f64>,
    /// Stage end time represented as milliseconds since the epoch.
    #[serde(rename="endMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_ms: Option<i64>,
    /// Unique ID for the stage within the plan.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<i64>,
    /// IDs for stages that are inputs to this stage.
    #[serde(rename="inputStages")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub input_stages: Option<Vec<i64>>,
    /// Human-readable name for the stage.
    
    pub name: Option<String>,
    /// Number of parallel input segments to be processed
    #[serde(rename="parallelInputs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parallel_inputs: Option<i64>,
    /// Milliseconds the average shard spent reading input.
    #[serde(rename="readMsAvg")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub read_ms_avg: Option<i64>,
    /// Milliseconds the slowest shard spent reading input.
    #[serde(rename="readMsMax")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub read_ms_max: Option<i64>,
    /// Relative amount of time the average shard spent reading input.
    #[serde(rename="readRatioAvg")]
    
    pub read_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent reading input.
    #[serde(rename="readRatioMax")]
    
    pub read_ratio_max: Option<f64>,
    /// Number of records read into the stage.
    #[serde(rename="recordsRead")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub records_read: Option<i64>,
    /// Number of records written by the stage.
    #[serde(rename="recordsWritten")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub records_written: Option<i64>,
    /// Total number of bytes written to shuffle.
    #[serde(rename="shuffleOutputBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shuffle_output_bytes: Option<i64>,
    /// Total number of bytes written to shuffle and spilled to disk.
    #[serde(rename="shuffleOutputBytesSpilled")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub shuffle_output_bytes_spilled: Option<i64>,
    /// Slot-milliseconds used by the stage.
    #[serde(rename="slotMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub slot_ms: Option<i64>,
    /// Stage start time represented as milliseconds since the epoch.
    #[serde(rename="startMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_ms: Option<i64>,
    /// Current status for this stage.
    
    pub status: Option<String>,
    /// List of operations within the stage in dependency order (approximately chronological).
    
    pub steps: Option<Vec<ExplainQueryStep>>,
    /// Milliseconds the average shard spent waiting to be scheduled.
    #[serde(rename="waitMsAvg")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub wait_ms_avg: Option<i64>,
    /// Milliseconds the slowest shard spent waiting to be scheduled.
    #[serde(rename="waitMsMax")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub wait_ms_max: Option<i64>,
    /// Relative amount of time the average shard spent waiting to be scheduled.
    #[serde(rename="waitRatioAvg")]
    
    pub wait_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent waiting to be scheduled.
    #[serde(rename="waitRatioMax")]
    
    pub wait_ratio_max: Option<f64>,
    /// Milliseconds the average shard spent on writing output.
    #[serde(rename="writeMsAvg")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub write_ms_avg: Option<i64>,
    /// Milliseconds the slowest shard spent on writing output.
    #[serde(rename="writeMsMax")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub write_ms_max: Option<i64>,
    /// Relative amount of time the average shard spent on writing output.
    #[serde(rename="writeRatioAvg")]
    
    pub write_ratio_avg: Option<f64>,
    /// Relative amount of time the slowest shard spent on writing output.
    #[serde(rename="writeRatioMax")]
    
    pub write_ratio_max: Option<f64>,
}

impl client::Part for ExplainQueryStage {}


/// An operation within a stage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExplainQueryStep {
    /// Machine-readable operation type.
    
    pub kind: Option<String>,
    /// Human-readable description of the step(s).
    
    pub substeps: Option<Vec<String>>,
}

impl client::Part for ExplainQueryStep {}


/// Explanation for a single feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Explanation {
    /// Attribution of feature.
    
    pub attribution: Option<f64>,
    /// The full feature name. For non-numerical features, will be formatted like `.`. Overall size of feature name will always be truncated to first 120 characters.
    #[serde(rename="featureName")]
    
    pub feature_name: Option<String>,
}

impl client::Part for Explanation {}


/// Statistics for the EXPORT DATA statement as part of Query Job. EXTRACT JOB statistics are populated in JobStatistics4.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExportDataStatistics {
    /// Number of destination files generated in case of EXPORT DATA statement only.
    #[serde(rename="fileCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub file_count: Option<i64>,
    /// [Alpha] Number of destination rows generated in case of EXPORT DATA statement only.
    #[serde(rename="rowCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub row_count: Option<i64>,
}

impl client::Part for ExportDataStatistics {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalDataConfiguration {
    /// Try to detect schema and format options automatically. Any option specified explicitly will be honored.
    
    pub autodetect: Option<bool>,
    /// Optional. Additional properties to set if sourceFormat is set to AVRO.
    #[serde(rename="avroOptions")]
    
    pub avro_options: Option<AvroOptions>,
    /// Optional. Additional options if sourceFormat is set to BIGTABLE.
    #[serde(rename="bigtableOptions")]
    
    pub bigtable_options: Option<BigtableOptions>,
    /// Optional. The compression type of the data source. Possible values include GZIP and NONE. The default value is NONE. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups, Avro, ORC and Parquet formats. An empty string is an invalid value.
    
    pub compression: Option<String>,
    /// Optional. The connection specifying the credentials to be used to read external storage, such as Azure Blob, Cloud Storage, or S3. The connection_id can have the form "<project\_id>.<location\_id>.<connection\_id>" or "projects/<project\_id>/locations/<location\_id>/connections/<connection\_id>".
    #[serde(rename="connectionId")]
    
    pub connection_id: Option<String>,
    /// Optional. Additional properties to set if sourceFormat is set to CSV.
    #[serde(rename="csvOptions")]
    
    pub csv_options: Option<CsvOptions>,
    /// Defines the list of possible SQL data types to which the source decimal values are converted. This list and the precision and the scale parameters of the decimal field determine the target type. In the order of NUMERIC, BIGNUMERIC, and STRING, a type is picked if it is in the specified list and if it supports the precision and the scale. STRING supports all precision and scale values. If none of the listed types supports the precision and the scale, the type supporting the widest range in the specified list is picked, and if a value exceeds the supported range when reading the data, an error will be thrown. Example: Suppose the value of this field is ["NUMERIC", "BIGNUMERIC"]. If (precision,scale) is: * (38,9) -> NUMERIC; * (39,9) -> BIGNUMERIC (NUMERIC cannot hold 30 integer digits); * (38,10) -> BIGNUMERIC (NUMERIC cannot hold 10 fractional digits); * (76,38) -> BIGNUMERIC; * (77,38) -> BIGNUMERIC (error if value exeeds supported range). This field cannot contain duplicate types. The order of the types in this field is ignored. For example, ["BIGNUMERIC", "NUMERIC"] is the same as ["NUMERIC", "BIGNUMERIC"] and NUMERIC always takes precedence over BIGNUMERIC. Defaults to ["NUMERIC", "STRING"] for ORC and ["NUMERIC"] for the other file formats.
    #[serde(rename="decimalTargetTypes")]
    
    pub decimal_target_types: Option<Vec<String>>,
    /// Optional. Specifies how source URIs are interpreted for constructing the file set to load. By default source URIs are expanded against the underlying storage. Other options include specifying manifest files. Only applicable to object storage systems.
    #[serde(rename="fileSetSpecType")]
    
    pub file_set_spec_type: Option<String>,
    /// Optional. Additional options if sourceFormat is set to GOOGLE_SHEETS.
    #[serde(rename="googleSheetsOptions")]
    
    pub google_sheets_options: Option<GoogleSheetsOptions>,
    /// Optional. When set, configures hive partitioning support. Not all storage formats support hive partitioning -- requesting hive partitioning on an unsupported format will lead to an error, as will providing an invalid specification.
    #[serde(rename="hivePartitioningOptions")]
    
    pub hive_partitioning_options: Option<HivePartitioningOptions>,
    /// Optional. Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names Google Cloud Bigtable: This setting is ignored. Google Cloud Datastore backups: This setting is ignored. Avro: This setting is ignored. ORC: This setting is ignored. Parquet: This setting is ignored.
    #[serde(rename="ignoreUnknownValues")]
    
    pub ignore_unknown_values: Option<bool>,
    /// Optional. Load option to be used together with source_format newline-delimited JSON to indicate that a variant of JSON is being loaded. To load newline-delimited GeoJSON, specify GEOJSON (and source_format must be set to NEWLINE_DELIMITED_JSON).
    #[serde(rename="jsonExtension")]
    
    pub json_extension: Option<String>,
    /// Optional. Additional properties to set if sourceFormat is set to JSON.
    #[serde(rename="jsonOptions")]
    
    pub json_options: Option<JsonOptions>,
    /// Optional. The maximum number of bad records that BigQuery can ignore when reading data. If the number of bad records exceeds this value, an invalid error is returned in the job result. The default value is 0, which requires that all records are valid. This setting is ignored for Google Cloud Bigtable, Google Cloud Datastore backups, Avro, ORC and Parquet formats.
    #[serde(rename="maxBadRecords")]
    
    pub max_bad_records: Option<i32>,
    /// Optional. Metadata Cache Mode for the table. Set this to enable caching of metadata from external data source.
    #[serde(rename="metadataCacheMode")]
    
    pub metadata_cache_mode: Option<String>,
    /// Optional. ObjectMetadata is used to create Object Tables. Object Tables contain a listing of objects (with their metadata) found at the source_uris. If ObjectMetadata is set, source_format should be omitted. Currently SIMPLE is the only supported Object Metadata type.
    #[serde(rename="objectMetadata")]
    
    pub object_metadata: Option<String>,
    /// Optional. Additional properties to set if sourceFormat is set to PARQUET.
    #[serde(rename="parquetOptions")]
    
    pub parquet_options: Option<ParquetOptions>,
    /// Optional. When creating an external table, the user can provide a reference file with the table schema. This is enabled for the following formats: AVRO, PARQUET, ORC.
    #[serde(rename="referenceFileSchemaUri")]
    
    pub reference_file_schema_uri: Option<String>,
    /// Optional. The schema for the data. Schema is required for CSV and JSON formats if autodetect is not on. Schema is disallowed for Google Cloud Bigtable, Cloud Datastore backups, Avro, ORC and Parquet formats.
    
    pub schema: Option<TableSchema>,
    /// [Required] The data format. For CSV files, specify "CSV". For Google sheets, specify "GOOGLE_SHEETS". For newline-delimited JSON, specify "NEWLINE_DELIMITED_JSON". For Avro files, specify "AVRO". For Google Cloud Datastore backups, specify "DATASTORE_BACKUP". For Apache Iceberg tables, specify "ICEBERG". For ORC files, specify "ORC". For Parquet files, specify "PARQUET". [Beta] For Google Cloud Bigtable, specify "BIGTABLE".
    #[serde(rename="sourceFormat")]
    
    pub source_format: Option<String>,
    /// [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups, exactly one URI can be specified. Also, the '*' wildcard character is not allowed.
    #[serde(rename="sourceUris")]
    
    pub source_uris: Option<Vec<String>>,
}

impl client::Part for ExternalDataConfiguration {}


/// Configures the access a dataset defined in an external metadata storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalDatasetReference {
    /// Required. The connection id that is used to access the external_source. Format: projects/{project_id}/locations/{location_id}/connections/{connection_id}
    
    pub connection: Option<String>,
    /// Required. External source that backs this dataset.
    #[serde(rename="externalSource")]
    
    pub external_source: Option<String>,
}

impl client::Part for ExternalDatasetReference {}


/// The external service cost is a portion of the total cost, these costs are not additive with total_bytes_billed. Moreover, this field only track external service costs that will show up as BigQuery costs (e.g. training BigQuery ML job with google cloud CAIP or Automl Tables services), not other costs which may be accrued by running the query (e.g. reading from Bigtable or Cloud Storage). The external service costs with different billing sku (e.g. CAIP job is charged based on VM usage) are converted to BigQuery billed_bytes and slot_ms with equivalent amount of US dollars. Services may not directly correlate to these metrics, but these are the equivalents for billing purposes. Output only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalServiceCost {
    /// External service cost in terms of bigquery bytes billed.
    #[serde(rename="bytesBilled")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_billed: Option<i64>,
    /// External service cost in terms of bigquery bytes processed.
    #[serde(rename="bytesProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_processed: Option<i64>,
    /// External service name.
    #[serde(rename="externalService")]
    
    pub external_service: Option<String>,
    /// Non-preemptable reserved slots used for external job. For example, reserved slots for Cloua AI Platform job are the VM usages converted to BigQuery slot with equivalent mount of price.
    #[serde(rename="reservedSlotCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub reserved_slot_count: Option<i64>,
    /// External service cost in terms of bigquery slot milliseconds.
    #[serde(rename="slotMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub slot_ms: Option<i64>,
}

impl client::Part for ExternalServiceCost {}


/// Representative value of a single feature within the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureValue {
    /// The categorical feature value.
    #[serde(rename="categoricalValue")]
    
    pub categorical_value: Option<CategoricalValue>,
    /// The feature column name.
    #[serde(rename="featureColumn")]
    
    pub feature_column: Option<String>,
    /// The numerical feature value. This is the centroid value for this feature.
    #[serde(rename="numericalValue")]
    
    pub numerical_value: Option<f64>,
}

impl client::Part for FeatureValue {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy row access policies](RowAccessPolicyGetIamPolicyCall) (request)
/// * [get iam policy tables](TableGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// Response object of GetQueryResults.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get query results jobs](JobGetQueryResultCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetQueryResultsResponse {
    /// Whether the query result was fetched from the query cache.
    #[serde(rename="cacheHit")]
    
    pub cache_hit: Option<bool>,
    /// Output only. The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful. For more information about error messages, see [Error messages](https://cloud.google.com/bigquery/docs/error-messages).
    
    pub errors: Option<Vec<ErrorProto>>,
    /// A hash of this response.
    
    pub etag: Option<String>,
    /// Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available.
    #[serde(rename="jobComplete")]
    
    pub job_complete: Option<bool>,
    /// Reference to the BigQuery Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults).
    #[serde(rename="jobReference")]
    
    pub job_reference: Option<JobReference>,
    /// The resource type of the response.
    
    pub kind: Option<String>,
    /// Output only. The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(rename="numDmlAffectedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_dml_affected_rows: Option<i64>,
    /// A token used for paging results. When this token is non-empty, it indicates additional results are available.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above. Present only when the query completes successfully. The REST-based representation of this data leverages a series of JSON f,v objects for indicating fields and values.
    
    pub rows: Option<Vec<TableRow>>,
    /// The schema of the results. Present only when the query completes successfully.
    
    pub schema: Option<TableSchema>,
    /// The total number of bytes processed for this query.
    #[serde(rename="totalBytesProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes_processed: Option<i64>,
    /// The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results. Present only when the query completes successfully.
    #[serde(rename="totalRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_rows: Option<u64>,
}

impl client::ResponseResult for GetQueryResultsResponse {}


/// Response object of GetServiceAccount
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get service account projects](ProjectGetServiceAccountCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetServiceAccountResponse {
    /// The service account email address.
    
    pub email: Option<String>,
    /// The resource type of the response.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for GetServiceAccountResponse {}


/// Global explanations containing the top most important features after training.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GlobalExplanation {
    /// Class label for this set of global explanations. Will be empty/null for binary logistic and linear regression models. Sorted alphabetically in descending order.
    #[serde(rename="classLabel")]
    
    pub class_label: Option<String>,
    /// A list of the top global explanations. Sorted by absolute value of attribution in descending order.
    
    pub explanations: Option<Vec<Explanation>>,
}

impl client::Part for GlobalExplanation {}


/// Options specific to Google Sheets data sources.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleSheetsOptions {
    /// Optional. Range of a sheet to query from. Only used when non-empty. Typical format: sheet_name!top_left_cell_id:bottom_right_cell_id For example: sheet1!A1:B20
    
    pub range: Option<String>,
    /// Optional. The number of rows at the top of a sheet that BigQuery will skip when reading the data. The default value is 0. This property is useful if you have header rows that should be skipped. When autodetect is on, the behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema.
    #[serde(rename="skipLeadingRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub skip_leading_rows: Option<i64>,
}

impl client::Part for GoogleSheetsOptions {}


/// High cardinality join detailed information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HighCardinalityJoin {
    /// Output only. Count of left input rows.
    #[serde(rename="leftRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub left_rows: Option<i64>,
    /// Output only. Count of the output rows.
    #[serde(rename="outputRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub output_rows: Option<i64>,
    /// Output only. Count of right input rows.
    #[serde(rename="rightRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub right_rows: Option<i64>,
    /// Output only. The index of the join operator in the ExplainQueryStep lists.
    #[serde(rename="stepIndex")]
    
    pub step_index: Option<i32>,
}

impl client::Part for HighCardinalityJoin {}


/// Options for configuring hive partitioning detect.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HivePartitioningOptions {
    /// Output only. For permanent external tables, this field is populated with the hive partition keys in the order they were inferred. The types of the partition keys can be deduced by checking the table schema (which will include the partition keys). Not every API will populate this field in the output. For example, Tables.Get will populate it, but Tables.List will not contain this field.
    
    pub fields: Option<Vec<String>>,
    /// Optional. When set, what mode of hive partitioning to use when reading data. The following modes are supported: * AUTO: automatically infer partition key name(s) and type(s). * STRINGS: automatically infer partition key name(s). All types are strings. * CUSTOM: partition key schema is encoded in the source URI prefix. Not all storage formats support hive partitioning. Requesting hive partitioning on an unsupported format will lead to an error. Currently supported formats are: JSON, CSV, ORC, Avro and Parquet.
    
    pub mode: Option<String>,
    /// Optional. If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified. Note that this field should only be true when creating a permanent external table or querying a temporary external table. Hive-partitioned loads with require_partition_filter explicitly set to true will fail.
    #[serde(rename="requirePartitionFilter")]
    
    pub require_partition_filter: Option<bool>,
    /// Optional. When hive partition detection is requested, a common prefix for all source uris must be required. The prefix must end immediately before the partition key encoding begins. For example, consider files following this data layout: gs://bucket/path_to_table/dt=2019-06-01/country=USA/id=7/file.avro gs://bucket/path_to_table/dt=2019-05-31/country=CA/id=3/file.avro When hive partitioning is requested with either AUTO or STRINGS detection, the common prefix can be either of gs://bucket/path_to_table or gs://bucket/path_to_table/. CUSTOM detection requires encoding the partitioning schema immediately after the common prefix. For CUSTOM, any of * gs://bucket/path_to_table/{dt:DATE}/{country:STRING}/{id:INTEGER} * gs://bucket/path_to_table/{dt:STRING}/{country:STRING}/{id:INTEGER} * gs://bucket/path_to_table/{dt:DATE}/{country:STRING}/{id:STRING} would all be valid source URI prefixes.
    #[serde(rename="sourceUriPrefix")]
    
    pub source_uri_prefix: Option<String>,
}

impl client::Part for HivePartitioningOptions {}


/// Hyperparameter search spaces. These should be a subset of training_options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HparamSearchSpaces {
    /// Activation functions of neural network models.
    #[serde(rename="activationFn")]
    
    pub activation_fn: Option<StringHparamSearchSpace>,
    /// Mini batch sample size.
    #[serde(rename="batchSize")]
    
    pub batch_size: Option<IntHparamSearchSpace>,
    /// Booster type for boosted tree models.
    #[serde(rename="boosterType")]
    
    pub booster_type: Option<StringHparamSearchSpace>,
    /// Subsample ratio of columns for each level for boosted tree models.
    #[serde(rename="colsampleBylevel")]
    
    pub colsample_bylevel: Option<DoubleHparamSearchSpace>,
    /// Subsample ratio of columns for each node(split) for boosted tree models.
    #[serde(rename="colsampleBynode")]
    
    pub colsample_bynode: Option<DoubleHparamSearchSpace>,
    /// Subsample ratio of columns when constructing each tree for boosted tree models.
    #[serde(rename="colsampleBytree")]
    
    pub colsample_bytree: Option<DoubleHparamSearchSpace>,
    /// Dart normalization type for boosted tree models.
    #[serde(rename="dartNormalizeType")]
    
    pub dart_normalize_type: Option<StringHparamSearchSpace>,
    /// Dropout probability for dnn model training and boosted tree models using dart booster.
    
    pub dropout: Option<DoubleHparamSearchSpace>,
    /// Hidden units for neural network models.
    #[serde(rename="hiddenUnits")]
    
    pub hidden_units: Option<IntArrayHparamSearchSpace>,
    /// L1 regularization coefficient.
    #[serde(rename="l1Reg")]
    
    pub l1_reg: Option<DoubleHparamSearchSpace>,
    /// L2 regularization coefficient.
    #[serde(rename="l2Reg")]
    
    pub l2_reg: Option<DoubleHparamSearchSpace>,
    /// Learning rate of training jobs.
    #[serde(rename="learnRate")]
    
    pub learn_rate: Option<DoubleHparamSearchSpace>,
    /// Maximum depth of a tree for boosted tree models.
    #[serde(rename="maxTreeDepth")]
    
    pub max_tree_depth: Option<IntHparamSearchSpace>,
    /// Minimum split loss for boosted tree models.
    #[serde(rename="minSplitLoss")]
    
    pub min_split_loss: Option<DoubleHparamSearchSpace>,
    /// Minimum sum of instance weight needed in a child for boosted tree models.
    #[serde(rename="minTreeChildWeight")]
    
    pub min_tree_child_weight: Option<IntHparamSearchSpace>,
    /// Number of clusters for k-means.
    #[serde(rename="numClusters")]
    
    pub num_clusters: Option<IntHparamSearchSpace>,
    /// Number of latent factors to train on.
    #[serde(rename="numFactors")]
    
    pub num_factors: Option<IntHparamSearchSpace>,
    /// Number of parallel trees for boosted tree models.
    #[serde(rename="numParallelTree")]
    
    pub num_parallel_tree: Option<IntHparamSearchSpace>,
    /// Optimizer of TF models.
    
    pub optimizer: Option<StringHparamSearchSpace>,
    /// Subsample the training data to grow tree to prevent overfitting for boosted tree models.
    
    pub subsample: Option<DoubleHparamSearchSpace>,
    /// Tree construction algorithm for boosted tree models.
    #[serde(rename="treeMethod")]
    
    pub tree_method: Option<StringHparamSearchSpace>,
    /// Hyperparameter for matrix factoration when implicit feedback type is specified.
    #[serde(rename="walsAlpha")]
    
    pub wals_alpha: Option<DoubleHparamSearchSpace>,
}

impl client::Part for HparamSearchSpaces {}


/// Training info of a trial in [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HparamTuningTrial {
    /// Ending time of the trial.
    #[serde(rename="endTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time_ms: Option<i64>,
    /// Error message for FAILED and INFEASIBLE trial.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// Loss computed on the eval data at the end of trial.
    #[serde(rename="evalLoss")]
    
    pub eval_loss: Option<f64>,
    /// Evaluation metrics of this trial calculated on the test data. Empty in Job API.
    #[serde(rename="evaluationMetrics")]
    
    pub evaluation_metrics: Option<EvaluationMetrics>,
    /// Hyperparameter tuning evaluation metrics of this trial calculated on the eval data. Unlike evaluation_metrics, only the fields corresponding to the hparam_tuning_objectives are set.
    #[serde(rename="hparamTuningEvaluationMetrics")]
    
    pub hparam_tuning_evaluation_metrics: Option<EvaluationMetrics>,
    /// The hyperprameters selected for this trial.
    
    pub hparams: Option<TrainingOptions>,
    /// Starting time of the trial.
    #[serde(rename="startTimeMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time_ms: Option<i64>,
    /// The status of the trial.
    
    pub status: Option<String>,
    /// Loss computed on the training data at the end of trial.
    #[serde(rename="trainingLoss")]
    
    pub training_loss: Option<f64>,
    /// 1-based index of the trial.
    #[serde(rename="trialId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub trial_id: Option<i64>,
}

impl client::Part for HparamTuningTrial {}


/// Reason about why no search index was used in the search query (or sub-query).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IndexUnusedReason {
    /// Specifies the base table involved in the reason that no search index was used.
    #[serde(rename="baseTable")]
    
    pub base_table: Option<TableReference>,
    /// Specifies the high-level reason for the scenario when no search index was used.
    
    pub code: Option<String>,
    /// Specifies the name of the unused search index, if available.
    #[serde(rename="indexName")]
    
    pub index_name: Option<String>,
    /// Free form human-readable reason for the scenario when no search index was used.
    
    pub message: Option<String>,
}

impl client::Part for IndexUnusedReason {}


/// Details about the input data change insight.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InputDataChange {
    /// Output only. Records read difference percentage compared to a previous run.
    #[serde(rename="recordsReadDiffPercentage")]
    
    pub records_read_diff_percentage: Option<f32>,
}

impl client::Part for InputDataChange {}


/// An array of int.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntArray {
    /// Elements in the int array.
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub elements: Option<Vec<i64>>,
}

impl client::Part for IntArray {}


/// Search space for int array.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntArrayHparamSearchSpace {
    /// Candidates for the int array parameter.
    
    pub candidates: Option<Vec<IntArray>>,
}

impl client::Part for IntArrayHparamSearchSpace {}


/// Discrete candidates of an int hyperparameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntCandidates {
    /// Candidates for the int parameter in increasing order.
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub candidates: Option<Vec<i64>>,
}

impl client::Part for IntCandidates {}


/// Search space for an int hyperparameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntHparamSearchSpace {
    /// Candidates of the int hyperparameter.
    
    pub candidates: Option<IntCandidates>,
    /// Range of the int hyperparameter.
    
    pub range: Option<IntRange>,
}

impl client::Part for IntHparamSearchSpace {}


/// Range of an int hyperparameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IntRange {
    /// Max value of the int parameter.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max: Option<i64>,
    /// Min value of the int parameter.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min: Option<i64>,
}

impl client::Part for IntRange {}


/// Information about a single iteration of the training run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IterationResult {
    /// Arima result.
    #[serde(rename="arimaResult")]
    
    pub arima_result: Option<ArimaResult>,
    /// Information about top clusters for clustering models.
    #[serde(rename="clusterInfos")]
    
    pub cluster_infos: Option<Vec<ClusterInfo>>,
    /// Time taken to run the iteration in milliseconds.
    #[serde(rename="durationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub duration_ms: Option<i64>,
    /// Loss computed on the eval data at the end of iteration.
    #[serde(rename="evalLoss")]
    
    pub eval_loss: Option<f64>,
    /// Index of the iteration, 0 based.
    
    pub index: Option<i32>,
    /// Learn rate used for this iteration.
    #[serde(rename="learnRate")]
    
    pub learn_rate: Option<f64>,
    /// The information of the principal components.
    #[serde(rename="principalComponentInfos")]
    
    pub principal_component_infos: Option<Vec<PrincipalComponentInfo>>,
    /// Loss computed on the training data at the end of iteration.
    #[serde(rename="trainingLoss")]
    
    pub training_loss: Option<f64>,
}

impl client::Part for IterationResult {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel jobs](JobCancelCall) (none)
/// * [delete jobs](JobDeleteCall) (none)
/// * [get jobs](JobGetCall) (response)
/// * [get query results jobs](JobGetQueryResultCall) (none)
/// * [insert jobs](JobInsertCall) (request|response)
/// * [list jobs](JobListCall) (none)
/// * [query jobs](JobQueryCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    /// Required. Describes the job configuration.
    
    pub configuration: Option<JobConfiguration>,
    /// Output only. A hash of this resource.
    
    pub etag: Option<String>,
    /// Output only. Opaque ID field of the job.
    
    pub id: Option<String>,
    /// Output only. If set, it provides the reason why a Job was created. If not set, it should be treated as the default: REQUESTED. This feature is not yet available. Jobs will always be created.
    #[serde(rename="jobCreationReason")]
    
    pub job_creation_reason: Option<JobCreationReason>,
    /// Optional. Reference describing the unique-per-user name of the job.
    #[serde(rename="jobReference")]
    
    pub job_reference: Option<JobReference>,
    /// Output only. The type of the resource.
    
    pub kind: Option<String>,
    /// Output only. [Full-projection-only] String representation of identity of requesting party. Populated for both first- and third-party identities. Only present for APIs that support third-party identities.
    
    pub principal_subject: Option<String>,
    /// Output only. A URL that can be used to access the resource again.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. Information about the job, including starting time and ending time of the job.
    
    pub statistics: Option<JobStatistics>,
    /// Output only. The status of this job. Examine this value when polling an asynchronous job to see if the job is complete.
    
    pub status: Option<JobStatus>,
    /// Output only. Email address of the user who ran the job.
    
    pub user_email: Option<String>,
}

impl client::RequestValue for Job {}
impl client::Resource for Job {}
impl client::ResponseResult for Job {}


/// Describes format of a jobs cancellation response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel jobs](JobCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobCancelResponse {
    /// The final state of the job.
    
    pub job: Option<Job>,
    /// The resource type of the response.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for JobCancelResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfiguration {
    /// [Pick one] Copies a table.
    
    pub copy: Option<JobConfigurationTableCopy>,
    /// Optional. If set, don't actually run this job. A valid query will return a mostly empty response with some processing statistics, while an invalid query will return the same error it would if it wasn't a dry run. Behavior of non-query jobs is undefined.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// [Pick one] Configures an extract job.
    
    pub extract: Option<JobConfigurationExtract>,
    /// Optional. Job timeout in milliseconds. If this time limit is exceeded, BigQuery might attempt to stop the job.
    #[serde(rename="jobTimeoutMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub job_timeout_ms: Option<i64>,
    /// Output only. The type of the job. Can be QUERY, LOAD, EXTRACT, COPY or UNKNOWN.
    #[serde(rename="jobType")]
    
    pub job_type: Option<String>,
    /// The labels associated with this job. You can use these to organize and group your jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    
    pub labels: Option<HashMap<String, String>>,
    /// [Pick one] Configures a load job.
    
    pub load: Option<JobConfigurationLoad>,
    /// [Pick one] Configures a query job.
    
    pub query: Option<JobConfigurationQuery>,
}

impl client::Part for JobConfiguration {}


/// JobConfigurationExtract configures a job that exports data from a BigQuery table into Google Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationExtract {
    /// Optional. The compression type to use for exported files. Possible values include DEFLATE, GZIP, NONE, SNAPPY, and ZSTD. The default value is NONE. Not all compression formats are support for all file formats. DEFLATE is only supported for Avro. ZSTD is only supported for Parquet. Not applicable when extracting models.
    
    pub compression: Option<String>,
    /// Optional. The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON, PARQUET, or AVRO for tables and ML_TF_SAVED_MODEL or ML_XGBOOST_BOOSTER for models. The default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV. The default value for models is ML_TF_SAVED_MODEL.
    #[serde(rename="destinationFormat")]
    
    pub destination_format: Option<String>,
    /// [Pick one] DEPRECATED: Use destinationUris instead, passing only one URI as necessary. The fully-qualified Google Cloud Storage URI where the extracted table should be written.
    #[serde(rename="destinationUri")]
    
    pub destination_uri: Option<String>,
    /// [Pick one] A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    #[serde(rename="destinationUris")]
    
    pub destination_uris: Option<Vec<String>>,
    /// Optional. When extracting data in CSV format, this defines the delimiter to use between fields in the exported data. Default is ','. Not applicable when extracting models.
    #[serde(rename="fieldDelimiter")]
    
    pub field_delimiter: Option<String>,
    /// Optional. Model extract options only applicable when extracting models.
    #[serde(rename="modelExtractOptions")]
    
    pub model_extract_options: Option<ModelExtractOptions>,
    /// Optional. Whether to print out a header row in the results. Default is true. Not applicable when extracting models.
    #[serde(rename="printHeader")]
    
    pub print_header: Option<bool>,
    /// A reference to the model being exported.
    #[serde(rename="sourceModel")]
    
    pub source_model: Option<ModelReference>,
    /// A reference to the table being exported.
    #[serde(rename="sourceTable")]
    
    pub source_table: Option<TableReference>,
    /// Whether to use logical types when extracting to AVRO format. Not applicable when extracting models.
    #[serde(rename="useAvroLogicalTypes")]
    
    pub use_avro_logical_types: Option<bool>,
}

impl client::Part for JobConfigurationExtract {}


/// JobConfigurationLoad contains the configuration properties for loading data into a destination table.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationLoad {
    /// Optional. Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats.
    #[serde(rename="allowJaggedRows")]
    
    pub allow_jagged_rows: Option<bool>,
    /// Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
    #[serde(rename="allowQuotedNewlines")]
    
    pub allow_quoted_newlines: Option<bool>,
    /// Optional. Indicates if we should automatically infer the options and schema for CSV and JSON sources.
    
    pub autodetect: Option<bool>,
    /// Clustering specification for the destination table.
    
    pub clustering: Option<Clustering>,
    /// Optional. Connection properties which can modify the load job behavior. Currently, only the 'session_id' connection property is supported, and is used to resolve _SESSION appearing as the dataset id.
    #[serde(rename="connectionProperties")]
    
    pub connection_properties: Option<Vec<ConnectionProperty>>,
    /// Optional. [Experimental] Configures the load job to only copy files to the destination BigLake managed table with an external storage_uri, without reading file content and writing them to new files. Copying files only is supported when: * source_uris are in the same external storage system as the destination table but they do not overlap with storage_uri of the destination table. * source_format is the same file format as the destination table. * destination_table is an existing BigLake managed table. Its schema does not have default value expression. It schema does not have type parameters other than precision and scale. * No options other than the above are specified.
    #[serde(rename="copyFilesOnly")]
    
    pub copy_files_only: Option<bool>,
    /// Optional. Specifies whether the job is allowed to create new tables. The following values are supported: * CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. * CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="createDisposition")]
    
    pub create_disposition: Option<String>,
    /// Optional. If this property is true, the job creates a new session using a randomly generated session_id. To continue using a created session with subsequent queries, pass the existing session identifier as a `ConnectionProperty` value. The session identifier is returned as part of the `SessionInfo` message within the query statistics. The new session's location will be set to `Job.JobReference.location` if it is present, otherwise it's set to the default location based on existing routing logic.
    #[serde(rename="createSession")]
    
    pub create_session: Option<bool>,
    /// Defines the list of possible SQL data types to which the source decimal values are converted. This list and the precision and the scale parameters of the decimal field determine the target type. In the order of NUMERIC, BIGNUMERIC, and STRING, a type is picked if it is in the specified list and if it supports the precision and the scale. STRING supports all precision and scale values. If none of the listed types supports the precision and the scale, the type supporting the widest range in the specified list is picked, and if a value exceeds the supported range when reading the data, an error will be thrown. Example: Suppose the value of this field is ["NUMERIC", "BIGNUMERIC"]. If (precision,scale) is: * (38,9) -> NUMERIC; * (39,9) -> BIGNUMERIC (NUMERIC cannot hold 30 integer digits); * (38,10) -> BIGNUMERIC (NUMERIC cannot hold 10 fractional digits); * (76,38) -> BIGNUMERIC; * (77,38) -> BIGNUMERIC (error if value exeeds supported range). This field cannot contain duplicate types. The order of the types in this field is ignored. For example, ["BIGNUMERIC", "NUMERIC"] is the same as ["NUMERIC", "BIGNUMERIC"] and NUMERIC always takes precedence over BIGNUMERIC. Defaults to ["NUMERIC", "STRING"] for ORC and ["NUMERIC"] for the other file formats.
    #[serde(rename="decimalTargetTypes")]
    
    pub decimal_target_types: Option<Vec<String>>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    #[serde(rename="destinationEncryptionConfiguration")]
    
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    /// [Required] The destination table to load the data into.
    #[serde(rename="destinationTable")]
    
    pub destination_table: Option<TableReference>,
    /// Optional. [Experimental] Properties with which to create the destination table if it is new.
    #[serde(rename="destinationTableProperties")]
    
    pub destination_table_properties: Option<DestinationTableProperties>,
    /// Optional. The character encoding of the data. The supported values are UTF-8, ISO-8859-1, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the `quote` and `fieldDelimiter` properties. If you don't specify an encoding, or if you specify a UTF-8 encoding when the CSV file is not UTF-8 encoded, BigQuery attempts to convert the data to UTF-8. Generally, your data loads successfully, but it may not match byte-for-byte what you expect. To avoid this, specify the correct encoding by using the `--encoding` flag. If BigQuery can't convert a character other than the ASCII `0` character, BigQuery converts the character to the standard Unicode replacement character: �.
    
    pub encoding: Option<String>,
    /// Optional. The separator character for fields in a CSV file. The separator is interpreted as a single byte. For files encoded in ISO-8859-1, any single character can be used as a separator. For files encoded in UTF-8, characters represented in decimal range 1-127 (U+0001-U+007F) can be used without any modification. UTF-8 characters encoded with multiple bytes (i.e. U+0080 and above) will have only the first byte used for separating fields. The remaining bytes will be treated as a part of the field. BigQuery also supports the escape sequence "\t" (U+0009) to specify a tab separator. The default value is comma (",", U+002C).
    #[serde(rename="fieldDelimiter")]
    
    pub field_delimiter: Option<String>,
    /// Optional. Specifies how source URIs are interpreted for constructing the file set to load. By default, source URIs are expanded against the underlying storage. You can also specify manifest files to control how the file set is constructed. This option is only applicable to object storage systems.
    #[serde(rename="fileSetSpecType")]
    
    pub file_set_spec_type: Option<String>,
    /// Optional. When set, configures hive partitioning support. Not all storage formats support hive partitioning -- requesting hive partitioning on an unsupported format will lead to an error, as will providing an invalid specification.
    #[serde(rename="hivePartitioningOptions")]
    
    pub hive_partitioning_options: Option<HivePartitioningOptions>,
    /// Optional. Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names in the table schema Avro, Parquet, ORC: Fields in the file schema that don't exist in the table schema.
    #[serde(rename="ignoreUnknownValues")]
    
    pub ignore_unknown_values: Option<bool>,
    /// Optional. Load option to be used together with source_format newline-delimited JSON to indicate that a variant of JSON is being loaded. To load newline-delimited GeoJSON, specify GEOJSON (and source_format must be set to NEWLINE_DELIMITED_JSON).
    #[serde(rename="jsonExtension")]
    
    pub json_extension: Option<String>,
    /// Optional. The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. The default value is 0, which requires that all records are valid. This is only supported for CSV and NEWLINE_DELIMITED_JSON file formats.
    #[serde(rename="maxBadRecords")]
    
    pub max_bad_records: Option<i32>,
    /// Optional. Specifies a string that represents a null value in a CSV file. For example, if you specify "\N", BigQuery interprets "\N" as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value.
    #[serde(rename="nullMarker")]
    
    pub null_marker: Option<String>,
    /// Optional. Additional properties to set if sourceFormat is set to PARQUET.
    #[serde(rename="parquetOptions")]
    
    pub parquet_options: Option<ParquetOptions>,
    /// Optional. When sourceFormat is set to "CSV", this indicates whether the embedded ASCII control characters (the first 32 characters in the ASCII-table, from '\x00' to '\x1F') are preserved.
    #[serde(rename="preserveAsciiControlCharacters")]
    
    pub preserve_ascii_control_characters: Option<bool>,
    /// If sourceFormat is set to "DATASTORE_BACKUP", indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result.
    #[serde(rename="projectionFields")]
    
    pub projection_fields: Option<Vec<String>>,
    /// Optional. The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true. To include the specific quote character within a quoted value, precede it with an additional matching quote character. For example, if you want to escape the default character ' " ', use ' "" '. @default "
    
    pub quote: Option<String>,
    /// Range partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified.
    #[serde(rename="rangePartitioning")]
    
    pub range_partitioning: Option<RangePartitioning>,
    /// Optional. The user can provide a reference file with the reader schema. This file is only loaded if it is part of source URIs, but is not loaded otherwise. It is enabled for the following formats: AVRO, PARQUET, ORC.
    #[serde(rename="referenceFileSchemaUri")]
    
    pub reference_file_schema_uri: Option<String>,
    /// Optional. The schema for the destination table. The schema can be omitted if the destination table already exists, or if you're loading data from Google Cloud Datastore.
    
    pub schema: Option<TableSchema>,
    /// [Deprecated] The inline schema. For CSV schemas, specify as "Field1:Type1[,Field2:Type2]*". For example, "foo:STRING, bar:INTEGER, baz:FLOAT".
    #[serde(rename="schemaInline")]
    
    pub schema_inline: Option<String>,
    /// [Deprecated] The format of the schemaInline property.
    #[serde(rename="schemaInlineFormat")]
    
    pub schema_inline_format: Option<String>,
    /// Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: * ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. * ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(rename="schemaUpdateOptions")]
    
    pub schema_update_options: Option<Vec<String>>,
    /// Optional. The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped. When autodetect is on, the behavior is the following: * skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected, the row is read as data. Otherwise data is read starting from the second row. * skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row. * skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected, row N is just skipped. Otherwise row N is used to extract column names for the detected schema.
    #[serde(rename="skipLeadingRows")]
    
    pub skip_leading_rows: Option<i32>,
    /// Optional. The format of the data files. For CSV files, specify "CSV". For datastore backups, specify "DATASTORE_BACKUP". For newline-delimited JSON, specify "NEWLINE_DELIMITED_JSON". For Avro, specify "AVRO". For parquet, specify "PARQUET". For orc, specify "ORC". The default value is CSV.
    #[serde(rename="sourceFormat")]
    
    pub source_format: Option<String>,
    /// [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '*' wildcard character is not allowed.
    #[serde(rename="sourceUris")]
    
    pub source_uris: Option<Vec<String>>,
    /// Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified.
    #[serde(rename="timePartitioning")]
    
    pub time_partitioning: Option<TimePartitioning>,
    /// Optional. If sourceFormat is set to "AVRO", indicates whether to interpret logical types as the corresponding BigQuery data type (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER).
    #[serde(rename="useAvroLogicalTypes")]
    
    pub use_avro_logical_types: Option<bool>,
    /// Optional. Specifies the action that occurs if the destination table already exists. The following values are supported: * WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the data, removes the constraints and uses the schema from the load job. * WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. * WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<String>,
}

impl client::Part for JobConfigurationLoad {}


/// JobConfigurationQuery configures a BigQuery query job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationQuery {
    /// Optional. If true and query uses legacy SQL dialect, allows the query to produce arbitrarily large result tables at a slight cost in performance. Requires destinationTable to be set. For GoogleSQL queries, this flag is ignored and large results are always allowed. However, you must still set destinationTable when result size exceeds the allowed maximum response size.
    #[serde(rename="allowLargeResults")]
    
    pub allow_large_results: Option<bool>,
    /// Clustering specification for the destination table.
    
    pub clustering: Option<Clustering>,
    /// Connection properties which can modify the query behavior.
    #[serde(rename="connectionProperties")]
    
    pub connection_properties: Option<Vec<ConnectionProperty>>,
    /// [Optional] Specifies whether the query should be executed as a continuous query. The default value is false.
    
    pub continuous: Option<bool>,
    /// Optional. Specifies whether the job is allowed to create new tables. The following values are supported: * CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. * CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="createDisposition")]
    
    pub create_disposition: Option<String>,
    /// If this property is true, the job creates a new session using a randomly generated session_id. To continue using a created session with subsequent queries, pass the existing session identifier as a `ConnectionProperty` value. The session identifier is returned as part of the `SessionInfo` message within the query statistics. The new session's location will be set to `Job.JobReference.location` if it is present, otherwise it's set to the default location based on existing routing logic.
    #[serde(rename="createSession")]
    
    pub create_session: Option<bool>,
    /// Optional. Specifies the default dataset to use for unqualified table names in the query. This setting does not alter behavior of unqualified dataset names. Setting the system variable `@@dataset_id` achieves the same behavior. See https://cloud.google.com/bigquery/docs/reference/system-variables for more information on system variables.
    #[serde(rename="defaultDataset")]
    
    pub default_dataset: Option<DatasetReference>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    #[serde(rename="destinationEncryptionConfiguration")]
    
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    /// Optional. Describes the table where the query results should be stored. This property must be set for large results that exceed the maximum response size. For queries that produce anonymous (cached) results, this field will be populated by BigQuery.
    #[serde(rename="destinationTable")]
    
    pub destination_table: Option<TableReference>,
    /// Optional. If true and query uses legacy SQL dialect, flattens all nested and repeated fields in the query results. allowLargeResults must be true if this is set to false. For GoogleSQL queries, this flag is ignored and results are never flattened.
    #[serde(rename="flattenResults")]
    
    pub flatten_results: Option<bool>,
    /// Optional. [Deprecated] Maximum billing tier allowed for this query. The billing tier controls the amount of compute resources allotted to the query, and multiplies the on-demand cost of the query accordingly. A query that runs within its allotted resources will succeed and indicate its billing tier in statistics.query.billingTier, but if the query exceeds its allotted resources, it will fail with billingTierLimitExceeded. WARNING: The billed byte amount can be multiplied by an amount up to this number! Most users should not need to alter this setting, and we recommend that you avoid introducing new uses of it.
    #[serde(rename="maximumBillingTier")]
    
    pub maximum_billing_tier: Option<i32>,
    /// Limits the bytes billed for this job. Queries that will have bytes billed beyond this limit will fail (without incurring a charge). If unspecified, this will be set to your project default.
    #[serde(rename="maximumBytesBilled")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_bytes_billed: Option<i64>,
    /// GoogleSQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[serde(rename="parameterMode")]
    
    pub parameter_mode: Option<String>,
    /// [Deprecated] This property is deprecated.
    #[serde(rename="preserveNulls")]
    
    pub preserve_nulls: Option<bool>,
    /// Optional. Specifies a priority for the query. Possible values include INTERACTIVE and BATCH. The default value is INTERACTIVE.
    
    pub priority: Option<String>,
    /// [Required] SQL query text to execute. The useLegacySql field can be used to indicate whether the query uses legacy SQL or GoogleSQL.
    
    pub query: Option<String>,
    /// Query parameters for GoogleSQL queries.
    #[serde(rename="queryParameters")]
    
    pub query_parameters: Option<Vec<QueryParameter>>,
    /// Range partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified.
    #[serde(rename="rangePartitioning")]
    
    pub range_partitioning: Option<RangePartitioning>,
    /// Allows the schema of the destination table to be updated as a side effect of the query job. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: * ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. * ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(rename="schemaUpdateOptions")]
    
    pub schema_update_options: Option<Vec<String>>,
    /// Options controlling the execution of scripts.
    #[serde(rename="scriptOptions")]
    
    pub script_options: Option<ScriptOptions>,
    /// Output only. System variables for GoogleSQL queries. A system variable is output if the variable is settable and its value differs from the system default. "@@" prefix is not included in the name of the System variables.
    #[serde(rename="systemVariables")]
    
    pub system_variables: Option<SystemVariables>,
    /// Optional. You can specify external table definitions, which operate as ephemeral tables that can be queried. These definitions are configured using a JSON map, where the string key represents the table identifier, and the value is the corresponding external data configuration object.
    #[serde(rename="tableDefinitions")]
    
    pub table_definitions: Option<HashMap<String, ExternalDataConfiguration>>,
    /// Time-based partitioning specification for the destination table. Only one of timePartitioning and rangePartitioning should be specified.
    #[serde(rename="timePartitioning")]
    
    pub time_partitioning: Option<TimePartitioning>,
    /// Optional. Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's GoogleSQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    #[serde(rename="useLegacySql")]
    
    pub use_legacy_sql: Option<bool>,
    /// Optional. Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. Moreover, the query cache is only available when a query does not have a destination table specified. The default value is true.
    #[serde(rename="useQueryCache")]
    
    pub use_query_cache: Option<bool>,
    /// Describes user-defined function resources used in the query.
    #[serde(rename="userDefinedFunctionResources")]
    
    pub user_defined_function_resources: Option<Vec<UserDefinedFunctionResource>>,
    /// Optional. Specifies the action that occurs if the destination table already exists. The following values are supported: * WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the data, removes the constraints, and uses the schema from the query result. * WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. * WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<String>,
}

impl client::Part for JobConfigurationQuery {}


/// JobConfigurationTableCopy configures a job that copies data from one table to another. For more information on copying tables, see [Copy a table](https://cloud.google.com/bigquery/docs/managing-tables#copy-table).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobConfigurationTableCopy {
    /// Optional. Specifies whether the job is allowed to create new tables. The following values are supported: * CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. * CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="createDisposition")]
    
    pub create_disposition: Option<String>,
    /// Custom encryption configuration (e.g., Cloud KMS keys).
    #[serde(rename="destinationEncryptionConfiguration")]
    
    pub destination_encryption_configuration: Option<EncryptionConfiguration>,
    /// Optional. The time when the destination table expires. Expired tables will be deleted and their storage reclaimed.
    #[serde(rename="destinationExpirationTime")]
    
    pub destination_expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Required] The destination table.
    #[serde(rename="destinationTable")]
    
    pub destination_table: Option<TableReference>,
    /// Optional. Supported operation types in table copy job.
    #[serde(rename="operationType")]
    
    pub operation_type: Option<String>,
    /// [Pick one] Source table to copy.
    #[serde(rename="sourceTable")]
    
    pub source_table: Option<TableReference>,
    /// [Pick one] Source tables to copy.
    #[serde(rename="sourceTables")]
    
    pub source_tables: Option<Vec<TableReference>>,
    /// Optional. Specifies the action that occurs if the destination table already exists. The following values are supported: * WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema and table constraints from the source table. * WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. * WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_EMPTY. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename="writeDisposition")]
    
    pub write_disposition: Option<String>,
}

impl client::Part for JobConfigurationTableCopy {}


/// Reason about why a Job was created from a [`jobs.query`](https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query) method when used with `JOB_CREATION_OPTIONAL` Job creation mode. For [`jobs.insert`](https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/insert) method calls it will always be `REQUESTED`. This feature is not yet available. Jobs will always be created.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobCreationReason {
    /// Output only. Specifies the high level reason why a Job was created.
    
    pub code: Option<String>,
}

impl client::Part for JobCreationReason {}


/// JobList is the response format for a jobs.list call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list jobs](JobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobList {
    /// A hash of this page of results.
    
    pub etag: Option<String>,
    /// List of jobs that were requested.
    
    pub jobs: Option<Vec<JobListJobs>>,
    /// The resource type of the response.
    
    pub kind: Option<String>,
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of skipped locations that were unreachable. For more information about BigQuery locations, see: https://cloud.google.com/bigquery/docs/locations. Example: "europe-west5"
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for JobList {}


/// A job reference is a fully qualified identifier for referring to a job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobReference {
    /// Required. The ID of the job. The ID must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-). The maximum length is 1,024 characters.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Optional. The geographic location of the job. The default value is US. For more information about BigQuery locations, see: https://cloud.google.com/bigquery/docs/locations
    
    pub location: Option<String>,
    /// Required. The ID of the project containing this job.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for JobReference {}


/// Statistics for a single job execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics {
    /// Output only. [TrustedTester] Job progress (0.0 -> 1.0) for LOAD and EXTRACT jobs.
    #[serde(rename="completionRatio")]
    
    pub completion_ratio: Option<f64>,
    /// Output only. Statistics for a copy job.
    
    pub copy: Option<JobStatistics5>,
    /// Output only. Creation time of this job, in milliseconds since the epoch. This field will be present on all jobs.
    #[serde(rename="creationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// Output only. Statistics for data-masking. Present only for query and extract jobs.
    #[serde(rename="dataMaskingStatistics")]
    
    pub data_masking_statistics: Option<DataMaskingStatistics>,
    /// Output only. End time of this job, in milliseconds since the epoch. This field will be present whenever a job is in the DONE state.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<i64>,
    /// Output only. Statistics for an extract job.
    
    pub extract: Option<JobStatistics4>,
    /// Output only. The duration in milliseconds of the execution of the final attempt of this job, as BigQuery may internally re-attempt to execute the job.
    #[serde(rename="finalExecutionDurationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub final_execution_duration_ms: Option<i64>,
    /// Output only. Statistics for a load job.
    
    pub load: Option<JobStatistics3>,
    /// Output only. Number of child jobs executed.
    #[serde(rename="numChildJobs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_child_jobs: Option<i64>,
    /// Output only. If this is a child job, specifies the job ID of the parent.
    #[serde(rename="parentJobId")]
    
    pub parent_job_id: Option<String>,
    /// Output only. Statistics for a query job.
    
    pub query: Option<JobStatistics2>,
    /// Output only. Quotas which delayed this job's start time.
    #[serde(rename="quotaDeferments")]
    
    pub quota_deferments: Option<Vec<String>>,
    /// Output only. Job resource usage breakdown by reservation. This field reported misleading information and will no longer be populated.
    #[serde(rename="reservationUsage")]
    
    pub reservation_usage: Option<Vec<JobStatisticsReservationUsage>>,
    /// Output only. Name of the primary reservation assigned to this job. Note that this could be different than reservations reported in the reservation usage field if parent reservations were used to execute this job.
    
    pub reservation_id: Option<String>,
    /// Output only. Statistics for row-level security. Present only for query and extract jobs.
    #[serde(rename="rowLevelSecurityStatistics")]
    
    pub row_level_security_statistics: Option<RowLevelSecurityStatistics>,
    /// Output only. If this a child job of a script, specifies information about the context of this job within the script.
    #[serde(rename="scriptStatistics")]
    
    pub script_statistics: Option<ScriptStatistics>,
    /// Output only. Information of the session if this job is part of one.
    #[serde(rename="sessionInfo")]
    
    pub session_info: Option<SessionInfo>,
    /// Output only. Start time of this job, in milliseconds since the epoch. This field will be present when the job transitions from the PENDING state to either RUNNING or DONE.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<i64>,
    /// Output only. Total bytes processed for the job.
    #[serde(rename="totalBytesProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes_processed: Option<i64>,
    /// Output only. Slot-milliseconds for the job.
    #[serde(rename="totalSlotMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_slot_ms: Option<i64>,
    /// Output only. [Alpha] Information of the multi-statement transaction if this job is part of one. This property is only expected on a child job or a job that is in a session. A script parent job is not part of the transaction started in the script.
    #[serde(rename="transactionInfo")]
    
    pub transaction_info: Option<TransactionInfo>,
}

impl client::Part for JobStatistics {}


/// Statistics for a query job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics2 {
    /// Output only. BI Engine specific Statistics.
    #[serde(rename="biEngineStatistics")]
    
    pub bi_engine_statistics: Option<BiEngineStatistics>,
    /// Output only. Billing tier for the job. This is a BigQuery-specific concept which is not related to the Google Cloud notion of "free tier". The value here is a measure of the query's resource consumption relative to the amount of data scanned. For on-demand queries, the limit is 100, and all queries within this limit are billed at the standard on-demand rates. On-demand queries that exceed this limit will fail with a billingTierLimitExceeded error.
    #[serde(rename="billingTier")]
    
    pub billing_tier: Option<i32>,
    /// Output only. Whether the query result was fetched from the query cache.
    #[serde(rename="cacheHit")]
    
    pub cache_hit: Option<bool>,
    /// Output only. Referenced dataset for DCL statement.
    #[serde(rename="dclTargetDataset")]
    
    pub dcl_target_dataset: Option<DatasetReference>,
    /// Output only. Referenced table for DCL statement.
    #[serde(rename="dclTargetTable")]
    
    pub dcl_target_table: Option<TableReference>,
    /// Output only. Referenced view for DCL statement.
    #[serde(rename="dclTargetView")]
    
    pub dcl_target_view: Option<TableReference>,
    /// Output only. The number of row access policies affected by a DDL statement. Present only for DROP ALL ROW ACCESS POLICIES queries.
    #[serde(rename="ddlAffectedRowAccessPolicyCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub ddl_affected_row_access_policy_count: Option<i64>,
    /// Output only. The table after rename. Present only for ALTER TABLE RENAME TO query.
    #[serde(rename="ddlDestinationTable")]
    
    pub ddl_destination_table: Option<TableReference>,
    /// Output only. The DDL operation performed, possibly dependent on the pre-existence of the DDL target.
    #[serde(rename="ddlOperationPerformed")]
    
    pub ddl_operation_performed: Option<String>,
    /// Output only. The DDL target dataset. Present only for CREATE/ALTER/DROP SCHEMA(dataset) queries.
    #[serde(rename="ddlTargetDataset")]
    
    pub ddl_target_dataset: Option<DatasetReference>,
    /// Output only. [Beta] The DDL target routine. Present only for CREATE/DROP FUNCTION/PROCEDURE queries.
    #[serde(rename="ddlTargetRoutine")]
    
    pub ddl_target_routine: Option<RoutineReference>,
    /// Output only. The DDL target row access policy. Present only for CREATE/DROP ROW ACCESS POLICY queries.
    #[serde(rename="ddlTargetRowAccessPolicy")]
    
    pub ddl_target_row_access_policy: Option<RowAccessPolicyReference>,
    /// Output only. The DDL target table. Present only for CREATE/DROP TABLE/VIEW and DROP ALL ROW ACCESS POLICIES queries.
    #[serde(rename="ddlTargetTable")]
    
    pub ddl_target_table: Option<TableReference>,
    /// Output only. Detailed statistics for DML statements INSERT, UPDATE, DELETE, MERGE or TRUNCATE.
    #[serde(rename="dmlStats")]
    
    pub dml_stats: Option<DmlStatistics>,
    /// Output only. The original estimate of bytes processed for the job.
    #[serde(rename="estimatedBytesProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_bytes_processed: Option<i64>,
    /// Output only. Stats for EXPORT DATA statement.
    #[serde(rename="exportDataStatistics")]
    
    pub export_data_statistics: Option<ExportDataStatistics>,
    /// Output only. Job cost breakdown as bigquery internal cost and external service costs.
    #[serde(rename="externalServiceCosts")]
    
    pub external_service_costs: Option<Vec<ExternalServiceCost>>,
    /// Output only. Statistics for a LOAD query.
    #[serde(rename="loadQueryStatistics")]
    
    pub load_query_statistics: Option<LoadQueryStatistics>,
    /// Output only. Statistics of materialized views of a query job.
    #[serde(rename="materializedViewStatistics")]
    
    pub materialized_view_statistics: Option<MaterializedViewStatistics>,
    /// Output only. Statistics of metadata cache usage in a query for BigLake tables.
    #[serde(rename="metadataCacheStatistics")]
    
    pub metadata_cache_statistics: Option<MetadataCacheStatistics>,
    /// Output only. Statistics of a BigQuery ML training job.
    #[serde(rename="mlStatistics")]
    
    pub ml_statistics: Option<MlStatistics>,
    /// Deprecated.
    #[serde(rename="modelTraining")]
    
    pub model_training: Option<BigQueryModelTraining>,
    /// Deprecated.
    #[serde(rename="modelTrainingCurrentIteration")]
    
    pub model_training_current_iteration: Option<i32>,
    /// Deprecated.
    #[serde(rename="modelTrainingExpectedTotalIteration")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub model_training_expected_total_iteration: Option<i64>,
    /// Output only. The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(rename="numDmlAffectedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_dml_affected_rows: Option<i64>,
    /// Output only. Performance insights.
    #[serde(rename="performanceInsights")]
    
    pub performance_insights: Option<PerformanceInsights>,
    /// Output only. Query optimization information for a QUERY job.
    #[serde(rename="queryInfo")]
    
    pub query_info: Option<QueryInfo>,
    /// Output only. Describes execution plan for the query.
    #[serde(rename="queryPlan")]
    
    pub query_plan: Option<Vec<ExplainQueryStage>>,
    /// Output only. Referenced routines for the job.
    #[serde(rename="referencedRoutines")]
    
    pub referenced_routines: Option<Vec<RoutineReference>>,
    /// Output only. Referenced tables for the job. Queries that reference more than 50 tables will not have a complete list.
    #[serde(rename="referencedTables")]
    
    pub referenced_tables: Option<Vec<TableReference>>,
    /// Output only. Job resource usage breakdown by reservation. This field reported misleading information and will no longer be populated.
    #[serde(rename="reservationUsage")]
    
    pub reservation_usage: Option<Vec<JobStatistics2ReservationUsage>>,
    /// Output only. The schema of the results. Present only for successful dry run of non-legacy SQL queries.
    
    pub schema: Option<TableSchema>,
    /// Output only. Search query specific statistics.
    #[serde(rename="searchStatistics")]
    
    pub search_statistics: Option<SearchStatistics>,
    /// Output only. Statistics of a Spark procedure job.
    #[serde(rename="sparkStatistics")]
    
    pub spark_statistics: Option<SparkStatistics>,
    /// Output only. The type of query statement, if valid. Possible values: * `SELECT`: [`SELECT`](https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_list) statement. * `ASSERT`: [`ASSERT`](https://cloud.google.com/bigquery/docs/reference/standard-sql/debugging-statements#assert) statement. * `INSERT`: [`INSERT`](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#insert_statement) statement. * `UPDATE`: [`UPDATE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#update_statement) statement. * `DELETE`: [`DELETE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language) statement. * `MERGE`: [`MERGE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-manipulation-language) statement. * `CREATE_TABLE`: [`CREATE TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_table_statement) statement, without `AS SELECT`. * `CREATE_TABLE_AS_SELECT`: [`CREATE TABLE AS SELECT`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#query_statement) statement. * `CREATE_VIEW`: [`CREATE VIEW`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_view_statement) statement. * `CREATE_MODEL`: [`CREATE MODEL`](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-create#create_model_statement) statement. * `CREATE_MATERIALIZED_VIEW`: [`CREATE MATERIALIZED VIEW`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_materialized_view_statement) statement. * `CREATE_FUNCTION`: [`CREATE FUNCTION`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_function_statement) statement. * `CREATE_TABLE_FUNCTION`: [`CREATE TABLE FUNCTION`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_table_function_statement) statement. * `CREATE_PROCEDURE`: [`CREATE PROCEDURE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_procedure) statement. * `CREATE_ROW_ACCESS_POLICY`: [`CREATE ROW ACCESS POLICY`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_row_access_policy_statement) statement. * `CREATE_SCHEMA`: [`CREATE SCHEMA`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_schema_statement) statement. * `CREATE_SNAPSHOT_TABLE`: [`CREATE SNAPSHOT TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_snapshot_table_statement) statement. * `CREATE_SEARCH_INDEX`: [`CREATE SEARCH INDEX`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_search_index_statement) statement. * `DROP_TABLE`: [`DROP TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_table_statement) statement. * `DROP_EXTERNAL_TABLE`: [`DROP EXTERNAL TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_external_table_statement) statement. * `DROP_VIEW`: [`DROP VIEW`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_view_statement) statement. * `DROP_MODEL`: [`DROP MODEL`](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-drop-model) statement. * `DROP_MATERIALIZED_VIEW`: [`DROP MATERIALIZED VIEW`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_materialized_view_statement) statement. * `DROP_FUNCTION` : [`DROP FUNCTION`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_function_statement) statement. * `DROP_TABLE_FUNCTION` : [`DROP TABLE FUNCTION`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_table_function) statement. * `DROP_PROCEDURE`: [`DROP PROCEDURE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_procedure_statement) statement. * `DROP_SEARCH_INDEX`: [`DROP SEARCH INDEX`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_search_index) statement. * `DROP_SCHEMA`: [`DROP SCHEMA`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_schema_statement) statement. * `DROP_SNAPSHOT_TABLE`: [`DROP SNAPSHOT TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_snapshot_table_statement) statement. * `DROP_ROW_ACCESS_POLICY`: [`DROP [ALL] ROW ACCESS POLICY|POLICIES`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#drop_row_access_policy_statement) statement. * `ALTER_TABLE`: [`ALTER TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#alter_table_set_options_statement) statement. * `ALTER_VIEW`: [`ALTER VIEW`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#alter_view_set_options_statement) statement. * `ALTER_MATERIALIZED_VIEW`: [`ALTER MATERIALIZED VIEW`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#alter_materialized_view_set_options_statement) statement. * `ALTER_SCHEMA`: [`ALTER SCHEMA`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#aalter_schema_set_options_statement) statement. * `SCRIPT`: [`SCRIPT`](https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language). * `TRUNCATE_TABLE`: [`TRUNCATE TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#truncate_table_statement) statement. * `CREATE_EXTERNAL_TABLE`: [`CREATE EXTERNAL TABLE`](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_external_table_statement) statement. * `EXPORT_DATA`: [`EXPORT DATA`](https://cloud.google.com/bigquery/docs/reference/standard-sql/other-statements#export_data_statement) statement. * `EXPORT_MODEL`: [`EXPORT MODEL`](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-export-model) statement. * `LOAD_DATA`: [`LOAD DATA`](https://cloud.google.com/bigquery/docs/reference/standard-sql/other-statements#load_data_statement) statement. * `CALL`: [`CALL`](https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#call) statement.
    #[serde(rename="statementType")]
    
    pub statement_type: Option<String>,
    /// Output only. Describes a timeline of job execution.
    
    pub timeline: Option<Vec<QueryTimelineSample>>,
    /// Output only. If the project is configured to use on-demand pricing, then this field contains the total bytes billed for the job. If the project is configured to use flat-rate pricing, then you are not billed for bytes and this field is informational only.
    #[serde(rename="totalBytesBilled")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes_billed: Option<i64>,
    /// Output only. Total bytes processed for the job.
    #[serde(rename="totalBytesProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes_processed: Option<i64>,
    /// Output only. For dry-run jobs, totalBytesProcessed is an estimate and this field specifies the accuracy of the estimate. Possible values can be: UNKNOWN: accuracy of the estimate is unknown. PRECISE: estimate is precise. LOWER_BOUND: estimate is lower bound of what the query would cost. UPPER_BOUND: estimate is upper bound of what the query would cost.
    #[serde(rename="totalBytesProcessedAccuracy")]
    
    pub total_bytes_processed_accuracy: Option<String>,
    /// Output only. Total number of partitions processed from all partitioned tables referenced in the job.
    #[serde(rename="totalPartitionsProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_partitions_processed: Option<i64>,
    /// Output only. Slot-milliseconds for the job.
    #[serde(rename="totalSlotMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_slot_ms: Option<i64>,
    /// Output only. Total bytes transferred for cross-cloud queries such as Cross Cloud Transfer and CREATE TABLE AS SELECT (CTAS).
    #[serde(rename="transferredBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub transferred_bytes: Option<i64>,
    /// Output only. GoogleSQL only: list of undeclared query parameters detected during a dry run validation.
    #[serde(rename="undeclaredQueryParameters")]
    
    pub undeclared_query_parameters: Option<Vec<QueryParameter>>,
    /// Output only. Vector Search query specific statistics.
    #[serde(rename="vectorSearchStatistics")]
    
    pub vector_search_statistics: Option<VectorSearchStatistics>,
}

impl client::Part for JobStatistics2 {}


/// Statistics for a load job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics3 {
    /// Output only. The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data.
    #[serde(rename="badRecords")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bad_records: Option<i64>,
    /// Output only. Number of bytes of source data in a load job.
    #[serde(rename="inputFileBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub input_file_bytes: Option<i64>,
    /// Output only. Number of source files in a load job.
    #[serde(rename="inputFiles")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub input_files: Option<i64>,
    /// Output only. Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change.
    #[serde(rename="outputBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub output_bytes: Option<i64>,
    /// Output only. Number of rows imported in a load job. Note that while an import job is in the running state, this value may change.
    #[serde(rename="outputRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub output_rows: Option<i64>,
    /// Output only. Describes a timeline of job execution.
    
    pub timeline: Option<Vec<QueryTimelineSample>>,
}

impl client::Part for JobStatistics3 {}


/// Statistics for an extract job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics4 {
    /// Output only. Number of files per destination URI or URI pattern specified in the extract configuration. These values will be in the same order as the URIs specified in the 'destinationUris' field.
    #[serde(rename="destinationUriFileCounts")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub destination_uri_file_counts: Option<Vec<i64>>,
    /// Output only. Number of user bytes extracted into the result. This is the byte count as computed by BigQuery for billing purposes and doesn't have any relationship with the number of actual result bytes extracted in the desired format.
    #[serde(rename="inputBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub input_bytes: Option<i64>,
    /// Output only. Describes a timeline of job execution.
    
    pub timeline: Option<Vec<QueryTimelineSample>>,
}

impl client::Part for JobStatistics4 {}


/// Statistics for a copy job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics5 {
    /// Output only. Number of logical bytes copied to the destination table.
    #[serde(rename="copiedLogicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub copied_logical_bytes: Option<i64>,
    /// Output only. Number of rows copied to the destination table.
    #[serde(rename="copiedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub copied_rows: Option<i64>,
}

impl client::Part for JobStatistics5 {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatus {
    /// Output only. Final error result of the job. If present, indicates that the job has completed and was unsuccessful.
    #[serde(rename="errorResult")]
    
    pub error_result: Option<ErrorProto>,
    /// Output only. The first errors encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has not completed or was unsuccessful.
    
    pub errors: Option<Vec<ErrorProto>>,
    /// Output only. Running state of the job. Valid states include 'PENDING', 'RUNNING', and 'DONE'.
    
    pub state: Option<String>,
}

impl client::Part for JobStatus {}


/// Represents a single JSON object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JsonObject(pub Option<HashMap<String, JsonValue>>);

impl client::Part for JsonObject {}


/// Json Options for load and make external tables.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JsonOptions {
    /// Optional. The character encoding of the data. The supported values are UTF-8, UTF-16BE, UTF-16LE, UTF-32BE, and UTF-32LE. The default value is UTF-8.
    
    pub encoding: Option<String>,
}

impl client::Part for JsonOptions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
/// The contained type is `Option<json::Value>`.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonValue(pub json::Value);

impl Default for JsonValue {
    fn default() -> JsonValue {
        JsonValue(json::Value::Null)
    }
}

impl client::Part for JsonValue {}


/// A dataset source type which refers to another BigQuery dataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinkedDatasetSource {
    /// The source dataset reference contains project numbers and not project ids.
    #[serde(rename="sourceDataset")]
    
    pub source_dataset: Option<DatasetReference>,
}

impl client::Part for LinkedDatasetSource {}


/// Response format for a single page when listing BigQuery ML models.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list models](ModelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    /// Models in the requested dataset. Only the following fields are populated: model_reference, model_type, creation_time, last_modified_time and labels.
    
    pub models: Option<Vec<Model>>,
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListModelsResponse {}


/// Describes the format of a single result page when listing routines.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list routines](RoutineListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRoutinesResponse {
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Routines in the requested dataset. Unless read_mask is set in the request, only the following fields are populated: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, language, and remote_function_options.
    
    pub routines: Option<Vec<Routine>>,
}

impl client::ResponseResult for ListRoutinesResponse {}


/// Response message for the ListRowAccessPolicies method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list row access policies](RowAccessPolicyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListRowAccessPoliciesResponse {
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Row access policies on the requested table.
    #[serde(rename="rowAccessPolicies")]
    
    pub row_access_policies: Option<Vec<RowAccessPolicy>>,
}

impl client::ResponseResult for ListRowAccessPoliciesResponse {}


/// Statistics for a LOAD query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoadQueryStatistics {
    /// Output only. The number of bad records encountered while processing a LOAD query. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data.
    #[serde(rename="badRecords")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bad_records: Option<i64>,
    /// Output only. This field is deprecated. The number of bytes of source data copied over the network for a `LOAD` query. `transferred_bytes` has the canonical value for physical transferred bytes, which is used for BigQuery Omni billing.
    #[serde(rename="bytesTransferred")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub bytes_transferred: Option<i64>,
    /// Output only. Number of bytes of source data in a LOAD query.
    #[serde(rename="inputFileBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub input_file_bytes: Option<i64>,
    /// Output only. Number of source files in a LOAD query.
    #[serde(rename="inputFiles")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub input_files: Option<i64>,
    /// Output only. Size of the loaded data in bytes. Note that while a LOAD query is in the running state, this value may change.
    #[serde(rename="outputBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub output_bytes: Option<i64>,
    /// Output only. Number of rows imported in a LOAD query. Note that while a LOAD query is in the running state, this value may change.
    #[serde(rename="outputRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub output_rows: Option<i64>,
}

impl client::Part for LoadQueryStatistics {}


/// A materialized view considered for a query job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedView {
    /// Whether the materialized view is chosen for the query. A materialized view can be chosen to rewrite multiple parts of the same query. If a materialized view is chosen to rewrite any part of the query, then this field is true, even if the materialized view was not chosen to rewrite others parts.
    
    pub chosen: Option<bool>,
    /// If present, specifies a best-effort estimation of the bytes saved by using the materialized view rather than its base tables.
    #[serde(rename="estimatedBytesSaved")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_bytes_saved: Option<i64>,
    /// If present, specifies the reason why the materialized view was not chosen for the query.
    #[serde(rename="rejectedReason")]
    
    pub rejected_reason: Option<String>,
    /// The candidate materialized view.
    #[serde(rename="tableReference")]
    
    pub table_reference: Option<TableReference>,
}

impl client::Part for MaterializedView {}


/// Definition and configuration of a materialized view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedViewDefinition {
    /// Optional. This option declares authors intention to construct a materialized view that will not be refreshed incrementally.
    #[serde(rename="allowNonIncrementalDefinition")]
    
    pub allow_non_incremental_definition: Option<bool>,
    /// Optional. Enable automatic refresh of the materialized view when the base table is updated. The default value is "true".
    #[serde(rename="enableRefresh")]
    
    pub enable_refresh: Option<bool>,
    /// Output only. The time when this materialized view was last refreshed, in milliseconds since the epoch.
    #[serde(rename="lastRefreshTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_refresh_time: Option<i64>,
    /// [Optional] Max staleness of data that could be returned when materizlized view is queried (formatted as Google SQL Interval type).
    #[serde(rename="maxStaleness")]
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub max_staleness: Option<Vec<u8>>,
    /// Required. A query whose results are persisted.
    
    pub query: Option<String>,
    /// Optional. The maximum frequency at which this materialized view will be refreshed. The default value is "1800000" (30 minutes).
    #[serde(rename="refreshIntervalMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub refresh_interval_ms: Option<i64>,
}

impl client::Part for MaterializedViewDefinition {}


/// Statistics of materialized views considered in a query job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedViewStatistics {
    /// Materialized views considered for the query job. Only certain materialized views are used. For a detailed list, see the child message. If many materialized views are considered, then the list might be incomplete.
    #[serde(rename="materializedView")]
    
    pub materialized_view: Option<Vec<MaterializedView>>,
}

impl client::Part for MaterializedViewStatistics {}


/// Status of a materialized view. The last refresh timestamp status is omitted here, but is present in the MaterializedViewDefinition message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MaterializedViewStatus {
    /// Output only. Error result of the last automatic refresh. If present, indicates that the last automatic refresh was unsuccessful.
    #[serde(rename="lastRefreshStatus")]
    
    pub last_refresh_status: Option<ErrorProto>,
    /// Output only. Refresh watermark of materialized view. The base tables' data were collected into the materialized view cache until this time.
    #[serde(rename="refreshWatermark")]
    
    pub refresh_watermark: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for MaterializedViewStatus {}


/// Statistics for metadata caching in BigLake tables.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetadataCacheStatistics {
    /// Set for the Metadata caching eligible tables referenced in the query.
    #[serde(rename="tableMetadataCacheUsage")]
    
    pub table_metadata_cache_usage: Option<Vec<TableMetadataCacheUsage>>,
}

impl client::Part for MetadataCacheStatistics {}


/// Job statistics specific to a BigQuery ML training job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MlStatistics {
    /// Output only. Trials of a [hyperparameter tuning job](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) sorted by trial_id.
    #[serde(rename="hparamTrials")]
    
    pub hparam_trials: Option<Vec<HparamTuningTrial>>,
    /// Results for all completed iterations. Empty for [hyperparameter tuning jobs](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview).
    #[serde(rename="iterationResults")]
    
    pub iteration_results: Option<Vec<IterationResult>>,
    /// Output only. Maximum number of iterations specified as max_iterations in the 'CREATE MODEL' query. The actual number of iterations may be less than this number due to early stop.
    #[serde(rename="maxIterations")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_iterations: Option<i64>,
    /// Output only. The type of the model that is being trained.
    #[serde(rename="modelType")]
    
    pub model_type: Option<String>,
    /// Output only. Training type of the job.
    #[serde(rename="trainingType")]
    
    pub training_type: Option<String>,
}

impl client::Part for MlStatistics {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete models](ModelDeleteCall) (none)
/// * [get models](ModelGetCall) (response)
/// * [list models](ModelListCall) (none)
/// * [patch models](ModelPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    /// The best trial_id across all training runs.
    #[serde(rename="bestTrialId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub best_trial_id: Option<i64>,
    /// Output only. The time when this model was created, in millisecs since the epoch.
    #[serde(rename="creationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// Output only. The default trial_id to use in TVFs when the trial_id is not passed in. For single-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, this is the best trial ID. For multi-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, this is the smallest trial ID among all Pareto optimal trials.
    #[serde(rename="defaultTrialId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub default_trial_id: Option<i64>,
    /// Optional. A user-friendly description of this model.
    
    pub description: Option<String>,
    /// Custom encryption configuration (e.g., Cloud KMS keys). This shows the encryption configuration of the model data while stored in BigQuery storage. This field can be used with PatchModel to update encryption key for an already encrypted model.
    #[serde(rename="encryptionConfiguration")]
    
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// Output only. A hash of this resource.
    
    pub etag: Option<String>,
    /// Optional. The time when this model expires, in milliseconds since the epoch. If not present, the model will persist indefinitely. Expired models will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created models.
    #[serde(rename="expirationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration_time: Option<i64>,
    /// Output only. Input feature columns for the model inference. If the model is trained with TRANSFORM clause, these are the input of the TRANSFORM clause.
    #[serde(rename="featureColumns")]
    
    pub feature_columns: Option<Vec<StandardSqlField>>,
    /// Optional. A descriptive name for this model.
    #[serde(rename="friendlyName")]
    
    pub friendly_name: Option<String>,
    /// Output only. All hyperparameter search spaces in this model.
    #[serde(rename="hparamSearchSpaces")]
    
    pub hparam_search_spaces: Option<HparamSearchSpaces>,
    /// Output only. Trials of a [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) model sorted by trial_id.
    #[serde(rename="hparamTrials")]
    
    pub hparam_trials: Option<Vec<HparamTuningTrial>>,
    /// Output only. Label columns that were used to train this model. The output of the model will have a "predicted_" prefix to these columns.
    #[serde(rename="labelColumns")]
    
    pub label_columns: Option<Vec<StandardSqlField>>,
    /// The labels associated with this model. You can use these to organize and group your models. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The time when this model was last modified, in millisecs since the epoch.
    #[serde(rename="lastModifiedTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_time: Option<i64>,
    /// Output only. The geographic location where the model resides. This value is inherited from the dataset.
    
    pub location: Option<String>,
    /// Required. Unique identifier for this model.
    #[serde(rename="modelReference")]
    
    pub model_reference: Option<ModelReference>,
    /// Output only. Type of the model resource.
    #[serde(rename="modelType")]
    
    pub model_type: Option<String>,
    /// Output only. For single-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, it only contains the best trial. For multi-objective [hyperparameter tuning](https://cloud.google.com/bigquery-ml/docs/reference/standard-sql/bigqueryml-syntax-hp-tuning-overview) models, it contains all Pareto optimal trials sorted by trial_id.
    #[serde(rename="optimalTrialIds")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub optimal_trial_ids: Option<Vec<i64>>,
    /// Output only. Remote model info
    #[serde(rename="remoteModelInfo")]
    
    pub remote_model_info: Option<RemoteModelInfo>,
    /// Information for all training runs in increasing order of start_time.
    #[serde(rename="trainingRuns")]
    
    pub training_runs: Option<Vec<TrainingRun>>,
    /// Output only. This field will be populated if a TRANSFORM clause was used to train a model. TRANSFORM clause (if used) takes feature_columns as input and outputs transform_columns. transform_columns then are used to train the model.
    #[serde(rename="transformColumns")]
    
    pub transform_columns: Option<Vec<TransformColumn>>,
}

impl client::RequestValue for Model {}
impl client::Resource for Model {}
impl client::ResponseResult for Model {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModelDefinition {
    /// Deprecated.
    #[serde(rename="modelOptions")]
    
    pub model_options: Option<ModelDefinitionModelOptions>,
    /// Deprecated.
    #[serde(rename="trainingRuns")]
    
    pub training_runs: Option<Vec<BqmlTrainingRun>>,
}

impl client::Part for ModelDefinition {}


/// Options related to model extraction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModelExtractOptions {
    /// The 1-based ID of the trial to be exported from a hyperparameter tuning model. If not specified, the trial with id = [Model](https://cloud.google.com/bigquery/docs/reference/rest/v2/models#resource:-model).defaultTrialId is exported. This field is ignored for models not trained with hyperparameter tuning.
    #[serde(rename="trialId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub trial_id: Option<i64>,
}

impl client::Part for ModelExtractOptions {}


/// Id path of a model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModelReference {
    /// Required. The ID of the dataset containing this model.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// Required. The ID of the model. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    #[serde(rename="modelId")]
    
    pub model_id: Option<String>,
    /// Required. The ID of the project containing this model.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for ModelReference {}


/// Evaluation metrics for multi-class classification/classifier models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiClassClassificationMetrics {
    /// Aggregate classification metrics.
    #[serde(rename="aggregateClassificationMetrics")]
    
    pub aggregate_classification_metrics: Option<AggregateClassificationMetrics>,
    /// Confusion matrix at different thresholds.
    #[serde(rename="confusionMatrixList")]
    
    pub confusion_matrix_list: Option<Vec<ConfusionMatrix>>,
}

impl client::Part for MultiClassClassificationMetrics {}


/// Parquet Options for load and make external tables.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ParquetOptions {
    /// Optional. Indicates whether to use schema inference specifically for Parquet LIST logical type.
    #[serde(rename="enableListInference")]
    
    pub enable_list_inference: Option<bool>,
    /// Optional. Indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default.
    #[serde(rename="enumAsString")]
    
    pub enum_as_string: Option<bool>,
}

impl client::Part for ParquetOptions {}


/// Performance insights for the job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceInsights {
    /// Output only. Average execution ms of previous runs. Indicates the job ran slow compared to previous executions. To find previous executions, use INFORMATION_SCHEMA tables and filter jobs with same query hash.
    #[serde(rename="avgPreviousExecutionMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub avg_previous_execution_ms: Option<i64>,
    /// Output only. Query stage performance insights compared to previous runs, for diagnosing performance regression.
    #[serde(rename="stagePerformanceChangeInsights")]
    
    pub stage_performance_change_insights: Option<Vec<StagePerformanceChangeInsight>>,
    /// Output only. Standalone query stage performance insights, for exploring potential improvements.
    #[serde(rename="stagePerformanceStandaloneInsights")]
    
    pub stage_performance_standalone_insights: Option<Vec<StagePerformanceStandaloneInsight>>,
}

impl client::Part for PerformanceInsights {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** `{ "bindings": [ { "role": "roles/resourcemanager.organizationAdmin", "members": [ "user:mike@example.com", "group:admins@example.com", "domain:google.com", "serviceAccount:my-project-id@appspot.gserviceaccount.com" ] }, { "role": "roles/resourcemanager.organizationViewer", "members": [ "user:eve@example.com" ], "condition": { "title": "expirable access", "description": "Does not grant access after Sep 2020", "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')", } } ], "etag": "BwWWja0YfJA=", "version": 3 }` **YAML example:** `bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') etag: BwWWja0YfJA= version: 3` For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy row access policies](RowAccessPolicyGetIamPolicyCall) (response)
/// * [get iam policy tables](TableGetIamPolicyCall) (response)
/// * [set iam policy tables](TableSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::standard_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// Principal component infos, used only for eigen decomposition based models, e.g., PCA. Ordered by explained_variance in the descending order.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrincipalComponentInfo {
    /// The explained_variance is pre-ordered in the descending order to compute the cumulative explained variance ratio.
    #[serde(rename="cumulativeExplainedVarianceRatio")]
    
    pub cumulative_explained_variance_ratio: Option<f64>,
    /// Explained variance by this principal component, which is simply the eigenvalue.
    #[serde(rename="explainedVariance")]
    
    pub explained_variance: Option<f64>,
    /// Explained_variance over the total explained variance.
    #[serde(rename="explainedVarianceRatio")]
    
    pub explained_variance_ratio: Option<f64>,
    /// Id of the principal component.
    #[serde(rename="principalComponentId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub principal_component_id: Option<i64>,
}

impl client::Part for PrincipalComponentInfo {}


/// Represents privacy policy that contains the privacy requirements specified by the data owner. Currently, this is only supported on views.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivacyPolicy {
    /// Optional. Policy used for aggregation thresholds.
    #[serde(rename="aggregationThresholdPolicy")]
    
    pub aggregation_threshold_policy: Option<AggregationThresholdPolicy>,
}

impl client::Part for PrivacyPolicy {}


/// Response object of ListProjects
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list projects](ProjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectList {
    /// A hash of the page of results.
    
    pub etag: Option<String>,
    /// The resource type of the response.
    
    pub kind: Option<String>,
    /// Use this token to request the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Projects to which the user has at least READ access.
    
    pub projects: Option<Vec<ProjectListProjects>>,
    /// The total number of projects in the page. A wrapper is used here because the field should still be in the response when the value is 0.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for ProjectList {}


/// A unique reference to a project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectReference {
    /// Required. ID of the project. Can be either the numeric ID or the assigned ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::Part for ProjectReference {}


/// Query optimization information for a QUERY job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryInfo {
    /// Output only. Information about query optimizations.
    #[serde(rename="optimizationDetails")]
    
    pub optimization_details: Option<HashMap<String, json::Value>>,
}

impl client::Part for QueryInfo {}


/// A parameter given to a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameter {
    /// Optional. If unset, this is a positional parameter. Otherwise, should be unique within a query.
    
    pub name: Option<String>,
    /// Required. The type of this parameter.
    #[serde(rename="parameterType")]
    
    pub parameter_type: Option<QueryParameterType>,
    /// Required. The value of this parameter.
    #[serde(rename="parameterValue")]
    
    pub parameter_value: Option<QueryParameterValue>,
}

impl client::Part for QueryParameter {}


/// The type of a query parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameterType {
    /// Optional. The type of the array's elements, if this is an array.
    #[serde(rename="arrayType")]
    
    pub array_type: Option<Option<Box<QueryParameterType>>>,
    /// Optional. The element type of the range, if this is a range.
    #[serde(rename="rangeElementType")]
    
    pub range_element_type: Option<Option<Box<QueryParameterType>>>,
    /// Optional. The types of the fields of this struct, in order, if this is a struct.
    #[serde(rename="structTypes")]
    
    pub struct_types: Option<Vec<QueryParameterTypeStructTypes>>,
    /// Required. The top level type of this field.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for QueryParameterType {}


/// The value of a query parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameterValue {
    /// Optional. The array values, if this is an array type.
    #[serde(rename="arrayValues")]
    
    pub array_values: Option<Vec<QueryParameterValue>>,
    /// Optional. The range value, if this is a range type.
    #[serde(rename="rangeValue")]
    
    pub range_value: Option<RangeValue>,
    /// The struct field values.
    #[serde(rename="structValues")]
    
    pub struct_values: Option<HashMap<String, QueryParameterValue>>,
    /// Optional. The value of this value, if a simple scalar type.
    
    pub value: Option<String>,
}

impl client::Part for QueryParameterValue {}


/// Describes the format of the jobs.query request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query jobs](JobQueryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryRequest {
    /// Optional. Connection properties which can modify the query behavior.
    #[serde(rename="connectionProperties")]
    
    pub connection_properties: Option<Vec<ConnectionProperty>>,
    /// [Optional] Specifies whether the query should be executed as a continuous query. The default value is false.
    
    pub continuous: Option<bool>,
    /// Optional. If true, creates a new session using a randomly generated session_id. If false, runs query with an existing session_id passed in ConnectionProperty, otherwise runs query in non-session mode. The session location will be set to QueryRequest.location if it is present, otherwise it's set to the default location based on existing routing logic.
    #[serde(rename="createSession")]
    
    pub create_session: Option<bool>,
    /// Optional. Specifies the default datasetId and projectId to assume for any unqualified table names in the query. If not set, all table names in the query string must be qualified in the format 'datasetId.tableId'.
    #[serde(rename="defaultDataset")]
    
    pub default_dataset: Option<DatasetReference>,
    /// Optional. If set to true, BigQuery doesn't run the job. Instead, if the query is valid, BigQuery returns statistics about the job such as how many bytes would be processed. If the query is invalid, an error returns. The default value is false.
    #[serde(rename="dryRun")]
    
    pub dry_run: Option<bool>,
    /// Optional. Output format adjustments.
    #[serde(rename="formatOptions")]
    
    pub format_options: Option<DataFormatOptions>,
    /// Optional. If not set, jobs are always required. If set, the query request will follow the behavior described JobCreationMode. This feature is not yet available. Jobs will always be created.
    #[serde(rename="jobCreationMode")]
    
    pub job_creation_mode: Option<String>,
    /// The resource type of the request.
    
    pub kind: Option<String>,
    /// Optional. The labels associated with this query. Labels can be used to organize and group query jobs. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label keys must start with a letter and each label in the list must have a different key.
    
    pub labels: Option<HashMap<String, String>>,
    /// The geographic location where the job should run. See details at https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    
    pub location: Option<String>,
    /// Optional. The maximum number of rows of data to return per page of results. Setting this flag to a small value such as 1000 and then paging through results might improve reliability when the query result set is large. In addition to this limit, responses are also limited to 10 MB. By default, there is no maximum row count, and only the byte limit applies.
    #[serde(rename="maxResults")]
    
    pub max_results: Option<u32>,
    /// Optional. Limits the bytes billed for this query. Queries with bytes billed above this limit will fail (without incurring a charge). If unspecified, the project default is used.
    #[serde(rename="maximumBytesBilled")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub maximum_bytes_billed: Option<i64>,
    /// GoogleSQL only. Set to POSITIONAL to use positional (?) query parameters or to NAMED to use named (@myparam) query parameters in this query.
    #[serde(rename="parameterMode")]
    
    pub parameter_mode: Option<String>,
    /// This property is deprecated.
    #[serde(rename="preserveNulls")]
    
    pub preserve_nulls: Option<bool>,
    /// Required. A query string to execute, using Google Standard SQL or legacy SQL syntax. Example: "SELECT COUNT(f1) FROM myProjectId.myDatasetId.myTableId".
    
    pub query: Option<String>,
    /// Query parameters for GoogleSQL queries.
    #[serde(rename="queryParameters")]
    
    pub query_parameters: Option<Vec<QueryParameter>>,
    /// Optional. A unique user provided identifier to ensure idempotent behavior for queries. Note that this is different from the job_id. It has the following properties: 1. It is case-sensitive, limited to up to 36 ASCII characters. A UUID is recommended. 2. Read only queries can ignore this token since they are nullipotent by definition. 3. For the purposes of idempotency ensured by the request_id, a request is considered duplicate of another only if they have the same request_id and are actually duplicates. When determining whether a request is a duplicate of another request, all parameters in the request that may affect the result are considered. For example, query, connection_properties, query_parameters, use_legacy_sql are parameters that affect the result and are considered when determining whether a request is a duplicate, but properties like timeout_ms don't affect the result and are thus not considered. Dry run query requests are never considered duplicate of another request. 4. When a duplicate mutating query request is detected, it returns: a. the results of the mutation if it completes successfully within the timeout. b. the running operation if it is still in progress at the end of the timeout. 5. Its lifetime is limited to 15 minutes. In other words, if two requests are sent with the same request_id, but more than 15 minutes apart, idempotency is not guaranteed.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// Optional. Optional: Specifies the maximum amount of time, in milliseconds, that the client is willing to wait for the query to complete. By default, this limit is 10 seconds (10,000 milliseconds). If the query is complete, the jobComplete field in the response is true. If the query has not yet completed, jobComplete is false. You can request a longer timeout period in the timeoutMs field. However, the call is not guaranteed to wait for the specified timeout; it typically returns after around 200 seconds (200,000 milliseconds), even if the query is not complete. If jobComplete is false, you can continue to wait for the query to complete by calling the getQueryResults method until the jobComplete field in the getQueryResults response is true.
    #[serde(rename="timeoutMs")]
    
    pub timeout_ms: Option<u32>,
    /// Specifies whether to use BigQuery's legacy SQL dialect for this query. The default value is true. If set to false, the query will use BigQuery's GoogleSQL: https://cloud.google.com/bigquery/sql-reference/ When useLegacySql is set to false, the value of flattenResults is ignored; query will be run as if flattenResults is false.
    #[serde(rename="useLegacySql")]
    
    pub use_legacy_sql: Option<bool>,
    /// Optional. Whether to look for the result in the query cache. The query cache is a best-effort cache that will be flushed whenever tables in the query are modified. The default value is true.
    #[serde(rename="useQueryCache")]
    
    pub use_query_cache: Option<bool>,
}

impl client::RequestValue for QueryRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [query jobs](JobQueryCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    /// Whether the query result was fetched from the query cache.
    #[serde(rename="cacheHit")]
    
    pub cache_hit: Option<bool>,
    /// Output only. Detailed statistics for DML statements INSERT, UPDATE, DELETE, MERGE or TRUNCATE.
    #[serde(rename="dmlStats")]
    
    pub dml_stats: Option<DmlStatistics>,
    /// Output only. The first errors or warnings encountered during the running of the job. The final message includes the number of errors that caused the process to stop. Errors here do not necessarily mean that the job has completed or was unsuccessful. For more information about error messages, see [Error messages](https://cloud.google.com/bigquery/docs/error-messages).
    
    pub errors: Option<Vec<ErrorProto>>,
    /// Whether the query has completed or not. If rows or totalRows are present, this will always be true. If this is false, totalRows will not be available.
    #[serde(rename="jobComplete")]
    
    pub job_complete: Option<bool>,
    /// Optional. Only relevant when a job_reference is present in the response. If job_reference is not present it will always be unset. When job_reference is present, this field should be interpreted as follows: If set, it will provide the reason of why a Job was created. If not set, it should be treated as the default: REQUESTED. This feature is not yet available. Jobs will always be created.
    #[serde(rename="jobCreationReason")]
    
    pub job_creation_reason: Option<JobCreationReason>,
    /// Reference to the Job that was created to run the query. This field will be present even if the original request timed out, in which case GetQueryResults can be used to read the results once the query has completed. Since this API only returns the first page of results, subsequent pages can be fetched via the same mechanism (GetQueryResults).
    #[serde(rename="jobReference")]
    
    pub job_reference: Option<JobReference>,
    /// The resource type.
    
    pub kind: Option<String>,
    /// Output only. The number of rows affected by a DML statement. Present only for DML statements INSERT, UPDATE or DELETE.
    #[serde(rename="numDmlAffectedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_dml_affected_rows: Option<i64>,
    /// A token used for paging results. A non-empty token indicates that additional results are available. To see additional results, query the [`jobs.getQueryResults`](https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/getQueryResults) method. For more information, see [Paging through table data](https://cloud.google.com/bigquery/docs/paging-results).
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Query ID for the completed query. This ID will be auto-generated. This field is not yet available and it is currently not guaranteed to be populated.
    #[serde(rename="queryId")]
    
    pub query_id: Option<String>,
    /// An object with as many results as can be contained within the maximum permitted reply size. To get any additional rows, you can call GetQueryResults and specify the jobReference returned above.
    
    pub rows: Option<Vec<TableRow>>,
    /// The schema of the results. Present only when the query completes successfully.
    
    pub schema: Option<TableSchema>,
    /// Output only. Information of the session if this job is part of one.
    #[serde(rename="sessionInfo")]
    
    pub session_info: Option<SessionInfo>,
    /// The total number of bytes processed for this query. If this query was a dry run, this is the number of bytes that would be processed if the query were run.
    #[serde(rename="totalBytesProcessed")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes_processed: Option<i64>,
    /// The total number of rows in the complete query result set, which can be more than the number of rows in this single page of results.
    #[serde(rename="totalRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_rows: Option<u64>,
}

impl client::ResponseResult for QueryResponse {}


/// Summary of the state of query execution at a given time.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryTimelineSample {
    /// Total number of active workers. This does not correspond directly to slot usage. This is the largest value observed since the last sample.
    #[serde(rename="activeUnits")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub active_units: Option<i64>,
    /// Total parallel units of work completed by this query.
    #[serde(rename="completedUnits")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub completed_units: Option<i64>,
    /// Milliseconds elapsed since the start of query execution.
    #[serde(rename="elapsedMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub elapsed_ms: Option<i64>,
    /// Units of work that can be scheduled immediately. Providing additional slots for these units of work will accelerate the query, if no other query in the reservation needs additional slots.
    #[serde(rename="estimatedRunnableUnits")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_runnable_units: Option<i64>,
    /// Total units of work remaining for the query. This number can be revised (increased or decreased) while the query is running.
    #[serde(rename="pendingUnits")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub pending_units: Option<i64>,
    /// Cumulative slot-ms consumed by the query.
    #[serde(rename="totalSlotMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_slot_ms: Option<i64>,
}

impl client::Part for QueryTimelineSample {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RangePartitioning {
    /// Required. [Experimental] The table is partitioned by this field. The field must be a top-level NULLABLE/REQUIRED field. The only supported type is INTEGER/INT64.
    
    pub field: Option<String>,
    /// [Experimental] Defines the ranges for range partitioning.
    
    pub range: Option<RangePartitioningRange>,
}

impl client::Part for RangePartitioning {}


/// Represents the value of a range.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RangeValue {
    /// Optional. The end value of the range. A missing value represents an unbounded end.
    
    pub end: Option<QueryParameterValue>,
    /// Optional. The start value of the range. A missing value represents an unbounded start.
    
    pub start: Option<QueryParameterValue>,
}

impl client::Part for RangeValue {}


/// Evaluation metrics used by weighted-ALS models specified by feedback_type=implicit.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RankingMetrics {
    /// Determines the goodness of a ranking by computing the percentile rank from the predicted confidence and dividing it by the original rank.
    #[serde(rename="averageRank")]
    
    pub average_rank: Option<f64>,
    /// Calculates a precision per user for all the items by ranking them and then averages all the precisions across all the users.
    #[serde(rename="meanAveragePrecision")]
    
    pub mean_average_precision: Option<f64>,
    /// Similar to the mean squared error computed in regression and explicit recommendation models except instead of computing the rating directly, the output from evaluate is computed against a preference which is 1 or 0 depending on if the rating exists or not.
    #[serde(rename="meanSquaredError")]
    
    pub mean_squared_error: Option<f64>,
    /// A metric to determine the goodness of a ranking calculated from the predicted confidence by comparing it to an ideal rank measured by the original ratings.
    #[serde(rename="normalizedDiscountedCumulativeGain")]
    
    pub normalized_discounted_cumulative_gain: Option<f64>,
}

impl client::Part for RankingMetrics {}


/// Evaluation metrics for regression and explicit feedback type matrix factorization models.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegressionMetrics {
    /// Mean absolute error.
    #[serde(rename="meanAbsoluteError")]
    
    pub mean_absolute_error: Option<f64>,
    /// Mean squared error.
    #[serde(rename="meanSquaredError")]
    
    pub mean_squared_error: Option<f64>,
    /// Mean squared log error.
    #[serde(rename="meanSquaredLogError")]
    
    pub mean_squared_log_error: Option<f64>,
    /// Median absolute error.
    #[serde(rename="medianAbsoluteError")]
    
    pub median_absolute_error: Option<f64>,
    /// R^2 score. This corresponds to r2_score in ML.EVALUATE.
    #[serde(rename="rSquared")]
    
    pub r_squared: Option<f64>,
}

impl client::Part for RegressionMetrics {}


/// Options for a remote user-defined function.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoteFunctionOptions {
    /// Fully qualified name of the user-provided connection object which holds the authentication information to send requests to the remote service. Format: ```"projects/{projectId}/locations/{locationId}/connections/{connectionId}"```
    
    pub connection: Option<String>,
    /// Endpoint of the user-provided remote service, e.g. ```https://us-east1-my_gcf_project.cloudfunctions.net/remote_add```
    
    pub endpoint: Option<String>,
    /// Max number of rows in each batch sent to the remote service. If absent or if 0, BigQuery dynamically decides the number of rows in a batch.
    #[serde(rename="maxBatchingRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_batching_rows: Option<i64>,
    /// User-defined context as a set of key/value pairs, which will be sent as function invocation context together with batched arguments in the requests to the remote service. The total number of bytes of keys and values must be less than 8KB.
    #[serde(rename="userDefinedContext")]
    
    pub user_defined_context: Option<HashMap<String, String>>,
}

impl client::Part for RemoteFunctionOptions {}


/// Remote Model Info
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoteModelInfo {
    /// Output only. Fully qualified name of the user-provided connection object of the remote model. Format: ```"projects/{project_id}/locations/{location_id}/connections/{connection_id}"```
    
    pub connection: Option<String>,
    /// Output only. The endpoint for remote model.
    
    pub endpoint: Option<String>,
    /// Output only. Max number of rows in each batch sent to the remote service. If unset, the number of rows in each batch is set dynamically.
    #[serde(rename="maxBatchingRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_batching_rows: Option<i64>,
    /// Output only. The model version for LLM.
    #[serde(rename="remoteModelVersion")]
    
    pub remote_model_version: Option<String>,
    /// Output only. The remote service type for remote model.
    #[serde(rename="remoteServiceType")]
    
    pub remote_service_type: Option<String>,
    /// Output only. The name of the speech recognizer to use for speech recognition. The expected format is `projects/{project}/locations/{location}/recognizers/{recognizer}`. Customers can specify this field at model creation. If not specified, a default recognizer `projects/{model project}/locations/global/recognizers/_` will be used. See more details at [recognizers](https://cloud.google.com/speech-to-text/v2/docs/reference/rest/v2/projects.locations.recognizers)
    #[serde(rename="speechRecognizer")]
    
    pub speech_recognizer: Option<String>,
}

impl client::Part for RemoteModelInfo {}


/// A user-defined function or a stored procedure.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete routines](RoutineDeleteCall) (none)
/// * [get routines](RoutineGetCall) (response)
/// * [insert routines](RoutineInsertCall) (request|response)
/// * [list routines](RoutineListCall) (none)
/// * [update routines](RoutineUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Routine {
    /// Optional.
    
    pub arguments: Option<Vec<Argument>>,
    /// Output only. The time when this routine was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// Optional. If set to `DATA_MASKING`, the function is validated and made available as a masking function. For more information, see [Create custom masking routines](https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask).
    #[serde(rename="dataGovernanceType")]
    
    pub data_governance_type: Option<String>,
    /// Required. The body of the routine. For functions, this is the expression in the AS clause. If language=SQL, it is the substring inside (but excluding) the parentheses. For example, for the function created with the following statement: `CREATE FUNCTION JoinLines(x string, y string) as (concat(x, "\n", y))` The definition_body is `concat(x, "\n", y)` (\n is not replaced with linebreak). If language=JAVASCRIPT, it is the evaluated string in the AS clause. For example, for the function created with the following statement: `CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return "\n";\n'` The definition_body is `return "\n";\n` Note that both \n are replaced with linebreaks.
    #[serde(rename="definitionBody")]
    
    pub definition_body: Option<String>,
    /// Optional. The description of the routine, if defined.
    
    pub description: Option<String>,
    /// Optional. The determinism level of the JavaScript UDF, if defined.
    #[serde(rename="determinismLevel")]
    
    pub determinism_level: Option<String>,
    /// Output only. A hash of this resource.
    
    pub etag: Option<String>,
    /// Optional. If language = "JAVASCRIPT", this field stores the path of the imported JAVASCRIPT libraries.
    #[serde(rename="importedLibraries")]
    
    pub imported_libraries: Option<Vec<String>>,
    /// Optional. Defaults to "SQL" if remote_function_options field is absent, not set otherwise.
    
    pub language: Option<String>,
    /// Output only. The time when this routine was last modified, in milliseconds since the epoch.
    #[serde(rename="lastModifiedTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_time: Option<i64>,
    /// Optional. Remote function specific options.
    #[serde(rename="remoteFunctionOptions")]
    
    pub remote_function_options: Option<RemoteFunctionOptions>,
    /// Optional. Can be set only if routine_type = "TABLE_VALUED_FUNCTION". If absent, the return table type is inferred from definition_body at query time in each query that references this routine. If present, then the columns in the evaluated table result will be cast to match the column types specified in return table type, at query time.
    #[serde(rename="returnTableType")]
    
    pub return_table_type: Option<StandardSqlTableType>,
    /// Optional if language = "SQL"; required otherwise. Cannot be set if routine_type = "TABLE_VALUED_FUNCTION". If absent, the return type is inferred from definition_body at query time in each query that references this routine. If present, then the evaluated result will be cast to the specified returned type at query time. For example, for the functions created with the following statements: * `CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);` * `CREATE FUNCTION Increment(x FLOAT64) AS (Add(x, 1));` * `CREATE FUNCTION Decrement(x FLOAT64) RETURNS FLOAT64 AS (Add(x, -1));` The return_type is `{type_kind: "FLOAT64"}` for `Add` and `Decrement`, and is absent for `Increment` (inferred as FLOAT64 at query time). Suppose the function `Add` is replaced by `CREATE OR REPLACE FUNCTION Add(x INT64, y INT64) AS (x + y);` Then the inferred return type of `Increment` is automatically changed to INT64 at query time, while the return type of `Decrement` remains FLOAT64.
    #[serde(rename="returnType")]
    
    pub return_type: Option<StandardSqlDataType>,
    /// Required. Reference describing the ID of this routine.
    #[serde(rename="routineReference")]
    
    pub routine_reference: Option<RoutineReference>,
    /// Required. The type of routine.
    #[serde(rename="routineType")]
    
    pub routine_type: Option<String>,
    /// Optional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration.
    #[serde(rename="securityMode")]
    
    pub security_mode: Option<String>,
    /// Optional. Spark specific options.
    #[serde(rename="sparkOptions")]
    
    pub spark_options: Option<SparkOptions>,
    /// Optional. Use this option to catch many common errors. Error checking is not exhaustive, and successfully creating a procedure doesn't guarantee that the procedure will successfully execute at runtime. If `strictMode` is set to `TRUE`, the procedure body is further checked for errors such as non-existent tables or columns. The `CREATE PROCEDURE` statement fails if the body fails any of these checks. If `strictMode` is set to `FALSE`, the procedure body is checked only for syntax. For procedures that invoke themselves recursively, specify `strictMode=FALSE` to avoid non-existent procedure errors during validation. Default value is `TRUE`.
    #[serde(rename="strictMode")]
    
    pub strict_mode: Option<bool>,
}

impl client::RequestValue for Routine {}
impl client::Resource for Routine {}
impl client::ResponseResult for Routine {}


/// Id path of a routine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RoutineReference {
    /// Required. The ID of the dataset containing this routine.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// Required. The ID of the project containing this routine.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. The ID of the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters.
    #[serde(rename="routineId")]
    
    pub routine_id: Option<String>,
}

impl client::Part for RoutineReference {}


/// A single row in the confusion matrix.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    /// The original label of this row.
    #[serde(rename="actualLabel")]
    
    pub actual_label: Option<String>,
    /// Info describing predicted label distribution.
    
    pub entries: Option<Vec<Entry>>,
}

impl client::Part for Row {}


/// Represents access on a subset of rows on the specified table, defined by its filter predicate. Access to the subset of rows is controlled by its IAM policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RowAccessPolicy {
    /// Output only. The time when this row access policy was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    
    pub creation_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. A hash of this resource.
    
    pub etag: Option<String>,
    /// Required. A SQL boolean expression that represents the rows defined by this row access policy, similar to the boolean expression in a WHERE clause of a SELECT query on a table. References to other tables, routines, and temporary functions are not supported. Examples: region="EU" date_field = CAST('2019-9-27' as DATE) nullable_field is not NULL numeric_field BETWEEN 1.0 AND 5.0
    #[serde(rename="filterPredicate")]
    
    pub filter_predicate: Option<String>,
    /// Output only. The time when this row access policy was last modified, in milliseconds since the epoch.
    #[serde(rename="lastModifiedTime")]
    
    pub last_modified_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Reference describing the ID of this row access policy.
    #[serde(rename="rowAccessPolicyReference")]
    
    pub row_access_policy_reference: Option<RowAccessPolicyReference>,
}

impl client::Part for RowAccessPolicy {}


/// Id path of a row access policy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RowAccessPolicyReference {
    /// Required. The ID of the dataset containing this row access policy.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// Required. The ID of the row access policy. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters.
    #[serde(rename="policyId")]
    
    pub policy_id: Option<String>,
    /// Required. The ID of the project containing this row access policy.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. The ID of the table containing this row access policy.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::Part for RowAccessPolicyReference {}


/// Statistics for row-level security.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RowLevelSecurityStatistics {
    /// Whether any accessed data was protected by row access policies.
    #[serde(rename="rowLevelSecurityApplied")]
    
    pub row_level_security_applied: Option<bool>,
}

impl client::Part for RowLevelSecurityStatistics {}


/// Options related to script execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScriptOptions {
    /// Determines which statement in the script represents the "key result", used to populate the schema and query results of the script job. Default is LAST.
    #[serde(rename="keyResultStatement")]
    
    pub key_result_statement: Option<String>,
    /// Limit on the number of bytes billed per statement. Exceeding this budget results in an error.
    #[serde(rename="statementByteBudget")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub statement_byte_budget: Option<i64>,
    /// Timeout period for each statement in a script.
    #[serde(rename="statementTimeoutMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub statement_timeout_ms: Option<i64>,
}

impl client::Part for ScriptOptions {}


/// Represents the location of the statement/expression being evaluated. Line and column numbers are defined as follows: - Line and column numbers start with one. That is, line 1 column 1 denotes the start of the script. - When inside a stored procedure, all line/column numbers are relative to the procedure body, not the script in which the procedure was defined. - Start/end positions exclude leading/trailing comments and whitespace. The end position always ends with a ";", when present. - Multi-byte Unicode characters are treated as just one column. - If the original script (or procedure definition) contains TAB characters, a tab "snaps" the indentation forward to the nearest multiple of 8 characters, plus 1. For example, a TAB on column 1, 2, 3, 4, 5, 6 , or 8 will advance the next character to column 9. A TAB on column 9, 10, 11, 12, 13, 14, 15, or 16 will advance the next character to column 17.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScriptStackFrame {
    /// Output only. One-based end column.
    #[serde(rename="endColumn")]
    
    pub end_column: Option<i32>,
    /// Output only. One-based end line.
    #[serde(rename="endLine")]
    
    pub end_line: Option<i32>,
    /// Output only. Name of the active procedure, empty if in a top-level script.
    #[serde(rename="procedureId")]
    
    pub procedure_id: Option<String>,
    /// Output only. One-based start column.
    #[serde(rename="startColumn")]
    
    pub start_column: Option<i32>,
    /// Output only. One-based start line.
    #[serde(rename="startLine")]
    
    pub start_line: Option<i32>,
    /// Output only. Text of the current statement/expression.
    
    pub text: Option<String>,
}

impl client::Part for ScriptStackFrame {}


/// Job statistics specific to the child job of a script.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScriptStatistics {
    /// Whether this child job was a statement or expression.
    #[serde(rename="evaluationKind")]
    
    pub evaluation_kind: Option<String>,
    /// Stack trace showing the line/column/procedure name of each frame on the stack at the point where the current evaluation happened. The leaf frame is first, the primary script is last. Never empty.
    #[serde(rename="stackFrames")]
    
    pub stack_frames: Option<Vec<ScriptStackFrame>>,
}

impl client::Part for ScriptStatistics {}


/// Statistics for a search query. Populated as part of JobStatistics2.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchStatistics {
    /// When `indexUsageMode` is `UNUSED` or `PARTIALLY_USED`, this field explains why indexes were not used in all or part of the search query. If `indexUsageMode` is `FULLY_USED`, this field is not populated.
    #[serde(rename="indexUnusedReasons")]
    
    pub index_unused_reasons: Option<Vec<IndexUnusedReason>>,
    /// Specifies the index usage mode for the query.
    #[serde(rename="indexUsageMode")]
    
    pub index_usage_mode: Option<String>,
}

impl client::Part for SearchStatistics {}


/// [Preview] Information related to sessions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Output only. The id of the session.
    #[serde(rename="sessionId")]
    
    pub session_id: Option<String>,
}

impl client::Part for SessionInfo {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set iam policy tables](TableSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Information about base table and snapshot time of the snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotDefinition {
    /// Required. Reference describing the ID of the table that was snapshot.
    #[serde(rename="baseTableReference")]
    
    pub base_table_reference: Option<TableReference>,
    /// Required. The time at which the base table was snapshot. This value is reported in the JSON response using RFC3339 format.
    #[serde(rename="snapshotTime")]
    
    pub snapshot_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for SnapshotDefinition {}


/// Spark job logs can be filtered by these fields in Cloud Logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkLoggingInfo {
    /// Output only. Project ID where the Spark logs were written.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Output only. Resource type used for logging.
    #[serde(rename="resourceType")]
    
    pub resource_type: Option<String>,
}

impl client::Part for SparkLoggingInfo {}


/// Options for a user-defined Spark routine.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkOptions {
    /// Archive files to be extracted into the working directory of each executor. For more information about Apache Spark, see [Apache Spark](https://spark.apache.org/docs/latest/index.html).
    #[serde(rename="archiveUris")]
    
    pub archive_uris: Option<Vec<String>>,
    /// Fully qualified name of the user-provided Spark connection object. Format: ```"projects/{project_id}/locations/{location_id}/connections/{connection_id}"```
    
    pub connection: Option<String>,
    /// Custom container image for the runtime environment.
    #[serde(rename="containerImage")]
    
    pub container_image: Option<String>,
    /// Files to be placed in the working directory of each executor. For more information about Apache Spark, see [Apache Spark](https://spark.apache.org/docs/latest/index.html).
    #[serde(rename="fileUris")]
    
    pub file_uris: Option<Vec<String>>,
    /// JARs to include on the driver and executor CLASSPATH. For more information about Apache Spark, see [Apache Spark](https://spark.apache.org/docs/latest/index.html).
    #[serde(rename="jarUris")]
    
    pub jar_uris: Option<Vec<String>>,
    /// The fully qualified name of a class in jar_uris, for example, com.example.wordcount. Exactly one of main_class and main_jar_uri field should be set for Java/Scala language type.
    #[serde(rename="mainClass")]
    
    pub main_class: Option<String>,
    /// The main file/jar URI of the Spark application. Exactly one of the definition_body field and the main_file_uri field must be set for Python. Exactly one of main_class and main_file_uri field should be set for Java/Scala language type.
    #[serde(rename="mainFileUri")]
    
    pub main_file_uri: Option<String>,
    /// Configuration properties as a set of key/value pairs, which will be passed on to the Spark application. For more information, see [Apache Spark](https://spark.apache.org/docs/latest/index.html) and the [procedure option list](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#procedure_option_list).
    
    pub properties: Option<HashMap<String, String>>,
    /// Python files to be placed on the PYTHONPATH for PySpark application. Supported file types: `.py`, `.egg`, and `.zip`. For more information about Apache Spark, see [Apache Spark](https://spark.apache.org/docs/latest/index.html).
    #[serde(rename="pyFileUris")]
    
    pub py_file_uris: Option<Vec<String>>,
    /// Runtime version. If not specified, the default runtime version is used.
    #[serde(rename="runtimeVersion")]
    
    pub runtime_version: Option<String>,
}

impl client::Part for SparkOptions {}


/// Statistics for a BigSpark query. Populated as part of JobStatistics2
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SparkStatistics {
    /// Output only. Endpoints returned from Dataproc. Key list: - history_server_endpoint: A link to Spark job UI.
    
    pub endpoints: Option<HashMap<String, String>>,
    /// Output only. The Google Cloud Storage bucket that is used as the default filesystem by the Spark application. This fields is only filled when the Spark procedure uses the INVOKER security mode. It is inferred from the system variable @@spark_proc_properties.staging_bucket if it is provided. Otherwise, BigQuery creates a default staging bucket for the job and returns the bucket name in this field. Example: * `gs://[bucket_name]`
    #[serde(rename="gcsStagingBucket")]
    
    pub gcs_staging_bucket: Option<String>,
    /// Output only. The Cloud KMS encryption key that is used to protect the resources created by the Spark job. If the Spark procedure uses DEFINER security mode, the Cloud KMS key is inferred from the Spark connection associated with the procedure if it is provided. Otherwise the key is inferred from the default key of the Spark connection's project if the CMEK organization policy is enforced. If the Spark procedure uses INVOKER security mode, the Cloud KMS encryption key is inferred from the system variable @@spark_proc_properties.kms_key_name if it is provided. Otherwise, the key is inferred fromt he default key of the BigQuery job's project if the CMEK organization policy is enforced. Example: * `projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]`
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Output only. Logging info is used to generate a link to Cloud Logging.
    #[serde(rename="loggingInfo")]
    
    pub logging_info: Option<SparkLoggingInfo>,
    /// Output only. Spark job ID if a Spark job is created successfully.
    #[serde(rename="sparkJobId")]
    
    pub spark_job_id: Option<String>,
    /// Output only. Location where the Spark job is executed. A location is selected by BigQueury for jobs configured to run in a multi-region.
    #[serde(rename="sparkJobLocation")]
    
    pub spark_job_location: Option<String>,
}

impl client::Part for SparkStatistics {}


/// Performance insights compared to the previous executions for a specific stage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StagePerformanceChangeInsight {
    /// Output only. Input data change insight of the query stage.
    #[serde(rename="inputDataChange")]
    
    pub input_data_change: Option<InputDataChange>,
    /// Output only. The stage id that the insight mapped to.
    #[serde(rename="stageId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub stage_id: Option<i64>,
}

impl client::Part for StagePerformanceChangeInsight {}


/// Standalone performance insights for a specific stage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StagePerformanceStandaloneInsight {
    /// Output only. If present, the stage had the following reasons for being disqualified from BI Engine execution.
    #[serde(rename="biEngineReasons")]
    
    pub bi_engine_reasons: Option<Vec<BiEngineReason>>,
    /// Output only. High cardinality joins in the stage.
    #[serde(rename="highCardinalityJoins")]
    
    pub high_cardinality_joins: Option<Vec<HighCardinalityJoin>>,
    /// Output only. True if the stage has insufficient shuffle quota.
    #[serde(rename="insufficientShuffleQuota")]
    
    pub insufficient_shuffle_quota: Option<bool>,
    /// Output only. True if the stage has a slot contention issue.
    #[serde(rename="slotContention")]
    
    pub slot_contention: Option<bool>,
    /// Output only. The stage id that the insight mapped to.
    #[serde(rename="stageId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub stage_id: Option<i64>,
}

impl client::Part for StagePerformanceStandaloneInsight {}


/// The data type of a variable such as a function argument. Examples include: * INT64: `{"typeKind": "INT64"}` * ARRAY: { "typeKind": "ARRAY", "arrayElementType": {"typeKind": "STRING"} } * STRUCT>: { "typeKind": "STRUCT", "structType": { "fields": [ { "name": "x", "type": {"typeKind": "STRING"} }, { "name": "y", "type": { "typeKind": "ARRAY", "arrayElementType": {"typeKind": "DATE"} } } ] } }
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StandardSqlDataType {
    /// The type of the array's elements, if type_kind = "ARRAY".
    #[serde(rename="arrayElementType")]
    
    pub array_element_type: Option<Option<Box<StandardSqlDataType>>>,
    /// The type of the range's elements, if type_kind = "RANGE".
    #[serde(rename="rangeElementType")]
    
    pub range_element_type: Option<Option<Box<StandardSqlDataType>>>,
    /// The fields of this struct, in order, if type_kind = "STRUCT".
    #[serde(rename="structType")]
    
    pub struct_type: Option<StandardSqlStructType>,
    /// Required. The top level type of this field. Can be any GoogleSQL data type (e.g., "INT64", "DATE", "ARRAY").
    #[serde(rename="typeKind")]
    
    pub type_kind: Option<String>,
}

impl client::Part for StandardSqlDataType {}


/// A field or a column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StandardSqlField {
    /// Optional. The name of this field. Can be absent for struct fields.
    
    pub name: Option<String>,
    /// Optional. The type of this parameter. Absent if not explicitly specified (e.g., CREATE FUNCTION statement can omit the return type; in this case the output parameter does not have this "type" field).
    #[serde(rename="type")]
    
    pub type_: Option<StandardSqlDataType>,
}

impl client::Part for StandardSqlField {}


/// The representation of a SQL STRUCT type.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StandardSqlStructType {
    /// Fields within the struct.
    
    pub fields: Option<Vec<StandardSqlField>>,
}

impl client::Part for StandardSqlStructType {}


/// A table type
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StandardSqlTableType {
    /// The columns in this table type
    
    pub columns: Option<Vec<StandardSqlField>>,
}

impl client::Part for StandardSqlTableType {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Streamingbuffer {
    /// Output only. A lower-bound estimate of the number of bytes currently in the streaming buffer.
    #[serde(rename="estimatedBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_bytes: Option<u64>,
    /// Output only. A lower-bound estimate of the number of rows currently in the streaming buffer.
    #[serde(rename="estimatedRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub estimated_rows: Option<u64>,
    /// Output only. Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available.
    #[serde(rename="oldestEntryTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub oldest_entry_time: Option<u64>,
}

impl client::Part for Streamingbuffer {}


/// Search space for string and enum.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StringHparamSearchSpace {
    /// Canididates for the string or enum parameter in lower case.
    
    pub candidates: Option<Vec<String>>,
}

impl client::Part for StringHparamSearchSpace {}


/// System variables given to a query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SystemVariables {
    /// Output only. Data type for each system variable.
    
    pub types: Option<HashMap<String, StandardSqlDataType>>,
    /// Output only. Value for each system variable.
    
    pub values: Option<HashMap<String, json::Value>>,
}

impl client::Part for SystemVariables {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete tables](TableDeleteCall) (none)
/// * [get tables](TableGetCall) (response)
/// * [get iam policy tables](TableGetIamPolicyCall) (none)
/// * [insert tables](TableInsertCall) (request|response)
/// * [list tables](TableListCall) (none)
/// * [patch tables](TablePatchCall) (request|response)
/// * [set iam policy tables](TableSetIamPolicyCall) (none)
/// * [test iam permissions tables](TableTestIamPermissionCall) (none)
/// * [update tables](TableUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Optional. Specifies the configuration of a BigLake managed table.
    #[serde(rename="biglakeConfiguration")]
    
    pub biglake_configuration: Option<BigLakeConfiguration>,
    /// Output only. Contains information about the clone. This value is set via the clone operation.
    #[serde(rename="cloneDefinition")]
    
    pub clone_definition: Option<CloneDefinition>,
    /// Clustering specification for the table. Must be specified with time-based partitioning, data in the table will be first partitioned and subsequently clustered.
    
    pub clustering: Option<Clustering>,
    /// Output only. The time when this table was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// Optional. Defines the default collation specification of new STRING fields in the table. During table creation or update, if a STRING field is added to this table without explicit collation specified, then the table inherits the table default collation. A change to this field affects only fields added afterwards, and does not alter the existing fields. The following values are supported: * 'und:ci': undetermined locale, case insensitive. * '': empty string. Default to case-sensitive behavior.
    #[serde(rename="defaultCollation")]
    
    pub default_collation: Option<String>,
    /// Optional. Defines the default rounding mode specification of new decimal fields (NUMERIC OR BIGNUMERIC) in the table. During table creation or update, if a decimal field is added to this table without an explicit rounding mode specified, then the field inherits the table default rounding mode. Changing this field doesn't affect existing fields.
    #[serde(rename="defaultRoundingMode")]
    
    pub default_rounding_mode: Option<String>,
    /// Optional. A user-friendly description of this table.
    
    pub description: Option<String>,
    /// Custom encryption configuration (e.g., Cloud KMS keys).
    #[serde(rename="encryptionConfiguration")]
    
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// Output only. A hash of this resource.
    
    pub etag: Option<String>,
    /// Optional. The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed. The defaultTableExpirationMs property of the encapsulating dataset can be used to set a default expirationTime on newly created tables.
    #[serde(rename="expirationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration_time: Option<i64>,
    /// Optional. Describes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table.
    #[serde(rename="externalDataConfiguration")]
    
    pub external_data_configuration: Option<ExternalDataConfiguration>,
    /// Optional. A descriptive name for this table.
    #[serde(rename="friendlyName")]
    
    pub friendly_name: Option<String>,
    /// Output only. An opaque ID uniquely identifying the table.
    
    pub id: Option<String>,
    /// The type of resource ID.
    
    pub kind: Option<String>,
    /// The labels associated with this table. You can use these to organize and group your tables. Label keys and values can be no longer than 63 characters, can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter and each label in the list must have a different key.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The time when this table was last modified, in milliseconds since the epoch.
    #[serde(rename="lastModifiedTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub last_modified_time: Option<u64>,
    /// Output only. The geographic location where the table resides. This value is inherited from the dataset.
    
    pub location: Option<String>,
    /// Optional. The materialized view definition.
    #[serde(rename="materializedView")]
    
    pub materialized_view: Option<MaterializedViewDefinition>,
    /// Output only. The materialized view status.
    #[serde(rename="materializedViewStatus")]
    
    pub materialized_view_status: Option<MaterializedViewStatus>,
    /// Optional. The maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of sql IntervalValue type.
    #[serde(rename="maxStaleness")]
    
    pub max_staleness: Option<String>,
    /// Deprecated.
    
    pub model: Option<ModelDefinition>,
    /// Output only. Number of logical bytes that are less than 90 days old.
    #[serde(rename="numActiveLogicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_active_logical_bytes: Option<i64>,
    /// Output only. Number of physical bytes less than 90 days old. This data is not kept in real time, and might be delayed by a few seconds to a few minutes.
    #[serde(rename="numActivePhysicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_active_physical_bytes: Option<i64>,
    /// Output only. The size of this table in logical bytes, excluding any data in the streaming buffer.
    #[serde(rename="numBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_bytes: Option<i64>,
    /// Output only. The number of logical bytes in the table that are considered "long-term storage".
    #[serde(rename="numLongTermBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_long_term_bytes: Option<i64>,
    /// Output only. Number of logical bytes that are more than 90 days old.
    #[serde(rename="numLongTermLogicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_long_term_logical_bytes: Option<i64>,
    /// Output only. Number of physical bytes more than 90 days old. This data is not kept in real time, and might be delayed by a few seconds to a few minutes.
    #[serde(rename="numLongTermPhysicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_long_term_physical_bytes: Option<i64>,
    /// Output only. The number of partitions present in the table or materialized view. This data is not kept in real time, and might be delayed by a few seconds to a few minutes.
    #[serde(rename="numPartitions")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_partitions: Option<i64>,
    /// Output only. The physical size of this table in bytes. This includes storage used for time travel.
    #[serde(rename="numPhysicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_physical_bytes: Option<i64>,
    /// Output only. The number of rows of data in this table, excluding any data in the streaming buffer.
    #[serde(rename="numRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_rows: Option<u64>,
    /// Output only. Number of physical bytes used by time travel storage (deleted or changed data). This data is not kept in real time, and might be delayed by a few seconds to a few minutes.
    #[serde(rename="numTimeTravelPhysicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_time_travel_physical_bytes: Option<i64>,
    /// Output only. Total number of logical bytes in the table or materialized view.
    #[serde(rename="numTotalLogicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_total_logical_bytes: Option<i64>,
    /// Output only. The physical size of this table in bytes. This also includes storage used for time travel. This data is not kept in real time, and might be delayed by a few seconds to a few minutes.
    #[serde(rename="numTotalPhysicalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_total_physical_bytes: Option<i64>,
    /// If specified, configures range partitioning for this table.
    #[serde(rename="rangePartitioning")]
    
    pub range_partitioning: Option<RangePartitioning>,
    /// Optional. Output only. Table references of all replicas currently active on the table.
    
    pub replicas: Option<Vec<TableReference>>,
    /// Optional. If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified.
    #[serde(rename="requirePartitionFilter")]
    
    pub require_partition_filter: Option<bool>,
    /// [Optional] The tags associated with this table. Tag keys are globally unique. See additional information on [tags](https://cloud.google.com/iam/docs/tags-access-control#definitions). An object containing a list of "key": value pairs. The key is the namespaced friendly name of the tag key, e.g. "12345/environment" where 12345 is parent id. The value is the friendly short name of the tag value, e.g. "production".
    #[serde(rename="resourceTags")]
    
    pub resource_tags: Option<HashMap<String, String>>,
    /// Optional. Describes the schema of this table.
    
    pub schema: Option<TableSchema>,
    /// Output only. A URL that can be used to access this resource again.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Output only. Contains information about the snapshot. This value is set via snapshot creation.
    #[serde(rename="snapshotDefinition")]
    
    pub snapshot_definition: Option<SnapshotDefinition>,
    /// Output only. Contains information regarding this table's streaming buffer, if one is present. This field will be absent if the table is not being streamed to or if there is no data in the streaming buffer.
    #[serde(rename="streamingBuffer")]
    
    pub streaming_buffer: Option<Streamingbuffer>,
    /// Optional. Tables Primary Key and Foreign Key information
    #[serde(rename="tableConstraints")]
    
    pub table_constraints: Option<TableConstraints>,
    /// Required. Reference describing the ID of this table.
    #[serde(rename="tableReference")]
    
    pub table_reference: Option<TableReference>,
    /// Optional. Table replication info for table created `AS REPLICA` DDL like: `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv`
    #[serde(rename="tableReplicationInfo")]
    
    pub table_replication_info: Option<TableReplicationInfo>,
    /// If specified, configures time-based partitioning for this table.
    #[serde(rename="timePartitioning")]
    
    pub time_partitioning: Option<TimePartitioning>,
    /// Output only. Describes the table type. The following values are supported: * `TABLE`: A normal BigQuery table. * `VIEW`: A virtual table defined by a SQL query. * `EXTERNAL`: A table that references data stored in an external storage system, such as Google Cloud Storage. * `MATERIALIZED_VIEW`: A precomputed view defined by a SQL query. * `SNAPSHOT`: An immutable BigQuery table that preserves the contents of a base table at a particular time. See additional information on [table snapshots](https://cloud.google.com/bigquery/docs/table-snapshots-intro). The default value is `TABLE`.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Optional. The view definition.
    
    pub view: Option<ViewDefinition>,
}

impl client::RequestValue for Table {}
impl client::Resource for Table {}
impl client::ResponseResult for Table {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableCell {
    /// no description provided
    
    pub v: Option<json::Value>,
}

impl client::Part for TableCell {}


/// The TableConstraints defines the primary key and foreign key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableConstraints {
    /// Optional. Present only if the table has a foreign key. The foreign key is not enforced.
    #[serde(rename="foreignKeys")]
    
    pub foreign_keys: Option<Vec<TableConstraintsForeignKeys>>,
    /// Represents the primary key constraint on a table's columns.
    #[serde(rename="primaryKey")]
    
    pub primary_key: Option<TableConstraintsPrimaryKey>,
}

impl client::Part for TableConstraints {}


/// Request for sending a single streaming insert.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert all tabledata](TabledataInsertAllCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllRequest {
    /// Optional. Accept rows that contain values that do not match the schema. The unknown values are ignored. Default is false, which treats unknown values as errors.
    #[serde(rename="ignoreUnknownValues")]
    
    pub ignore_unknown_values: Option<bool>,
    /// Optional. The resource type of the response. The value is not checked at the backend. Historically, it has been set to "bigquery#tableDataInsertAllRequest" but you are not required to set it.
    
    pub kind: Option<String>,
    /// no description provided
    
    pub rows: Option<Vec<TableDataInsertAllRequestRows>>,
    /// Optional. Insert all valid rows of a request, even if invalid rows exist. The default value is false, which causes the entire request to fail if any invalid rows exist.
    #[serde(rename="skipInvalidRows")]
    
    pub skip_invalid_rows: Option<bool>,
    /// Optional. If specified, treats the destination table as a base template, and inserts the rows into an instance table named "{destination}{templateSuffix}". BigQuery will manage creation of the instance table, using the schema of the base template table. See https://cloud.google.com/bigquery/streaming-data-into-bigquery#template-tables for considerations when working with templates tables.
    #[serde(rename="templateSuffix")]
    
    pub template_suffix: Option<String>,
    /// Optional. Unique request trace id. Used for debugging purposes only. It is case-sensitive, limited to up to 36 ASCII characters. A UUID is recommended.
    #[serde(rename="traceId")]
    
    pub trace_id: Option<String>,
}

impl client::RequestValue for TableDataInsertAllRequest {}


/// Describes the format of a streaming insert response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert all tabledata](TabledataInsertAllCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllResponse {
    /// Describes specific errors encountered while processing the request.
    #[serde(rename="insertErrors")]
    
    pub insert_errors: Option<Vec<TableDataInsertAllResponseInsertErrors>>,
    /// Returns "bigquery#tableDataInsertAllResponse".
    
    pub kind: Option<String>,
}

impl client::ResponseResult for TableDataInsertAllResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tabledata](TabledataListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataList {
    /// A hash of this page of results.
    
    pub etag: Option<String>,
    /// The resource type of the response.
    
    pub kind: Option<String>,
    /// A token used for paging results. Providing this token instead of the startIndex parameter can help you retrieve stable results when an underlying table is changing.
    #[serde(rename="pageToken")]
    
    pub page_token: Option<String>,
    /// Rows of results.
    
    pub rows: Option<Vec<TableRow>>,
    /// Total rows of the entire table. In order to show default value 0 we have to present it as string.
    #[serde(rename="totalRows")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_rows: Option<i64>,
}

impl client::ResponseResult for TableDataList {}


/// A field in TableSchema
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableFieldSchema {
    /// Deprecated.
    
    pub categories: Option<TableFieldSchemaCategories>,
    /// Optional. Field collation can be set only when the type of field is STRING. The following values are supported: * 'und:ci': undetermined locale, case insensitive. * '': empty string. Default to case-sensitive behavior.
    
    pub collation: Option<String>,
    /// Optional. A SQL expression to specify the [default value] (https://cloud.google.com/bigquery/docs/default-values) for this field.
    #[serde(rename="defaultValueExpression")]
    
    pub default_value_expression: Option<String>,
    /// Optional. The field description. The maximum length is 1,024 characters.
    
    pub description: Option<String>,
    /// Optional. Describes the nested schema fields if the type property is set to RECORD.
    
    pub fields: Option<Vec<TableFieldSchema>>,
    /// Optional. Maximum length of values of this field for STRINGS or BYTES. If max_length is not specified, no maximum length constraint is imposed on this field. If type = "STRING", then max_length represents the maximum UTF-8 length of strings in this field. If type = "BYTES", then max_length represents the maximum number of bytes in this field. It is invalid to set this field if type ≠ "STRING" and ≠ "BYTES".
    #[serde(rename="maxLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_length: Option<i64>,
    /// Optional. The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE.
    
    pub mode: Option<String>,
    /// Required. The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 300 characters.
    
    pub name: Option<String>,
    /// Optional. The policy tags attached to this field, used for field-level access control. If not set, defaults to empty policy_tags.
    #[serde(rename="policyTags")]
    
    pub policy_tags: Option<TableFieldSchemaPolicyTags>,
    /// Optional. Precision (maximum number of total digits in base 10) and scale (maximum number of digits in the fractional part in base 10) constraints for values of this field for NUMERIC or BIGNUMERIC. It is invalid to set precision or scale if type ≠ "NUMERIC" and ≠ "BIGNUMERIC". If precision and scale are not specified, no value range constraint is imposed on this field insofar as values are permitted by the type. Values of this NUMERIC or BIGNUMERIC field must be in this range when: * Precision (P) and scale (S) are specified: [-10P-S + 10-S, 10P-S - 10-S] * Precision (P) is specified but not scale (and thus scale is interpreted to be equal to zero): [-10P + 1, 10P - 1]. Acceptable values for precision and scale if both are specified: * If type = "NUMERIC": 1 ≤ precision - scale ≤ 29 and 0 ≤ scale ≤ 9. * If type = "BIGNUMERIC": 1 ≤ precision - scale ≤ 38 and 0 ≤ scale ≤ 38. Acceptable values for precision if only precision is specified but not scale (and thus scale is interpreted to be equal to zero): * If type = "NUMERIC": 1 ≤ precision ≤ 29. * If type = "BIGNUMERIC": 1 ≤ precision ≤ 38. If scale is specified but not precision, then it is invalid.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub precision: Option<i64>,
    /// Represents the type of a field element.
    #[serde(rename="rangeElementType")]
    
    pub range_element_type: Option<TableFieldSchemaRangeElementType>,
    /// Optional. Specifies the rounding mode to be used when storing values of NUMERIC and BIGNUMERIC type.
    #[serde(rename="roundingMode")]
    
    pub rounding_mode: Option<String>,
    /// Optional. See documentation for precision.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub scale: Option<i64>,
    /// Required. The field data type. Possible values include: * STRING * BYTES * INTEGER (or INT64) * FLOAT (or FLOAT64) * BOOLEAN (or BOOL) * TIMESTAMP * DATE * TIME * DATETIME * GEOGRAPHY * NUMERIC * BIGNUMERIC * JSON * RECORD (or STRUCT) Use of RECORD/STRUCT indicates that the field contains a nested schema.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for TableFieldSchema {}


/// Partial projection of the metadata for a given table in a list response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list tables](TableListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableList {
    /// A hash of this page of results.
    
    pub etag: Option<String>,
    /// The type of list.
    
    pub kind: Option<String>,
    /// A token to request the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Tables in the requested dataset.
    
    pub tables: Option<Vec<TableListTables>>,
    /// The total number of tables in the dataset.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for TableList {}


/// Table level detail on the usage of metadata caching. Only set for Metadata caching eligible tables referenced in the query.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableMetadataCacheUsage {
    /// Free form human-readable reason metadata caching was unused for the job.
    
    pub explanation: Option<String>,
    /// Metadata caching eligible table referenced in the query.
    #[serde(rename="tableReference")]
    
    pub table_reference: Option<TableReference>,
    /// [Table type](https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#Table.FIELDS.type).
    #[serde(rename="tableType")]
    
    pub table_type: Option<String>,
    /// Reason for not using metadata caching for the table.
    #[serde(rename="unusedReason")]
    
    pub unused_reason: Option<String>,
}

impl client::Part for TableMetadataCacheUsage {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableReference {
    /// Required. The ID of the dataset containing this table.
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// Required. The ID of the project containing this table.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Required. The ID of the table. The ID can contain Unicode characters in category L (letter), M (mark), N (number), Pc (connector, including underscore), Pd (dash), and Zs (space). For more information, see [General Category](https://wikipedia.org/wiki/Unicode_character_property#General_Category). The maximum length is 1,024 characters. Certain operations allow suffixing of the table ID with a partition decorator, such as `sample_table$20190123`.
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::Part for TableReference {}


/// Replication info of a table created using `AS REPLICA` DDL like: `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv`
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableReplicationInfo {
    /// Optional. Output only. If source is a materialized view, this field signifies the last refresh time of the source.
    #[serde(rename="replicatedSourceLastRefreshTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub replicated_source_last_refresh_time: Option<i64>,
    /// Optional. Output only. Replication error that will permanently stopped table replication.
    #[serde(rename="replicationError")]
    
    pub replication_error: Option<ErrorProto>,
    /// Required. Specifies the interval at which the source table is polled for updates.
    #[serde(rename="replicationIntervalMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub replication_interval_ms: Option<i64>,
    /// Optional. Output only. Replication status of configured replication.
    #[serde(rename="replicationStatus")]
    
    pub replication_status: Option<String>,
    /// Required. Source table reference that is replicated.
    #[serde(rename="sourceTable")]
    
    pub source_table: Option<TableReference>,
}

impl client::Part for TableReplicationInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableRow {
    /// Represents a single row in the result set, consisting of one or more fields.
    
    pub f: Option<Vec<TableCell>>,
}

impl client::Part for TableRow {}


/// Schema of a table
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableSchema {
    /// Describes the fields in a table.
    
    pub fields: Option<Vec<TableFieldSchema>>,
}

impl client::Part for TableSchema {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions row access policies](RowAccessPolicyTestIamPermissionCall) (request)
/// * [test iam permissions tables](TableTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions row access policies](RowAccessPolicyTestIamPermissionCall) (response)
/// * [test iam permissions tables](TableTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimePartitioning {
    /// Optional. Number of milliseconds for which to keep the storage for a partition. A wrapper is used here because 0 is an invalid value.
    #[serde(rename="expirationMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration_ms: Option<i64>,
    /// Optional. If not set, the table is partitioned by pseudo column '_PARTITIONTIME'; if set, the table is partitioned by this field. The field must be a top-level TIMESTAMP or DATE field. Its mode must be NULLABLE or REQUIRED. A wrapper is used here because an empty string is an invalid value.
    
    pub field: Option<String>,
    /// If set to true, queries over this table require a partition filter that can be used for partition elimination to be specified. This field is deprecated; please set the field with the same name on the table itself instead. This field needs a wrapper because we want to output the default value, false, if the user explicitly set it.
    #[serde(rename="requirePartitionFilter")]
    
    pub require_partition_filter: Option<bool>,
    /// Required. The supported types are DAY, HOUR, MONTH, and YEAR, which will generate one partition per day, hour, month, and year, respectively.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for TimePartitioning {}


/// Options used in model training.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrainingOptions {
    /// Activation function of the neural nets.
    #[serde(rename="activationFn")]
    
    pub activation_fn: Option<String>,
    /// If true, detect step changes and make data adjustment in the input time series.
    #[serde(rename="adjustStepChanges")]
    
    pub adjust_step_changes: Option<bool>,
    /// Whether to use approximate feature contribution method in XGBoost model explanation for global explain.
    #[serde(rename="approxGlobalFeatureContrib")]
    
    pub approx_global_feature_contrib: Option<bool>,
    /// Whether to enable auto ARIMA or not.
    #[serde(rename="autoArima")]
    
    pub auto_arima: Option<bool>,
    /// The max value of the sum of non-seasonal p and q.
    #[serde(rename="autoArimaMaxOrder")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub auto_arima_max_order: Option<i64>,
    /// The min value of the sum of non-seasonal p and q.
    #[serde(rename="autoArimaMinOrder")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub auto_arima_min_order: Option<i64>,
    /// Whether to calculate class weights automatically based on the popularity of each label.
    #[serde(rename="autoClassWeights")]
    
    pub auto_class_weights: Option<bool>,
    /// Batch size for dnn models.
    #[serde(rename="batchSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub batch_size: Option<i64>,
    /// Booster type for boosted tree models.
    #[serde(rename="boosterType")]
    
    pub booster_type: Option<String>,
    /// Budget in hours for AutoML training.
    #[serde(rename="budgetHours")]
    
    pub budget_hours: Option<f64>,
    /// Whether or not p-value test should be computed for this model. Only available for linear and logistic regression models.
    #[serde(rename="calculatePValues")]
    
    pub calculate_p_values: Option<bool>,
    /// Categorical feature encoding method.
    #[serde(rename="categoryEncodingMethod")]
    
    pub category_encoding_method: Option<String>,
    /// If true, clean spikes and dips in the input time series.
    #[serde(rename="cleanSpikesAndDips")]
    
    pub clean_spikes_and_dips: Option<bool>,
    /// Enums for color space, used for processing images in Object Table. See more details at https://www.tensorflow.org/io/tutorials/colorspace.
    #[serde(rename="colorSpace")]
    
    pub color_space: Option<String>,
    /// Subsample ratio of columns for each level for boosted tree models.
    #[serde(rename="colsampleBylevel")]
    
    pub colsample_bylevel: Option<f64>,
    /// Subsample ratio of columns for each node(split) for boosted tree models.
    #[serde(rename="colsampleBynode")]
    
    pub colsample_bynode: Option<f64>,
    /// Subsample ratio of columns when constructing each tree for boosted tree models.
    #[serde(rename="colsampleBytree")]
    
    pub colsample_bytree: Option<f64>,
    /// Type of normalization algorithm for boosted tree models using dart booster.
    #[serde(rename="dartNormalizeType")]
    
    pub dart_normalize_type: Option<String>,
    /// The data frequency of a time series.
    #[serde(rename="dataFrequency")]
    
    pub data_frequency: Option<String>,
    /// The column to split data with. This column won't be used as a feature. 1. When data_split_method is CUSTOM, the corresponding column should be boolean. The rows with true value tag are eval data, and the false are training data. 2. When data_split_method is SEQ, the first DATA_SPLIT_EVAL_FRACTION rows (from smallest to largest) in the corresponding column are used as training data, and the rest are eval data. It respects the order in Orderable data types: https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#data-type-properties
    #[serde(rename="dataSplitColumn")]
    
    pub data_split_column: Option<String>,
    /// The fraction of evaluation data over the whole input data. The rest of data will be used as training data. The format should be double. Accurate to two decimal places. Default value is 0.2.
    #[serde(rename="dataSplitEvalFraction")]
    
    pub data_split_eval_fraction: Option<f64>,
    /// The data split type for training and evaluation, e.g. RANDOM.
    #[serde(rename="dataSplitMethod")]
    
    pub data_split_method: Option<String>,
    /// If true, perform decompose time series and save the results.
    #[serde(rename="decomposeTimeSeries")]
    
    pub decompose_time_series: Option<bool>,
    /// Distance type for clustering models.
    #[serde(rename="distanceType")]
    
    pub distance_type: Option<String>,
    /// Dropout probability for dnn models.
    
    pub dropout: Option<f64>,
    /// Whether to stop early when the loss doesn't improve significantly any more (compared to min_relative_progress). Used only for iterative training algorithms.
    #[serde(rename="earlyStop")]
    
    pub early_stop: Option<bool>,
    /// If true, enable global explanation during training.
    #[serde(rename="enableGlobalExplain")]
    
    pub enable_global_explain: Option<bool>,
    /// Feedback type that specifies which algorithm to run for matrix factorization.
    #[serde(rename="feedbackType")]
    
    pub feedback_type: Option<String>,
    /// Whether the model should include intercept during model training.
    #[serde(rename="fitIntercept")]
    
    pub fit_intercept: Option<bool>,
    /// Hidden units for dnn models.
    #[serde(rename="hiddenUnits")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub hidden_units: Option<Vec<i64>>,
    /// The geographical region based on which the holidays are considered in time series modeling. If a valid value is specified, then holiday effects modeling is enabled.
    #[serde(rename="holidayRegion")]
    
    pub holiday_region: Option<String>,
    /// A list of geographical regions that are used for time series modeling.
    #[serde(rename="holidayRegions")]
    
    pub holiday_regions: Option<Vec<String>>,
    /// The number of periods ahead that need to be forecasted.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub horizon: Option<i64>,
    /// The target evaluation metrics to optimize the hyperparameters for.
    #[serde(rename="hparamTuningObjectives")]
    
    pub hparam_tuning_objectives: Option<Vec<String>>,
    /// Include drift when fitting an ARIMA model.
    #[serde(rename="includeDrift")]
    
    pub include_drift: Option<bool>,
    /// Specifies the initial learning rate for the line search learn rate strategy.
    #[serde(rename="initialLearnRate")]
    
    pub initial_learn_rate: Option<f64>,
    /// Name of input label columns in training data.
    #[serde(rename="inputLabelColumns")]
    
    pub input_label_columns: Option<Vec<String>>,
    /// Name of the instance weight column for training data. This column isn't be used as a feature.
    #[serde(rename="instanceWeightColumn")]
    
    pub instance_weight_column: Option<String>,
    /// Number of integral steps for the integrated gradients explain method.
    #[serde(rename="integratedGradientsNumSteps")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub integrated_gradients_num_steps: Option<i64>,
    /// Item column specified for matrix factorization models.
    #[serde(rename="itemColumn")]
    
    pub item_column: Option<String>,
    /// The column used to provide the initial centroids for kmeans algorithm when kmeans_initialization_method is CUSTOM.
    #[serde(rename="kmeansInitializationColumn")]
    
    pub kmeans_initialization_column: Option<String>,
    /// The method used to initialize the centroids for kmeans algorithm.
    #[serde(rename="kmeansInitializationMethod")]
    
    pub kmeans_initialization_method: Option<String>,
    /// L1 regularization coefficient to activations.
    #[serde(rename="l1RegActivation")]
    
    pub l1_reg_activation: Option<f64>,
    /// L1 regularization coefficient.
    #[serde(rename="l1Regularization")]
    
    pub l1_regularization: Option<f64>,
    /// L2 regularization coefficient.
    #[serde(rename="l2Regularization")]
    
    pub l2_regularization: Option<f64>,
    /// Weights associated with each label class, for rebalancing the training data. Only applicable for classification models.
    #[serde(rename="labelClassWeights")]
    
    pub label_class_weights: Option<HashMap<String, f64>>,
    /// Learning rate in training. Used only for iterative training algorithms.
    #[serde(rename="learnRate")]
    
    pub learn_rate: Option<f64>,
    /// The strategy to determine learn rate for the current iteration.
    #[serde(rename="learnRateStrategy")]
    
    pub learn_rate_strategy: Option<String>,
    /// Type of loss function used during training run.
    #[serde(rename="lossType")]
    
    pub loss_type: Option<String>,
    /// The maximum number of iterations in training. Used only for iterative training algorithms.
    #[serde(rename="maxIterations")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_iterations: Option<i64>,
    /// Maximum number of trials to run in parallel.
    #[serde(rename="maxParallelTrials")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_parallel_trials: Option<i64>,
    /// The maximum number of time points in a time series that can be used in modeling the trend component of the time series. Don't use this option with the `timeSeriesLengthFraction` or `minTimeSeriesLength` options.
    #[serde(rename="maxTimeSeriesLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_time_series_length: Option<i64>,
    /// Maximum depth of a tree for boosted tree models.
    #[serde(rename="maxTreeDepth")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_tree_depth: Option<i64>,
    /// When early_stop is true, stops training when accuracy improvement is less than 'min_relative_progress'. Used only for iterative training algorithms.
    #[serde(rename="minRelativeProgress")]
    
    pub min_relative_progress: Option<f64>,
    /// Minimum split loss for boosted tree models.
    #[serde(rename="minSplitLoss")]
    
    pub min_split_loss: Option<f64>,
    /// The minimum number of time points in a time series that are used in modeling the trend component of the time series. If you use this option you must also set the `timeSeriesLengthFraction` option. This training option ensures that enough time points are available when you use `timeSeriesLengthFraction` in trend modeling. This is particularly important when forecasting multiple time series in a single query using `timeSeriesIdColumn`. If the total number of time points is less than the `minTimeSeriesLength` value, then the query uses all available time points.
    #[serde(rename="minTimeSeriesLength")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_time_series_length: Option<i64>,
    /// Minimum sum of instance weight needed in a child for boosted tree models.
    #[serde(rename="minTreeChildWeight")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_tree_child_weight: Option<i64>,
    /// The model registry.
    #[serde(rename="modelRegistry")]
    
    pub model_registry: Option<String>,
    /// Google Cloud Storage URI from which the model was imported. Only applicable for imported models.
    #[serde(rename="modelUri")]
    
    pub model_uri: Option<String>,
    /// A specification of the non-seasonal part of the ARIMA model: the three components (p, d, q) are the AR order, the degree of differencing, and the MA order.
    #[serde(rename="nonSeasonalOrder")]
    
    pub non_seasonal_order: Option<ArimaOrder>,
    /// Number of clusters for clustering models.
    #[serde(rename="numClusters")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_clusters: Option<i64>,
    /// Num factors specified for matrix factorization models.
    #[serde(rename="numFactors")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_factors: Option<i64>,
    /// Number of parallel trees constructed during each iteration for boosted tree models.
    #[serde(rename="numParallelTree")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_parallel_tree: Option<i64>,
    /// Number of principal components to keep in the PCA model. Must be <= the number of features.
    #[serde(rename="numPrincipalComponents")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_principal_components: Option<i64>,
    /// Number of trials to run this hyperparameter tuning job.
    #[serde(rename="numTrials")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_trials: Option<i64>,
    /// Optimization strategy for training linear regression models.
    #[serde(rename="optimizationStrategy")]
    
    pub optimization_strategy: Option<String>,
    /// Optimizer used for training the neural nets.
    
    pub optimizer: Option<String>,
    /// The minimum ratio of cumulative explained variance that needs to be given by the PCA model.
    #[serde(rename="pcaExplainedVarianceRatio")]
    
    pub pca_explained_variance_ratio: Option<f64>,
    /// The solver for PCA.
    #[serde(rename="pcaSolver")]
    
    pub pca_solver: Option<String>,
    /// Number of paths for the sampled Shapley explain method.
    #[serde(rename="sampledShapleyNumPaths")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sampled_shapley_num_paths: Option<i64>,
    /// If true, scale the feature values by dividing the feature standard deviation. Currently only apply to PCA.
    #[serde(rename="scaleFeatures")]
    
    pub scale_features: Option<bool>,
    /// Whether to standardize numerical features. Default to true.
    #[serde(rename="standardizeFeatures")]
    
    pub standardize_features: Option<bool>,
    /// Subsample fraction of the training data to grow tree to prevent overfitting for boosted tree models.
    
    pub subsample: Option<f64>,
    /// Based on the selected TF version, the corresponding docker image is used to train external models.
    #[serde(rename="tfVersion")]
    
    pub tf_version: Option<String>,
    /// Column to be designated as time series data for ARIMA model.
    #[serde(rename="timeSeriesDataColumn")]
    
    pub time_series_data_column: Option<String>,
    /// The time series id column that was used during ARIMA model training.
    #[serde(rename="timeSeriesIdColumn")]
    
    pub time_series_id_column: Option<String>,
    /// The time series id columns that were used during ARIMA model training.
    #[serde(rename="timeSeriesIdColumns")]
    
    pub time_series_id_columns: Option<Vec<String>>,
    /// The fraction of the interpolated length of the time series that's used to model the time series trend component. All of the time points of the time series are used to model the non-trend component. This training option accelerates modeling training without sacrificing much forecasting accuracy. You can use this option with `minTimeSeriesLength` but not with `maxTimeSeriesLength`.
    #[serde(rename="timeSeriesLengthFraction")]
    
    pub time_series_length_fraction: Option<f64>,
    /// Column to be designated as time series timestamp for ARIMA model.
    #[serde(rename="timeSeriesTimestampColumn")]
    
    pub time_series_timestamp_column: Option<String>,
    /// Tree construction algorithm for boosted tree models.
    #[serde(rename="treeMethod")]
    
    pub tree_method: Option<String>,
    /// Smoothing window size for the trend component. When a positive value is specified, a center moving average smoothing is applied on the history trend. When the smoothing window is out of the boundary at the beginning or the end of the trend, the first element or the last element is padded to fill the smoothing window before the average is applied.
    #[serde(rename="trendSmoothingWindowSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub trend_smoothing_window_size: Option<i64>,
    /// User column specified for matrix factorization models.
    #[serde(rename="userColumn")]
    
    pub user_column: Option<String>,
    /// The version aliases to apply in Vertex AI model registry. Always overwrite if the version aliases exists in a existing model.
    #[serde(rename="vertexAiModelVersionAliases")]
    
    pub vertex_ai_model_version_aliases: Option<Vec<String>>,
    /// Hyperparameter for matrix factoration when implicit feedback type is specified.
    #[serde(rename="walsAlpha")]
    
    pub wals_alpha: Option<f64>,
    /// Whether to train a model from the last checkpoint.
    #[serde(rename="warmStart")]
    
    pub warm_start: Option<bool>,
    /// User-selected XGBoost versions for training of XGBoost models.
    #[serde(rename="xgboostVersion")]
    
    pub xgboost_version: Option<String>,
}

impl client::Part for TrainingOptions {}


/// Information about a single training query run for the model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TrainingRun {
    /// Output only. Global explanation contains the explanation of top features on the class level. Applies to classification models only.
    #[serde(rename="classLevelGlobalExplanations")]
    
    pub class_level_global_explanations: Option<Vec<GlobalExplanation>>,
    /// Output only. Data split result of the training run. Only set when the input data is actually split.
    #[serde(rename="dataSplitResult")]
    
    pub data_split_result: Option<DataSplitResult>,
    /// Output only. The evaluation metrics over training/eval data that were computed at the end of training.
    #[serde(rename="evaluationMetrics")]
    
    pub evaluation_metrics: Option<EvaluationMetrics>,
    /// Output only. Global explanation contains the explanation of top features on the model level. Applies to both regression and classification models.
    #[serde(rename="modelLevelGlobalExplanation")]
    
    pub model_level_global_explanation: Option<GlobalExplanation>,
    /// Output only. Output of each iteration run, results.size() <= max_iterations.
    
    pub results: Option<Vec<IterationResult>>,
    /// Output only. The start time of this training run.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Options that were used for this training run, includes user specified and default options that were used.
    #[serde(rename="trainingOptions")]
    
    pub training_options: Option<TrainingOptions>,
    /// Output only. The start time of this training run, in milliseconds since epoch.
    #[serde(rename="trainingStartTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub training_start_time: Option<i64>,
    /// The model id in the [Vertex AI Model Registry](https://cloud.google.com/vertex-ai/docs/model-registry/introduction) for this training run.
    #[serde(rename="vertexAiModelId")]
    
    pub vertex_ai_model_id: Option<String>,
    /// Output only. The model version in the [Vertex AI Model Registry](https://cloud.google.com/vertex-ai/docs/model-registry/introduction) for this training run.
    #[serde(rename="vertexAiModelVersion")]
    
    pub vertex_ai_model_version: Option<String>,
}

impl client::Part for TrainingRun {}


/// [Alpha] Information of a multi-statement transaction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    /// Output only. [Alpha] Id of the transaction.
    #[serde(rename="transactionId")]
    
    pub transaction_id: Option<String>,
}

impl client::Part for TransactionInfo {}


/// Information about a single transform column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransformColumn {
    /// Output only. Name of the column.
    
    pub name: Option<String>,
    /// Output only. The SQL expression used in the column transform.
    #[serde(rename="transformSql")]
    
    pub transform_sql: Option<String>,
    /// Output only. Data type of the column after the transform.
    #[serde(rename="type")]
    
    pub type_: Option<StandardSqlDataType>,
}

impl client::Part for TransformColumn {}


/// Request format for undeleting a dataset.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [undelete datasets](DatasetUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteDatasetRequest {
    /// Optional. The exact time when the dataset was deleted. If not specified, it will undelete the most recently deleted version.
    #[serde(rename="deletionTime")]
    
    pub deletion_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for UndeleteDatasetRequest {}


///  This is used for defining User Defined Function (UDF) resources only when using legacy SQL. Users of GoogleSQL should leverage either DDL (e.g. CREATE [TEMPORARY] FUNCTION ... ) or the Routines API to define UDF resources. For additional information on migrating, see: https://cloud.google.com/bigquery/docs/reference/standard-sql/migrating-from-legacy-sql#differences_in_user-defined_javascript_functions
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserDefinedFunctionResource {
    /// [Pick one] An inline resource that contains code for a user-defined function (UDF). Providing a inline code resource is equivalent to providing a URI for a file containing the same code.
    #[serde(rename="inlineCode")]
    
    pub inline_code: Option<String>,
    /// [Pick one] A code resource to load from a Google Cloud Storage URI (gs://bucket/path).
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
}

impl client::Part for UserDefinedFunctionResource {}


/// Statistics for a vector search query. Populated as part of JobStatistics2.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VectorSearchStatistics {
    /// When `indexUsageMode` is `UNUSED` or `PARTIALLY_USED`, this field explains why indexes were not used in all or part of the vector search query. If `indexUsageMode` is `FULLY_USED`, this field is not populated.
    #[serde(rename="indexUnusedReasons")]
    
    pub index_unused_reasons: Option<Vec<IndexUnusedReason>>,
    /// Specifies the index usage mode for the query.
    #[serde(rename="indexUsageMode")]
    
    pub index_usage_mode: Option<String>,
}

impl client::Part for VectorSearchStatistics {}


/// Describes the definition of a logical view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ViewDefinition {
    /// Optional. Specifices the privacy policy for the view.
    #[serde(rename="privacyPolicy")]
    
    pub privacy_policy: Option<PrivacyPolicy>,
    /// Required. A query that BigQuery executes when the view is referenced.
    
    pub query: Option<String>,
    /// True if the column names are explicitly specified. For example by using the 'CREATE VIEW v(c1, c2) AS ...' syntax. Can only be set for GoogleSQL views.
    #[serde(rename="useExplicitColumnNames")]
    
    pub use_explicit_column_names: Option<bool>,
    /// Specifies whether to use BigQuery's legacy SQL for this view. The default value is true. If set to false, the view will use BigQuery's GoogleSQL: https://cloud.google.com/bigquery/sql-reference/ Queries and views that reference this view must use the same flag value. A wrapper is used here because the default value is True.
    #[serde(rename="useLegacySql")]
    
    pub use_legacy_sql: Option<bool>,
    /// Describes user-defined function resources used in the query.
    #[serde(rename="userDefinedFunctionResources")]
    
    pub user_defined_function_resources: Option<Vec<UserDefinedFunctionResource>>,
}

impl client::Part for ViewDefinition {}


/// Deprecated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BqmlTrainingRunTrainingOptions {
    /// no description provided
    #[serde(rename="earlyStop")]
    
    pub early_stop: Option<bool>,
    /// no description provided
    #[serde(rename="l1Reg")]
    
    pub l1_reg: Option<f64>,
    /// no description provided
    #[serde(rename="l2Reg")]
    
    pub l2_reg: Option<f64>,
    /// no description provided
    #[serde(rename="learnRate")]
    
    pub learn_rate: Option<f64>,
    /// no description provided
    #[serde(rename="learnRateStrategy")]
    
    pub learn_rate_strategy: Option<String>,
    /// no description provided
    #[serde(rename="lineSearchInitLearnRate")]
    
    pub line_search_init_learn_rate: Option<f64>,
    /// no description provided
    #[serde(rename="maxIteration")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_iteration: Option<i64>,
    /// no description provided
    #[serde(rename="minRelProgress")]
    
    pub min_rel_progress: Option<f64>,
    /// no description provided
    #[serde(rename="warmStart")]
    
    pub warm_start: Option<bool>,
}

impl client::NestedType for BqmlTrainingRunTrainingOptions {}
impl client::Part for BqmlTrainingRunTrainingOptions {}


/// An object that defines dataset access for an entity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetAccess {
    /// [Pick one] A grant authorizing all resources of a particular type in a particular dataset access to this dataset. Only views are supported for now. The role field is not required when this field is set. If that dataset is deleted and re-created, its access needs to be granted again via an update operation.
    
    pub dataset: Option<DatasetAccessEntry>,
    /// [Pick one] A domain to grant access to. Any users signed in with the domain specified will be granted the specified access. Example: "example.com". Maps to IAM policy member "domain:DOMAIN".
    
    pub domain: Option<String>,
    /// [Pick one] An email address of a Google Group to grant access to. Maps to IAM policy member "group:GROUP".
    #[serde(rename="groupByEmail")]
    
    pub group_by_email: Option<String>,
    /// [Pick one] Some other type of member that appears in the IAM Policy but isn't a user, group, domain, or special group.
    #[serde(rename="iamMember")]
    
    pub iam_member: Option<String>,
    /// An IAM role ID that should be granted to the user, group, or domain specified in this access entry. The following legacy mappings will be applied: OWNER <=> roles/bigquery.dataOwner WRITER <=> roles/bigquery.dataEditor READER <=> roles/bigquery.dataViewer This field will accept any of the above formats, but will return only the legacy format. For example, if you set this field to "roles/bigquery.dataOwner", it will be returned back as "OWNER".
    
    pub role: Option<String>,
    /// [Pick one] A routine from a different dataset to grant access to. Queries executed against that routine will have read access to views/tables/routines in this dataset. Only UDF is supported for now. The role field is not required when this field is set. If that routine is updated by any user, access to the routine needs to be granted again via an update operation.
    
    pub routine: Option<RoutineReference>,
    /// [Pick one] A special group to grant access to. Possible values include: projectOwners: Owners of the enclosing project. projectReaders: Readers of the enclosing project. projectWriters: Writers of the enclosing project. allAuthenticatedUsers: All authenticated BigQuery users. Maps to similarly-named IAM members.
    #[serde(rename="specialGroup")]
    
    pub special_group: Option<String>,
    /// [Pick one] An email address of a user to grant access to. For example: fred@example.com. Maps to IAM policy member "user:EMAIL" or "serviceAccount:EMAIL".
    #[serde(rename="userByEmail")]
    
    pub user_by_email: Option<String>,
    /// [Pick one] A view from a different dataset to grant access to. Queries executed against that view will have read access to views/tables/routines in this dataset. The role field is not required when this field is set. If that view is updated by any user, access to the view needs to be granted again via an update operation.
    
    pub view: Option<TableReference>,
}

impl client::NestedType for DatasetAccess {}
impl client::Part for DatasetAccess {}


/// A global tag managed by Resource Manager. https://cloud.google.com/iam/docs/tags-access-control#definitions
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetTags {
    /// Required. The namespaced friendly name of the tag key, e.g. "12345/environment" where 12345 is org id.
    #[serde(rename="tagKey")]
    
    pub tag_key: Option<String>,
    /// Required. The friendly short name of the tag value, e.g. "production".
    #[serde(rename="tagValue")]
    
    pub tag_value: Option<String>,
}

impl client::NestedType for DatasetTags {}
impl client::Part for DatasetTags {}


/// A dataset resource with only a subset of fields, to be returned in a list of datasets.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DatasetListDatasets {
    /// The dataset reference. Use this property to access specific parts of the dataset's ID, such as project ID or dataset ID.
    #[serde(rename="datasetReference")]
    
    pub dataset_reference: Option<DatasetReference>,
    /// An alternate name for the dataset. The friendly name is purely decorative in nature.
    #[serde(rename="friendlyName")]
    
    pub friendly_name: Option<String>,
    /// The fully-qualified, unique, opaque ID of the dataset.
    
    pub id: Option<String>,
    /// The resource type. This property always returns the value "bigquery#dataset"
    
    pub kind: Option<String>,
    /// The labels associated with this dataset. You can use these to organize and group your datasets.
    
    pub labels: Option<HashMap<String, String>>,
    /// The geographic location where the dataset resides.
    
    pub location: Option<String>,
}

impl client::NestedType for DatasetListDatasets {}
impl client::Part for DatasetListDatasets {}


/// ListFormatJob is a partial projection of job information returned as part of a jobs.list response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobListJobs {
    /// Required. Describes the job configuration.
    
    pub configuration: Option<JobConfiguration>,
    /// A result object that will be present only if the job has failed.
    #[serde(rename="errorResult")]
    
    pub error_result: Option<ErrorProto>,
    /// Unique opaque ID of the job.
    
    pub id: Option<String>,
    /// Unique opaque ID of the job.
    #[serde(rename="jobReference")]
    
    pub job_reference: Option<JobReference>,
    /// The resource type.
    
    pub kind: Option<String>,
    /// [Full-projection-only] String representation of identity of requesting party. Populated for both first- and third-party identities. Only present for APIs that support third-party identities.
    
    pub principal_subject: Option<String>,
    /// Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed.
    
    pub state: Option<String>,
    /// Output only. Information about the job, including starting time and ending time of the job.
    
    pub statistics: Option<JobStatistics>,
    /// [Full-projection-only] Describes the status of this job.
    
    pub status: Option<JobStatus>,
    /// [Full-projection-only] Email address of the user who ran the job.
    
    pub user_email: Option<String>,
}

impl client::NestedType for JobListJobs {}
impl client::Part for JobListJobs {}


/// Job resource usage breakdown by reservation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatisticsReservationUsage {
    /// Reservation name or "unreserved" for on-demand resources usage.
    
    pub name: Option<String>,
    /// Total slot milliseconds used by the reservation for a particular job.
    #[serde(rename="slotMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub slot_ms: Option<i64>,
}

impl client::NestedType for JobStatisticsReservationUsage {}
impl client::Part for JobStatisticsReservationUsage {}


/// Job resource usage breakdown by reservation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JobStatistics2ReservationUsage {
    /// Reservation name or "unreserved" for on-demand resources usage.
    
    pub name: Option<String>,
    /// Total slot milliseconds used by the reservation for a particular job.
    #[serde(rename="slotMs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub slot_ms: Option<i64>,
}

impl client::NestedType for JobStatistics2ReservationUsage {}
impl client::Part for JobStatistics2ReservationUsage {}


/// Deprecated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModelDefinitionModelOptions {
    /// no description provided
    
    pub labels: Option<Vec<String>>,
    /// no description provided
    #[serde(rename="lossType")]
    
    pub loss_type: Option<String>,
    /// no description provided
    #[serde(rename="modelType")]
    
    pub model_type: Option<String>,
}

impl client::NestedType for ModelDefinitionModelOptions {}
impl client::Part for ModelDefinitionModelOptions {}


/// Information about a single project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectListProjects {
    /// A descriptive name for this project. A wrapper is used here because friendlyName can be set to the empty string.
    #[serde(rename="friendlyName")]
    
    pub friendly_name: Option<String>,
    /// An opaque ID of this project.
    
    pub id: Option<String>,
    /// The resource type.
    
    pub kind: Option<String>,
    /// The numeric ID of this project.
    #[serde(rename="numericId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub numeric_id: Option<u64>,
    /// A unique reference to this project.
    #[serde(rename="projectReference")]
    
    pub project_reference: Option<ProjectReference>,
}

impl client::NestedType for ProjectListProjects {}
impl client::Part for ProjectListProjects {}


/// The type of a struct parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct QueryParameterTypeStructTypes {
    /// Optional. Human-oriented description of the field.
    
    pub description: Option<String>,
    /// Optional. The name of this field.
    
    pub name: Option<String>,
    /// Required. The type of this field.
    #[serde(rename="type")]
    
    pub type_: Option<QueryParameterType>,
}

impl client::NestedType for QueryParameterTypeStructTypes {}
impl client::Part for QueryParameterTypeStructTypes {}


/// [Experimental] Defines the ranges for range partitioning.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RangePartitioningRange {
    /// [Experimental] The end of range partitioning, exclusive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end: Option<i64>,
    /// [Experimental] The width of each interval.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub interval: Option<i64>,
    /// [Experimental] The start of range partitioning, inclusive.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start: Option<i64>,
}

impl client::NestedType for RangePartitioningRange {}
impl client::Part for RangePartitioningRange {}


/// Represents a foreign key constraint on a table's columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableConstraintsForeignKeys {
    /// Required. The columns that compose the foreign key.
    #[serde(rename="columnReferences")]
    
    pub column_references: Option<Vec<TableConstraintsForeignKeysColumnReferences>>,
    /// Optional. Set only if the foreign key constraint is named.
    
    pub name: Option<String>,
    /// no description provided
    #[serde(rename="referencedTable")]
    
    pub referenced_table: Option<TableConstraintsForeignKeysReferencedTable>,
}

impl client::NestedType for TableConstraintsForeignKeys {}
impl client::Part for TableConstraintsForeignKeys {}


/// The pair of the foreign key column and primary key column.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableConstraintsForeignKeysColumnReferences {
    /// Required. The column in the primary key that are referenced by the referencing_column.
    #[serde(rename="referencedColumn")]
    
    pub referenced_column: Option<String>,
    /// Required. The column that composes the foreign key.
    #[serde(rename="referencingColumn")]
    
    pub referencing_column: Option<String>,
}

impl client::NestedType for TableConstraintsForeignKeysColumnReferences {}
impl client::Part for TableConstraintsForeignKeysColumnReferences {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableConstraintsForeignKeysReferencedTable {
    /// no description provided
    #[serde(rename="datasetId")]
    
    pub dataset_id: Option<String>,
    /// no description provided
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// no description provided
    #[serde(rename="tableId")]
    
    pub table_id: Option<String>,
}

impl client::NestedType for TableConstraintsForeignKeysReferencedTable {}
impl client::Part for TableConstraintsForeignKeysReferencedTable {}


/// Represents the primary key constraint on a table's columns.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableConstraintsPrimaryKey {
    /// Required. The columns that are composed of the primary key constraint.
    
    pub columns: Option<Vec<String>>,
}

impl client::NestedType for TableConstraintsPrimaryKey {}
impl client::Part for TableConstraintsPrimaryKey {}


/// Data for a single insertion row.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllRequestRows {
    /// Insertion ID for best-effort deduplication. This feature is not recommended, and users seeking stronger insertion semantics are encouraged to use other mechanisms such as the BigQuery Write API.
    #[serde(rename="insertId")]
    
    pub insert_id: Option<String>,
    /// Data for a single row.
    
    pub json: Option<JsonObject>,
}

impl client::NestedType for TableDataInsertAllRequestRows {}
impl client::Part for TableDataInsertAllRequestRows {}


/// Error details about a single row's insertion.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableDataInsertAllResponseInsertErrors {
    /// Error information for the row indicated by the index property.
    
    pub errors: Option<Vec<ErrorProto>>,
    /// The index of the row that error applies to.
    
    pub index: Option<u32>,
}

impl client::NestedType for TableDataInsertAllResponseInsertErrors {}
impl client::Part for TableDataInsertAllResponseInsertErrors {}


/// Deprecated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableFieldSchemaCategories {
    /// Deprecated.
    
    pub names: Option<Vec<String>>,
}

impl client::NestedType for TableFieldSchemaCategories {}
impl client::Part for TableFieldSchemaCategories {}


/// Optional. The policy tags attached to this field, used for field-level access control. If not set, defaults to empty policy_tags.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableFieldSchemaPolicyTags {
    /// A list of policy tag resource names. For example, "projects/1/locations/eu/taxonomies/2/policyTags/3". At most 1 policy tag is currently allowed.
    
    pub names: Option<Vec<String>>,
}

impl client::NestedType for TableFieldSchemaPolicyTags {}
impl client::Part for TableFieldSchemaPolicyTags {}


/// Represents the type of a field element.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableFieldSchemaRangeElementType {
    /// Required. The type of a field element. See TableFieldSchema.type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for TableFieldSchemaRangeElementType {}
impl client::Part for TableFieldSchemaRangeElementType {}


/// Tables in the requested dataset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableListTables {
    /// Clustering specification for this table, if configured.
    
    pub clustering: Option<Clustering>,
    /// Output only. The time when this table was created, in milliseconds since the epoch.
    #[serde(rename="creationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub creation_time: Option<i64>,
    /// The time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed.
    #[serde(rename="expirationTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration_time: Option<i64>,
    /// The user-friendly name for this table.
    #[serde(rename="friendlyName")]
    
    pub friendly_name: Option<String>,
    /// An opaque ID of the table.
    
    pub id: Option<String>,
    /// The resource type.
    
    pub kind: Option<String>,
    /// The labels associated with this table. You can use these to organize and group your tables.
    
    pub labels: Option<HashMap<String, String>>,
    /// The range partitioning for this table.
    #[serde(rename="rangePartitioning")]
    
    pub range_partitioning: Option<RangePartitioning>,
    /// Optional. If set to true, queries including this table must specify a partition filter. This filter is used for partition elimination.
    #[serde(rename="requirePartitionFilter")]
    
    pub require_partition_filter: Option<bool>,
    /// A reference uniquely identifying table.
    #[serde(rename="tableReference")]
    
    pub table_reference: Option<TableReference>,
    /// The time-based partitioning for this table.
    #[serde(rename="timePartitioning")]
    
    pub time_partitioning: Option<TimePartitioning>,
    /// The type of table.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Information about a logical view.
    
    pub view: Option<TableListTablesView>,
}

impl client::NestedType for TableListTables {}
impl client::Part for TableListTables {}


/// Information about a logical view.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TableListTablesView {
    /// Specifices the privacy policy for the view.
    #[serde(rename="privacyPolicy")]
    
    pub privacy_policy: Option<PrivacyPolicy>,
    /// True if view is defined in legacy SQL dialect, false if in GoogleSQL.
    #[serde(rename="useLegacySql")]
    
    pub use_legacy_sql: Option<bool>,
}

impl client::NestedType for TableListTablesView {}
impl client::Part for TableListTablesView {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *dataset* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `undelete(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.datasets();
/// # }
/// ```
pub struct DatasetMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for DatasetMethods<'a, S> {}

impl<'a, S> DatasetMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the dataset being deleted
    /// * `datasetId` - Required. Dataset ID of dataset being deleted
    pub fn delete(&self, project_id: &str, dataset_id: &str) -> DatasetDeleteCall<'a, S> {
        DatasetDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delete_contents: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the dataset specified by datasetID.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the requested dataset
    /// * `datasetId` - Required. Dataset ID of the requested dataset
    pub fn get(&self, project_id: &str, dataset_id: &str) -> DatasetGetCall<'a, S> {
        DatasetGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _dataset_view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new empty dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the new dataset
    pub fn insert(&self, request: Dataset, project_id: &str) -> DatasetInsertCall<'a, S> {
        DatasetInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all datasets in the specified project to which the user has been granted the READER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the datasets to be listed
    pub fn list(&self, project_id: &str) -> DatasetListCall<'a, S> {
        DatasetListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _all: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports RFC5789 patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the dataset being updated
    /// * `datasetId` - Required. Dataset ID of the dataset being updated
    pub fn patch(&self, request: Dataset, project_id: &str, dataset_id: &str) -> DatasetPatchCall<'a, S> {
        DatasetPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Undeletes a dataset which is within time travel window based on datasetId. If a time is specified, the dataset version deleted at that time is undeleted, else the last live version is undeleted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the dataset to be undeleted
    /// * `datasetId` - Required. Dataset ID of dataset being deleted
    pub fn undelete(&self, request: UndeleteDatasetRequest, project_id: &str, dataset_id: &str) -> DatasetUndeleteCall<'a, S> {
        DatasetUndeleteCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the dataset being updated
    /// * `datasetId` - Required. Dataset ID of the dataset being updated
    pub fn update(&self, request: Dataset, project_id: &str, dataset_id: &str) -> DatasetUpdateCall<'a, S> {
        DatasetUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *job* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `cancel(...)`, `delete(...)`, `get(...)`, `get_query_results(...)`, `insert(...)`, `list(...)` and `query(...)`
/// // to build up your call.
/// let rb = hub.jobs();
/// # }
/// ```
pub struct JobMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for JobMethods<'a, S> {}

impl<'a, S> JobMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the job to cancel
    /// * `jobId` - Required. Job ID of the job to cancel
    pub fn cancel(&self, project_id: &str, job_id: &str) -> JobCancelCall<'a, S> {
        JobCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _location: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Requests the deletion of the metadata of a job. This call returns when the job's metadata is deleted.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the job for which metadata is to be deleted.
    /// * `jobId` - Required. Job ID of the job for which metadata is to be deleted. If this is a parent job which has child jobs, the metadata from all child jobs will be deleted as well. Direct deletion of the metadata of child jobs is not allowed.
    pub fn delete(&self, project_id: &str, job_id: &str) -> JobDeleteCall<'a, S> {
        JobDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _location: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the requested job.
    /// * `jobId` - Required. Job ID of the requested job.
    pub fn get(&self, project_id: &str, job_id: &str) -> JobGetCall<'a, S> {
        JobGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _location: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RPC to get the results of a query job.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the query job.
    /// * `jobId` - Required. Job ID of the query job.
    pub fn get_query_results(&self, project_id: &str, job_id: &str) -> JobGetQueryResultCall<'a, S> {
        JobGetQueryResultCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _job_id: job_id.to_string(),
            _timeout_ms: Default::default(),
            _start_index: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _location: Default::default(),
            _format_options_use_int64_timestamp: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts a new asynchronous job. This API has two different kinds of endpoint URIs, as this method supports a variety of use cases. * The *Metadata* URI is used for most interactions, as it accepts the job configuration directly. * The *Upload* URI is ONLY for the case when you're sending both a load job configuration and a data stream together. In this case, the Upload URI accepts the job configuration and the data as two distinct multipart MIME parts.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID of project that will be billed for the job.
    pub fn insert(&self, request: Job, project_id: &str) -> JobInsertCall<'a, S> {
        JobInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID of the jobs to list.
    pub fn list(&self, project_id: &str) -> JobListCall<'a, S> {
        JobListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _state_filter: Default::default(),
            _projection: Default::default(),
            _parent_job_id: Default::default(),
            _page_token: Default::default(),
            _min_creation_time: Default::default(),
            _max_results: Default::default(),
            _max_creation_time: Default::default(),
            _all_users: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the query request.
    pub fn query(&self, request: QueryRequest, project_id: &str) -> JobQueryCall<'a, S> {
        JobQueryCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *model* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `patch(...)`
/// // to build up your call.
/// let rb = hub.models();
/// # }
/// ```
pub struct ModelMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for ModelMethods<'a, S> {}

impl<'a, S> ModelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the model specified by modelId from the dataset.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the model to delete.
    /// * `datasetId` - Required. Dataset ID of the model to delete.
    /// * `modelId` - Required. Model ID of the model to delete.
    pub fn delete(&self, project_id: &str, dataset_id: &str, model_id: &str) -> ModelDeleteCall<'a, S> {
        ModelDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _model_id: model_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified model resource by model ID.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the requested model.
    /// * `datasetId` - Required. Dataset ID of the requested model.
    /// * `modelId` - Required. Model ID of the requested model.
    pub fn get(&self, project_id: &str, dataset_id: &str, model_id: &str) -> ModelGetCall<'a, S> {
        ModelGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _model_id: model_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all models in the specified dataset. Requires the READER dataset role. After retrieving the list of models, you can get information about a particular model by calling the models.get method.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the models to list.
    /// * `datasetId` - Required. Dataset ID of the models to list.
    pub fn list(&self, project_id: &str, dataset_id: &str) -> ModelListCall<'a, S> {
        ModelListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patch specific fields in the specified model.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the model to patch.
    /// * `datasetId` - Required. Dataset ID of the model to patch.
    /// * `modelId` - Required. Model ID of the model to patch.
    pub fn patch(&self, request: Model, project_id: &str, dataset_id: &str, model_id: &str) -> ModelPatchCall<'a, S> {
        ModelPatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _model_id: model_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_service_account(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for ProjectMethods<'a, S> {}

impl<'a, S> ProjectMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// RPC to get the service account for a project used for interactions with Google Cloud KMS
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. ID of the project.
    pub fn get_service_account(&self, project_id: &str) -> ProjectGetServiceAccountCall<'a, S> {
        ProjectGetServiceAccountCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// RPC to list projects to which the user has been granted any project role. Users of this method are encouraged to consider the [Resource Manager](https://cloud.google.com/resource-manager/docs/) API, which provides the underlying data for this method and has more capabilities.
    pub fn list(&self) -> ProjectListCall<'a, S> {
        ProjectListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *routine* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.routines();
/// # }
/// ```
pub struct RoutineMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for RoutineMethods<'a, S> {}

impl<'a, S> RoutineMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the routine specified by routineId from the dataset.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the routine to delete
    /// * `datasetId` - Required. Dataset ID of the routine to delete
    /// * `routineId` - Required. Routine ID of the routine to delete
    pub fn delete(&self, project_id: &str, dataset_id: &str, routine_id: &str) -> RoutineDeleteCall<'a, S> {
        RoutineDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _routine_id: routine_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified routine resource by routine ID.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the requested routine
    /// * `datasetId` - Required. Dataset ID of the requested routine
    /// * `routineId` - Required. Routine ID of the requested routine
    pub fn get(&self, project_id: &str, dataset_id: &str, routine_id: &str) -> RoutineGetCall<'a, S> {
        RoutineGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _routine_id: routine_id.to_string(),
            _read_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new routine in the dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the new routine
    /// * `datasetId` - Required. Dataset ID of the new routine
    pub fn insert(&self, request: Routine, project_id: &str, dataset_id: &str) -> RoutineInsertCall<'a, S> {
        RoutineInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all routines in the specified dataset. Requires the READER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the routines to list
    /// * `datasetId` - Required. Dataset ID of the routines to list
    pub fn list(&self, project_id: &str, dataset_id: &str) -> RoutineListCall<'a, S> {
        RoutineListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _read_mask: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing routine. The update method replaces the entire Routine resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the routine to update
    /// * `datasetId` - Required. Dataset ID of the routine to update
    /// * `routineId` - Required. Routine ID of the routine to update
    pub fn update(&self, request: Routine, project_id: &str, dataset_id: &str, routine_id: &str) -> RoutineUpdateCall<'a, S> {
        RoutineUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _routine_id: routine_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *rowAccessPolicy* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_iam_policy(...)`, `list(...)` and `test_iam_permissions(...)`
/// // to build up your call.
/// let rb = hub.row_access_policies();
/// # }
/// ```
pub struct RowAccessPolicyMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for RowAccessPolicyMethods<'a, S> {}

impl<'a, S> RowAccessPolicyMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> RowAccessPolicyGetIamPolicyCall<'a, S> {
        RowAccessPolicyGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all row access policies on the specified table.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the row access policies to list.
    /// * `datasetId` - Required. Dataset ID of row access policies to list.
    /// * `tableId` - Required. Table ID of the table to list row access policies.
    pub fn list(&self, project_id: &str, dataset_id: &str, table_id: &str) -> RowAccessPolicyListCall<'a, S> {
        RowAccessPolicyListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> RowAccessPolicyTestIamPermissionCall<'a, S> {
        RowAccessPolicyTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *tabledata* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert_all(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.tabledata();
/// # }
/// ```
pub struct TabledataMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for TabledataMethods<'a, S> {}

impl<'a, S> TabledataMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Streams data into BigQuery one record at a time without needing to run a load job.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the destination.
    /// * `datasetId` - Required. Dataset ID of the destination.
    /// * `tableId` - Required. Table ID of the destination.
    pub fn insert_all(&self, request: TableDataInsertAllRequest, project_id: &str, dataset_id: &str, table_id: &str) -> TabledataInsertAllCall<'a, S> {
        TabledataInsertAllCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List the content of a table in rows.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project id of the table to list.
    /// * `datasetId` - Required. Dataset id of the table to list.
    /// * `tableId` - Required. Table id of the table to list.
    pub fn list(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TabledataListCall<'a, S> {
        TabledataListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _start_index: Default::default(),
            _selected_fields: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _format_options_use_int64_timestamp: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *table* resources.
/// It is not used directly, but through the [`Bigquery`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_bigquery2 as bigquery2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.tables();
/// # }
/// ```
pub struct TableMethods<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
}

impl<'a, S> client::MethodsBuilder for TableMethods<'a, S> {}

impl<'a, S> TableMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the table to delete
    /// * `datasetId` - Required. Dataset ID of the table to delete
    /// * `tableId` - Required. Table ID of the table to delete
    pub fn delete(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TableDeleteCall<'a, S> {
        TableDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the requested table
    /// * `datasetId` - Required. Dataset ID of the requested table
    /// * `tableId` - Required. Table ID of the requested table
    pub fn get(&self, project_id: &str, dataset_id: &str, table_id: &str) -> TableGetCall<'a, S> {
        TableGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _view: Default::default(),
            _selected_fields: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn get_iam_policy(&self, request: GetIamPolicyRequest, resource: &str) -> TableGetIamPolicyCall<'a, S> {
        TableGetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new, empty table in the dataset.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the new table
    /// * `datasetId` - Required. Dataset ID of the new table
    pub fn insert(&self, request: Table, project_id: &str, dataset_id: &str) -> TableInsertCall<'a, S> {
        TableInsertCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all tables in the specified dataset. Requires the READER dataset role.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Required. Project ID of the tables to list
    /// * `datasetId` - Required. Dataset ID of the tables to list
    pub fn list(&self, project_id: &str, dataset_id: &str) -> TableListCall<'a, S> {
        TableListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports RFC5789 patch semantics.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the table to update
    /// * `datasetId` - Required. Dataset ID of the table to update
    /// * `tableId` - Required. Table ID of the table to update
    pub fn patch(&self, request: Table, project_id: &str, dataset_id: &str, table_id: &str) -> TablePatchCall<'a, S> {
        TablePatchCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _autodetect_schema: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn set_iam_policy(&self, request: SetIamPolicyRequest, resource: &str) -> TableSetIamPolicyCall<'a, S> {
        TableSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `resource` - REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    pub fn test_iam_permissions(&self, request: TestIamPermissionsRequest, resource: &str) -> TableTestIamPermissionCall<'a, S> {
        TableTestIamPermissionCall {
            hub: self.hub,
            _request: request,
            _resource: resource.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates information in an existing table. The update method replaces the entire Table resource, whereas the patch method only replaces fields that are provided in the submitted Table resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Required. Project ID of the table to update
    /// * `datasetId` - Required. Dataset ID of the table to update
    /// * `tableId` - Required. Table ID of the table to update
    pub fn update(&self, request: Table, project_id: &str, dataset_id: &str, table_id: &str) -> TableUpdateCall<'a, S> {
        TableUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _dataset_id: dataset_id.to_string(),
            _table_id: table_id.to_string(),
            _autodetect_schema: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Deletes the dataset specified by the datasetId value. Before you can delete a dataset, you must delete all its tables, either manually or by specifying deleteContents. Immediately after deletion, you can create another dataset with the same name.
///
/// A builder for the *delete* method supported by a *dataset* resource.
/// It is not used directly, but through a [`DatasetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().delete("projectId", "datasetId")
///              .delete_contents(true)
///              .doit().await;
/// # }
/// ```
pub struct DatasetDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _delete_contents: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DatasetDeleteCall<'a, S> {}

impl<'a, S> DatasetDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.datasets.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["projectId", "datasetId", "deleteContents"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        if let Some(value) = self._delete_contents.as_ref() {
            params.push("deleteContents", value.to_string());
        }

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Project ID of the dataset being deleted
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetDeleteCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of dataset being deleted
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetDeleteCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// If True, delete all the tables in the dataset. If False and the dataset contains tables, the request will fail. Default is False
    ///
    /// Sets the *delete contents* query property to the given value.
    pub fn delete_contents(mut self, new_value: bool) -> DatasetDeleteCall<'a, S> {
        self._delete_contents = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DatasetDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DatasetDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DatasetDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DatasetDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns the dataset specified by datasetID.
///
/// A builder for the *get* method supported by a *dataset* resource.
/// It is not used directly, but through a [`DatasetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().get("projectId", "datasetId")
///              .dataset_view("duo")
///              .doit().await;
/// # }
/// ```
pub struct DatasetGetCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _dataset_view: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DatasetGetCall<'a, S> {}

impl<'a, S> DatasetGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.datasets.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "datasetView"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        if let Some(value) = self._dataset_view.as_ref() {
            params.push("datasetView", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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


    /// Required. Project ID of the requested dataset
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetGetCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the requested dataset
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetGetCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Optional. Specifies the view that determines which dataset information is returned. By default, metadata and ACL information are returned.
    ///
    /// Sets the *dataset view* query property to the given value.
    pub fn dataset_view(mut self, new_value: &str) -> DatasetGetCall<'a, S> {
        self._dataset_view = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DatasetGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DatasetGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DatasetGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DatasetGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a new empty dataset.
///
/// A builder for the *insert* method supported by a *dataset* resource.
/// It is not used directly, but through a [`DatasetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Dataset;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Dataset::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().insert(req, "projectId")
///              .doit().await;
/// # }
/// ```
pub struct DatasetInsertCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Dataset,
    _project_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DatasetInsertCall<'a, S> {}

impl<'a, S> DatasetInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.datasets.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["projectId"];
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
    pub fn request(mut self, new_value: Dataset) -> DatasetInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the new dataset
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetInsertCall<'a, S> {
        self._project_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DatasetInsertCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DatasetInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DatasetInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DatasetInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all datasets in the specified project to which the user has been granted the READER dataset role.
///
/// A builder for the *list* method supported by a *dataset* resource.
/// It is not used directly, but through a [`DatasetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().list("projectId")
///              .page_token("ut")
///              .max_results(89)
///              .filter("rebum.")
///              .all(true)
///              .doit().await;
/// # }
/// ```
pub struct DatasetListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _filter: Option<String>,
    _all: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DatasetListCall<'a, S> {}

impl<'a, S> DatasetListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DatasetList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.datasets.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "pageToken", "maxResults", "filter", "all"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("projectId", self._project_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._all.as_ref() {
            params.push("all", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["projectId"];
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


    /// Required. Project ID of the datasets to be listed
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetListCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DatasetListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> DatasetListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// An expression for filtering the results of the request by label. The syntax is "labels.<name>\[:<value>\]". Multiple filters can be ANDed together by connecting with a space. Example: "labels.department:receiving labels.active". See [Filtering datasets using labels](https://cloud.google.com/bigquery/docs/labeling-datasets#filtering_datasets_using_labels) for details.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> DatasetListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Whether to list all datasets, including hidden ones
    ///
    /// Sets the *all* query property to the given value.
    pub fn all(mut self, new_value: bool) -> DatasetListCall<'a, S> {
        self._all = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DatasetListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DatasetListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DatasetListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DatasetListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource. This method supports RFC5789 patch semantics.
///
/// A builder for the *patch* method supported by a *dataset* resource.
/// It is not used directly, but through a [`DatasetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Dataset;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Dataset::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().patch(req, "projectId", "datasetId")
///              .doit().await;
/// # }
/// ```
pub struct DatasetPatchCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Dataset,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DatasetPatchCall<'a, S> {}

impl<'a, S> DatasetPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.datasets.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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
                    .method(hyper::Method::PATCH)
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
    pub fn request(mut self, new_value: Dataset) -> DatasetPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the dataset being updated
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetPatchCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the dataset being updated
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetPatchCall<'a, S> {
        self._dataset_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DatasetPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DatasetPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DatasetPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DatasetPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Undeletes a dataset which is within time travel window based on datasetId. If a time is specified, the dataset version deleted at that time is undeleted, else the last live version is undeleted.
///
/// A builder for the *undelete* method supported by a *dataset* resource.
/// It is not used directly, but through a [`DatasetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::UndeleteDatasetRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UndeleteDatasetRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().undelete(req, "projectId", "datasetId")
///              .doit().await;
/// # }
/// ```
pub struct DatasetUndeleteCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: UndeleteDatasetRequest,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DatasetUndeleteCall<'a, S> {}

impl<'a, S> DatasetUndeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.datasets.undelete",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}:undelete";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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
    pub fn request(mut self, new_value: UndeleteDatasetRequest) -> DatasetUndeleteCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the dataset to be undeleted
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetUndeleteCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of dataset being deleted
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetUndeleteCall<'a, S> {
        self._dataset_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DatasetUndeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetUndeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DatasetUndeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DatasetUndeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DatasetUndeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates information in an existing dataset. The update method replaces the entire dataset resource, whereas the patch method only replaces fields that are provided in the submitted dataset resource.
///
/// A builder for the *update* method supported by a *dataset* resource.
/// It is not used directly, but through a [`DatasetMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Dataset;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Dataset::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.datasets().update(req, "projectId", "datasetId")
///              .doit().await;
/// # }
/// ```
pub struct DatasetUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Dataset,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DatasetUpdateCall<'a, S> {}

impl<'a, S> DatasetUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Dataset)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.datasets.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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
    pub fn request(mut self, new_value: Dataset) -> DatasetUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the dataset being updated
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> DatasetUpdateCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the dataset being updated
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> DatasetUpdateCall<'a, S> {
        self._dataset_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DatasetUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DatasetUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DatasetUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DatasetUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DatasetUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Requests that a job be cancelled. This call will return immediately, and the client will need to poll for the job status to see if the cancel completed successfully. Cancelled jobs may still incur costs.
///
/// A builder for the *cancel* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().cancel("projectId", "jobId")
///              .location("labore")
///              .doit().await;
/// # }
/// ```
pub struct JobCancelCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _job_id: String,
    _location: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobCancelCall<'a, S> {}

impl<'a, S> JobCancelCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, JobCancelResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.cancel",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId", "jobId", "location"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("jobId", self._job_id);
        if let Some(value) = self._location.as_ref() {
            params.push("location", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/jobs/{+jobId}/cancel";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["jobId", "projectId"];
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
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
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


    /// Required. Project ID of the job to cancel
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobCancelCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Job ID of the job to cancel
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobCancelCall<'a, S> {
        self._job_id = new_value.to_string();
        self
    }
    /// The geographic location of the job. You must specify the location to run the job for the following scenarios: - If the location to run a job is not in the `us` or the `eu` multi-regional location - If the job's location is in a single region (for example, `us-central1`) For more information, see https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    ///
    /// Sets the *location* query property to the given value.
    pub fn location(mut self, new_value: &str) -> JobCancelCall<'a, S> {
        self._location = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobCancelCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobCancelCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobCancelCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobCancelCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobCancelCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Requests the deletion of the metadata of a job. This call returns when the job's metadata is deleted.
///
/// A builder for the *delete* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().delete("projectId", "jobId")
///              .location("sed")
///              .doit().await;
/// # }
/// ```
pub struct JobDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _job_id: String,
    _location: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobDeleteCall<'a, S> {}

impl<'a, S> JobDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["projectId", "jobId", "location"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("jobId", self._job_id);
        if let Some(value) = self._location.as_ref() {
            params.push("location", value);
        }

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/jobs/{+jobId}/delete";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["jobId", "projectId"];
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
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Project ID of the job for which metadata is to be deleted.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobDeleteCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Job ID of the job for which metadata is to be deleted. If this is a parent job which has child jobs, the metadata from all child jobs will be deleted as well. Direct deletion of the metadata of child jobs is not allowed.
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobDeleteCall<'a, S> {
        self._job_id = new_value.to_string();
        self
    }
    /// The geographic location of the job. Required. See details at: https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    ///
    /// Sets the *location* query property to the given value.
    pub fn location(mut self, new_value: &str) -> JobDeleteCall<'a, S> {
        self._location = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns information about a specific job. Job information is available for a six month period after creation. Requires that you're the person who ran the job, or have the Is Owner project role.
///
/// A builder for the *get* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().get("projectId", "jobId")
///              .location("kasd")
///              .doit().await;
/// # }
/// ```
pub struct JobGetCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _job_id: String,
    _location: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobGetCall<'a, S> {}

impl<'a, S> JobGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "jobId", "location"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("jobId", self._job_id);
        if let Some(value) = self._location.as_ref() {
            params.push("location", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/jobs/{+jobId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["jobId", "projectId"];
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


    /// Required. Project ID of the requested job.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobGetCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Job ID of the requested job.
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobGetCall<'a, S> {
        self._job_id = new_value.to_string();
        self
    }
    /// The geographic location of the job. You must specify the location to run the job for the following scenarios: - If the location to run a job is not in the `us` or the `eu` multi-regional location - If the job's location is in a single region (for example, `us-central1`) For more information, see https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    ///
    /// Sets the *location* query property to the given value.
    pub fn location(mut self, new_value: &str) -> JobGetCall<'a, S> {
        self._location = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// RPC to get the results of a query job.
///
/// A builder for the *getQueryResults* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().get_query_results("projectId", "jobId")
///              .timeout_ms(77)
///              .start_index(33)
///              .page_token("vero")
///              .max_results(70)
///              .location("sed")
///              .format_options_use_int64_timestamp(false)
///              .doit().await;
/// # }
/// ```
pub struct JobGetQueryResultCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _job_id: String,
    _timeout_ms: Option<u32>,
    _start_index: Option<u64>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _location: Option<String>,
    _format_options_use_int64_timestamp: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobGetQueryResultCall<'a, S> {}

impl<'a, S> JobGetQueryResultCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetQueryResultsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.getQueryResults",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "jobId", "timeoutMs", "startIndex", "pageToken", "maxResults", "location", "formatOptions.useInt64Timestamp"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("jobId", self._job_id);
        if let Some(value) = self._timeout_ms.as_ref() {
            params.push("timeoutMs", value.to_string());
        }
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._location.as_ref() {
            params.push("location", value);
        }
        if let Some(value) = self._format_options_use_int64_timestamp.as_ref() {
            params.push("formatOptions.useInt64Timestamp", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/queries/{+jobId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+jobId}", "jobId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["jobId", "projectId"];
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


    /// Required. Project ID of the query job.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobGetQueryResultCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Job ID of the query job.
    ///
    /// Sets the *job id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn job_id(mut self, new_value: &str) -> JobGetQueryResultCall<'a, S> {
        self._job_id = new_value.to_string();
        self
    }
    /// Optional: Specifies the maximum amount of time, in milliseconds, that the client is willing to wait for the query to complete. By default, this limit is 10 seconds (10,000 milliseconds). If the query is complete, the jobComplete field in the response is true. If the query has not yet completed, jobComplete is false. You can request a longer timeout period in the timeoutMs field. However, the call is not guaranteed to wait for the specified timeout; it typically returns after around 200 seconds (200,000 milliseconds), even if the query is not complete. If jobComplete is false, you can continue to wait for the query to complete by calling the getQueryResults method until the jobComplete field in the getQueryResults response is true.
    ///
    /// Sets the *timeout ms* query property to the given value.
    pub fn timeout_ms(mut self, new_value: u32) -> JobGetQueryResultCall<'a, S> {
        self._timeout_ms = Some(new_value);
        self
    }
    /// Zero-based index of the starting row.
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u64) -> JobGetQueryResultCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// Page token, returned by a previous call, to request the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobGetQueryResultCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to read.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> JobGetQueryResultCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The geographic location of the job. You must specify the location to run the job for the following scenarios: - If the location to run a job is not in the `us` or the `eu` multi-regional location - If the job's location is in a single region (for example, `us-central1`) For more information, see https://cloud.google.com/bigquery/docs/locations#specifying_your_location.
    ///
    /// Sets the *location* query property to the given value.
    pub fn location(mut self, new_value: &str) -> JobGetQueryResultCall<'a, S> {
        self._location = Some(new_value.to_string());
        self
    }
    /// Optional. Output timestamp as usec int64. Default is false.
    ///
    /// Sets the *format options.use int64 timestamp* query property to the given value.
    pub fn format_options_use_int64_timestamp(mut self, new_value: bool) -> JobGetQueryResultCall<'a, S> {
        self._format_options_use_int64_timestamp = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobGetQueryResultCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobGetQueryResultCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobGetQueryResultCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobGetQueryResultCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobGetQueryResultCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Starts a new asynchronous job. This API has two different kinds of endpoint URIs, as this method supports a variety of use cases. * The *Metadata* URI is used for most interactions, as it accepts the job configuration directly. * The *Upload* URI is ONLY for the case when you're sending both a load job configuration and a data stream together. In this case, the Upload URI accepts the job configuration and the data as two distinct multipart MIME parts.
///
/// A builder for the *insert* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Job;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Job::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().insert(req, "projectId")
///              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct JobInsertCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Job,
    _project_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobInsertCall<'a, S> {}

impl<'a, S> JobInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{

    /// Perform the operation you have build so far, but without uploading. This is used to e.g. renaming or updating the description for a file
    pub async fn doit_without_upload(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Job)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/jobs";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["projectId"];
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



    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: client::UploadProtocol) -> client::Result<(hyper::Response<hyper::body::Body>, Job)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) =
            if protocol == client::UploadProtocol::Resumable {
                (self.hub._root_url.clone() + "resumable/upload/bigquery/v2/projects/{+projectId}/jobs", "resumable")
            } else if protocol == client::UploadProtocol::Simple {
                (self.hub._root_url.clone() + "upload/bigquery/v2/projects/{+projectId}/jobs", "multipart")
            } else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["projectId"];
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

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

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
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let mut mp_reader: client::MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        client::UploadProtocol::Simple => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            (&mut mp_reader as &mut (dyn io::Read + Send), client::MultiPartReader::mime_type())
                        },
                        _ => (&mut request_value_reader as &mut (dyn io::Read + Send), json_mime_type.clone()),
                    };
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .uri(url.as_str())
                        .header(USER_AGENT, self.hub._user_agent.clone());
    
                    if let Some(token) = token.as_ref() {
                        req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                    }
    
                    upload_url_from_server = true;
                    if protocol == client::UploadProtocol::Resumable {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let mut body_reader_bytes = vec![];
                            body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                            let request = req_builder
                                .header(CONTENT_TYPE, content_type.to_string())
                                .body(hyper::body::Body::from(body_reader_bytes));
    
                    client.request(request.unwrap()).await
    
                }
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
                    if protocol == client::UploadProtocol::Resumable {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!("Bearer {}", token.ok_or_else(|| client::Error::MissingToken("resumable upload requires token".into()))?.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
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

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 0kb
    /// * *valid mime types*: '*/*'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Job)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, client::UploadProtocol::Resumable).await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 0kb
    /// * *valid mime types*: '*/*'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Job)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, client::UploadProtocol::Simple).await
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Job) -> JobInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Project ID of project that will be billed for the job.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobInsertCall<'a, S> {
        self._project_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobInsertCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all jobs that you started in the specified project. Job information is available for a six month period after creation. The job list is sorted in reverse chronological order, by job creation time. Requires the Can View project role, or the Is Owner project role if you set the allUsers property.
///
/// A builder for the *list* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().list("projectId")
///              .add_state_filter("et")
///              .projection("et")
///              .parent_job_id("sadipscing")
///              .page_token("Stet")
///              .min_creation_time(2)
///              .max_results(81)
///              .max_creation_time(25)
///              .all_users(false)
///              .doit().await;
/// # }
/// ```
pub struct JobListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _state_filter: Vec<String>,
    _projection: Option<String>,
    _parent_job_id: Option<String>,
    _page_token: Option<String>,
    _min_creation_time: Option<u64>,
    _max_results: Option<u32>,
    _max_creation_time: Option<u64>,
    _all_users: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobListCall<'a, S> {}

impl<'a, S> JobListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, JobList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "stateFilter", "projection", "parentJobId", "pageToken", "minCreationTime", "maxResults", "maxCreationTime", "allUsers"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("projectId", self._project_id);
        if self._state_filter.len() > 0 {
            for f in self._state_filter.iter() {
                params.push("stateFilter", f);
            }
        }
        if let Some(value) = self._projection.as_ref() {
            params.push("projection", value);
        }
        if let Some(value) = self._parent_job_id.as_ref() {
            params.push("parentJobId", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._min_creation_time.as_ref() {
            params.push("minCreationTime", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._max_creation_time.as_ref() {
            params.push("maxCreationTime", value.to_string());
        }
        if let Some(value) = self._all_users.as_ref() {
            params.push("allUsers", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/jobs";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["projectId"];
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


    /// Project ID of the jobs to list.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobListCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Filter for job state
    ///
    /// Append the given value to the *state filter* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_state_filter(mut self, new_value: &str) -> JobListCall<'a, S> {
        self._state_filter.push(new_value.to_string());
        self
    }
    /// Restrict information returned to a set of selected fields
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> JobListCall<'a, S> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// If set, show only child jobs of the specified parent. Otherwise, show all top-level jobs.
    ///
    /// Sets the *parent job id* query property to the given value.
    pub fn parent_job_id(mut self, new_value: &str) -> JobListCall<'a, S> {
        self._parent_job_id = Some(new_value.to_string());
        self
    }
    /// Page token, returned by a previous call, to request the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> JobListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Min value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created after or at this timestamp are returned.
    ///
    /// Sets the *min creation time* query property to the given value.
    pub fn min_creation_time(mut self, new_value: u64) -> JobListCall<'a, S> {
        self._min_creation_time = Some(new_value);
        self
    }
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> JobListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// Max value for job creation time, in milliseconds since the POSIX epoch. If set, only jobs created before or at this timestamp are returned.
    ///
    /// Sets the *max creation time* query property to the given value.
    pub fn max_creation_time(mut self, new_value: u64) -> JobListCall<'a, S> {
        self._max_creation_time = Some(new_value);
        self
    }
    /// Whether to display jobs owned by all users in the project. Default False.
    ///
    /// Sets the *all users* query property to the given value.
    pub fn all_users(mut self, new_value: bool) -> JobListCall<'a, S> {
        self._all_users = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Runs a BigQuery SQL query synchronously and returns query results if the query completes within a specified timeout.
///
/// A builder for the *query* method supported by a *job* resource.
/// It is not used directly, but through a [`JobMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::QueryRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = QueryRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.jobs().query(req, "projectId")
///              .doit().await;
/// # }
/// ```
pub struct JobQueryCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: QueryRequest,
    _project_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for JobQueryCall<'a, S> {}

impl<'a, S> JobQueryCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, QueryResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.jobs.query",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/queries";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["projectId"];
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
    pub fn request(mut self, new_value: QueryRequest) -> JobQueryCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the query request.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> JobQueryCall<'a, S> {
        self._project_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> JobQueryCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> JobQueryCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> JobQueryCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> JobQueryCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> JobQueryCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes the model specified by modelId from the dataset.
///
/// A builder for the *delete* method supported by a *model* resource.
/// It is not used directly, but through a [`ModelMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.models().delete("projectId", "datasetId", "modelId")
///              .doit().await;
/// # }
/// ```
pub struct ModelDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _model_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ModelDeleteCall<'a, S> {}

impl<'a, S> ModelDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.models.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["projectId", "datasetId", "modelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("modelId", self._model_id);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/models/{+modelId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+modelId}", "modelId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["modelId", "datasetId", "projectId"];
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
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Project ID of the model to delete.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ModelDeleteCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the model to delete.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> ModelDeleteCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Model ID of the model to delete.
    ///
    /// Sets the *model id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn model_id(mut self, new_value: &str) -> ModelDeleteCall<'a, S> {
        self._model_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ModelDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ModelDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ModelDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ModelDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ModelDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the specified model resource by model ID.
///
/// A builder for the *get* method supported by a *model* resource.
/// It is not used directly, but through a [`ModelMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.models().get("projectId", "datasetId", "modelId")
///              .doit().await;
/// # }
/// ```
pub struct ModelGetCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _model_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ModelGetCall<'a, S> {}

impl<'a, S> ModelGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Model)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.models.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "modelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("modelId", self._model_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/models/{+modelId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+modelId}", "modelId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["modelId", "datasetId", "projectId"];
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


    /// Required. Project ID of the requested model.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ModelGetCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the requested model.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> ModelGetCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Model ID of the requested model.
    ///
    /// Sets the *model id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn model_id(mut self, new_value: &str) -> ModelGetCall<'a, S> {
        self._model_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ModelGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ModelGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ModelGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ModelGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ModelGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all models in the specified dataset. Requires the READER dataset role. After retrieving the list of models, you can get information about a particular model by calling the models.get method.
///
/// A builder for the *list* method supported by a *model* resource.
/// It is not used directly, but through a [`ModelMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.models().list("projectId", "datasetId")
///              .page_token("takimata")
///              .max_results(55)
///              .doit().await;
/// # }
/// ```
pub struct ModelListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ModelListCall<'a, S> {}

impl<'a, S> ModelListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListModelsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.models.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/models";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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


    /// Required. Project ID of the models to list.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ModelListCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the models to list.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> ModelListCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Page token, returned by a previous call to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ModelListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ModelListCall<'a, S> {
        self._max_results = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ModelListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ModelListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ModelListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ModelListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ModelListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Patch specific fields in the specified model.
///
/// A builder for the *patch* method supported by a *model* resource.
/// It is not used directly, but through a [`ModelMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Model;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Model::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.models().patch(req, "projectId", "datasetId", "modelId")
///              .doit().await;
/// # }
/// ```
pub struct ModelPatchCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Model,
    _project_id: String,
    _dataset_id: String,
    _model_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ModelPatchCall<'a, S> {}

impl<'a, S> ModelPatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Model)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.models.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "projectId", "datasetId", "modelId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("modelId", self._model_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/models/{+modelId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+modelId}", "modelId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["modelId", "datasetId", "projectId"];
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
                    .method(hyper::Method::PATCH)
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
    pub fn request(mut self, new_value: Model) -> ModelPatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the model to patch.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ModelPatchCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the model to patch.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> ModelPatchCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Model ID of the model to patch.
    ///
    /// Sets the *model id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn model_id(mut self, new_value: &str) -> ModelPatchCall<'a, S> {
        self._model_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ModelPatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ModelPatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ModelPatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ModelPatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ModelPatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// RPC to get the service account for a project used for interactions with Google Cloud KMS
///
/// A builder for the *getServiceAccount* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get_service_account("projectId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectGetServiceAccountCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectGetServiceAccountCall<'a, S> {}

impl<'a, S> ProjectGetServiceAccountCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetServiceAccountResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.projects.getServiceAccount",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("projectId", self._project_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/serviceAccount";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["projectId"];
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


    /// Required. ID of the project.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectGetServiceAccountCall<'a, S> {
        self._project_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectGetServiceAccountCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetServiceAccountCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectGetServiceAccountCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectGetServiceAccountCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectGetServiceAccountCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// RPC to list projects to which the user has been granted any project role. Users of this method are encouraged to consider the [Resource Manager](https://cloud.google.com/resource-manager/docs/) API, which provides the underlying data for this method and has more capabilities.
///
/// A builder for the *list* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().list()
///              .page_token("amet.")
///              .max_results(71)
///              .doit().await;
/// # }
/// ```
pub struct ProjectListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for ProjectListCall<'a, S> {}

impl<'a, S> ProjectListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ProjectList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.projects.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
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


    /// Page token, returned by a previous call, to request the next page of results. If not present, no further pages are present.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// `maxResults` unset returns all results, up to 50 per page. Additionally, the number of projects in a page may be fewer than `maxResults` because projects are retrieved and then filtered to only projects with the BigQuery API enabled.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ProjectListCall<'a, S> {
        self._max_results = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes the routine specified by routineId from the dataset.
///
/// A builder for the *delete* method supported by a *routine* resource.
/// It is not used directly, but through a [`RoutineMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.routines().delete("projectId", "datasetId", "routineId")
///              .doit().await;
/// # }
/// ```
pub struct RoutineDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _routine_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RoutineDeleteCall<'a, S> {}

impl<'a, S> RoutineDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.routines.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["projectId", "datasetId", "routineId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("routineId", self._routine_id);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/routines/{+routineId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+routineId}", "routineId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["routineId", "datasetId", "projectId"];
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
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Project ID of the routine to delete
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> RoutineDeleteCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the routine to delete
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> RoutineDeleteCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Routine ID of the routine to delete
    ///
    /// Sets the *routine id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn routine_id(mut self, new_value: &str) -> RoutineDeleteCall<'a, S> {
        self._routine_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RoutineDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RoutineDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RoutineDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RoutineDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RoutineDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the specified routine resource by routine ID.
///
/// A builder for the *get* method supported by a *routine* resource.
/// It is not used directly, but through a [`RoutineMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.routines().get("projectId", "datasetId", "routineId")
///              .read_mask(&Default::default())
///              .doit().await;
/// # }
/// ```
pub struct RoutineGetCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _routine_id: String,
    _read_mask: Option<client::FieldMask>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RoutineGetCall<'a, S> {}

impl<'a, S> RoutineGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Routine)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.routines.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "routineId", "readMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("routineId", self._routine_id);
        if let Some(value) = self._read_mask.as_ref() {
            params.push("readMask", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/routines/{+routineId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+routineId}", "routineId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["routineId", "datasetId", "projectId"];
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


    /// Required. Project ID of the requested routine
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> RoutineGetCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the requested routine
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> RoutineGetCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Routine ID of the requested routine
    ///
    /// Sets the *routine id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn routine_id(mut self, new_value: &str) -> RoutineGetCall<'a, S> {
        self._routine_id = new_value.to_string();
        self
    }
    /// If set, only the Routine fields in the field mask are returned in the response. If unset, all Routine fields are returned.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: client::FieldMask) -> RoutineGetCall<'a, S> {
        self._read_mask = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RoutineGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RoutineGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RoutineGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RoutineGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RoutineGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a new routine in the dataset.
///
/// A builder for the *insert* method supported by a *routine* resource.
/// It is not used directly, but through a [`RoutineMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Routine;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Routine::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.routines().insert(req, "projectId", "datasetId")
///              .doit().await;
/// # }
/// ```
pub struct RoutineInsertCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Routine,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RoutineInsertCall<'a, S> {}

impl<'a, S> RoutineInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Routine)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.routines.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/routines";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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
    pub fn request(mut self, new_value: Routine) -> RoutineInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the new routine
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> RoutineInsertCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the new routine
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> RoutineInsertCall<'a, S> {
        self._dataset_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RoutineInsertCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RoutineInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RoutineInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RoutineInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RoutineInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all routines in the specified dataset. Requires the READER dataset role.
///
/// A builder for the *list* method supported by a *routine* resource.
/// It is not used directly, but through a [`RoutineMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.routines().list("projectId", "datasetId")
///              .read_mask(&Default::default())
///              .page_token("amet.")
///              .max_results(84)
///              .filter("sadipscing")
///              .doit().await;
/// # }
/// ```
pub struct RoutineListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _read_mask: Option<client::FieldMask>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RoutineListCall<'a, S> {}

impl<'a, S> RoutineListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListRoutinesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.routines.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "readMask", "pageToken", "maxResults", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(8 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        if let Some(value) = self._read_mask.as_ref() {
            params.push("readMask", value.to_string());
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/routines";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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


    /// Required. Project ID of the routines to list
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> RoutineListCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the routines to list
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> RoutineListCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// If set, then only the Routine fields in the field mask, as well as project_id, dataset_id and routine_id, are returned in the response. If unset, then the following Routine fields are returned: etag, project_id, dataset_id, routine_id, routine_type, creation_time, last_modified_time, and language.
    ///
    /// Sets the *read mask* query property to the given value.
    pub fn read_mask(mut self, new_value: client::FieldMask) -> RoutineListCall<'a, S> {
        self._read_mask = Some(new_value);
        self
    }
    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> RoutineListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> RoutineListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// If set, then only the Routines matching this filter are returned. The supported format is `routineType:{RoutineType}`, where `{RoutineType}` is a RoutineType enum. For example: `routineType:SCALAR_FUNCTION`.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> RoutineListCall<'a, S> {
        self._filter = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RoutineListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RoutineListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RoutineListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RoutineListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RoutineListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates information in an existing routine. The update method replaces the entire Routine resource.
///
/// A builder for the *update* method supported by a *routine* resource.
/// It is not used directly, but through a [`RoutineMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Routine;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Routine::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.routines().update(req, "projectId", "datasetId", "routineId")
///              .doit().await;
/// # }
/// ```
pub struct RoutineUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Routine,
    _project_id: String,
    _dataset_id: String,
    _routine_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RoutineUpdateCall<'a, S> {}

impl<'a, S> RoutineUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Routine)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.routines.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "projectId", "datasetId", "routineId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("routineId", self._routine_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/routines/{+routineId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+routineId}", "routineId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["routineId", "datasetId", "projectId"];
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
    pub fn request(mut self, new_value: Routine) -> RoutineUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the routine to update
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> RoutineUpdateCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the routine to update
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> RoutineUpdateCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Routine ID of the routine to update
    ///
    /// Sets the *routine id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn routine_id(mut self, new_value: &str) -> RoutineUpdateCall<'a, S> {
        self._routine_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RoutineUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RoutineUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RoutineUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RoutineUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RoutineUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
///
/// A builder for the *getIamPolicy* method supported by a *rowAccessPolicy* resource.
/// It is not used directly, but through a [`RowAccessPolicyMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::GetIamPolicyRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIamPolicyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.row_access_policies().get_iam_policy(req, "resource")
///              .doit().await;
/// # }
/// ```
pub struct RowAccessPolicyGetIamPolicyCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: GetIamPolicyRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RowAccessPolicyGetIamPolicyCall<'a, S> {}

impl<'a, S> RowAccessPolicyGetIamPolicyCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.rowAccessPolicies.getIamPolicy",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("resource", self._resource);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{+resource}:getIamPolicy";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["resource"];
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
    pub fn request(mut self, new_value: GetIamPolicyRequest) -> RowAccessPolicyGetIamPolicyCall<'a, S> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> RowAccessPolicyGetIamPolicyCall<'a, S> {
        self._resource = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RowAccessPolicyGetIamPolicyCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RowAccessPolicyGetIamPolicyCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RowAccessPolicyGetIamPolicyCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RowAccessPolicyGetIamPolicyCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RowAccessPolicyGetIamPolicyCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all row access policies on the specified table.
///
/// A builder for the *list* method supported by a *rowAccessPolicy* resource.
/// It is not used directly, but through a [`RowAccessPolicyMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.row_access_policies().list("projectId", "datasetId", "tableId")
///              .page_token("et")
///              .page_size(-39)
///              .doit().await;
/// # }
/// ```
pub struct RowAccessPolicyListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RowAccessPolicyListCall<'a, S> {}

impl<'a, S> RowAccessPolicyListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListRowAccessPoliciesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.rowAccessPolicies.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "tableId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("tableId", self._table_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}/rowAccessPolicies";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+tableId}", "tableId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["tableId", "datasetId", "projectId"];
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


    /// Required. Project ID of the row access policies to list.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> RowAccessPolicyListCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of row access policies to list.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> RowAccessPolicyListCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Table ID of the table to list row access policies.
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> RowAccessPolicyListCall<'a, S> {
        self._table_id = new_value.to_string();
        self
    }
    /// Page token, returned by a previous call, to request the next page of results.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> RowAccessPolicyListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> RowAccessPolicyListCall<'a, S> {
        self._page_size = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RowAccessPolicyListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RowAccessPolicyListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RowAccessPolicyListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RowAccessPolicyListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RowAccessPolicyListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
///
/// A builder for the *testIamPermissions* method supported by a *rowAccessPolicy* resource.
/// It is not used directly, but through a [`RowAccessPolicyMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::TestIamPermissionsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestIamPermissionsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.row_access_policies().test_iam_permissions(req, "resource")
///              .doit().await;
/// # }
/// ```
pub struct RowAccessPolicyTestIamPermissionCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: TestIamPermissionsRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for RowAccessPolicyTestIamPermissionCall<'a, S> {}

impl<'a, S> RowAccessPolicyTestIamPermissionCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TestIamPermissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.rowAccessPolicies.testIamPermissions",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("resource", self._resource);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{+resource}:testIamPermissions";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["resource"];
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
    pub fn request(mut self, new_value: TestIamPermissionsRequest) -> RowAccessPolicyTestIamPermissionCall<'a, S> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> RowAccessPolicyTestIamPermissionCall<'a, S> {
        self._resource = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> RowAccessPolicyTestIamPermissionCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> RowAccessPolicyTestIamPermissionCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> RowAccessPolicyTestIamPermissionCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> RowAccessPolicyTestIamPermissionCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> RowAccessPolicyTestIamPermissionCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Streams data into BigQuery one record at a time without needing to run a load job.
///
/// A builder for the *insertAll* method supported by a *tabledata* resource.
/// It is not used directly, but through a [`TabledataMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::TableDataInsertAllRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TableDataInsertAllRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tabledata().insert_all(req, "projectId", "datasetId", "tableId")
///              .doit().await;
/// # }
/// ```
pub struct TabledataInsertAllCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: TableDataInsertAllRequest,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TabledataInsertAllCall<'a, S> {}

impl<'a, S> TabledataInsertAllCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TableDataInsertAllResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tabledata.insertAll",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId", "datasetId", "tableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("tableId", self._table_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}/insertAll";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+tableId}", "tableId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["tableId", "datasetId", "projectId"];
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
    pub fn request(mut self, new_value: TableDataInsertAllRequest) -> TabledataInsertAllCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the destination.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TabledataInsertAllCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the destination.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TabledataInsertAllCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Table ID of the destination.
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TabledataInsertAllCall<'a, S> {
        self._table_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TabledataInsertAllCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TabledataInsertAllCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TabledataInsertAllCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TabledataInsertAllCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TabledataInsertAllCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List the content of a table in rows.
///
/// A builder for the *list* method supported by a *tabledata* resource.
/// It is not used directly, but through a [`TabledataMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tabledata().list("projectId", "datasetId", "tableId")
///              .start_index(72)
///              .selected_fields("dolores")
///              .page_token("dolores")
///              .max_results(33)
///              .format_options_use_int64_timestamp(false)
///              .doit().await;
/// # }
/// ```
pub struct TabledataListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _start_index: Option<u64>,
    _selected_fields: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _format_options_use_int64_timestamp: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TabledataListCall<'a, S> {}

impl<'a, S> TabledataListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TableDataList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tabledata.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "tableId", "startIndex", "selectedFields", "pageToken", "maxResults", "formatOptions.useInt64Timestamp"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("tableId", self._table_id);
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._selected_fields.as_ref() {
            params.push("selectedFields", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._format_options_use_int64_timestamp.as_ref() {
            params.push("formatOptions.useInt64Timestamp", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}/data";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+tableId}", "tableId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["tableId", "datasetId", "projectId"];
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


    /// Required. Project id of the table to list.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TabledataListCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset id of the table to list.
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TabledataListCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Table id of the table to list.
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TabledataListCall<'a, S> {
        self._table_id = new_value.to_string();
        self
    }
    /// Start row index of the table.
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u64) -> TabledataListCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// Subset of fields to return, supports select into sub fields. Example: selected_fields = "a,e.d.f";
    ///
    /// Sets the *selected fields* query property to the given value.
    pub fn selected_fields(mut self, new_value: &str) -> TabledataListCall<'a, S> {
        self._selected_fields = Some(new_value.to_string());
        self
    }
    /// To retrieve the next page of table data, set this field to the string provided in the pageToken field of the response body from your previous call to tabledata.list.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TabledataListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Row limit of the table.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> TabledataListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// Optional. Output timestamp as usec int64. Default is false.
    ///
    /// Sets the *format options.use int64 timestamp* query property to the given value.
    pub fn format_options_use_int64_timestamp(mut self, new_value: bool) -> TabledataListCall<'a, S> {
        self._format_options_use_int64_timestamp = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TabledataListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TabledataListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TabledataListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TabledataListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TabledataListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Deletes the table specified by tableId from the dataset. If the table contains data, all the data will be deleted.
///
/// A builder for the *delete* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().delete("projectId", "datasetId", "tableId")
///              .doit().await;
/// # }
/// ```
pub struct TableDeleteCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableDeleteCall<'a, S> {}

impl<'a, S> TableDeleteCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.delete",
                               http_method: hyper::Method::DELETE });

        for &field in ["projectId", "datasetId", "tableId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("tableId", self._table_id);

        params.extend(self._additional_params.iter());

        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+tableId}", "tableId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["tableId", "datasetId", "projectId"];
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
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }


                        let request = req_builder
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
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. Project ID of the table to delete
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableDeleteCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the table to delete
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableDeleteCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Table ID of the table to delete
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TableDeleteCall<'a, S> {
        self._table_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableDeleteCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableDeleteCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableDeleteCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableDeleteCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableDeleteCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the specified table resource by table ID. This method does not return the data in the table, it only returns the table resource, which describes the structure of this table.
///
/// A builder for the *get* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().get("projectId", "datasetId", "tableId")
///              .view("At")
///              .selected_fields("sadipscing")
///              .doit().await;
/// # }
/// ```
pub struct TableGetCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _view: Option<String>,
    _selected_fields: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableGetCall<'a, S> {}

impl<'a, S> TableGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "tableId", "view", "selectedFields"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("tableId", self._table_id);
        if let Some(value) = self._view.as_ref() {
            params.push("view", value);
        }
        if let Some(value) = self._selected_fields.as_ref() {
            params.push("selectedFields", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+tableId}", "tableId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["tableId", "datasetId", "projectId"];
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


    /// Required. Project ID of the requested table
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableGetCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the requested table
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableGetCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Table ID of the requested table
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TableGetCall<'a, S> {
        self._table_id = new_value.to_string();
        self
    }
    /// Optional. Specifies the view that determines which table information is returned. By default, basic table information and storage statistics (STORAGE_STATS) are returned.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> TableGetCall<'a, S> {
        self._view = Some(new_value.to_string());
        self
    }
    /// List of table schema fields to return (comma-separated). If unspecified, all fields are returned. A fieldMask cannot be used here because the fields will automatically be converted from camelCase to snake_case and the conversion will fail if there are underscores. Since these are fields in BigQuery table schemas, underscores are allowed.
    ///
    /// Sets the *selected fields* query property to the given value.
    pub fn selected_fields(mut self, new_value: &str) -> TableGetCall<'a, S> {
        self._selected_fields = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableGetCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.
///
/// A builder for the *getIamPolicy* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::GetIamPolicyRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = GetIamPolicyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().get_iam_policy(req, "resource")
///              .doit().await;
/// # }
/// ```
pub struct TableGetIamPolicyCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: GetIamPolicyRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableGetIamPolicyCall<'a, S> {}

impl<'a, S> TableGetIamPolicyCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.getIamPolicy",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("resource", self._resource);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{+resource}:getIamPolicy";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["resource"];
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
    pub fn request(mut self, new_value: GetIamPolicyRequest) -> TableGetIamPolicyCall<'a, S> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> TableGetIamPolicyCall<'a, S> {
        self._resource = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableGetIamPolicyCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableGetIamPolicyCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableGetIamPolicyCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableGetIamPolicyCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableGetIamPolicyCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Creates a new, empty table in the dataset.
///
/// A builder for the *insert* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Table;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Table::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().insert(req, "projectId", "datasetId")
///              .doit().await;
/// # }
/// ```
pub struct TableInsertCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Table,
    _project_id: String,
    _dataset_id: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableInsertCall<'a, S> {}

impl<'a, S> TableInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "projectId", "datasetId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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
    pub fn request(mut self, new_value: Table) -> TableInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the new table
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableInsertCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the new table
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableInsertCall<'a, S> {
        self._dataset_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableInsertCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableInsertCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableInsertCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableInsertCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Lists all tables in the specified dataset. Requires the READER dataset role.
///
/// A builder for the *list* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().list("projectId", "datasetId")
///              .page_token("amet")
///              .max_results(44)
///              .doit().await;
/// # }
/// ```
pub struct TableListCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _project_id: String,
    _dataset_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableListCall<'a, S> {}

impl<'a, S> TableListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TableList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "projectId", "datasetId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["datasetId", "projectId"];
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


    /// Required. Project ID of the tables to list
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableListCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the tables to list
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableListCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Page token, returned by a previous call, to request the next page of results
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> TableListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of results to return in a single response page. Leverage the page tokens to iterate through the entire collection.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> TableListCall<'a, S> {
        self._max_results = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates information in an existing table. The update method replaces the entire table resource, whereas the patch method only replaces fields that are provided in the submitted table resource. This method supports RFC5789 patch semantics.
///
/// A builder for the *patch* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Table;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Table::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().patch(req, "projectId", "datasetId", "tableId")
///              .autodetect_schema(true)
///              .doit().await;
/// # }
/// ```
pub struct TablePatchCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Table,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _autodetect_schema: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TablePatchCall<'a, S> {}

impl<'a, S> TablePatchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.patch",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "projectId", "datasetId", "tableId", "autodetect_schema"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("tableId", self._table_id);
        if let Some(value) = self._autodetect_schema.as_ref() {
            params.push("autodetect_schema", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+tableId}", "tableId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["tableId", "datasetId", "projectId"];
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
                    .method(hyper::Method::PATCH)
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
    pub fn request(mut self, new_value: Table) -> TablePatchCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the table to update
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TablePatchCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the table to update
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TablePatchCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Table ID of the table to update
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TablePatchCall<'a, S> {
        self._table_id = new_value.to_string();
        self
    }
    /// Optional.  When true will autodetect schema, else will keep original schema
    ///
    /// Sets the *autodetect_schema* query property to the given value.
    pub fn autodetect_schema(mut self, new_value: bool) -> TablePatchCall<'a, S> {
        self._autodetect_schema = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TablePatchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TablePatchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TablePatchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TablePatchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TablePatchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.
///
/// A builder for the *setIamPolicy* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::SetIamPolicyRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = SetIamPolicyRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().set_iam_policy(req, "resource")
///              .doit().await;
/// # }
/// ```
pub struct TableSetIamPolicyCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: SetIamPolicyRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableSetIamPolicyCall<'a, S> {}

impl<'a, S> TableSetIamPolicyCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.setIamPolicy",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("resource", self._resource);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{+resource}:setIamPolicy";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["resource"];
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
    pub fn request(mut self, new_value: SetIamPolicyRequest) -> TableSetIamPolicyCall<'a, S> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> TableSetIamPolicyCall<'a, S> {
        self._resource = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableSetIamPolicyCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableSetIamPolicyCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableSetIamPolicyCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableSetIamPolicyCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableSetIamPolicyCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.
///
/// A builder for the *testIamPermissions* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::TestIamPermissionsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestIamPermissionsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().test_iam_permissions(req, "resource")
///              .doit().await;
/// # }
/// ```
pub struct TableTestIamPermissionCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: TestIamPermissionsRequest,
    _resource: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableTestIamPermissionCall<'a, S> {}

impl<'a, S> TableTestIamPermissionCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TestIamPermissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.testIamPermissions",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "resource"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("resource", self._resource);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{+resource}:testIamPermissions";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+resource}", "resource")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["resource"];
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
    pub fn request(mut self, new_value: TestIamPermissionsRequest) -> TableTestIamPermissionCall<'a, S> {
        self._request = new_value;
        self
    }
    /// REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
    ///
    /// Sets the *resource* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn resource(mut self, new_value: &str) -> TableTestIamPermissionCall<'a, S> {
        self._resource = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableTestIamPermissionCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableTestIamPermissionCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableTestIamPermissionCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableTestIamPermissionCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableTestIamPermissionCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Updates information in an existing table. The update method replaces the entire Table resource, whereas the patch method only replaces fields that are provided in the submitted Table resource.
///
/// A builder for the *update* method supported by a *table* resource.
/// It is not used directly, but through a [`TableMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_bigquery2 as bigquery2;
/// use bigquery2::api::Table;
/// # async fn dox() {
/// # use std::default::Default;
/// # use bigquery2::{Bigquery, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Bigquery::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Table::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.tables().update(req, "projectId", "datasetId", "tableId")
///              .autodetect_schema(true)
///              .doit().await;
/// # }
/// ```
pub struct TableUpdateCall<'a, S>
    where S: 'a {

    hub: &'a Bigquery<S>,
    _request: Table,
    _project_id: String,
    _dataset_id: String,
    _table_id: String,
    _autodetect_schema: Option<bool>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for TableUpdateCall<'a, S> {}

impl<'a, S> TableUpdateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Table)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "bigquery.tables.update",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "projectId", "datasetId", "tableId", "autodetect_schema"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("datasetId", self._dataset_id);
        params.push("tableId", self._table_id);
        if let Some(value) = self._autodetect_schema.as_ref() {
            params.push("autodetect_schema", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "projects/{+projectId}/datasets/{+datasetId}/tables/{+tableId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+projectId}", "projectId"), ("{+datasetId}", "datasetId"), ("{+tableId}", "tableId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["tableId", "datasetId", "projectId"];
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
    pub fn request(mut self, new_value: Table) -> TableUpdateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Required. Project ID of the table to update
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> TableUpdateCall<'a, S> {
        self._project_id = new_value.to_string();
        self
    }
    /// Required. Dataset ID of the table to update
    ///
    /// Sets the *dataset id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn dataset_id(mut self, new_value: &str) -> TableUpdateCall<'a, S> {
        self._dataset_id = new_value.to_string();
        self
    }
    /// Required. Table ID of the table to update
    ///
    /// Sets the *table id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn table_id(mut self, new_value: &str) -> TableUpdateCall<'a, S> {
        self._table_id = new_value.to_string();
        self
    }
    /// Optional.  When true will autodetect schema, else will keep original schema
    ///
    /// Sets the *autodetect_schema* query property to the given value.
    pub fn autodetect_schema(mut self, new_value: bool) -> TableUpdateCall<'a, S> {
        self._autodetect_schema = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> TableUpdateCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TableUpdateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TableUpdateCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TableUpdateCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> TableUpdateCall<'a, S> {
        self._scopes.clear();
        self
    }
}


