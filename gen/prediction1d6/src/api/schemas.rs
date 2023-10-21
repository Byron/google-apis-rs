use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [analyze trainedmodels](TrainedmodelAnalyzeCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Analyze {
    /// Description of the data the model was trained on.
    #[serde(rename="dataDescription")]
    
    pub data_description: Option<AnalyzeDataDescription>,
    /// List of errors with the data.
    
    pub errors: Option<Vec<HashMap<String, String>>>,
    /// The unique name for the predictive model.
    
    pub id: Option<String>,
    /// What kind of resource this is.
    
    pub kind: Option<String>,
    /// Description of the model.
    #[serde(rename="modelDescription")]
    
    pub model_description: Option<AnalyzeModelDescription>,
    /// A URL to re-request this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for Analyze {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [predict hostedmodels](HostedmodelPredictCall) (request)
/// * [predict trainedmodels](TrainedmodelPredictCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    /// Input to the model for a prediction.
    
    pub input: Option<InputInput>,
}

impl client::RequestValue for Input {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert trainedmodels](TrainedmodelInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Insert {
    /// The unique name for the predictive model.
    
    pub id: Option<String>,
    /// Type of predictive model (classification or regression).
    #[serde(rename="modelType")]
    
    pub model_type: Option<String>,
    /// The Id of the model to be copied over.
    #[serde(rename="sourceModel")]
    
    pub source_model: Option<String>,
    /// Google storage location of the training data file.
    #[serde(rename="storageDataLocation")]
    
    pub storage_data_location: Option<String>,
    /// Google storage location of the preprocessing pmml file.
    #[serde(rename="storagePMMLLocation")]
    
    pub storage_pmml_location: Option<String>,
    /// Google storage location of the pmml model file.
    #[serde(rename="storagePMMLModelLocation")]
    
    pub storage_pmml_model_location: Option<String>,
    /// Instances to train model on.
    #[serde(rename="trainingInstances")]
    
    pub training_instances: Option<Vec<InsertTrainingInstances>>,
    /// A class weighting function, which allows the importance weights for class labels to be specified (Categorical models only).
    
    pub utility: Option<Vec<HashMap<String, f64>>>,
}

impl client::RequestValue for Insert {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get trainedmodels](TrainedmodelGetCall) (response)
/// * [insert trainedmodels](TrainedmodelInsertCall) (response)
/// * [update trainedmodels](TrainedmodelUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Insert2 {
    /// Insert time of the model (as a RFC 3339 timestamp).
    
    pub created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The unique name for the predictive model.
    
    pub id: Option<String>,
    /// What kind of resource this is.
    
    pub kind: Option<String>,
    /// Model metadata.
    #[serde(rename="modelInfo")]
    
    pub model_info: Option<Insert2ModelInfo>,
    /// Type of predictive model (CLASSIFICATION or REGRESSION).
    #[serde(rename="modelType")]
    
    pub model_type: Option<String>,
    /// A URL to re-request this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Google storage location of the training data file.
    #[serde(rename="storageDataLocation")]
    
    pub storage_data_location: Option<String>,
    /// Google storage location of the preprocessing pmml file.
    #[serde(rename="storagePMMLLocation")]
    
    pub storage_pmml_location: Option<String>,
    /// Google storage location of the pmml model file.
    #[serde(rename="storagePMMLModelLocation")]
    
    pub storage_pmml_model_location: Option<String>,
    /// Training completion time (as a RFC 3339 timestamp).
    #[serde(rename="trainingComplete")]
    
    pub training_complete: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The current status of the training job. This can be one of following: RUNNING; DONE; ERROR; ERROR: TRAINING JOB NOT FOUND
    #[serde(rename="trainingStatus")]
    
    pub training_status: Option<String>,
}

impl client::ResponseResult for Insert2 {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list trainedmodels](TrainedmodelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct List {
    /// List of models.
    
    pub items: Option<Vec<Insert2>>,
    /// What kind of resource this is.
    
    pub kind: Option<String>,
    /// Pagination token to fetch the next page, if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A URL to re-request this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for List {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [predict hostedmodels](HostedmodelPredictCall) (response)
/// * [predict trainedmodels](TrainedmodelPredictCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    /// The unique name for the predictive model.
    
    pub id: Option<String>,
    /// What kind of resource this is.
    
    pub kind: Option<String>,
    /// The most likely class label (Categorical models only).
    #[serde(rename="outputLabel")]
    
    pub output_label: Option<String>,
    /// A list of class labels with their estimated probabilities (Categorical models only).
    #[serde(rename="outputMulti")]
    
    pub output_multi: Option<Vec<OutputOutputMulti>>,
    /// The estimated regression value (Regression models only).
    #[serde(rename="outputValue")]
    
    pub output_value: Option<String>,
    /// A URL to re-request this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::ResponseResult for Output {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [update trainedmodels](TrainedmodelUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Update {
    /// The input features for this instance.
    #[serde(rename="csvInstance")]
    
    pub csv_instance: Option<Vec<json::Value>>,
    /// The generic output value - could be regression or class label.
    
    pub output: Option<String>,
}

impl client::RequestValue for Update {}


/// Description of the data the model was trained on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescription {
    /// Description of the input features in the data set.
    
    pub features: Option<Vec<AnalyzeDataDescriptionFeatures>>,
    /// Description of the output value or label.
    #[serde(rename="outputFeature")]
    
    pub output_feature: Option<AnalyzeDataDescriptionOutputFeature>,
}

impl client::NestedType for AnalyzeDataDescription {}
impl client::Part for AnalyzeDataDescription {}


/// Description of the input features in the data set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionFeatures {
    /// Description of the categorical values of this feature.
    
    pub categorical: Option<AnalyzeDataDescriptionFeaturesCategorical>,
    /// The feature index.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub index: Option<i64>,
    /// Description of the numeric values of this feature.
    
    pub numeric: Option<AnalyzeDataDescriptionFeaturesNumeric>,
    /// Description of multiple-word text values of this feature.
    
    pub text: Option<AnalyzeDataDescriptionFeaturesText>,
}

impl client::NestedType for AnalyzeDataDescriptionFeatures {}
impl client::Part for AnalyzeDataDescriptionFeatures {}


/// Description of the categorical values of this feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionFeaturesCategorical {
    /// Number of categorical values for this feature in the data.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// List of all the categories for this feature in the data set.
    
    pub values: Option<Vec<AnalyzeDataDescriptionFeaturesCategoricalValues>>,
}

impl client::NestedType for AnalyzeDataDescriptionFeaturesCategorical {}
impl client::Part for AnalyzeDataDescriptionFeaturesCategorical {}


/// List of all the categories for this feature in the data set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionFeaturesCategoricalValues {
    /// Number of times this feature had this value.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The category name.
    
    pub value: Option<String>,
}

impl client::NestedType for AnalyzeDataDescriptionFeaturesCategoricalValues {}
impl client::Part for AnalyzeDataDescriptionFeaturesCategoricalValues {}


/// Description of the numeric values of this feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionFeaturesNumeric {
    /// Number of numeric values for this feature in the data set.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Mean of the numeric values of this feature in the data set.
    
    pub mean: Option<String>,
    /// Variance of the numeric values of this feature in the data set.
    
    pub variance: Option<String>,
}

impl client::NestedType for AnalyzeDataDescriptionFeaturesNumeric {}
impl client::Part for AnalyzeDataDescriptionFeaturesNumeric {}


/// Description of multiple-word text values of this feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionFeaturesText {
    /// Number of multiple-word text values for this feature.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
}

impl client::NestedType for AnalyzeDataDescriptionFeaturesText {}
impl client::Part for AnalyzeDataDescriptionFeaturesText {}


/// Description of the output value or label.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionOutputFeature {
    /// Description of the output values in the data set.
    
    pub numeric: Option<AnalyzeDataDescriptionOutputFeatureNumeric>,
    /// Description of the output labels in the data set.
    
    pub text: Option<Vec<AnalyzeDataDescriptionOutputFeatureText>>,
}

impl client::NestedType for AnalyzeDataDescriptionOutputFeature {}
impl client::Part for AnalyzeDataDescriptionOutputFeature {}


/// Description of the output values in the data set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionOutputFeatureNumeric {
    /// Number of numeric output values in the data set.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Mean of the output values in the data set.
    
    pub mean: Option<String>,
    /// Variance of the output values in the data set.
    
    pub variance: Option<String>,
}

impl client::NestedType for AnalyzeDataDescriptionOutputFeatureNumeric {}
impl client::Part for AnalyzeDataDescriptionOutputFeatureNumeric {}


/// Description of the output labels in the data set.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeDataDescriptionOutputFeatureText {
    /// Number of times the output label occurred in the data set.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The output label.
    
    pub value: Option<String>,
}

impl client::NestedType for AnalyzeDataDescriptionOutputFeatureText {}
impl client::Part for AnalyzeDataDescriptionOutputFeatureText {}


/// Description of the model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzeModelDescription {
    /// An output confusion matrix. This shows an estimate for how this model will do in predictions. This is first indexed by the true class label. For each true class label, this provides a pair {predicted_label, count}, where count is the estimated number of times the model will predict the predicted label given the true label. Will not output if more then 100 classes (Categorical models only).
    #[serde(rename="confusionMatrix")]
    
    pub confusion_matrix: Option<HashMap<String, HashMap<String, String>>>,
    /// A list of the confusion matrix row totals.
    #[serde(rename="confusionMatrixRowTotals")]
    
    pub confusion_matrix_row_totals: Option<HashMap<String, String>>,
    /// Basic information about the model.
    
    pub modelinfo: Option<Insert2>,
}

impl client::NestedType for AnalyzeModelDescription {}
impl client::Part for AnalyzeModelDescription {}


/// Input to the model for a prediction.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InputInput {
    /// A list of input features, these can be strings or doubles.
    #[serde(rename="csvInstance")]
    
    pub csv_instance: Option<Vec<json::Value>>,
}

impl client::NestedType for InputInput {}
impl client::Part for InputInput {}


/// Instances to train model on.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InsertTrainingInstances {
    /// The input features for this instance.
    #[serde(rename="csvInstance")]
    
    pub csv_instance: Option<Vec<json::Value>>,
    /// The generic output value - could be regression or class label.
    
    pub output: Option<String>,
}

impl client::NestedType for InsertTrainingInstances {}
impl client::Part for InsertTrainingInstances {}


/// Model metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Insert2ModelInfo {
    /// Estimated accuracy of model taking utility weights into account (Categorical models only).
    #[serde(rename="classWeightedAccuracy")]
    
    pub class_weighted_accuracy: Option<String>,
    /// A number between 0.0 and 1.0, where 1.0 is 100% accurate. This is an estimate, based on the amount and quality of the training data, of the estimated prediction accuracy. You can use this is a guide to decide whether the results are accurate enough for your needs. This estimate will be more reliable if your real input data is similar to your training data (Categorical models only).
    #[serde(rename="classificationAccuracy")]
    
    pub classification_accuracy: Option<String>,
    /// An estimated mean squared error. The can be used to measure the quality of the predicted model (Regression models only).
    #[serde(rename="meanSquaredError")]
    
    pub mean_squared_error: Option<String>,
    /// Type of predictive model (CLASSIFICATION or REGRESSION).
    #[serde(rename="modelType")]
    
    pub model_type: Option<String>,
    /// Number of valid data instances used in the trained model.
    #[serde(rename="numberInstances")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub number_instances: Option<i64>,
    /// Number of class labels in the trained model (Categorical models only).
    #[serde(rename="numberLabels")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub number_labels: Option<i64>,
}

impl client::NestedType for Insert2ModelInfo {}
impl client::Part for Insert2ModelInfo {}


/// A list of class labels with their estimated probabilities (Categorical models only).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OutputOutputMulti {
    /// The class label.
    
    pub label: Option<String>,
    /// The probability of the class label.
    
    pub score: Option<String>,
}

impl client::NestedType for OutputOutputMulti {}
impl client::Part for OutputOutputMulti {}


