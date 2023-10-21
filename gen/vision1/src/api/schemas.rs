use super::*;
/// Request message for the `AddProductToProductSet` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations product sets add product projects](ProjectLocationProductSetAddProductCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddProductToProductSetRequest {
    /// Required. The resource name for the Product to be added to this ProductSet. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    
    pub product: Option<String>,
}

impl client::RequestValue for AddProductToProductSetRequest {}


/// A request to annotate one single file, e.g. a PDF, TIFF or GIF file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateFileRequest {
    /// Required. Requested features.
    
    pub features: Option<Vec<Feature>>,
    /// Additional context that may accompany the image(s) in the file.
    #[serde(rename="imageContext")]
    
    pub image_context: Option<ImageContext>,
    /// Required. Information about the input file.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<InputConfig>,
    /// Pages of the file to perform image annotation. Pages starts from 1, we assume the first page of the file is page 1. At most 5 pages are supported per request. Pages can be negative. Page 1 means the first page. Page 2 means the second page. Page -1 means the last page. Page -2 means the second to the last page. If the file is GIF instead of PDF or TIFF, page refers to GIF frames. If this field is empty, by default the service performs image annotation for the first 5 pages of the file.
    
    pub pages: Option<Vec<i32>>,
}

impl client::Part for AnnotateFileRequest {}


/// Response to a single file annotation request. A file may contain one or more images, which individually have their own responses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateFileResponse {
    /// If set, represents the error message for the failed request. The `responses` field will not be set in this case.
    
    pub error: Option<Status>,
    /// Information about the file for which this response is generated.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<InputConfig>,
    /// Individual responses to images found within the file. This field will be empty if the `error` field is set.
    
    pub responses: Option<Vec<AnnotateImageResponse>>,
    /// This field gives the total number of pages in the file.
    #[serde(rename="totalPages")]
    
    pub total_pages: Option<i32>,
}

impl client::Part for AnnotateFileResponse {}


/// Request for performing Google Cloud Vision API tasks over a user-provided image, with user-requested features, and with context information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateImageRequest {
    /// Requested features.
    
    pub features: Option<Vec<Feature>>,
    /// The image to be processed.
    
    pub image: Option<Image>,
    /// Additional context that may accompany the image.
    #[serde(rename="imageContext")]
    
    pub image_context: Option<ImageContext>,
}

impl client::Part for AnnotateImageRequest {}


/// Response to an image annotation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateImageResponse {
    /// If present, contextual information is needed to understand where this image comes from.
    
    pub context: Option<ImageAnnotationContext>,
    /// If present, crop hints have completed successfully.
    #[serde(rename="cropHintsAnnotation")]
    
    pub crop_hints_annotation: Option<CropHintsAnnotation>,
    /// If set, represents the error message for the operation. Note that filled-in image annotations are guaranteed to be correct, even when `error` is set.
    
    pub error: Option<Status>,
    /// If present, face detection has completed successfully.
    #[serde(rename="faceAnnotations")]
    
    pub face_annotations: Option<Vec<FaceAnnotation>>,
    /// If present, text (OCR) detection or document (OCR) text detection has completed successfully. This annotation provides the structural hierarchy for the OCR detected text.
    #[serde(rename="fullTextAnnotation")]
    
    pub full_text_annotation: Option<TextAnnotation>,
    /// If present, image properties were extracted successfully.
    #[serde(rename="imagePropertiesAnnotation")]
    
    pub image_properties_annotation: Option<ImageProperties>,
    /// If present, label detection has completed successfully.
    #[serde(rename="labelAnnotations")]
    
    pub label_annotations: Option<Vec<EntityAnnotation>>,
    /// If present, landmark detection has completed successfully.
    #[serde(rename="landmarkAnnotations")]
    
    pub landmark_annotations: Option<Vec<EntityAnnotation>>,
    /// If present, localized object detection has completed successfully. This will be sorted descending by confidence score.
    #[serde(rename="localizedObjectAnnotations")]
    
    pub localized_object_annotations: Option<Vec<LocalizedObjectAnnotation>>,
    /// If present, logo detection has completed successfully.
    #[serde(rename="logoAnnotations")]
    
    pub logo_annotations: Option<Vec<EntityAnnotation>>,
    /// If present, product search has completed successfully.
    #[serde(rename="productSearchResults")]
    
    pub product_search_results: Option<ProductSearchResults>,
    /// If present, safe-search annotation has completed successfully.
    #[serde(rename="safeSearchAnnotation")]
    
    pub safe_search_annotation: Option<SafeSearchAnnotation>,
    /// If present, text (OCR) detection has completed successfully.
    #[serde(rename="textAnnotations")]
    
    pub text_annotations: Option<Vec<EntityAnnotation>>,
    /// If present, web detection has completed successfully.
    #[serde(rename="webDetection")]
    
    pub web_detection: Option<WebDetection>,
}

impl client::Part for AnnotateImageResponse {}


/// An offline file annotation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsyncAnnotateFileRequest {
    /// Required. Requested features.
    
    pub features: Option<Vec<Feature>>,
    /// Additional context that may accompany the image(s) in the file.
    #[serde(rename="imageContext")]
    
    pub image_context: Option<ImageContext>,
    /// Required. Information about the input file.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<InputConfig>,
    /// Required. The desired output location and metadata (e.g. format).
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<OutputConfig>,
}

impl client::Part for AsyncAnnotateFileRequest {}


/// Multiple async file annotation requests are batched into a single service call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [async batch annotate files](FileAsyncBatchAnnotateCall) (request)
/// * [files async batch annotate projects](ProjectFileAsyncBatchAnnotateCall) (request)
/// * [locations files async batch annotate projects](ProjectLocationFileAsyncBatchAnnotateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsyncBatchAnnotateFilesRequest {
    /// Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`.
    
    pub parent: Option<String>,
    /// Required. Individual async file annotation requests for this batch.
    
    pub requests: Option<Vec<AsyncAnnotateFileRequest>>,
}

impl client::RequestValue for AsyncBatchAnnotateFilesRequest {}


/// Request for async image annotation for a list of images.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [async batch annotate images](ImageAsyncBatchAnnotateCall) (request)
/// * [images async batch annotate projects](ProjectImageAsyncBatchAnnotateCall) (request)
/// * [locations images async batch annotate projects](ProjectLocationImageAsyncBatchAnnotateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AsyncBatchAnnotateImagesRequest {
    /// Required. The desired output location and metadata (e.g. format).
    #[serde(rename="outputConfig")]
    
    pub output_config: Option<OutputConfig>,
    /// Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`.
    
    pub parent: Option<String>,
    /// Required. Individual image annotation requests for this batch.
    
    pub requests: Option<Vec<AnnotateImageRequest>>,
}

impl client::RequestValue for AsyncBatchAnnotateImagesRequest {}


/// A list of requests to annotate files using the BatchAnnotateFiles API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate files](FileAnnotateCall) (request)
/// * [files annotate projects](ProjectFileAnnotateCall) (request)
/// * [locations files annotate projects](ProjectLocationFileAnnotateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAnnotateFilesRequest {
    /// Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`.
    
    pub parent: Option<String>,
    /// Required. The list of file annotation requests. Right now we support only one AnnotateFileRequest in BatchAnnotateFilesRequest.
    
    pub requests: Option<Vec<AnnotateFileRequest>>,
}

impl client::RequestValue for BatchAnnotateFilesRequest {}


/// A list of file annotation responses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate files](FileAnnotateCall) (response)
/// * [files annotate projects](ProjectFileAnnotateCall) (response)
/// * [locations files annotate projects](ProjectLocationFileAnnotateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAnnotateFilesResponse {
    /// The list of file annotation responses, each response corresponding to each AnnotateFileRequest in BatchAnnotateFilesRequest.
    
    pub responses: Option<Vec<AnnotateFileResponse>>,
}

impl client::ResponseResult for BatchAnnotateFilesResponse {}


/// Multiple image annotation requests are batched into a single service call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate images](ImageAnnotateCall) (request)
/// * [images annotate projects](ProjectImageAnnotateCall) (request)
/// * [locations images annotate projects](ProjectLocationImageAnnotateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAnnotateImagesRequest {
    /// Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`.
    
    pub parent: Option<String>,
    /// Required. Individual image annotation requests for this batch.
    
    pub requests: Option<Vec<AnnotateImageRequest>>,
}

impl client::RequestValue for BatchAnnotateImagesRequest {}


/// Response to a batch image annotation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate images](ImageAnnotateCall) (response)
/// * [images annotate projects](ProjectImageAnnotateCall) (response)
/// * [locations images annotate projects](ProjectLocationImageAnnotateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAnnotateImagesResponse {
    /// Individual responses to image annotation requests within the batch.
    
    pub responses: Option<Vec<AnnotateImageResponse>>,
}

impl client::ResponseResult for BatchAnnotateImagesResponse {}


/// Logical element on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    /// Detected block type (text, image etc) for this block.
    #[serde(rename="blockType")]
    
    pub block_type: Option<String>,
    /// The bounding box for the block. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    
    pub bounding_box: Option<BoundingPoly>,
    /// Confidence of the OCR results on the block. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// List of paragraphs in this block (if this blocks is of type text).
    
    pub paragraphs: Option<Vec<Paragraph>>,
    /// Additional information detected for the block.
    
    pub property: Option<TextProperty>,
}

impl client::Part for Block {}


/// A bounding polygon for the detected image annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BoundingPoly {
    /// The bounding polygon normalized vertices.
    #[serde(rename="normalizedVertices")]
    
    pub normalized_vertices: Option<Vec<NormalizedVertex>>,
    /// The bounding polygon vertices.
    
    pub vertices: Option<Vec<Vertex>>,
}

impl client::Part for BoundingPoly {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](OperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Represents a color in the RGBA color space. This representation is designed for simplicity of conversion to/from color representations in various languages over compactness. For example, the fields of this representation can be trivially provided to the constructor of `java.awt.Color` in Java; it can also be trivially provided to UIColor's `+colorWithRed:green:blue:alpha` method in iOS; and, with just a little work, it can be easily formatted into a CSS `rgba()` string in JavaScript. This reference page doesn't carry information about the absolute color space that should be used to interpret the RGB value (e.g. sRGB, Adobe RGB, DCI-P3, BT.2020, etc.). By default, applications should assume the sRGB color space. When color equality needs to be decided, implementations, unless documented otherwise, treat two colors as equal if all their red, green, blue, and alpha values each differ by at most 1e-5. Example (Java): import com.google.type.Color; // ... public static java.awt.Color fromProto(Color protocolor) { float alpha = protocolor.hasAlpha() ? protocolor.getAlpha().getValue() : 1.0; return new java.awt.Color( protocolor.getRed(), protocolor.getGreen(), protocolor.getBlue(), alpha); } public static Color toProto(java.awt.Color color) { float red = (float) color.getRed(); float green = (float) color.getGreen(); float blue = (float) color.getBlue(); float denominator = 255.0; Color.Builder resultBuilder = Color .newBuilder() .setRed(red / denominator) .setGreen(green / denominator) .setBlue(blue / denominator); int alpha = color.getAlpha(); if (alpha != 255) { result.setAlpha( FloatValue .newBuilder() .setValue(((float) alpha) / denominator) .build()); } return resultBuilder.build(); } // ... Example (iOS / Obj-C): // ... static UIColor* fromProto(Color* protocolor) { float red = [protocolor red]; float green = [protocolor green]; float blue = [protocolor blue]; FloatValue* alpha_wrapper = [protocolor alpha]; float alpha = 1.0; if (alpha_wrapper != nil) { alpha = [alpha_wrapper value]; } return [UIColor colorWithRed:red green:green blue:blue alpha:alpha]; } static Color* toProto(UIColor* color) { CGFloat red, green, blue, alpha; if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) { return nil; } Color* result = [[Color alloc] init]; [result setRed:red]; [result setGreen:green]; [result setBlue:blue]; if (alpha <= 0.9999) { [result setAlpha:floatWrapperWithValue(alpha)]; } [result autorelease]; return result; } // ... Example (JavaScript): // ... var protoToCssColor = function(rgb_color) { var redFrac = rgb_color.red || 0.0; var greenFrac = rgb_color.green || 0.0; var blueFrac = rgb_color.blue || 0.0; var red = Math.floor(redFrac * 255); var green = Math.floor(greenFrac * 255); var blue = Math.floor(blueFrac * 255); if (!('alpha' in rgb_color)) { return rgbToCssColor(red, green, blue); } var alphaFrac = rgb_color.alpha.value || 0.0; var rgbParams = [red, green, blue].join(','); return ['rgba(', rgbParams, ',', alphaFrac, ')'].join(''); }; var rgbToCssColor = function(red, green, blue) { var rgbNumber = new Number((red << 16) | (green << 8) | blue); var hexString = rgbNumber.toString(16); var missingZeros = 6 - hexString.length; var resultBuilder = ['#']; for (var i = 0; i < missingZeros; i++) { resultBuilder.push('0'); } resultBuilder.push(hexString); return resultBuilder.join(''); }; // ...
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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


/// Color information consists of RGB channels, score, and the fraction of the image that the color occupies in the image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColorInfo {
    /// RGB components of the color.
    
    pub color: Option<Color>,
    /// The fraction of pixels the color occupies in the image. Value in range [0, 1].
    #[serde(rename="pixelFraction")]
    
    pub pixel_fraction: Option<f32>,
    /// Image-specific score for this color. Value in range [0, 1].
    
    pub score: Option<f32>,
}

impl client::Part for ColorInfo {}


/// Single crop hint that is used to generate a new crop when serving an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropHint {
    /// The bounding polygon for the crop region. The coordinates of the bounding box are in the original image's scale.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<BoundingPoly>,
    /// Confidence of this being a salient region. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// Fraction of importance of this salient region with respect to the original image.
    #[serde(rename="importanceFraction")]
    
    pub importance_fraction: Option<f32>,
}

impl client::Part for CropHint {}


/// Set of crop hints that are used to generate new crops when serving images.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropHintsAnnotation {
    /// Crop hint results.
    #[serde(rename="cropHints")]
    
    pub crop_hints: Option<Vec<CropHint>>,
}

impl client::Part for CropHintsAnnotation {}


/// Parameters for crop hints annotation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropHintsParams {
    /// Aspect ratios in floats, representing the ratio of the width to the height of the image. For example, if the desired aspect ratio is 4/3, the corresponding float value should be 1.33333. If not specified, the best possible crop is returned. The number of provided aspect ratios is limited to a maximum of 16; any aspect ratios provided after the 16th are ignored.
    #[serde(rename="aspectRatios")]
    
    pub aspect_ratios: Option<Vec<f32>>,
}

impl client::Part for CropHintsParams {}


/// Detected start or end of a structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectedBreak {
    /// True if break prepends the element.
    #[serde(rename="isPrefix")]
    
    pub is_prefix: Option<bool>,
    /// Detected break type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for DetectedBreak {}


/// Detected language for a structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectedLanguage {
    /// Confidence of detected language. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for DetectedLanguage {}


/// Set of dominant colors and their corresponding scores.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DominantColorsAnnotation {
    /// RGB color values with their score and pixel fraction.
    
    pub colors: Option<Vec<ColorInfo>>,
}

impl client::Part for DominantColorsAnnotation {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](OperationCancelCall) (response)
/// * [delete operations](OperationDeleteCall) (response)
/// * [locations product sets add product projects](ProjectLocationProductSetAddProductCall) (response)
/// * [locations product sets delete projects](ProjectLocationProductSetDeleteCall) (response)
/// * [locations product sets remove product projects](ProjectLocationProductSetRemoveProductCall) (response)
/// * [locations products reference images delete projects](ProjectLocationProductReferenceImageDeleteCall) (response)
/// * [locations products delete projects](ProjectLocationProductDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// Set of detected entity features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityAnnotation {
    /// Image region to which this entity belongs. Not produced for `LABEL_DETECTION` features.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<BoundingPoly>,
    /// **Deprecated. Use `score` instead.** The accuracy of the entity detection in an image. For example, for an image in which the "Eiffel Tower" entity is detected, this field represents the confidence that there is a tower in the query image. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// Entity textual description, expressed in its `locale` language.
    
    pub description: Option<String>,
    /// The language code for the locale in which the entity textual `description` is expressed.
    
    pub locale: Option<String>,
    /// The location information for the detected entity. Multiple `LocationInfo` elements can be present because one location may indicate the location of the scene in the image, and another location may indicate the location of the place where the image was taken. Location information is usually present for landmarks.
    
    pub locations: Option<Vec<LocationInfo>>,
    /// Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/).
    
    pub mid: Option<String>,
    /// Some entities may have optional user-supplied `Property` (name/value) fields, such a score or string that qualifies the entity.
    
    pub properties: Option<Vec<Property>>,
    /// Overall score of the result. Range [0, 1].
    
    pub score: Option<f32>,
    /// The relevancy of the ICA (Image Content Annotation) label to the image. For example, the relevancy of "tower" is likely higher to an image containing the detected "Eiffel Tower" than to an image containing a detected distant towering building, even though the confidence that there is a tower in each image may be the same. Range [0, 1].
    
    pub topicality: Option<f32>,
}

impl client::Part for EntityAnnotation {}


/// A face annotation object contains the results of face detection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FaceAnnotation {
    /// Anger likelihood.
    #[serde(rename="angerLikelihood")]
    
    pub anger_likelihood: Option<String>,
    /// Blurred likelihood.
    #[serde(rename="blurredLikelihood")]
    
    pub blurred_likelihood: Option<String>,
    /// The bounding polygon around the face. The coordinates of the bounding box are in the original image's scale. The bounding box is computed to "frame" the face in accordance with human expectations. It is based on the landmarker results. Note that one or more x and/or y coordinates may not be generated in the `BoundingPoly` (the polygon will be unbounded) if only a partial face appears in the image to be annotated.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<BoundingPoly>,
    /// Detection confidence. Range [0, 1].
    #[serde(rename="detectionConfidence")]
    
    pub detection_confidence: Option<f32>,
    /// The `fd_bounding_poly` bounding polygon is tighter than the `boundingPoly`, and encloses only the skin part of the face. Typically, it is used to eliminate the face from any image analysis that detects the "amount of skin" visible in an image. It is not based on the landmarker results, only on the initial face detection, hence the fd (face detection) prefix.
    #[serde(rename="fdBoundingPoly")]
    
    pub fd_bounding_poly: Option<BoundingPoly>,
    /// Headwear likelihood.
    #[serde(rename="headwearLikelihood")]
    
    pub headwear_likelihood: Option<String>,
    /// Joy likelihood.
    #[serde(rename="joyLikelihood")]
    
    pub joy_likelihood: Option<String>,
    /// Face landmarking confidence. Range [0, 1].
    #[serde(rename="landmarkingConfidence")]
    
    pub landmarking_confidence: Option<f32>,
    /// Detected face landmarks.
    
    pub landmarks: Option<Vec<Landmark>>,
    /// Yaw angle, which indicates the leftward/rightward angle that the face is pointing relative to the vertical plane perpendicular to the image. Range [-180,180].
    #[serde(rename="panAngle")]
    
    pub pan_angle: Option<f32>,
    /// Roll angle, which indicates the amount of clockwise/anti-clockwise rotation of the face relative to the image vertical about the axis perpendicular to the face. Range [-180,180].
    #[serde(rename="rollAngle")]
    
    pub roll_angle: Option<f32>,
    /// Sorrow likelihood.
    #[serde(rename="sorrowLikelihood")]
    
    pub sorrow_likelihood: Option<String>,
    /// Surprise likelihood.
    #[serde(rename="surpriseLikelihood")]
    
    pub surprise_likelihood: Option<String>,
    /// Pitch angle, which indicates the upwards/downwards angle that the face is pointing relative to the image's horizontal plane. Range [-180,180].
    #[serde(rename="tiltAngle")]
    
    pub tilt_angle: Option<f32>,
    /// Under-exposed likelihood.
    #[serde(rename="underExposedLikelihood")]
    
    pub under_exposed_likelihood: Option<String>,
}

impl client::Part for FaceAnnotation {}


/// The type of Google Cloud Vision API detection to perform, and the maximum number of results to return for that type. Multiple `Feature` objects can be specified in the `features` list.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Feature {
    /// Maximum number of results of this type. Does not apply to `TEXT_DETECTION`, `DOCUMENT_TEXT_DETECTION`, or `CROP_HINTS`.
    #[serde(rename="maxResults")]
    
    pub max_results: Option<i32>,
    /// Model to use for the feature. Supported values: "builtin/stable" (the default if unset) and "builtin/latest". `DOCUMENT_TEXT_DETECTION` and `TEXT_DETECTION` also support "builtin/weekly" for the bleeding edge release updated weekly.
    
    pub model: Option<String>,
    /// The feature type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Feature {}


/// The Google Cloud Storage location where the output will be written to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsDestination {
    /// Google Cloud Storage URI prefix where the results will be stored. Results will be in JSON format and preceded by its corresponding input URI prefix. This field can either represent a gcs file prefix or gcs directory. In either case, the uri should be unique because in order to get all of the output files, you will need to do a wildcard gcs search on the uri prefix you provide. Examples: * File Prefix: gs://bucket-name/here/filenameprefix The output files will be created in gs://bucket-name/here/ and the names of the output files will begin with "filenameprefix". * Directory Prefix: gs://bucket-name/some/location/ The output files will be created in gs://bucket-name/some/location/ and the names of the output files could be anything because there was no filename prefix specified. If multiple outputs, each response is still AnnotateFileResponse, each of which contains some subset of the full list of AnnotateImageResponse. Multiple outputs can happen if, for example, the output JSON is too large and overflows into multiple sharded files.
    
    pub uri: Option<String>,
}

impl client::Part for GcsDestination {}


/// The Google Cloud Storage location where the input will be read from.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GcsSource {
    /// Google Cloud Storage URI for the input file. This must only be a Google Cloud Storage object. Wildcards are not currently supported.
    
    pub uri: Option<String>,
}

impl client::Part for GcsSource {}


/// Information about the products similar to a single product in a query image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupedResult {
    /// The bounding polygon around the product detected in the query image.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<BoundingPoly>,
    /// List of generic predictions for the object in the bounding box.
    #[serde(rename="objectAnnotations")]
    
    pub object_annotations: Option<Vec<ObjectAnnotation>>,
    /// List of results, one for each product match.
    
    pub results: Option<Vec<Result>>,
}

impl client::Part for GroupedResult {}


/// Client image to perform Google Cloud Vision API tasks over.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate images](ImageAnnotateCall) (none)
/// * [async batch annotate images](ImageAsyncBatchAnnotateCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// Image content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateImages requests. It does not work for AsyncBatchAnnotateImages requests.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// Google Cloud Storage image location, or publicly-accessible image URL. If both `content` and `source` are provided for an image, `content` takes precedence and is used to perform the image annotation request.
    
    pub source: Option<ImageSource>,
}

impl client::Resource for Image {}


/// If an image was produced from a file (e.g. a PDF), this message gives information about the source of that image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageAnnotationContext {
    /// If the file was a PDF or TIFF, this field gives the page number within the file used to produce the image.
    #[serde(rename="pageNumber")]
    
    pub page_number: Option<i32>,
    /// The URI of the file used to produce the image.
    
    pub uri: Option<String>,
}

impl client::Part for ImageAnnotationContext {}


/// Image context and/or feature-specific parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageContext {
    /// Parameters for crop hints annotation request.
    #[serde(rename="cropHintsParams")]
    
    pub crop_hints_params: Option<CropHintsParams>,
    /// List of languages to use for TEXT_DETECTION. In most cases, an empty value yields the best results since it enables automatic language detection. For languages based on the Latin alphabet, setting `language_hints` is not needed. In rare cases, when the language of the text in the image is known, setting a hint will help get better results (although it will be a significant hindrance if the hint is wrong). Text detection returns an error if one or more of the specified languages is not one of the [supported languages](https://cloud.google.com/vision/docs/languages).
    #[serde(rename="languageHints")]
    
    pub language_hints: Option<Vec<String>>,
    /// Not used.
    #[serde(rename="latLongRect")]
    
    pub lat_long_rect: Option<LatLongRect>,
    /// Parameters for product search.
    #[serde(rename="productSearchParams")]
    
    pub product_search_params: Option<ProductSearchParams>,
    /// Parameters for text detection and document text detection.
    #[serde(rename="textDetectionParams")]
    
    pub text_detection_params: Option<TextDetectionParams>,
    /// Parameters for web detection.
    #[serde(rename="webDetectionParams")]
    
    pub web_detection_params: Option<WebDetectionParams>,
}

impl client::Part for ImageContext {}


/// Stores image properties, such as dominant colors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageProperties {
    /// If present, dominant colors completed successfully.
    #[serde(rename="dominantColors")]
    
    pub dominant_colors: Option<DominantColorsAnnotation>,
}

impl client::Part for ImageProperties {}


/// External image source (Google Cloud Storage or web URL image location).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageSource {
    /// **Use `image_uri` instead.** The Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info.
    #[serde(rename="gcsImageUri")]
    
    pub gcs_image_uri: Option<String>,
    /// The URI of the source image. Can be either: 1. A Google Cloud Storage URI of the form `gs://bucket_name/object_name`. Object versioning is not supported. See [Google Cloud Storage Request URIs](https://cloud.google.com/storage/docs/reference-uris) for more info. 2. A publicly-accessible image HTTP/HTTPS URL. When fetching images from HTTP/HTTPS URLs, Google cannot guarantee that the request will be completed. Your request may fail if the specified host denies the request (e.g. due to request throttling or DOS prevention), or if Google throttles requests to the site for abuse prevention. You should not depend on externally-hosted images for production applications. When both `gcs_image_uri` and `image_uri` are specified, `image_uri` takes precedence.
    #[serde(rename="imageUri")]
    
    pub image_uri: Option<String>,
}

impl client::Part for ImageSource {}


/// The Google Cloud Storage location for a csv file which preserves a list of ImportProductSetRequests in each line.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportProductSetsGcsSource {
    /// The Google Cloud Storage URI of the input csv file. The URI must start with `gs://`. The format of the input csv file should be one image per line. In each line, there are 8 columns. 1. image-uri 2. image-id 3. product-set-id 4. product-id 5. product-category 6. product-display-name 7. labels 8. bounding-poly The `image-uri`, `product-set-id`, `product-id`, and `product-category` columns are required. All other columns are optional. If the `ProductSet` or `Product` specified by the `product-set-id` and `product-id` values does not exist, then the system will create a new `ProductSet` or `Product` for the image. In this case, the `product-display-name` column refers to display_name, the `product-category` column refers to product_category, and the `labels` column refers to product_labels. The `image-id` column is optional but must be unique if provided. If it is empty, the system will automatically assign a unique id to the image. The `product-display-name` column is optional. If it is empty, the system sets the display_name field for the product to a space (" "). You can update the `display_name` later by using the API. If a `Product` with the specified `product-id` already exists, then the system ignores the `product-display-name`, `product-category`, and `labels` columns. The `labels` column (optional) is a line containing a list of comma-separated key-value pairs, in the following format: "key_1=value_1,key_2=value_2,...,key_n=value_n" The `bounding-poly` column (optional) identifies one region of interest from the image in the same manner as `CreateReferenceImage`. If you do not specify the `bounding-poly` column, then the system will try to detect regions of interest automatically. At most one `bounding-poly` column is allowed per line. If the image contains multiple regions of interest, add a line to the CSV file that includes the same product information, and the `bounding-poly` values for each region of interest. The `bounding-poly` column must contain an even number of comma-separated numbers, in the format "p1_x,p1_y,p2_x,p2_y,...,pn_x,pn_y". Use non-negative integers for absolute bounding polygons, and float values in [0, 1] for normalized bounding polygons. The system will resize the image if the image resolution is too large to process (larger than 20MP).
    #[serde(rename="csvFileUri")]
    
    pub csv_file_uri: Option<String>,
}

impl client::Part for ImportProductSetsGcsSource {}


/// The input content for the `ImportProductSets` method.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportProductSetsInputConfig {
    /// The Google Cloud Storage location for a csv file which preserves a list of ImportProductSetRequests in each line.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<ImportProductSetsGcsSource>,
}

impl client::Part for ImportProductSetsInputConfig {}


/// Request message for the `ImportProductSets` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations product sets import projects](ProjectLocationProductSetImportCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImportProductSetsRequest {
    /// Required. The input content for the list of requests.
    #[serde(rename="inputConfig")]
    
    pub input_config: Option<ImportProductSetsInputConfig>,
}

impl client::RequestValue for ImportProductSetsRequest {}


/// The desired input location and metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InputConfig {
    /// File content, represented as a stream of bytes. Note: As with all `bytes` fields, protobuffers use a pure binary representation, whereas JSON representations use base64. Currently, this field only works for BatchAnnotateFiles requests. It does not work for AsyncBatchAnnotateFiles requests.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub content: Option<Vec<u8>>,
    /// The Google Cloud Storage location to read the input from.
    #[serde(rename="gcsSource")]
    
    pub gcs_source: Option<GcsSource>,
    /// The type of the file. Currently only "application/pdf", "image/tiff" and "image/gif" are supported. Wildcards are not supported.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
}

impl client::Part for InputConfig {}


/// A product label represented as a key-value pair.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KeyValue {
    /// The key of the label attached to the product. Cannot be empty and cannot exceed 128 bytes.
    
    pub key: Option<String>,
    /// The value of the label attached to the product. Cannot be empty and cannot exceed 128 bytes.
    
    pub value: Option<String>,
}

impl client::Part for KeyValue {}


/// A face-specific landmark (for example, a face feature).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Landmark {
    /// Face landmark position.
    
    pub position: Option<Position>,
    /// Face landmark type.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for Landmark {}


/// An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this object must conform to the WGS84 standard. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    
    pub longitude: Option<f64>,
}

impl client::Part for LatLng {}


/// Rectangle determined by min and max `LatLng` pairs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLongRect {
    /// Max lat/long pair.
    #[serde(rename="maxLatLng")]
    
    pub max_lat_lng: Option<LatLng>,
    /// Min lat/long pair.
    #[serde(rename="minLatLng")]
    
    pub min_lat_lng: Option<LatLng>,
}

impl client::Part for LatLongRect {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list operations](OperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// Response message for the `ListProductSets` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations product sets list projects](ProjectLocationProductSetListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProductSetsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of ProductSets.
    #[serde(rename="productSets")]
    
    pub product_sets: Option<Vec<ProductSet>>,
}

impl client::ResponseResult for ListProductSetsResponse {}


/// Response message for the `ListProductsInProductSet` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations product sets products list projects](ProjectLocationProductSetProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProductsInProductSetResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of Products.
    
    pub products: Option<Vec<Product>>,
}

impl client::ResponseResult for ListProductsInProductSetResponse {}


/// Response message for the `ListProducts` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations products list projects](ProjectLocationProductListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProductsResponse {
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// List of products.
    
    pub products: Option<Vec<Product>>,
}

impl client::ResponseResult for ListProductsResponse {}


/// Response message for the `ListReferenceImages` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations products reference images list projects](ProjectLocationProductReferenceImageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListReferenceImagesResponse {
    /// The next_page_token returned from a previous List request, if any.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The maximum number of items to return. Default 10, maximum 100.
    #[serde(rename="pageSize")]
    
    pub page_size: Option<i32>,
    /// The list of reference images.
    #[serde(rename="referenceImages")]
    
    pub reference_images: Option<Vec<ReferenceImage>>,
}

impl client::ResponseResult for ListReferenceImagesResponse {}


/// Set of detected objects with bounding boxes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedObjectAnnotation {
    /// Image region to which this object belongs. This must be populated.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<BoundingPoly>,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Object ID that should align with EntityAnnotation mid.
    
    pub mid: Option<String>,
    /// Object name, expressed in its `language_code` language.
    
    pub name: Option<String>,
    /// Score of the result. Range [0, 1].
    
    pub score: Option<f32>,
}

impl client::Part for LocalizedObjectAnnotation {}


/// Detected entity location information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationInfo {
    /// lat/long location coordinates.
    #[serde(rename="latLng")]
    
    pub lat_lng: Option<LatLng>,
}

impl client::Part for LocationInfo {}


/// A vertex represents a 2D point in the image. NOTE: the normalized vertex coordinates are relative to the original image and range from 0 to 1.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NormalizedVertex {
    /// X coordinate.
    
    pub x: Option<f32>,
    /// Y coordinate.
    
    pub y: Option<f32>,
}

impl client::Part for NormalizedVertex {}


/// Prediction for what the object in the bounding box is.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAnnotation {
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// Object ID that should align with EntityAnnotation mid.
    
    pub mid: Option<String>,
    /// Object name, expressed in its `language_code` language.
    
    pub name: Option<String>,
    /// Score of the result. Range [0, 1].
    
    pub score: Option<f32>,
}

impl client::Part for ObjectAnnotation {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [async batch annotate files](FileAsyncBatchAnnotateCall) (response)
/// * [async batch annotate images](ImageAsyncBatchAnnotateCall) (response)
/// * [operations get locations](LocationOperationGetCall) (response)
/// * [cancel operations](OperationCancelCall) (none)
/// * [delete operations](OperationDeleteCall) (none)
/// * [get operations](OperationGetCall) (response)
/// * [list operations](OperationListCall) (none)
/// * [files async batch annotate projects](ProjectFileAsyncBatchAnnotateCall) (response)
/// * [images async batch annotate projects](ProjectImageAsyncBatchAnnotateCall) (response)
/// * [locations files async batch annotate projects](ProjectLocationFileAsyncBatchAnnotateCall) (response)
/// * [locations images async batch annotate projects](ProjectLocationImageAsyncBatchAnnotateCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations product sets import projects](ProjectLocationProductSetImportCall) (response)
/// * [locations products purge projects](ProjectLocationProductPurgeCall) (response)
/// * [operations get projects](ProjectOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// The desired output location and metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    /// The max number of response protos to put into each output JSON file on Google Cloud Storage. The valid range is [1, 100]. If not specified, the default value is 20. For example, for one pdf file with 100 pages, 100 response protos will be generated. If `batch_size` = 20, then 5 json files each containing 20 response protos will be written under the prefix `gcs_destination`.`uri`. Currently, batch_size only applies to GcsDestination, with potential future support for other output configurations.
    #[serde(rename="batchSize")]
    
    pub batch_size: Option<i32>,
    /// The Google Cloud Storage location to write the output(s) to.
    #[serde(rename="gcsDestination")]
    
    pub gcs_destination: Option<GcsDestination>,
}

impl client::Part for OutputConfig {}


/// Detected page from OCR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    /// List of blocks of text, images etc on this page.
    
    pub blocks: Option<Vec<Block>>,
    /// Confidence of the OCR results on the page. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// Page height. For PDFs the unit is points. For images (including TIFFs) the unit is pixels.
    
    pub height: Option<i32>,
    /// Additional information detected on the page.
    
    pub property: Option<TextProperty>,
    /// Page width. For PDFs the unit is points. For images (including TIFFs) the unit is pixels.
    
    pub width: Option<i32>,
}

impl client::Part for Page {}


/// Structural unit of text representing a number of words in certain order.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Paragraph {
    /// The bounding box for the paragraph. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    
    pub bounding_box: Option<BoundingPoly>,
    /// Confidence of the OCR results for the paragraph. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// Additional information detected for the paragraph.
    
    pub property: Option<TextProperty>,
    /// List of all words in this paragraph.
    
    pub words: Option<Vec<Word>>,
}

impl client::Part for Paragraph {}


/// A 3D position in the image, used primarily for Face detection landmarks. A valid Position must have both x and y coordinates. The position coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate.
    
    pub x: Option<f32>,
    /// Y coordinate.
    
    pub y: Option<f32>,
    /// Z coordinate (or depth).
    
    pub z: Option<f32>,
}

impl client::Part for Position {}


/// A Product contains ReferenceImages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations products create projects](ProjectLocationProductCreateCall) (request|response)
/// * [locations products get projects](ProjectLocationProductGetCall) (response)
/// * [locations products patch projects](ProjectLocationProductPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// User-provided metadata to be stored with this product. Must be at most 4096 characters long.
    
    pub description: Option<String>,
    /// The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product.
    
    pub name: Option<String>,
    /// Immutable. The category for the product identified by the reference image. This should be one of "homegoods-v2", "apparel-v2", "toys-v2", "packagedgoods-v1" or "general-v1". The legacy categories "homegoods", "apparel", and "toys" are still supported, but these should not be used for new products.
    #[serde(rename="productCategory")]
    
    pub product_category: Option<String>,
    /// Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. "1199". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet.
    #[serde(rename="productLabels")]
    
    pub product_labels: Option<Vec<KeyValue>>,
}

impl client::RequestValue for Product {}
impl client::ResponseResult for Product {}


/// Parameters for a product search request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductSearchParams {
    /// The bounding polygon around the area of interest in the image. If it is not specified, system discretion will be applied.
    #[serde(rename="boundingPoly")]
    
    pub bounding_poly: Option<BoundingPoly>,
    /// The filtering expression. This can be used to restrict search results based on Product labels. We currently support an AND of OR of key-value expressions, where each expression within an OR must have the same key. An '=' should be used to connect the key and value. For example, "(color = red OR color = blue) AND brand = Google" is acceptable, but "(color = red OR brand = Google)" is not acceptable. "color: red" is not acceptable because it uses a ':' instead of an '='.
    
    pub filter: Option<String>,
    /// The list of product categories to search in. Currently, we only consider the first category, and either "homegoods-v2", "apparel-v2", "toys-v2", "packagedgoods-v1", or "general-v1" should be specified. The legacy categories "homegoods", "apparel", and "toys" are still supported but will be deprecated. For new products, please use "homegoods-v2", "apparel-v2", or "toys-v2" for better product search accuracy. It is recommended to migrate existing products to these categories as well.
    #[serde(rename="productCategories")]
    
    pub product_categories: Option<Vec<String>>,
    /// The resource name of a ProductSet to be searched for similar images. Format is: `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`.
    #[serde(rename="productSet")]
    
    pub product_set: Option<String>,
}

impl client::Part for ProductSearchParams {}


/// Results for a product search request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductSearchResults {
    /// Timestamp of the index which provided these results. Products added to the product set and products removed from the product set after this time are not reflected in the current results.
    #[serde(rename="indexTime")]
    
    pub index_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// List of results grouped by products detected in the query image. Each entry corresponds to one bounding polygon in the query image, and contains the matching products specific to that region. There may be duplicate product matches in the union of all the per-product results.
    #[serde(rename="productGroupedResults")]
    
    pub product_grouped_results: Option<Vec<GroupedResult>>,
    /// List of results, one for each product match.
    
    pub results: Option<Vec<Result>>,
}

impl client::Part for ProductSearchResults {}


/// A ProductSet contains Products. A ProductSet can contain a maximum of 1 million reference images. If the limit is exceeded, periodic indexing will fail.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations product sets create projects](ProjectLocationProductSetCreateCall) (request|response)
/// * [locations product sets get projects](ProjectLocationProductSetGetCall) (response)
/// * [locations product sets patch projects](ProjectLocationProductSetPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductSet {
    /// The user-provided name for this ProductSet. Must not be empty. Must be at most 4096 characters long.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. If there was an error with indexing the product set, the field is populated. This field is ignored when creating a ProductSet.
    #[serde(rename="indexError")]
    
    pub index_error: Option<Status>,
    /// Output only. The time at which this ProductSet was last indexed. Query results will reflect all updates before this time. If this ProductSet has never been indexed, this timestamp is the default value "1970-01-01T00:00:00Z". This field is ignored when creating a ProductSet.
    #[serde(rename="indexTime")]
    
    pub index_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The resource name of the ProductSet. Format is: `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`. This field is ignored when creating a ProductSet.
    
    pub name: Option<String>,
}

impl client::RequestValue for ProductSet {}
impl client::ResponseResult for ProductSet {}


/// Config to control which ProductSet contains the Products to be deleted.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductSetPurgeConfig {
    /// The ProductSet that contains the Products to delete. If a Product is a member of product_set_id in addition to other ProductSets, the Product will still be deleted.
    #[serde(rename="productSetId")]
    
    pub product_set_id: Option<String>,
}

impl client::Part for ProductSetPurgeConfig {}


/// A `Property` consists of a user-supplied name/value pair.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    /// Name of the property.
    
    pub name: Option<String>,
    /// Value of numeric properties.
    #[serde(rename="uint64Value")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub uint64_value: Option<u64>,
    /// Value of the property.
    
    pub value: Option<String>,
}

impl client::Part for Property {}


/// Request message for the `PurgeProducts` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations products purge projects](ProjectLocationProductPurgeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PurgeProductsRequest {
    /// If delete_orphan_products is true, all Products that are not in any ProductSet will be deleted.
    #[serde(rename="deleteOrphanProducts")]
    
    pub delete_orphan_products: Option<bool>,
    /// The default value is false. Override this value to true to actually perform the purge.
    
    pub force: Option<bool>,
    /// Specify which ProductSet contains the Products to be deleted.
    #[serde(rename="productSetPurgeConfig")]
    
    pub product_set_purge_config: Option<ProductSetPurgeConfig>,
}

impl client::RequestValue for PurgeProductsRequest {}


/// A `ReferenceImage` represents a product image and its associated metadata, such as bounding boxes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations products reference images create projects](ProjectLocationProductReferenceImageCreateCall) (request|response)
/// * [locations products reference images get projects](ProjectLocationProductReferenceImageGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReferenceImage {
    /// Optional. Bounding polygons around the areas of interest in the reference image. If this field is empty, the system will try to detect regions of interest. At most 10 bounding polygons will be used. The provided shape is converted into a non-rotated rectangle. Once converted, the small edge of the rectangle must be greater than or equal to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5 is not).
    #[serde(rename="boundingPolys")]
    
    pub bounding_polys: Option<Vec<BoundingPoly>>,
    /// The resource name of the reference image. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`. This field is ignored when creating a reference image.
    
    pub name: Option<String>,
    /// Required. The Google Cloud Storage URI of the reference image. The URI must start with `gs://`.
    
    pub uri: Option<String>,
}

impl client::RequestValue for ReferenceImage {}
impl client::ResponseResult for ReferenceImage {}


/// Request message for the `RemoveProductFromProductSet` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations product sets remove product projects](ProjectLocationProductSetRemoveProductCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveProductFromProductSetRequest {
    /// Required. The resource name for the Product to be removed from this ProductSet. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`
    
    pub product: Option<String>,
}

impl client::RequestValue for RemoveProductFromProductSetRequest {}


/// Information about a product.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Result {
    /// The resource name of the image from the product that is the closest match to the query.
    
    pub image: Option<String>,
    /// The Product.
    
    pub product: Option<Product>,
    /// A confidence level on the match, ranging from 0 (no confidence) to 1 (full confidence).
    
    pub score: Option<f32>,
}

impl client::Part for Result {}


/// Set of features pertaining to the image, computed by computer vision methods over safe-search verticals (for example, adult, spoof, medical, violence).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SafeSearchAnnotation {
    /// Represents the adult content likelihood for the image. Adult content may contain elements such as nudity, pornographic images or cartoons, or sexual activities.
    
    pub adult: Option<String>,
    /// Likelihood that this is a medical image.
    
    pub medical: Option<String>,
    /// Likelihood that the request image contains racy content. Racy content may include (but is not limited to) skimpy or sheer clothing, strategically covered nudity, lewd or provocative poses, or close-ups of sensitive body areas.
    
    pub racy: Option<String>,
    /// Spoof likelihood. The likelihood that an modification was made to the image's canonical version to make it appear funny or offensive.
    
    pub spoof: Option<String>,
    /// Likelihood that this image contains violent content.
    
    pub violence: Option<String>,
}

impl client::Part for SafeSearchAnnotation {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// A single symbol representation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Symbol {
    /// The bounding box for the symbol. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    
    pub bounding_box: Option<BoundingPoly>,
    /// Confidence of the OCR results for the symbol. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// Additional information detected for the symbol.
    
    pub property: Option<TextProperty>,
    /// The actual UTF-8 representation of the symbol.
    
    pub text: Option<String>,
}

impl client::Part for Symbol {}


/// TextAnnotation contains a structured representation of OCR extracted text. The hierarchy of an OCR extracted text structure is like this: TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol Each structural component, starting from Page, may further have their own properties. Properties describe detected languages, breaks etc.. Please refer to the TextAnnotation.TextProperty message definition below for more detail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextAnnotation {
    /// List of pages detected by OCR.
    
    pub pages: Option<Vec<Page>>,
    /// UTF-8 text detected on the pages.
    
    pub text: Option<String>,
}

impl client::Part for TextAnnotation {}


/// Parameters for text detections. This is used to control TEXT_DETECTION and DOCUMENT_TEXT_DETECTION features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextDetectionParams {
    /// A list of advanced OCR options to fine-tune OCR behavior.
    #[serde(rename="advancedOcrOptions")]
    
    pub advanced_ocr_options: Option<Vec<String>>,
    /// By default, Cloud Vision API only includes confidence score for DOCUMENT_TEXT_DETECTION result. Set the flag to true to include confidence score for TEXT_DETECTION as well.
    #[serde(rename="enableTextDetectionConfidenceScore")]
    
    pub enable_text_detection_confidence_score: Option<bool>,
}

impl client::Part for TextDetectionParams {}


/// Additional information detected on the structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextProperty {
    /// Detected start or end of a text segment.
    #[serde(rename="detectedBreak")]
    
    pub detected_break: Option<DetectedBreak>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    
    pub detected_languages: Option<Vec<DetectedLanguage>>,
}

impl client::Part for TextProperty {}


/// A vertex represents a 2D point in the image. NOTE: the vertex coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    /// X coordinate.
    
    pub x: Option<i32>,
    /// Y coordinate.
    
    pub y: Option<i32>,
}

impl client::Part for Vertex {}


/// Relevant information for the image from the Internet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebDetection {
    /// The service's best guess as to the topic of the request image. Inferred from similar images on the open web.
    #[serde(rename="bestGuessLabels")]
    
    pub best_guess_labels: Option<Vec<WebLabel>>,
    /// Fully matching images from the Internet. Can include resized copies of the query image.
    #[serde(rename="fullMatchingImages")]
    
    pub full_matching_images: Option<Vec<WebImage>>,
    /// Web pages containing the matching images from the Internet.
    #[serde(rename="pagesWithMatchingImages")]
    
    pub pages_with_matching_images: Option<Vec<WebPage>>,
    /// Partial matching images from the Internet. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops.
    #[serde(rename="partialMatchingImages")]
    
    pub partial_matching_images: Option<Vec<WebImage>>,
    /// The visually similar image results.
    #[serde(rename="visuallySimilarImages")]
    
    pub visually_similar_images: Option<Vec<WebImage>>,
    /// Deduced entities from similar images on the Internet.
    #[serde(rename="webEntities")]
    
    pub web_entities: Option<Vec<WebEntity>>,
}

impl client::Part for WebDetection {}


/// Parameters for web detection request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebDetectionParams {
    /// Whether to include results derived from the geo information in the image.
    #[serde(rename="includeGeoResults")]
    
    pub include_geo_results: Option<bool>,
}

impl client::Part for WebDetectionParams {}


/// Entity deduced from similar images on the Internet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebEntity {
    /// Canonical description of the entity, in English.
    
    pub description: Option<String>,
    /// Opaque entity ID.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
    /// Overall relevancy score for the entity. Not normalized and not comparable across different image queries.
    
    pub score: Option<f32>,
}

impl client::Part for WebEntity {}


/// Metadata for online images.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebImage {
    /// (Deprecated) Overall relevancy score for the image.
    
    pub score: Option<f32>,
    /// The result image URL.
    
    pub url: Option<String>,
}

impl client::Part for WebImage {}


/// Label to provide extra metadata for the web detection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebLabel {
    /// Label for extra metadata.
    
    pub label: Option<String>,
    /// The BCP-47 language code for `label`, such as "en-US" or "sr-Latn". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
}

impl client::Part for WebLabel {}


/// Metadata for web pages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebPage {
    /// Fully matching images on the page. Can include resized copies of the query image.
    #[serde(rename="fullMatchingImages")]
    
    pub full_matching_images: Option<Vec<WebImage>>,
    /// Title for the web page, may contain HTML markups.
    #[serde(rename="pageTitle")]
    
    pub page_title: Option<String>,
    /// Partial matching images on the page. Those images are similar enough to share some key-point features. For example an original image will likely have partial matching for its crops.
    #[serde(rename="partialMatchingImages")]
    
    pub partial_matching_images: Option<Vec<WebImage>>,
    /// (Deprecated) Overall relevancy score for the web page.
    
    pub score: Option<f32>,
    /// The result web page URL.
    
    pub url: Option<String>,
}

impl client::Part for WebPage {}


/// A word representation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Word {
    /// The bounding box for the word. The vertices are in the order of top-left, top-right, bottom-right, bottom-left. When a rotation of the bounding box is detected the rotation is represented as around the top-left corner as defined when the text is read in the 'natural' orientation. For example: * when the text is horizontal it might look like: 0----1 | | 3----2 * when it's rotated 180 degrees around the top-left corner it becomes: 2----3 | | 1----0 and the vertex order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    
    pub bounding_box: Option<BoundingPoly>,
    /// Confidence of the OCR results for the word. Range [0, 1].
    
    pub confidence: Option<f32>,
    /// Additional information detected for the word.
    
    pub property: Option<TextProperty>,
    /// List of symbols in the word. The order of the symbols follows the natural reading order.
    
    pub symbols: Option<Vec<Symbol>>,
}

impl client::Part for Word {}


