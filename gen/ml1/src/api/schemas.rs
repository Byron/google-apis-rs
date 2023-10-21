use super::*;
/// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can’t be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [explain projects](ProjectExplainCall) (response)
/// * [predict projects](ProjectPredictCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleApi__HttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::ResponseResult for GoogleApi__HttpBody {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_AutomatedStoppingConfig_DecayCurveAutomatedStoppingConfig {
    /// If true, measurement.elapsed_time is used as the x-axis of each Trials Decay Curve. Otherwise, Measurement.steps will be used as the x-axis.
    #[serde(rename="useElapsedTime")]
    
    pub use_elapsed_time: Option<bool>,
}

impl client::Part for GoogleCloudMlV1_AutomatedStoppingConfig_DecayCurveAutomatedStoppingConfig {}


/// The median automated stopping rule stops a pending trial if the trial's best objective_value is strictly below the median 'performance' of all completed trials reported up to the trial's last measurement. Currently, 'performance' refers to the running average of the objective values reported by the trial in each measurement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_AutomatedStoppingConfig_MedianAutomatedStoppingConfig {
    /// If true, the median automated stopping rule applies to measurement.use_elapsed_time, which means the elapsed_time field of the current trial's latest measurement is used to compute the median objective value for each completed trial.
    #[serde(rename="useElapsedTime")]
    
    pub use_elapsed_time: Option<bool>,
}

impl client::Part for GoogleCloudMlV1_AutomatedStoppingConfig_MedianAutomatedStoppingConfig {}


/// An observed value of a metric.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric {
    /// The objective value at this training step.
    #[serde(rename="objectiveValue")]
    
    pub objective_value: Option<f64>,
    /// The global training step for this metric.
    #[serde(rename="trainingStep")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub training_step: Option<i64>,
}

impl client::Part for GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric {}


/// A message representing a metric in the measurement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_Measurement_Metric {
    /// Required. Metric name.
    
    pub metric: Option<String>,
    /// Required. The value for this metric.
    
    pub value: Option<f64>,
}

impl client::Part for GoogleCloudMlV1_Measurement_Metric {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_CategoricalValueSpec {
    /// Must be specified if type is `CATEGORICAL`. The list of possible categories.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudMlV1_StudyConfigParameterSpec_CategoricalValueSpec {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_DiscreteValueSpec {
    /// Must be specified if type is `DISCRETE`. A list of feasible points. The list should be in strictly increasing order. For instance, this parameter might have possible settings of 1.5, 2.5, and 4.0. This list should not contain more than 1,000 values.
    
    pub values: Option<Vec<f64>>,
}

impl client::Part for GoogleCloudMlV1_StudyConfigParameterSpec_DiscreteValueSpec {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_DoubleValueSpec {
    /// Must be specified if type is `DOUBLE`. Maximum value of the parameter.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<f64>,
    /// Must be specified if type is `DOUBLE`. Minimum value of the parameter.
    #[serde(rename="minValue")]
    
    pub min_value: Option<f64>,
}

impl client::Part for GoogleCloudMlV1_StudyConfigParameterSpec_DoubleValueSpec {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_IntegerValueSpec {
    /// Must be specified if type is `INTEGER`. Maximum value of the parameter.
    #[serde(rename="maxValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_value: Option<i64>,
    /// Must be specified if type is `INTEGER`. Minimum value of the parameter.
    #[serde(rename="minValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub min_value: Option<i64>,
}

impl client::Part for GoogleCloudMlV1_StudyConfigParameterSpec_IntegerValueSpec {}


/// Represents the spec to match categorical values from parent parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentCategoricalValueSpec {
    /// Matches values of the parent parameter with type 'CATEGORICAL'. All values must exist in `categorical_value_spec` of parent parameter.
    
    pub values: Option<Vec<String>>,
}

impl client::Part for GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentCategoricalValueSpec {}


/// Represents the spec to match discrete values from parent parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentDiscreteValueSpec {
    /// Matches values of the parent parameter with type 'DISCRETE'. All values must exist in `discrete_value_spec` of parent parameter.
    
    pub values: Option<Vec<f64>>,
}

impl client::Part for GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentDiscreteValueSpec {}


/// Represents the spec to match integer values from parent parameter.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentIntValueSpec {
    /// Matches values of the parent parameter with type 'INTEGER'. All values must lie in `integer_value_spec` of parent parameter.
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub values: Option<Vec<i64>>,
}

impl client::Part for GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentIntValueSpec {}


/// Represents a metric to optimize.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfig_MetricSpec {
    /// Required. The optimization goal of the metric.
    
    pub goal: Option<String>,
    /// Required. The name of the metric.
    
    pub metric: Option<String>,
}

impl client::Part for GoogleCloudMlV1_StudyConfig_MetricSpec {}


/// Represents a single parameter to optimize.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_StudyConfig_ParameterSpec {
    /// The value spec for a 'CATEGORICAL' parameter.
    #[serde(rename="categoricalValueSpec")]
    
    pub categorical_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_CategoricalValueSpec>,
    /// A child node is active if the parameter's value matches the child node's matching_parent_values. If two items in child_parameter_specs have the same name, they must have disjoint matching_parent_values.
    #[serde(rename="childParameterSpecs")]
    
    pub child_parameter_specs: Option<Vec<GoogleCloudMlV1_StudyConfig_ParameterSpec>>,
    /// The value spec for a 'DISCRETE' parameter.
    #[serde(rename="discreteValueSpec")]
    
    pub discrete_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_DiscreteValueSpec>,
    /// The value spec for a 'DOUBLE' parameter.
    #[serde(rename="doubleValueSpec")]
    
    pub double_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_DoubleValueSpec>,
    /// The value spec for an 'INTEGER' parameter.
    #[serde(rename="integerValueSpec")]
    
    pub integer_value_spec: Option<GoogleCloudMlV1_StudyConfigParameterSpec_IntegerValueSpec>,
    /// Required. The parameter name must be unique amongst all ParameterSpecs.
    
    pub parameter: Option<String>,
    /// no description provided
    #[serde(rename="parentCategoricalValues")]
    
    pub parent_categorical_values: Option<GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentCategoricalValueSpec>,
    /// no description provided
    #[serde(rename="parentDiscreteValues")]
    
    pub parent_discrete_values: Option<GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentDiscreteValueSpec>,
    /// no description provided
    #[serde(rename="parentIntValues")]
    
    pub parent_int_values: Option<GoogleCloudMlV1_StudyConfigParameterSpec_MatchingParentIntValueSpec>,
    /// How the parameter should be scaled. Leave unset for categorical parameters.
    #[serde(rename="scaleType")]
    
    pub scale_type: Option<String>,
    /// Required. The type of the parameter.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudMlV1_StudyConfig_ParameterSpec {}


/// A message representing a parameter to be tuned. Contains the name of the parameter and the suggested value to use for this trial.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1_Trial_Parameter {
    /// Must be set if ParameterType is DOUBLE or DISCRETE.
    #[serde(rename="floatValue")]
    
    pub float_value: Option<f64>,
    /// Must be set if ParameterType is INTEGER
    #[serde(rename="intValue")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub int_value: Option<i64>,
    /// The name of the parameter.
    
    pub parameter: Option<String>,
    /// Must be set if ParameterTypeis CATEGORICAL
    #[serde(rename="stringValue")]
    
    pub string_value: Option<String>,
}

impl client::Part for GoogleCloudMlV1_Trial_Parameter {}


/// Represents a hardware accelerator request config. Note that the AcceleratorConfig can be used in both Jobs and Versions. Learn more about [accelerators for training](https://cloud.google.com/ml-engine/docs/using-gpus) and [accelerators for online prediction](https://cloud.google.com/ml-engine/docs/machine-types-online-prediction#gpus).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__AcceleratorConfig {
    /// The number of accelerators to attach to each machine running the job.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// The type of accelerator to use.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudMlV1__AcceleratorConfig {}


/// The request message for the AddTrialMeasurement service method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials add measurement projects](ProjectLocationStudyTrialAddMeasurementCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__AddTrialMeasurementRequest {
    /// Required. The measurement to be added to a trial.
    
    pub measurement: Option<GoogleCloudMlV1__Measurement>,
}

impl client::RequestValue for GoogleCloudMlV1__AddTrialMeasurementRequest {}


/// Options for automatically scaling a model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__AutoScaling {
    /// The maximum number of nodes to scale this model under load. The actual value will depend on resource quota and availability.
    #[serde(rename="maxNodes")]
    
    pub max_nodes: Option<i32>,
    /// MetricSpec contains the specifications to use to calculate the desired nodes count.
    
    pub metrics: Option<Vec<GoogleCloudMlV1__MetricSpec>>,
    /// Optional. The minimum number of nodes to allocate for this model. These nodes are always up, starting from the time the model is deployed. Therefore, the cost of operating this model will be at least `rate` * `min_nodes` * number of hours since last billing cycle, where `rate` is the cost per node-hour as documented in the [pricing guide](https://cloud.google.com/ml-engine/docs/pricing), even if no predictions are performed. There is additional cost for each prediction performed. Unlike manual scaling, if the load gets too heavy for the nodes that are up, the service will automatically add nodes to handle the increased load as well as scale back as traffic drops, always maintaining at least `min_nodes`. You will be charged for the time in which additional nodes are used. If `min_nodes` is not specified and AutoScaling is used with a [legacy (MLS1) machine type](https://cloud.google.com/ml-engine/docs/machine-types-online-prediction), `min_nodes` defaults to 0, in which case, when traffic to a model stops (and after a cool-down period), nodes will be shut down and no charges will be incurred until traffic to the model resumes. If `min_nodes` is not specified and AutoScaling is used with a [Compute Engine (N1) machine type](https://cloud.google.com/ml-engine/docs/machine-types-online-prediction), `min_nodes` defaults to 1. `min_nodes` must be at least 1 for use with a Compute Engine machine type. You can set `min_nodes` when creating the model version, and you can also update `min_nodes` for an existing version: update_body.json: { ‘autoScaling’: { ‘minNodes’: 5 } } HTTP request: PATCH https://ml.googleapis.com/v1/{name=projects/*/models/*/versions/\*}?update_mask=autoScaling.minNodes -d @./update_body.json 
    #[serde(rename="minNodes")]
    
    pub min_nodes: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__AutoScaling {}


/// Configuration for Automated Early Stopping of Trials. If no implementation_config is set, automated early stopping will not be run.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__AutomatedStoppingConfig {
    /// no description provided
    #[serde(rename="decayCurveStoppingConfig")]
    
    pub decay_curve_stopping_config: Option<GoogleCloudMlV1_AutomatedStoppingConfig_DecayCurveAutomatedStoppingConfig>,
    /// no description provided
    #[serde(rename="medianAutomatedStoppingConfig")]
    
    pub median_automated_stopping_config: Option<GoogleCloudMlV1_AutomatedStoppingConfig_MedianAutomatedStoppingConfig>,
}

impl client::Part for GoogleCloudMlV1__AutomatedStoppingConfig {}


/// Represents output related to a built-in algorithm Job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__BuiltInAlgorithmOutput {
    /// Framework on which the built-in algorithm was trained.
    
    pub framework: Option<String>,
    /// The Cloud Storage path to the `model/` directory where the training job saves the trained model. Only set for successful jobs that don't use hyperparameter tuning.
    #[serde(rename="modelPath")]
    
    pub model_path: Option<String>,
    /// Python version on which the built-in algorithm was trained.
    #[serde(rename="pythonVersion")]
    
    pub python_version: Option<String>,
    /// AI Platform runtime version on which the built-in algorithm was trained.
    #[serde(rename="runtimeVersion")]
    
    pub runtime_version: Option<String>,
}

impl client::Part for GoogleCloudMlV1__BuiltInAlgorithmOutput {}


/// Request message for the CancelJob method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs cancel projects](ProjectJobCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__CancelJobRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudMlV1__CancelJobRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Capability {
    /// Available accelerators for the capability.
    #[serde(rename="availableAccelerators")]
    
    pub available_accelerators: Option<Vec<String>>,
    /// no description provided
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudMlV1__Capability {}


/// The request message for the CheckTrialEarlyStoppingState service method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials check early stopping state projects](ProjectLocationStudyTrialCheckEarlyStoppingStateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__CheckTrialEarlyStoppingStateRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudMlV1__CheckTrialEarlyStoppingStateRequest {}


/// The request message for the CompleteTrial service method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials complete projects](ProjectLocationStudyTrialCompleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__CompleteTrialRequest {
    /// Optional. If provided, it will be used as the completed trial's final_measurement; Otherwise, the service will auto-select a previously reported measurement as the final-measurement
    #[serde(rename="finalMeasurement")]
    
    pub final_measurement: Option<GoogleCloudMlV1__Measurement>,
    /// Optional. A human readable reason why the trial was infeasible. This should only be provided if `trial_infeasible` is true.
    #[serde(rename="infeasibleReason")]
    
    pub infeasible_reason: Option<String>,
    /// Optional. True if the trial cannot be run with the given Parameter, and final_measurement will be ignored.
    #[serde(rename="trialInfeasible")]
    
    pub trial_infeasible: Option<bool>,
}

impl client::RequestValue for GoogleCloudMlV1__CompleteTrialRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Config {
    /// The service account Cloud ML uses to run on TPU node.
    #[serde(rename="tpuServiceAccount")]
    
    pub tpu_service_account: Option<String>,
}

impl client::Part for GoogleCloudMlV1__Config {}


/// Represents a network port in a single container. This message is a subset of the [Kubernetes ContainerPort v1 core specification](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#containerport-v1-core).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ContainerPort {
    /// Number of the port to expose on the container. This must be a valid port number: 0 < PORT_NUMBER < 65536.
    #[serde(rename="containerPort")]
    
    pub container_port: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__ContainerPort {}


/// Specification of a custom container for serving predictions. This message is a subset of the [Kubernetes Container v1 core specification](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ContainerSpec {
    /// Immutable. Specifies arguments for the command that runs when the container starts. This overrides the container’s [`CMD`](https://docs.docker.com/engine/reference/builder/#cmd). Specify this field as an array of executable and arguments, similar to a Docker `CMD`’s “default parameters” form. If you don’t specify this field but do specify the command field, then the command from the `command` field runs without any additional arguments. See the [Kubernetes documentation about how the `command` and `args` fields interact with a container’s `ENTRYPOINT` and `CMD`](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes). If you don’t specify this field and don’t specify the `commmand` field, then the container’s [`ENTRYPOINT`](https://docs.docker.com/engine/reference/builder/#cmd) and `CMD` determine what runs based on their default behavior. See the [Docker documentation about how `CMD` and `ENTRYPOINT` interact](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact). In this field, you can reference [environment variables set by AI Platform Prediction](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements#aip-variables) and environment variables set in the env field. You cannot reference environment variables set in the Docker image. In order for environment variables to be expanded, reference them by using the following syntax: $( VARIABLE_NAME) Note that this differs from Bash variable expansion, which does not use parentheses. If a variable cannot be resolved, the reference in the input string is used unchanged. To avoid variable expansion, you can escape this syntax with `$$`; for example: $$(VARIABLE_NAME) This field corresponds to the `args` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core).
    
    pub args: Option<Vec<String>>,
    /// Immutable. Specifies the command that runs when the container starts. This overrides the container’s [`ENTRYPOINT`](https://docs.docker.com/engine/reference/builder/#entrypoint). Specify this field as an array of executable and arguments, similar to a Docker `ENTRYPOINT`’s “exec” form, not its “shell” form. If you do not specify this field, then the container’s `ENTRYPOINT` runs, in conjunction with the args field or the container’s [`CMD`](https://docs.docker.com/engine/reference/builder/#cmd), if either exists. If this field is not specified and the container does not have an `ENTRYPOINT`, then refer to the [Docker documentation about how `CMD` and `ENTRYPOINT` interact](https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact). If you specify this field, then you can also specify the `args` field to provide additional arguments for this command. However, if you specify this field, then the container’s `CMD` is ignored. See the [Kubernetes documentation about how the `command` and `args` fields interact with a container’s `ENTRYPOINT` and `CMD`](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#notes). In this field, you can reference [environment variables set by AI Platform Prediction](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements#aip-variables) and environment variables set in the env field. You cannot reference environment variables set in the Docker image. In order for environment variables to be expanded, reference them by using the following syntax: $( VARIABLE_NAME) Note that this differs from Bash variable expansion, which does not use parentheses. If a variable cannot be resolved, the reference in the input string is used unchanged. To avoid variable expansion, you can escape this syntax with `$$`; for example: $$(VARIABLE_NAME) This field corresponds to the `command` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core).
    
    pub command: Option<Vec<String>>,
    /// Immutable. List of environment variables to set in the container. After the container starts running, code running in the container can read these environment variables. Additionally, the command and args fields can reference these variables. Later entries in this list can also reference earlier entries. For example, the following example sets the variable `VAR_2` to have the value `foo bar`: ```json [ { "name": "VAR_1", "value": "foo" }, { "name": "VAR_2", "value": "$(VAR_1) bar" } ] ``` If you switch the order of the variables in the example, then the expansion does not occur. This field corresponds to the `env` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core).
    
    pub env: Option<Vec<GoogleCloudMlV1__EnvVar>>,
    /// URI of the Docker image to be used as the custom container for serving predictions. This URI must identify [an image in Artifact Registry](https://cloud.google.com/artifact-registry/docs/overview) and begin with the hostname `{REGION}-docker.pkg.dev`, where `{REGION}` is replaced by the region that matches AI Platform Prediction [regional endpoint](https://cloud.google.com/ai-platform/prediction/docs/regional-endpoints) that you are using. For example, if you are using the `us-central1-ml.googleapis.com` endpoint, then this URI must begin with `us-central1-docker.pkg.dev`. To use a custom container, the [AI Platform Google-managed service account](https://cloud.google.com/ai-platform/prediction/docs/custom-service-account#default) must have permission to pull (read) the Docker image at this URI. The AI Platform Google-managed service account has the following format: `service-{PROJECT_NUMBER}@cloud-ml.google.com.iam.gserviceaccount.com` {PROJECT_NUMBER} is replaced by your Google Cloud project number. By default, this service account has necessary permissions to pull an Artifact Registry image in the same Google Cloud project where you are using AI Platform Prediction. In this case, no configuration is necessary. If you want to use an image from a different Google Cloud project, learn how to [grant the Artifact Registry Reader (roles/artifactregistry.reader) role for a repository](https://cloud.google.com/artifact-registry/docs/access-control#grant-repo) to your projet’s AI Platform Google-managed service account. To learn about the requirements for the Docker image itself, read [Custom container requirements](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements).
    
    pub image: Option<String>,
    /// Immutable. List of ports to expose from the container. AI Platform Prediction sends any prediction requests that it receives to the first port on this list. AI Platform Prediction also sends [liveness and health checks](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements#health) to this port. If you do not specify this field, it defaults to following value: `json [ { "containerPort": 8080 } ] ` AI Platform Prediction does not use ports other than the first one listed. This field corresponds to the `ports` field of the [Kubernetes Containers v1 core API](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#container-v1-core).
    
    pub ports: Option<Vec<GoogleCloudMlV1__ContainerPort>>,
}

impl client::Part for GoogleCloudMlV1__ContainerSpec {}


/// Represents the config of disk options.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__DiskConfig {
    /// Size in GB of the boot disk (default is 100GB).
    #[serde(rename="bootDiskSizeGb")]
    
    pub boot_disk_size_gb: Option<i32>,
    /// Type of the boot disk (default is "pd-ssd"). Valid values: "pd-ssd" (Persistent Disk Solid State Drive) or "pd-standard" (Persistent Disk Hard Disk Drive).
    #[serde(rename="bootDiskType")]
    
    pub boot_disk_type: Option<String>,
}

impl client::Part for GoogleCloudMlV1__DiskConfig {}


/// Represents a custom encryption key configuration that can be applied to a resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__EncryptionConfig {
    /// The Cloud KMS resource identifier of the customer-managed encryption key used to protect a resource, such as a training job. It has the following format: `projects/{PROJECT_ID}/locations/{REGION}/keyRings/{KEY_RING_NAME}/cryptoKeys/{KEY_NAME}`
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for GoogleCloudMlV1__EncryptionConfig {}


/// Represents an environment variable to be made available in a container. This message is a subset of the [Kubernetes EnvVar v1 core specification](https://kubernetes.io/docs/reference/generated/kubernetes-api/v1.18/#envvar-v1-core).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__EnvVar {
    /// Name of the environment variable. Must be a [valid C identifier](https://github.com/kubernetes/kubernetes/blob/v1.18.8/staging/src/k8s.io/apimachinery/pkg/util/validation/validation.go#L258) and must not begin with the prefix `AIP_`.
    
    pub name: Option<String>,
    /// Value of the environment variable. Defaults to an empty string. In this field, you can reference [environment variables set by AI Platform Prediction](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements#aip-variables) and environment variables set earlier in the same env field as where this message occurs. You cannot reference environment variables set in the Docker image. In order for environment variables to be expanded, reference them by using the following syntax: $(VARIABLE_NAME) Note that this differs from Bash variable expansion, which does not use parentheses. If a variable cannot be resolved, the reference in the input string is used unchanged. To avoid variable expansion, you can escape this syntax with `$$`; for example: $$(VARIABLE_NAME)
    
    pub value: Option<String>,
}

impl client::Part for GoogleCloudMlV1__EnvVar {}


/// Request for explanations to be issued against a trained model.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [explain projects](ProjectExplainCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ExplainRequest {
    /// Required. The explanation request body.
    #[serde(rename="httpBody")]
    
    pub http_body: Option<GoogleApi__HttpBody>,
}

impl client::RequestValue for GoogleCloudMlV1__ExplainRequest {}


/// Message holding configuration options for explaining model predictions. There are three feature attribution methods supported for TensorFlow models: integrated gradients, sampled Shapley, and XRAI. [Learn more about feature attributions.](https://cloud.google.com/ai-platform/prediction/docs/ai-explanations/overview)
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ExplanationConfig {
    /// Attributes credit by computing the Aumann-Shapley value taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1703.01365
    #[serde(rename="integratedGradientsAttribution")]
    
    pub integrated_gradients_attribution: Option<GoogleCloudMlV1__IntegratedGradientsAttribution>,
    /// An attribution method that approximates Shapley values for features that contribute to the label being predicted. A sampling strategy is used to approximate the value rather than considering all subsets of features.
    #[serde(rename="sampledShapleyAttribution")]
    
    pub sampled_shapley_attribution: Option<GoogleCloudMlV1__SampledShapleyAttribution>,
    /// Attributes credit by computing the XRAI taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1906.02825 Currently only implemented for models with natural image inputs.
    #[serde(rename="xraiAttribution")]
    
    pub xrai_attribution: Option<GoogleCloudMlV1__XraiAttribution>,
}

impl client::Part for GoogleCloudMlV1__ExplanationConfig {}


/// Returns service account information associated with a project.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get config projects](ProjectGetConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__GetConfigResponse {
    /// no description provided
    
    pub config: Option<GoogleCloudMlV1__Config>,
    /// The service account Cloud ML uses to access resources in the project.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// The project number for `service_account`.
    #[serde(rename="serviceAccountProject")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub service_account_project: Option<i64>,
}

impl client::ResponseResult for GoogleCloudMlV1__GetConfigResponse {}


/// Represents the result of a single hyperparameter tuning trial from a training job. The TrainingOutput object that is returned on successful completion of a training job with hyperparameter tuning includes a list of HyperparameterOutput objects, one for each successful trial.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__HyperparameterOutput {
    /// All recorded object metrics for this trial. This field is not currently populated.
    #[serde(rename="allMetrics")]
    
    pub all_metrics: Option<Vec<GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric>>,
    /// Details related to built-in algorithms jobs. Only set for trials of built-in algorithms jobs that have succeeded.
    #[serde(rename="builtInAlgorithmOutput")]
    
    pub built_in_algorithm_output: Option<GoogleCloudMlV1__BuiltInAlgorithmOutput>,
    /// Output only. End time for the trial.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The final objective metric seen for this trial.
    #[serde(rename="finalMetric")]
    
    pub final_metric: Option<GoogleCloudMlV1_HyperparameterOutput_HyperparameterMetric>,
    /// The hyperparameters given to this trial.
    
    pub hyperparameters: Option<HashMap<String, String>>,
    /// True if the trial is stopped early.
    #[serde(rename="isTrialStoppedEarly")]
    
    pub is_trial_stopped_early: Option<bool>,
    /// Output only. Start time for the trial.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The detailed state of the trial.
    
    pub state: Option<String>,
    /// The trial id for these results.
    #[serde(rename="trialId")]
    
    pub trial_id: Option<String>,
    /// URIs for accessing [interactive shells](https://cloud.google.com/ai-platform/training/docs/monitor-debug-interactive-shell) (one URI for each training node). Only available if this trial is part of a hyperparameter tuning job and the job's training_input.enable_web_access is `true`. The keys are names of each node in the training job; for example, `master-replica-0` for the master node, `worker-replica-0` for the first worker, and `ps-replica-0` for the first parameter server. The values are the URIs for each node's interactive shell.
    #[serde(rename="webAccessUris")]
    
    pub web_access_uris: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudMlV1__HyperparameterOutput {}


/// Represents a set of hyperparameters to optimize.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__HyperparameterSpec {
    /// Optional. The search algorithm specified for the hyperparameter tuning job. Uses the default AI Platform hyperparameter tuning algorithm if unspecified.
    
    pub algorithm: Option<String>,
    /// Optional. Indicates if the hyperparameter tuning job enables auto trial early stopping.
    #[serde(rename="enableTrialEarlyStopping")]
    
    pub enable_trial_early_stopping: Option<bool>,
    /// Required. The type of goal to use for tuning. Available types are `MAXIMIZE` and `MINIMIZE`. Defaults to `MAXIMIZE`.
    
    pub goal: Option<String>,
    /// Optional. The TensorFlow summary tag name to use for optimizing trials. For current versions of TensorFlow, this tag name should exactly match what is shown in TensorBoard, including all scopes. For versions of TensorFlow prior to 0.12, this should be only the tag passed to tf.Summary. By default, "training/hptuning/metric" will be used.
    #[serde(rename="hyperparameterMetricTag")]
    
    pub hyperparameter_metric_tag: Option<String>,
    /// Optional. The number of failed trials that need to be seen before failing the hyperparameter tuning job. You can specify this field to override the default failing criteria for AI Platform hyperparameter tuning jobs. Defaults to zero, which means the service decides when a hyperparameter job should fail.
    #[serde(rename="maxFailedTrials")]
    
    pub max_failed_trials: Option<i32>,
    /// Optional. The number of training trials to run concurrently. You can reduce the time it takes to perform hyperparameter tuning by adding trials in parallel. However, each trail only benefits from the information gained in completed trials. That means that a trial does not get access to the results of trials running at the same time, which could reduce the quality of the overall optimization. Each trial will use the same scale tier and machine types. Defaults to one.
    #[serde(rename="maxParallelTrials")]
    
    pub max_parallel_trials: Option<i32>,
    /// Optional. How many training trials should be attempted to optimize the specified hyperparameters. Defaults to one.
    #[serde(rename="maxTrials")]
    
    pub max_trials: Option<i32>,
    /// Required. The set of parameters to tune.
    
    pub params: Option<Vec<GoogleCloudMlV1__ParameterSpec>>,
    /// Optional. The prior hyperparameter tuning job id that users hope to continue with. The job id will be used to find the corresponding vizier study guid and resume the study.
    #[serde(rename="resumePreviousJobId")]
    
    pub resume_previous_job_id: Option<String>,
}

impl client::Part for GoogleCloudMlV1__HyperparameterSpec {}


/// Attributes credit by computing the Aumann-Shapley value taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1703.01365
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__IntegratedGradientsAttribution {
    /// Number of steps for approximating the path integral. A good value to start is 50 and gradually increase until the sum to diff property is met within the desired error range.
    #[serde(rename="numIntegralSteps")]
    
    pub num_integral_steps: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__IntegratedGradientsAttribution {}


/// Represents a training or prediction job.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs create projects](ProjectJobCreateCall) (request|response)
/// * [jobs get projects](ProjectJobGetCall) (response)
/// * [jobs patch projects](ProjectJobPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Job {
    /// Output only. When the job was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. When the job processing was completed.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The details of a failure or a cancellation.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a job from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform job updates in order to avoid race conditions: An `etag` is returned in the response to `GetJob`, and systems are expected to put that etag in the request to `UpdateJob` to ensure that their change will be applied to the same version of the job.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Required. The user-specified id of the job.
    #[serde(rename="jobId")]
    
    pub job_id: Option<String>,
    /// Output only. It's only effect when the job is in QUEUED state. If it's positive, it indicates the job's position in the job scheduler. It's 0 when the job is already scheduled.
    #[serde(rename="jobPosition")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub job_position: Option<i64>,
    /// Optional. One or more labels that you can add, to organize your jobs. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels.
    
    pub labels: Option<HashMap<String, String>>,
    /// Input parameters to create a prediction job.
    #[serde(rename="predictionInput")]
    
    pub prediction_input: Option<GoogleCloudMlV1__PredictionInput>,
    /// The current prediction job result.
    #[serde(rename="predictionOutput")]
    
    pub prediction_output: Option<GoogleCloudMlV1__PredictionOutput>,
    /// Output only. When the job processing was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The detailed state of a job.
    
    pub state: Option<String>,
    /// Input parameters to create a training job.
    #[serde(rename="trainingInput")]
    
    pub training_input: Option<GoogleCloudMlV1__TrainingInput>,
    /// The current training job result.
    #[serde(rename="trainingOutput")]
    
    pub training_output: Option<GoogleCloudMlV1__TrainingOutput>,
}

impl client::RequestValue for GoogleCloudMlV1__Job {}
impl client::ResponseResult for GoogleCloudMlV1__Job {}


/// Response message for the ListJobs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs list projects](ProjectJobListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListJobsResponse {
    /// The list of jobs.
    
    pub jobs: Option<Vec<GoogleCloudMlV1__Job>>,
    /// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudMlV1__ListJobsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListLocationsResponse {
    /// Locations where at least one type of CMLE capability is available.
    
    pub locations: Option<Vec<GoogleCloudMlV1__Location>>,
    /// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudMlV1__ListLocationsResponse {}


/// Response message for the ListModels method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [models list projects](ProjectModelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListModelsResponse {
    /// The list of models.
    
    pub models: Option<Vec<GoogleCloudMlV1__Model>>,
    /// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleCloudMlV1__ListModelsResponse {}


/// The request message for the ListTrials service method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials list optimal trials projects](ProjectLocationStudyTrialListOptimalTrialCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListOptimalTrialsRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudMlV1__ListOptimalTrialsRequest {}


/// The response message for the ListOptimalTrials method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials list optimal trials projects](ProjectLocationStudyTrialListOptimalTrialCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListOptimalTrialsResponse {
    /// The pareto-optimal trials for multiple objective study or the optimal trial for single objective study. The definition of pareto-optimal can be checked in wiki page. https://en.wikipedia.org/wiki/Pareto_efficiency
    
    pub trials: Option<Vec<GoogleCloudMlV1__Trial>>,
}

impl client::ResponseResult for GoogleCloudMlV1__ListOptimalTrialsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies list projects](ProjectLocationStudyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListStudiesResponse {
    /// The studies associated with the project.
    
    pub studies: Option<Vec<GoogleCloudMlV1__Study>>,
}

impl client::ResponseResult for GoogleCloudMlV1__ListStudiesResponse {}


/// The response message for the ListTrials method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials list projects](ProjectLocationStudyTrialListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListTrialsResponse {
    /// The trials associated with the study.
    
    pub trials: Option<Vec<GoogleCloudMlV1__Trial>>,
}

impl client::ResponseResult for GoogleCloudMlV1__ListTrialsResponse {}


/// Response message for the ListVersions method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [models versions list projects](ProjectModelVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ListVersionsResponse {
    /// Optional. Pass this token as the `page_token` field of the request for a subsequent call.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of versions.
    
    pub versions: Option<Vec<GoogleCloudMlV1__Version>>,
}

impl client::ResponseResult for GoogleCloudMlV1__ListVersionsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Location {
    /// Capabilities available in the location.
    
    pub capabilities: Option<Vec<GoogleCloudMlV1__Capability>>,
    /// no description provided
    
    pub name: Option<String>,
}

impl client::ResponseResult for GoogleCloudMlV1__Location {}


/// Options for manually scaling a model.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ManualScaling {
    /// The number of nodes to allocate for this model. These nodes are always up, starting from the time the model is deployed, so the cost of operating this model will be proportional to `nodes` * number of hours since last billing cycle plus the cost for each prediction performed.
    
    pub nodes: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__ManualScaling {}


/// A message representing a measurement.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Measurement {
    /// Output only. Time that the trial has been running at the point of this measurement.
    #[serde(rename="elapsedTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub elapsed_time: Option<client::chrono::Duration>,
    /// Provides a list of metrics that act as inputs into the objective function.
    
    pub metrics: Option<Vec<GoogleCloudMlV1_Measurement_Metric>>,
    /// The number of steps a machine learning model has been trained for. Must be non-negative.
    #[serde(rename="stepCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub step_count: Option<i64>,
}

impl client::Part for GoogleCloudMlV1__Measurement {}


/// MetricSpec contains the specifications to use to calculate the desired nodes count when autoscaling is enabled.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__MetricSpec {
    /// metric name.
    
    pub name: Option<String>,
    /// Target specifies the target value for the given metric; once real metric deviates from the threshold by a certain percentage, the node count changes.
    
    pub target: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__MetricSpec {}


/// Represents a machine learning solution. A model can have multiple versions, each of which is a deployed, trained model ready to receive prediction requests. The model itself is just a container.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [models create projects](ProjectModelCreateCall) (request|response)
/// * [models get projects](ProjectModelGetCall) (response)
/// * [models patch projects](ProjectModelPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Model {
    /// Output only. The default version of the model. This version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.models.versions.setDefault.
    #[serde(rename="defaultVersion")]
    
    pub default_version: Option<GoogleCloudMlV1__Version>,
    /// Optional. The description specified for the model when it was created.
    
    pub description: Option<String>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetModel`, and systems are expected to put that etag in the request to `UpdateModel` to ensure that their change will be applied to the model as intended.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Optional. One or more labels that you can add, to organize your models. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. Note that this field is not updatable for mls1* models.
    
    pub labels: Option<HashMap<String, String>>,
    /// Required. The name specified for the model when it was created. The model name must be unique within the project it is created in.
    
    pub name: Option<String>,
    /// Optional. If true, online prediction nodes send `stderr` and `stdout` streams to Cloud Logging. These can be more verbose than the standard access logs (see `onlinePredictionLogging`) and can incur higher cost. However, they are helpful for debugging. Note that [logs may incur a cost](https://cloud.google.com/stackdriver/pricing), especially if your project receives prediction requests at a high QPS. Estimate your costs before enabling this option. Default is false.
    #[serde(rename="onlinePredictionConsoleLogging")]
    
    pub online_prediction_console_logging: Option<bool>,
    /// Optional. If true, online prediction access logs are sent to Cloud Logging. These logs are like standard server access logs, containing information like timestamp and latency for each request. Note that [logs may incur a cost](https://cloud.google.com/stackdriver/pricing), especially if your project receives prediction requests at a high queries per second rate (QPS). Estimate your costs before enabling this option. Default is false.
    #[serde(rename="onlinePredictionLogging")]
    
    pub online_prediction_logging: Option<bool>,
    /// Optional. The list of regions where the model is going to be deployed. Only one region per model is supported. Defaults to 'us-central1' if nothing is set. See the available regions for AI Platform services. Note: * No matter where a model is deployed, it can always be accessed by users from anywhere, both for online and batch prediction. * The region for a batch prediction job is set by the region field when submitting the batch prediction job and does not take its value from this field.
    
    pub regions: Option<Vec<String>>,
}

impl client::RequestValue for GoogleCloudMlV1__Model {}
impl client::ResponseResult for GoogleCloudMlV1__Model {}


/// Represents a single hyperparameter to optimize.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ParameterSpec {
    /// Required if type is `CATEGORICAL`. The list of possible categories.
    #[serde(rename="categoricalValues")]
    
    pub categorical_values: Option<Vec<String>>,
    /// Required if type is `DISCRETE`. A list of feasible points. The list should be in strictly increasing order. For instance, this parameter might have possible settings of 1.5, 2.5, and 4.0. This list should not contain more than 1,000 values.
    #[serde(rename="discreteValues")]
    
    pub discrete_values: Option<Vec<f64>>,
    /// Required if type is `DOUBLE` or `INTEGER`. This field should be unset if type is `CATEGORICAL`. This value should be integers if type is `INTEGER`.
    #[serde(rename="maxValue")]
    
    pub max_value: Option<f64>,
    /// Required if type is `DOUBLE` or `INTEGER`. This field should be unset if type is `CATEGORICAL`. This value should be integers if type is INTEGER.
    #[serde(rename="minValue")]
    
    pub min_value: Option<f64>,
    /// Required. The parameter name must be unique amongst all ParameterConfigs in a HyperparameterSpec message. E.g., "learning_rate".
    #[serde(rename="parameterName")]
    
    pub parameter_name: Option<String>,
    /// Optional. How the parameter should be scaled to the hypercube. Leave unset for categorical parameters. Some kind of scaling is strongly recommended for real or integral parameters (e.g., `UNIT_LINEAR_SCALE`).
    #[serde(rename="scaleType")]
    
    pub scale_type: Option<String>,
    /// Required. The type of the parameter.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for GoogleCloudMlV1__ParameterSpec {}


/// Request for predictions to be issued against a trained model.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [predict projects](ProjectPredictCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__PredictRequest {
    /// Required. The prediction request body. Refer to the [request body details section](#request-body-details) for more information on how to structure your request.
    #[serde(rename="httpBody")]
    
    pub http_body: Option<GoogleApi__HttpBody>,
}

impl client::RequestValue for GoogleCloudMlV1__PredictRequest {}


/// Represents input parameters for a prediction job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__PredictionInput {
    /// Optional. Number of records per batch, defaults to 64. The service will buffer batch_size number of records in memory before invoking one Tensorflow prediction call internally. So take the record size and memory available into consideration when setting this parameter.
    #[serde(rename="batchSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub batch_size: Option<i64>,
    /// Required. The format of the input data files.
    #[serde(rename="dataFormat")]
    
    pub data_format: Option<String>,
    /// Required. The Cloud Storage location of the input data files. May contain wildcards.
    #[serde(rename="inputPaths")]
    
    pub input_paths: Option<Vec<String>>,
    /// Optional. The maximum number of workers to be used for parallel processing. Defaults to 10 if not specified.
    #[serde(rename="maxWorkerCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub max_worker_count: Option<i64>,
    /// Use this field if you want to use the default version for the specified model. The string must use the following format: `"projects/YOUR_PROJECT/models/YOUR_MODEL"`
    #[serde(rename="modelName")]
    
    pub model_name: Option<String>,
    /// Optional. Format of the output data files, defaults to JSON.
    #[serde(rename="outputDataFormat")]
    
    pub output_data_format: Option<String>,
    /// Required. The output Google Cloud Storage location.
    #[serde(rename="outputPath")]
    
    pub output_path: Option<String>,
    /// Required. The Google Compute Engine region to run the prediction job in. See the available regions for AI Platform services.
    
    pub region: Option<String>,
    /// Optional. The AI Platform runtime version to use for this batch prediction. If not set, AI Platform will pick the runtime version used during the CreateVersion request for this model version, or choose the latest stable version when model version information is not available such as when the model is specified by uri.
    #[serde(rename="runtimeVersion")]
    
    pub runtime_version: Option<String>,
    /// Optional. The name of the signature defined in the SavedModel to use for this job. Please refer to [SavedModel](https://tensorflow.github.io/serving/serving_basic.html) for information about how to use signatures. Defaults to [DEFAULT_SERVING_SIGNATURE_DEF_KEY](https://www.tensorflow.org/api_docs/python/tf/saved_model/signature_constants) , which is "serving_default".
    #[serde(rename="signatureName")]
    
    pub signature_name: Option<String>,
    /// Use this field if you want to specify a Google Cloud Storage path for the model to use.
    
    pub uri: Option<String>,
    /// Use this field if you want to specify a version of the model to use. The string is formatted the same way as `model_version`, with the addition of the version information: `"projects/YOUR_PROJECT/models/YOUR_MODEL/versions/YOUR_VERSION"`
    #[serde(rename="versionName")]
    
    pub version_name: Option<String>,
}

impl client::Part for GoogleCloudMlV1__PredictionInput {}


/// Represents results of a prediction job.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__PredictionOutput {
    /// The number of data instances which resulted in errors.
    #[serde(rename="errorCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub error_count: Option<i64>,
    /// Node hours used by the batch prediction job.
    #[serde(rename="nodeHours")]
    
    pub node_hours: Option<f64>,
    /// The output Google Cloud Storage location provided at the job creation time.
    #[serde(rename="outputPath")]
    
    pub output_path: Option<String>,
    /// The number of generated predictions.
    #[serde(rename="predictionCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub prediction_count: Option<i64>,
}

impl client::Part for GoogleCloudMlV1__PredictionOutput {}


/// Represents the configuration for a replica in a cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__ReplicaConfig {
    /// Represents the type and number of accelerators used by the replica. [Learn about restrictions on accelerator configurations for training.](https://cloud.google.com/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu)
    #[serde(rename="acceleratorConfig")]
    
    pub accelerator_config: Option<GoogleCloudMlV1__AcceleratorConfig>,
    /// Arguments to the entrypoint command. The following rules apply for container_command and container_args: - If you do not supply command or args: The defaults defined in the Docker image are used. - If you supply a command but no args: The default EntryPoint and the default Cmd defined in the Docker image are ignored. Your command is run without any arguments. - If you supply only args: The default Entrypoint defined in the Docker image is run with the args that you supplied. - If you supply a command and args: The default Entrypoint and the default Cmd defined in the Docker image are ignored. Your command is run with your args. It cannot be set if custom container image is not provided. Note that this field and [TrainingInput.args] are mutually exclusive, i.e., both cannot be set at the same time.
    #[serde(rename="containerArgs")]
    
    pub container_args: Option<Vec<String>>,
    /// The command with which the replica's custom container is run. If provided, it will override default ENTRYPOINT of the docker image. If not provided, the docker image's ENTRYPOINT is used. It cannot be set if custom container image is not provided. Note that this field and [TrainingInput.args] are mutually exclusive, i.e., both cannot be set at the same time.
    #[serde(rename="containerCommand")]
    
    pub container_command: Option<Vec<String>>,
    /// Represents the configuration of disk options.
    #[serde(rename="diskConfig")]
    
    pub disk_config: Option<GoogleCloudMlV1__DiskConfig>,
    /// The Docker image to run on the replica. This image must be in Container Registry. Learn more about [configuring custom containers](https://cloud.google.com/ai-platform/training/docs/distributed-training-containers).
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
    /// The AI Platform runtime version that includes a TensorFlow version matching the one used in the custom container. This field is required if the replica is a TPU worker that uses a custom container. Otherwise, do not specify this field. This must be a [runtime version that currently supports training with TPUs](https://cloud.google.com/ml-engine/docs/tensorflow/runtime-version-list#tpu-support). Note that the version of TensorFlow included in a runtime version may differ from the numbering of the runtime version itself, because it may have a different [patch version](https://www.tensorflow.org/guide/version_compat#semantic_versioning_20). In this field, you must specify the runtime version (TensorFlow minor version). For example, if your custom container runs TensorFlow `1.x.y`, specify `1.x`.
    #[serde(rename="tpuTfVersion")]
    
    pub tpu_tf_version: Option<String>,
}

impl client::Part for GoogleCloudMlV1__ReplicaConfig {}


/// Configuration for logging request-response pairs to a BigQuery table. Online prediction requests to a model version and the responses to these requests are converted to raw strings and saved to the specified BigQuery table. Logging is constrained by [BigQuery quotas and limits](https://cloud.google.com/bigquery/quotas). If your project exceeds BigQuery quotas or limits, AI Platform Prediction does not log request-response pairs, but it continues to serve predictions. If you are using [continuous evaluation](https://cloud.google.com/ml-engine/docs/continuous-evaluation/), you do not need to specify this configuration manually. Setting up continuous evaluation automatically enables logging of request-response pairs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__RequestLoggingConfig {
    /// Required. Fully qualified BigQuery table name in the following format: “ project_id.dataset_name.table_name“ The specified table must already exist, and the “Cloud ML Service Agent” for your project must have permission to write to it. The table must have the following [schema](https://cloud.google.com/bigquery/docs/schemas): Field nameType Mode model STRING REQUIRED model_version STRING REQUIRED time TIMESTAMP REQUIRED raw_data STRING REQUIRED raw_prediction STRING NULLABLE groundtruth STRING NULLABLE 
    #[serde(rename="bigqueryTableName")]
    
    pub bigquery_table_name: Option<String>,
    /// Percentage of requests to be logged, expressed as a fraction from 0 to 1. For example, if you want to log 10% of requests, enter `0.1`. The sampling window is the lifetime of the model version. Defaults to 0.
    #[serde(rename="samplingPercentage")]
    
    pub sampling_percentage: Option<f64>,
}

impl client::Part for GoogleCloudMlV1__RequestLoggingConfig {}


/// Specifies HTTP paths served by a custom container. AI Platform Prediction sends requests to these paths on the container; the custom container must run an HTTP server that responds to these requests with appropriate responses. Read [Custom container requirements](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements) for details on how to create your container image to meet these requirements.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__RouteMap {
    /// HTTP path on the container to send health checkss to. AI Platform Prediction intermittently sends GET requests to this path on the container’s IP address and port to check that the container is healthy. Read more about [health checks](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements#checks). For example, if you set this field to `/bar`, then AI Platform Prediction intermittently sends a GET request to the `/bar` path on the port of your container specified by the first value of Version.container.ports. If you don’t specify this field, it defaults to the following value: /v1/models/ MODEL/versions/VERSION The placeholders in this value are replaced as follows: * MODEL: The name of the parent Model. This does not include the “projects/PROJECT_ID/models/” prefix that the API returns in output; it is the bare model name, as provided to projects.models.create. * VERSION: The name of the model version. This does not include the “projects/PROJECT_ID /models/MODEL/versions/” prefix that the API returns in output; it is the bare version name, as provided to projects.models.versions.create.
    
    pub health: Option<String>,
    /// HTTP path on the container to send prediction requests to. AI Platform Prediction forwards requests sent using projects.predict to this path on the container's IP address and port. AI Platform Prediction then returns the container's response in the API response. For example, if you set this field to `/foo`, then when AI Platform Prediction receives a prediction request, it forwards the request body in a POST request to the `/foo` path on the port of your container specified by the first value of Version.container.ports. If you don't specify this field, it defaults to the following value: /v1/models/MODEL/versions/VERSION:predict The placeholders in this value are replaced as follows: * MODEL: The name of the parent Model. This does not include the "projects/PROJECT_ID/models/" prefix that the API returns in output; it is the bare model name, as provided to projects.models.create. * VERSION: The name of the model version. This does not include the "projects/PROJECT_ID/models/MODEL/versions/" prefix that the API returns in output; it is the bare version name, as provided to projects.models.versions.create.
    
    pub predict: Option<String>,
}

impl client::Part for GoogleCloudMlV1__RouteMap {}


/// An attribution method that approximates Shapley values for features that contribute to the label being predicted. A sampling strategy is used to approximate the value rather than considering all subsets of features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__SampledShapleyAttribution {
    /// The number of feature permutations to consider when approximating the Shapley values.
    #[serde(rename="numPaths")]
    
    pub num_paths: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__SampledShapleyAttribution {}


/// All parameters related to scheduling of training jobs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Scheduling {
    /// Optional. The maximum job running time, expressed in seconds. The field can contain up to nine fractional digits, terminated by `s`. If not specified, this field defaults to `604800s` (seven days). If the training job is still running after this duration, AI Platform Training cancels it. The duration is measured from when the job enters the `RUNNING` state; therefore it does not overlap with the duration limited by Scheduling.max_wait_time. For example, if you want to ensure your job runs for no more than 2 hours, set this field to `7200s` (2 hours * 60 minutes / hour * 60 seconds / minute). If you submit your training job using the `gcloud` tool, you can [specify this field in a `config.yaml` file](https://cloud.google.com/ai-platform/training/docs/training-jobs#formatting_your_configuration_parameters). For example: `yaml trainingInput: scheduling: maxRunningTime: 7200s `
    #[serde(rename="maxRunningTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_running_time: Option<client::chrono::Duration>,
    /// Optional. The maximum job wait time, expressed in seconds. The field can contain up to nine fractional digits, terminated by `s`. If not specified, there is no limit to the wait time. The minimum for this field is `1800s` (30 minutes). If the training job has not entered the `RUNNING` state after this duration, AI Platform Training cancels it. After the job begins running, it can no longer be cancelled due to the maximum wait time. Therefore the duration limited by this field does not overlap with the duration limited by Scheduling.max_running_time. For example, if the job temporarily stops running and retries due to a [VM restart](https://cloud.google.com/ai-platform/training/docs/overview#restarts), this cannot lead to a maximum wait time cancellation. However, independently of this constraint, AI Platform Training might stop a job if there are too many retries due to exhausted resources in a region. The following example describes how you might use this field: To cancel your job if it doesn’t start running within 1 hour, set this field to `3600s` (1 hour * 60 minutes / hour * 60 seconds / minute). If the job is still in the `QUEUED` or `PREPARING` state after an hour of waiting, AI Platform Training cancels the job. If you submit your training job using the `gcloud` tool, you can [specify this field in a `config.yaml` file](https://cloud.google.com/ai-platform/training/docs/training-jobs#formatting_your_configuration_parameters). For example: `yaml trainingInput: scheduling: maxWaitTime: 3600s `
    #[serde(rename="maxWaitTime")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub max_wait_time: Option<client::chrono::Duration>,
    /// Optional. Job scheduling will be based on this priority, which in the range [0, 1000]. The bigger the number, the higher the priority. Default to 0 if not set. If there are multiple jobs requesting same type of accelerators, the high priority job will be scheduled prior to ones with low priority.
    
    pub priority: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__Scheduling {}


/// Request message for the SetDefaultVersion request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [models versions set default projects](ProjectModelVersionSetDefaultCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__SetDefaultVersionRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudMlV1__SetDefaultVersionRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials stop projects](ProjectLocationStudyTrialStopCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__StopTrialRequest { _never_set: Option<bool> }

impl client::RequestValue for GoogleCloudMlV1__StopTrialRequest {}


/// A message representing a Study.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies create projects](ProjectLocationStudyCreateCall) (request|response)
/// * [locations studies get projects](ProjectLocationStudyGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Study {
    /// Output only. Time at which the study was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED.
    #[serde(rename="inactiveReason")]
    
    pub inactive_reason: Option<String>,
    /// Output only. The name of a study.
    
    pub name: Option<String>,
    /// Output only. The detailed state of a study.
    
    pub state: Option<String>,
    /// Required. Configuration of the study.
    #[serde(rename="studyConfig")]
    
    pub study_config: Option<GoogleCloudMlV1__StudyConfig>,
}

impl client::RequestValue for GoogleCloudMlV1__Study {}
impl client::ResponseResult for GoogleCloudMlV1__Study {}


/// Represents configuration of a study.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__StudyConfig {
    /// The search algorithm specified for the study.
    
    pub algorithm: Option<String>,
    /// Configuration for automated stopping of unpromising Trials.
    #[serde(rename="automatedStoppingConfig")]
    
    pub automated_stopping_config: Option<GoogleCloudMlV1__AutomatedStoppingConfig>,
    /// Metric specs for the study.
    
    pub metrics: Option<Vec<GoogleCloudMlV1_StudyConfig_MetricSpec>>,
    /// Required. The set of parameters to tune.
    
    pub parameters: Option<Vec<GoogleCloudMlV1_StudyConfig_ParameterSpec>>,
}

impl client::Part for GoogleCloudMlV1__StudyConfig {}


/// The request message for the SuggestTrial service method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials suggest projects](ProjectLocationStudyTrialSuggestCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__SuggestTrialsRequest {
    /// Required. The identifier of the client that is requesting the suggestion. If multiple SuggestTrialsRequests have the same `client_id`, the service will return the identical suggested trial if the trial is pending, and provide a new trial if the last suggested trial was completed.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Required. The number of suggestions requested.
    #[serde(rename="suggestionCount")]
    
    pub suggestion_count: Option<i32>,
}

impl client::RequestValue for GoogleCloudMlV1__SuggestTrialsRequest {}


/// Represents input parameters for a training job. When using the gcloud command to submit your training job, you can specify the input parameters as command-line arguments and/or in a YAML configuration file referenced from the –config command-line argument. For details, see the guide to [submitting a training job](https://cloud.google.com/ai-platform/training/docs/training-jobs).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__TrainingInput {
    /// Optional. Command-line arguments passed to the training application when it starts. If your job uses a custom container, then the arguments are passed to the container's `ENTRYPOINT` command.
    
    pub args: Option<Vec<String>>,
    /// Optional. Whether you want AI Platform Training to enable [interactive shell access](https://cloud.google.com/ai-platform/training/docs/monitor-debug-interactive-shell) to training containers. If set to `true`, you can access interactive shells at the URIs given by TrainingOutput.web_access_uris or HyperparameterOutput.web_access_uris (within TrainingOutput.trials).
    #[serde(rename="enableWebAccess")]
    
    pub enable_web_access: Option<bool>,
    /// Optional. Options for using customer-managed encryption keys (CMEK) to protect resources created by a training job, instead of using Google’s default encryption. If this is set, then all resources created by the training job will be encrypted with the customer-managed encryption key that you specify. [Learn how and when to use CMEK with AI Platform Training](https://cloud.google.com/ai-platform/training/docs/cmek).
    #[serde(rename="encryptionConfig")]
    
    pub encryption_config: Option<GoogleCloudMlV1__EncryptionConfig>,
    /// Optional. The configuration for evaluators. You should only set `evaluatorConfig.acceleratorConfig` if `evaluatorType` is set to a Compute Engine machine type. [Learn about restrictions on accelerator configurations for training.](https://cloud.google.com/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `evaluatorConfig.imageUri` only if you build a custom image for your evaluator. If `evaluatorConfig.imageUri` has not been set, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom containers](https://cloud.google.com/ai-platform/training/docs/distributed-training-containers).
    #[serde(rename="evaluatorConfig")]
    
    pub evaluator_config: Option<GoogleCloudMlV1__ReplicaConfig>,
    /// Optional. The number of evaluator replicas to use for the training job. Each replica in the cluster will be of the type specified in `evaluator_type`. This value can only be used when `scale_tier` is set to `CUSTOM`. If you set this value, you must also set `evaluator_type`. The default value is zero.
    #[serde(rename="evaluatorCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub evaluator_count: Option<i64>,
    /// Optional. Specifies the type of virtual machine to use for your training job's evaluator nodes. The supported values are the same as those described in the entry for `masterType`. This value must be consistent with the category of machine type that `masterType` uses. In other words, both must be Compute Engine machine types or both must be legacy machine types. This value must be present when `scaleTier` is set to `CUSTOM` and `evaluatorCount` is greater than zero.
    #[serde(rename="evaluatorType")]
    
    pub evaluator_type: Option<String>,
    /// Optional. The set of Hyperparameters to tune.
    
    pub hyperparameters: Option<GoogleCloudMlV1__HyperparameterSpec>,
    /// Optional. A Google Cloud Storage path in which to store training outputs and other data needed for training. This path is passed to your TensorFlow program as the '--job-dir' command-line argument. The benefit of specifying this field is that Cloud ML validates the path for use in training.
    #[serde(rename="jobDir")]
    
    pub job_dir: Option<String>,
    /// Optional. The configuration for your master worker. You should only set `masterConfig.acceleratorConfig` if `masterType` is set to a Compute Engine machine type. Learn about [restrictions on accelerator configurations for training.](https://cloud.google.com/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `masterConfig.imageUri` only if you build a custom image. Only one of `masterConfig.imageUri` and `runtimeVersion` should be set. Learn more about [configuring custom containers](https://cloud.google.com/ai-platform/training/docs/distributed-training-containers).
    #[serde(rename="masterConfig")]
    
    pub master_config: Option<GoogleCloudMlV1__ReplicaConfig>,
    /// Optional. Specifies the type of virtual machine to use for your training job’s master worker. You must specify this field when `scaleTier` is set to `CUSTOM`. You can use certain Compute Engine machine types directly in this field. See the [list of compatible Compute Engine machine types](https://cloud.google.com/ai-platform/training/docs/machine-types#compute-engine-machine-types). Alternatively, you can use the certain legacy machine types in this field. See the [list of legacy machine types](https://cloud.google.com/ai-platform/training/docs/machine-types#legacy-machine-types). Finally, if you want to use a TPU for training, specify `cloud_tpu` in this field. Learn more about the [special configuration options for training with TPUs](https://cloud.google.com/ai-platform/training/docs/using-tpus#configuring_a_custom_tpu_machine).
    #[serde(rename="masterType")]
    
    pub master_type: Option<String>,
    /// Optional. The full name of the [Compute Engine network](https://cloud.google.com/vpc/docs/vpc) to which the Job is peered. For example, `projects/12345/global/networks/myVPC`. The format of this field is `projects/{project}/global/networks/{network}`, where {project} is a project number (like `12345`) and {network} is network name. Private services access must already be configured for the network. If left unspecified, the Job is not peered with any network. [Learn about using VPC Network Peering.](https://cloud.google.com/ai-platform/training/docs/vpc-peering).
    
    pub network: Option<String>,
    /// Required. The Google Cloud Storage location of the packages with the training program and any additional dependencies. The maximum number of package URIs is 100.
    #[serde(rename="packageUris")]
    
    pub package_uris: Option<Vec<String>>,
    /// Optional. The configuration for parameter servers. You should only set `parameterServerConfig.acceleratorConfig` if `parameterServerType` is set to a Compute Engine machine type. [Learn about restrictions on accelerator configurations for training.](https://cloud.google.com/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `parameterServerConfig.imageUri` only if you build a custom image for your parameter server. If `parameterServerConfig.imageUri` has not been set, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom containers](https://cloud.google.com/ai-platform/training/docs/distributed-training-containers).
    #[serde(rename="parameterServerConfig")]
    
    pub parameter_server_config: Option<GoogleCloudMlV1__ReplicaConfig>,
    /// Optional. The number of parameter server replicas to use for the training job. Each replica in the cluster will be of the type specified in `parameter_server_type`. This value can only be used when `scale_tier` is set to `CUSTOM`. If you set this value, you must also set `parameter_server_type`. The default value is zero.
    #[serde(rename="parameterServerCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub parameter_server_count: Option<i64>,
    /// Optional. Specifies the type of virtual machine to use for your training job's parameter server. The supported values are the same as those described in the entry for `master_type`. This value must be consistent with the category of machine type that `masterType` uses. In other words, both must be Compute Engine machine types or both must be legacy machine types. This value must be present when `scaleTier` is set to `CUSTOM` and `parameter_server_count` is greater than zero.
    #[serde(rename="parameterServerType")]
    
    pub parameter_server_type: Option<String>,
    /// Required. The Python module name to run after installing the packages.
    #[serde(rename="pythonModule")]
    
    pub python_module: Option<String>,
    /// Optional. The version of Python used in training. You must either specify this field or specify `masterConfig.imageUri`. The following Python versions are available: * Python ‘3.7’ is available when `runtime_version` is set to ‘1.15’ or later. * Python ‘3.5’ is available when `runtime_version` is set to a version from ‘1.4’ to ‘1.14’. * Python ‘2.7’ is available when `runtime_version` is set to ‘1.15’ or earlier. Read more about the Python versions available for [each runtime version](https://cloud.google.com/ml-engine/docs/runtime-version-list).
    #[serde(rename="pythonVersion")]
    
    pub python_version: Option<String>,
    /// Required. The region to run the training job in. See the [available regions](https://cloud.google.com/ai-platform/training/docs/regions) for AI Platform Training.
    
    pub region: Option<String>,
    /// Optional. The AI Platform runtime version to use for training. You must either specify this field or specify `masterConfig.imageUri`. For more information, see the [runtime version list](https://cloud.google.com/ai-platform/training/docs/runtime-version-list) and learn [how to manage runtime versions](https://cloud.google.com/ai-platform/training/docs/versioning).
    #[serde(rename="runtimeVersion")]
    
    pub runtime_version: Option<String>,
    /// Required. Specifies the machine types, the number of replicas for workers and parameter servers.
    #[serde(rename="scaleTier")]
    
    pub scale_tier: Option<String>,
    /// Optional. Scheduling options for a training job.
    
    pub scheduling: Option<GoogleCloudMlV1__Scheduling>,
    /// Optional. The email address of a service account to use when running the training appplication. You must have the `iam.serviceAccounts.actAs` permission for the specified service account. In addition, the AI Platform Training Google-managed service account must have the `roles/iam.serviceAccountAdmin` role for the specified service account. [Learn more about configuring a service account.](https://cloud.google.com/ai-platform/training/docs/custom-service-account) If not specified, the AI Platform Training Google-managed service account is used by default.
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Optional. Use `chief` instead of `master` in the `TF_CONFIG` environment variable when training with a custom container. Defaults to `false`. [Learn more about this field.](https://cloud.google.com/ai-platform/training/docs/distributed-training-details#chief-versus-master) This field has no effect for training jobs that don’t use a custom container.
    #[serde(rename="useChiefInTfConfig")]
    
    pub use_chief_in_tf_config: Option<bool>,
    /// Optional. The configuration for workers. You should only set `workerConfig.acceleratorConfig` if `workerType` is set to a Compute Engine machine type. [Learn about restrictions on accelerator configurations for training.](https://cloud.google.com/ai-platform/training/docs/using-gpus#compute-engine-machine-types-with-gpu) Set `workerConfig.imageUri` only if you build a custom image for your worker. If `workerConfig.imageUri` has not been set, AI Platform uses the value of `masterConfig.imageUri`. Learn more about [configuring custom containers](https://cloud.google.com/ai-platform/training/docs/distributed-training-containers).
    #[serde(rename="workerConfig")]
    
    pub worker_config: Option<GoogleCloudMlV1__ReplicaConfig>,
    /// Optional. The number of worker replicas to use for the training job. Each replica in the cluster will be of the type specified in `worker_type`. This value can only be used when `scale_tier` is set to `CUSTOM`. If you set this value, you must also set `worker_type`. The default value is zero.
    #[serde(rename="workerCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub worker_count: Option<i64>,
    /// Optional. Specifies the type of virtual machine to use for your training job’s worker nodes. The supported values are the same as those described in the entry for `masterType`. This value must be consistent with the category of machine type that `masterType` uses. In other words, both must be Compute Engine machine types or both must be legacy machine types. If you use `cloud_tpu` for this value, see special instructions for [configuring a custom TPU machine](https://cloud.google.com/ml-engine/docs/tensorflow/using-tpus#configuring_a_custom_tpu_machine). This value must be present when `scaleTier` is set to `CUSTOM` and `workerCount` is greater than zero.
    #[serde(rename="workerType")]
    
    pub worker_type: Option<String>,
}

impl client::Part for GoogleCloudMlV1__TrainingInput {}


/// Represents results of a training job. Output only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__TrainingOutput {
    /// Details related to built-in algorithms jobs. Only set for built-in algorithms jobs.
    #[serde(rename="builtInAlgorithmOutput")]
    
    pub built_in_algorithm_output: Option<GoogleCloudMlV1__BuiltInAlgorithmOutput>,
    /// The number of hyperparameter tuning trials that completed successfully. Only set for hyperparameter tuning jobs.
    #[serde(rename="completedTrialCount")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub completed_trial_count: Option<i64>,
    /// The amount of ML units consumed by the job.
    #[serde(rename="consumedMLUnits")]
    
    pub consumed_ml_units: Option<f64>,
    /// The TensorFlow summary tag name used for optimizing hyperparameter tuning trials. See [`HyperparameterSpec.hyperparameterMetricTag`](#HyperparameterSpec.FIELDS.hyperparameter_metric_tag) for more information. Only set for hyperparameter tuning jobs.
    #[serde(rename="hyperparameterMetricTag")]
    
    pub hyperparameter_metric_tag: Option<String>,
    /// Whether this job is a built-in Algorithm job.
    #[serde(rename="isBuiltInAlgorithmJob")]
    
    pub is_built_in_algorithm_job: Option<bool>,
    /// Whether this job is a hyperparameter tuning job.
    #[serde(rename="isHyperparameterTuningJob")]
    
    pub is_hyperparameter_tuning_job: Option<bool>,
    /// Results for individual Hyperparameter trials. Only set for hyperparameter tuning jobs.
    
    pub trials: Option<Vec<GoogleCloudMlV1__HyperparameterOutput>>,
    /// Output only. URIs for accessing [interactive shells](https://cloud.google.com/ai-platform/training/docs/monitor-debug-interactive-shell) (one URI for each training node). Only available if training_input.enable_web_access is `true`. The keys are names of each node in the training job; for example, `master-replica-0` for the master node, `worker-replica-0` for the first worker, and `ps-replica-0` for the first parameter server. The values are the URIs for each node's interactive shell.
    #[serde(rename="webAccessUris")]
    
    pub web_access_uris: Option<HashMap<String, String>>,
}

impl client::Part for GoogleCloudMlV1__TrainingOutput {}


/// A message representing a trial.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations studies trials add measurement projects](ProjectLocationStudyTrialAddMeasurementCall) (response)
/// * [locations studies trials complete projects](ProjectLocationStudyTrialCompleteCall) (response)
/// * [locations studies trials create projects](ProjectLocationStudyTrialCreateCall) (request|response)
/// * [locations studies trials get projects](ProjectLocationStudyTrialGetCall) (response)
/// * [locations studies trials stop projects](ProjectLocationStudyTrialStopCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Trial {
    /// Output only. The identifier of the client that originally requested this trial.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Output only. Time at which the trial's status changed to COMPLETED.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The final measurement containing the objective value.
    #[serde(rename="finalMeasurement")]
    
    pub final_measurement: Option<GoogleCloudMlV1__Measurement>,
    /// Output only. A human readable string describing why the trial is infeasible. This should only be set if trial_infeasible is true.
    #[serde(rename="infeasibleReason")]
    
    pub infeasible_reason: Option<String>,
    /// A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_time). These are used for early stopping computations.
    
    pub measurements: Option<Vec<GoogleCloudMlV1__Measurement>>,
    /// Output only. Name of the trial assigned by the service.
    
    pub name: Option<String>,
    /// The parameters of the trial.
    
    pub parameters: Option<Vec<GoogleCloudMlV1_Trial_Parameter>>,
    /// Output only. Time at which the trial was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The detailed state of a trial.
    
    pub state: Option<String>,
    /// Output only. If true, the parameters in this trial are not attempted again.
    #[serde(rename="trialInfeasible")]
    
    pub trial_infeasible: Option<bool>,
}

impl client::RequestValue for GoogleCloudMlV1__Trial {}
impl client::ResponseResult for GoogleCloudMlV1__Trial {}


/// Represents a version of the model. Each version is a trained model deployed in the cloud, ready to handle prediction requests. A model can have multiple versions. You can get information about all of the versions of a given model by calling projects.models.versions.list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [models versions create projects](ProjectModelVersionCreateCall) (request)
/// * [models versions get projects](ProjectModelVersionGetCall) (response)
/// * [models versions patch projects](ProjectModelVersionPatchCall) (request)
/// * [models versions set default projects](ProjectModelVersionSetDefaultCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__Version {
    /// Optional. Accelerator config for using GPUs for online prediction (beta). Only specify this field if you have specified a Compute Engine (N1) machine type in the `machineType` field. Learn more about [using GPUs for online prediction](https://cloud.google.com/ml-engine/docs/machine-types-online-prediction#gpus).
    #[serde(rename="acceleratorConfig")]
    
    pub accelerator_config: Option<GoogleCloudMlV1__AcceleratorConfig>,
    /// Automatically scale the number of nodes used to serve the model in response to increases and decreases in traffic. Care should be taken to ramp up traffic according to the model's ability to scale or you will start seeing increases in latency and 429 response codes.
    #[serde(rename="autoScaling")]
    
    pub auto_scaling: Option<GoogleCloudMlV1__AutoScaling>,
    /// Optional. Specifies a custom container to use for serving predictions. If you specify this field, then `machineType` is required. If you specify this field, then `deploymentUri` is optional. If you specify this field, then you must not specify `runtimeVersion`, `packageUris`, `framework`, `pythonVersion`, or `predictionClass`.
    
    pub container: Option<GoogleCloudMlV1__ContainerSpec>,
    /// Output only. The time the version was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The Cloud Storage URI of a directory containing trained model artifacts to be used to create the model version. See the [guide to deploying models](https://cloud.google.com/ai-platform/prediction/docs/deploying-models) for more information. The total number of files under this directory must not exceed 1000. During projects.models.versions.create, AI Platform Prediction copies all files from the specified directory to a location managed by the service. From then on, AI Platform Prediction uses these copies of the model artifacts to serve predictions, not the original files in Cloud Storage, so this location is useful only as a historical record. If you specify container, then this field is optional. Otherwise, it is required. Learn [how to use this field with a custom container](https://cloud.google.com/ai-platform/prediction/docs/custom-container-requirements#artifacts).
    #[serde(rename="deploymentUri")]
    
    pub deployment_uri: Option<String>,
    /// Optional. The description specified for the version when it was created.
    
    pub description: Option<String>,
    /// Output only. The details of a failure or a cancellation.
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetVersion`, and systems are expected to put that etag in the request to `UpdateVersion` to ensure that their change will be applied to the model as intended.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Optional. Configures explainability features on the model's version. Some explanation features require additional metadata to be loaded as part of the model payload.
    #[serde(rename="explanationConfig")]
    
    pub explanation_config: Option<GoogleCloudMlV1__ExplanationConfig>,
    /// Optional. The machine learning framework AI Platform uses to train this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`, `XGBOOST`. If you do not specify a framework, AI Platform will analyze files in the deployment_uri to determine a framework. If you choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version of the model to 1.4 or greater. Do **not** specify a framework if you’re deploying a [custom prediction routine](https://cloud.google.com/ai-platform/prediction/docs/custom-prediction-routines) or if you’re using a [custom container](https://cloud.google.com/ai-platform/prediction/docs/use-custom-container).
    
    pub framework: Option<String>,
    /// Output only. If true, this version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.methods.versions.setDefault.
    #[serde(rename="isDefault")]
    
    pub is_default: Option<bool>,
    /// Optional. One or more labels that you can add, to organize your model versions. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. Note that this field is not updatable for mls1* models.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. The [AI Platform (Unified) `Model`](https://cloud.google.com/ai-platform-unified/docs/reference/rest/v1beta1/projects.locations.models) ID for the last [model migration](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified).
    #[serde(rename="lastMigrationModelId")]
    
    pub last_migration_model_id: Option<String>,
    /// Output only. The last time this version was successfully [migrated to AI Platform (Unified)](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified).
    #[serde(rename="lastMigrationTime")]
    
    pub last_migration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The time the version was last used for prediction.
    #[serde(rename="lastUseTime")]
    
    pub last_use_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. The type of machine on which to serve the model. Currently only applies to online prediction service. To learn about valid values for this field, read [Choosing a machine type for online prediction](https://cloud.google.com/ai-platform/prediction/docs/machine-types-online-prediction). If this field is not specified and you are using a [regional endpoint](https://cloud.google.com/ai-platform/prediction/docs/regional-endpoints), then the machine type defaults to `n1-standard-2`. If this field is not specified and you are using the global endpoint (`ml.googleapis.com`), then the machine type defaults to `mls1-c1-m2`.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
    /// Manually select the number of nodes to use for serving the model. You should generally use `auto_scaling` with an appropriate `min_nodes` instead, but this option is available if you want more predictable billing. Beware that latency and error rates will increase if the traffic exceeds that capability of the system to serve it based on the selected number of nodes.
    #[serde(rename="manualScaling")]
    
    pub manual_scaling: Option<GoogleCloudMlV1__ManualScaling>,
    /// Required. The name specified for the version when it was created. The version name must be unique within the model it is created in.
    
    pub name: Option<String>,
    /// Optional. Cloud Storage paths (`gs://…`) of packages for [custom prediction routines](https://cloud.google.com/ml-engine/docs/tensorflow/custom-prediction-routines) or [scikit-learn pipelines with custom code](https://cloud.google.com/ml-engine/docs/scikit/exporting-for-prediction#custom-pipeline-code). For a custom prediction routine, one of these packages must contain your Predictor class (see [`predictionClass`](#Version.FIELDS.prediction_class)). Additionally, include any dependencies used by your Predictor or scikit-learn pipeline uses that are not already included in your selected [runtime version](https://cloud.google.com/ml-engine/docs/tensorflow/runtime-version-list). If you specify this field, you must also set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater.
    #[serde(rename="packageUris")]
    
    pub package_uris: Option<Vec<String>>,
    /// Optional. The fully qualified name (module_name.class_name) of a class that implements the Predictor interface described in this reference field. The module containing this class should be included in a package provided to the [`packageUris` field](#Version.FIELDS.package_uris). Specify this field if and only if you are deploying a [custom prediction routine (beta)](https://cloud.google.com/ml-engine/docs/tensorflow/custom-prediction-routines). If you specify this field, you must set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater and you must set `machineType` to a [legacy (MLS1) machine type](https://cloud.google.com/ml-engine/docs/machine-types-online-prediction). The following code sample provides the Predictor interface: class Predictor(object): “”“Interface for constructing custom predictors.”“” def predict(self, instances, \*\*kwargs): “”“Performs custom prediction. Instances are the decoded values from the request. They have already been deserialized from JSON. Args: instances: A list of prediction input instances. \*\*kwargs: A dictionary of keyword args provided as additional fields on the predict request body. Returns: A list of outputs containing the prediction results. This list must be JSON serializable. “”“ raise NotImplementedError() @classmethod def from_path(cls, model_dir): “”“Creates an instance of Predictor using the given path. Loading of the predictor should be done in this method. Args: model_dir: The local directory that contains the exported model file along with any additional files uploaded when creating the version resource. Returns: An instance implementing this Predictor class. “”“ raise NotImplementedError() Learn more about [the Predictor interface and custom prediction routines](https://cloud.google.com/ml-engine/docs/tensorflow/custom-prediction-routines).
    #[serde(rename="predictionClass")]
    
    pub prediction_class: Option<String>,
    /// Required. The version of Python used in prediction. The following Python versions are available: * Python ‘3.7’ is available when `runtime_version` is set to ‘1.15’ or later. * Python ‘3.5’ is available when `runtime_version` is set to a version from ‘1.4’ to ‘1.14’. * Python ‘2.7’ is available when `runtime_version` is set to ‘1.15’ or earlier. Read more about the Python versions available for [each runtime version](https://cloud.google.com/ml-engine/docs/runtime-version-list).
    #[serde(rename="pythonVersion")]
    
    pub python_version: Option<String>,
    /// Optional. *Only* specify this field in a projects.models.versions.patch request. Specifying it in a projects.models.versions.create request has no effect. Configures the request-response pair logging on predictions from this Version.
    #[serde(rename="requestLoggingConfig")]
    
    pub request_logging_config: Option<GoogleCloudMlV1__RequestLoggingConfig>,
    /// Optional. Specifies paths on a custom container's HTTP server where AI Platform Prediction sends certain requests. If you specify this field, then you must also specify the `container` field. If you specify the `container` field and do not specify this field, it defaults to the following: ```json { "predict": "/v1/models/MODEL/versions/VERSION:predict", "health": "/v1/models/MODEL/versions/VERSION" } ``` See RouteMap for more details about these default values.
    
    pub routes: Option<GoogleCloudMlV1__RouteMap>,
    /// Required. The AI Platform runtime version to use for this deployment. For more information, see the [runtime version list](https://cloud.google.com/ml-engine/docs/runtime-version-list) and [how to manage runtime versions](https://cloud.google.com/ml-engine/docs/versioning).
    #[serde(rename="runtimeVersion")]
    
    pub runtime_version: Option<String>,
    /// Optional. Specifies the service account for resource access control. If you specify this field, then you must also specify either the `containerSpec` or the `predictionClass` field. Learn more about [using a custom service account](https://cloud.google.com/ai-platform/prediction/docs/custom-service-account).
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// Output only. The state of a version.
    
    pub state: Option<String>,
}

impl client::RequestValue for GoogleCloudMlV1__Version {}
impl client::ResponseResult for GoogleCloudMlV1__Version {}


/// Attributes credit by computing the XRAI taking advantage of the model's fully differentiable structure. Refer to this paper for more details: https://arxiv.org/abs/1906.02825 Currently only implemented for models with natural image inputs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCloudMlV1__XraiAttribution {
    /// Number of steps for approximating the path integral. A good value to start is 50 and gradually increase until the sum to diff property is met within the desired error range.
    #[serde(rename="numIntegralSteps")]
    
    pub num_integral_steps: Option<i32>,
}

impl client::Part for GoogleCloudMlV1__XraiAttribution {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<GoogleIamV1__AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for GoogleIamV1__AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for GoogleIamV1__AuditLogConfig {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<GoogleType__Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for GoogleIamV1__Binding {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs get iam policy projects](ProjectJobGetIamPolicyCall) (response)
/// * [jobs set iam policy projects](ProjectJobSetIamPolicyCall) (response)
/// * [models get iam policy projects](ProjectModelGetIamPolicyCall) (response)
/// * [models set iam policy projects](ProjectModelSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<GoogleIamV1__AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<GoogleIamV1__Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for GoogleIamV1__Policy {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs set iam policy projects](ProjectJobSetIamPolicyCall) (request)
/// * [models set iam policy projects](ProjectModelSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<GoogleIamV1__Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for GoogleIamV1__SetIamPolicyRequest {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs test iam permissions projects](ProjectJobTestIamPermissionCall) (request)
/// * [models test iam permissions projects](ProjectModelTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for GoogleIamV1__TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs test iam permissions projects](ProjectJobTestIamPermissionCall) (response)
/// * [models test iam permissions projects](ProjectModelTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleIamV1__TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for GoogleIamV1__TestIamPermissionsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [operations list projects](ProjectOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunning__ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<GoogleLongrunning__Operation>>,
}

impl client::ResponseResult for GoogleLongrunning__ListOperationsResponse {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations studies trials check early stopping state projects](ProjectLocationStudyTrialCheckEarlyStoppingStateCall) (response)
/// * [locations studies trials suggest projects](ProjectLocationStudyTrialSuggestCall) (response)
/// * [models versions create projects](ProjectModelVersionCreateCall) (response)
/// * [models versions delete projects](ProjectModelVersionDeleteCall) (response)
/// * [models versions patch projects](ProjectModelVersionPatchCall) (response)
/// * [models delete projects](ProjectModelDeleteCall) (response)
/// * [models patch projects](ProjectModelPatchCall) (response)
/// * [operations get projects](ProjectOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleLongrunning__Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpc__Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for GoogleLongrunning__Operation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [jobs cancel projects](ProjectJobCancelCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations studies trials delete projects](ProjectLocationStudyTrialDeleteCall) (response)
/// * [locations studies delete projects](ProjectLocationStudyDeleteCall) (response)
/// * [operations cancel projects](ProjectOperationCancelCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobuf__Empty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobuf__Empty {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpc__Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpc__Status {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleType__Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for GoogleType__Expr {}


