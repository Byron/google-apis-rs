use super::*;
/// Information about the claim.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1Claim {
    /// The date that the claim was made.
    #[serde(rename="claimDate")]
    
    pub claim_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// One or more reviews of this claim (namely, a fact-checking article).
    #[serde(rename="claimReview")]
    
    pub claim_review: Option<Vec<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview>>,
    /// A person or organization stating the claim. For instance, "John Doe".
    
    pub claimant: Option<String>,
    /// The claim text. For instance, "Crime has doubled in the last 2 years."
    
    pub text: Option<String>,
}

impl client::Part for GoogleFactcheckingFactchecktoolsV1alpha1Claim {}


/// Information about the claim author.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor {
    /// Corresponds to `ClaimReview.itemReviewed.author.image`.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// Corresponds to `ClaimReview.itemReviewed.author.jobTitle`.
    #[serde(rename="jobTitle")]
    
    pub job_title: Option<String>,
    /// A person or organization stating the claim. For instance, "John Doe". Corresponds to `ClaimReview.itemReviewed.author.name`.
    
    pub name: Option<String>,
    /// Corresponds to `ClaimReview.itemReviewed.author.sameAs`.
    #[serde(rename="sameAs")]
    
    pub same_as: Option<String>,
}

impl client::Part for GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor {}


/// Information about the claim rating.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating {
    /// For numeric ratings, the best value possible in the scale from worst to best. Corresponds to `ClaimReview.reviewRating.bestRating`.
    #[serde(rename="bestRating")]
    
    pub best_rating: Option<i32>,
    /// Corresponds to `ClaimReview.reviewRating.image`.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// Corresponds to `ClaimReview.reviewRating.ratingExplanation`.
    #[serde(rename="ratingExplanation")]
    
    pub rating_explanation: Option<String>,
    /// A numeric rating of this claim, in the range worstRating — bestRating inclusive. Corresponds to `ClaimReview.reviewRating.ratingValue`.
    #[serde(rename="ratingValue")]
    
    pub rating_value: Option<i32>,
    /// The truthfulness rating as a human-readible short word or phrase. Corresponds to `ClaimReview.reviewRating.alternateName`.
    #[serde(rename="textualRating")]
    
    pub textual_rating: Option<String>,
    /// For numeric ratings, the worst value possible in the scale from worst to best. Corresponds to `ClaimReview.reviewRating.worstRating`.
    #[serde(rename="worstRating")]
    
    pub worst_rating: Option<i32>,
}

impl client::Part for GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating {}


/// Information about a claim review.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview {
    /// The language this review was written in. For instance, "en" or "de".
    #[serde(rename="languageCode")]
    
    pub language_code: Option<String>,
    /// The publisher of this claim review.
    
    pub publisher: Option<GoogleFactcheckingFactchecktoolsV1alpha1Publisher>,
    /// The date the claim was reviewed.
    #[serde(rename="reviewDate")]
    
    pub review_date: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Textual rating. For instance, "Mostly false".
    #[serde(rename="textualRating")]
    
    pub textual_rating: Option<String>,
    /// The title of this claim review, if it can be determined.
    
    pub title: Option<String>,
    /// The URL of this claim review.
    
    pub url: Option<String>,
}

impl client::Part for GoogleFactcheckingFactchecktoolsV1alpha1ClaimReview {}


/// Information about the claim review author.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor {
    /// Corresponds to `ClaimReview.author.image`.
    #[serde(rename="imageUrl")]
    
    pub image_url: Option<String>,
    /// Name of the organization that is publishing the fact check. Corresponds to `ClaimReview.author.name`.
    
    pub name: Option<String>,
}

impl client::Part for GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor {}


/// Fields for an individual `ClaimReview` element. Except for sub-messages that group fields together, each of these fields correspond those in https://schema.org/ClaimReview. We list the precise mapping for each field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup {
    /// A list of links to works in which this claim appears, aside from the one specified in `claim_first_appearance`. Corresponds to `ClaimReview.itemReviewed[@type=Claim].appearance.url`.
    #[serde(rename="claimAppearances")]
    
    pub claim_appearances: Option<Vec<String>>,
    /// Info about the author of this claim.
    #[serde(rename="claimAuthor")]
    
    pub claim_author: Option<GoogleFactcheckingFactchecktoolsV1alpha1ClaimAuthor>,
    /// The date when the claim was made or entered public discourse. Corresponds to `ClaimReview.itemReviewed.datePublished`.
    #[serde(rename="claimDate")]
    
    pub claim_date: Option<String>,
    /// A link to a work in which this claim first appears. Corresponds to `ClaimReview.itemReviewed[@type=Claim].firstAppearance.url`.
    #[serde(rename="claimFirstAppearance")]
    
    pub claim_first_appearance: Option<String>,
    /// The location where this claim was made. Corresponds to `ClaimReview.itemReviewed.name`.
    #[serde(rename="claimLocation")]
    
    pub claim_location: Option<String>,
    /// A short summary of the claim being evaluated. Corresponds to `ClaimReview.claimReviewed`.
    #[serde(rename="claimReviewed")]
    
    pub claim_reviewed: Option<String>,
    /// Info about the rating of this claim review.
    
    pub rating: Option<GoogleFactcheckingFactchecktoolsV1alpha1ClaimRating>,
    /// This field is optional, and will default to the page URL. We provide this field to allow you the override the default value, but the only permitted override is the page URL plus an optional anchor link ("page jump"). Corresponds to `ClaimReview.url`
    
    pub url: Option<String>,
}

impl client::Part for GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup {}


/// Holds one or more instances of `ClaimReview` markup for a webpage.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create pages](PageCreateCall) (request|response)
/// * [get pages](PageGetCall) (response)
/// * [update pages](PageUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage {
    /// Info about the author of this claim review. Similar to the above, semantically these are page-level fields, and each `ClaimReview` on this page will contain the same values.
    #[serde(rename="claimReviewAuthor")]
    
    pub claim_review_author: Option<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewAuthor>,
    /// A list of individual claim reviews for this page. Each item in the list corresponds to one `ClaimReview` element.
    #[serde(rename="claimReviewMarkups")]
    
    pub claim_review_markups: Option<Vec<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkup>>,
    /// The name of this `ClaimReview` markup page resource, in the form of `pages/{page_id}`. Except for update requests, this field is output-only and should not be set by the user.
    
    pub name: Option<String>,
    /// The URL of the page associated with this `ClaimReview` markup. While every individual `ClaimReview` has its own URL field, semantically this is a page-level field, and each `ClaimReview` on this page will use this value unless individually overridden. Corresponds to `ClaimReview.url`
    #[serde(rename="pageUrl")]
    
    pub page_url: Option<String>,
    /// The date when the fact check was published. Similar to the URL, semantically this is a page-level field, and each `ClaimReview` on this page will contain the same value. Corresponds to `ClaimReview.datePublished`
    #[serde(rename="publishDate")]
    
    pub publish_date: Option<String>,
    /// The version ID for this markup. Except for update requests, this field is output-only and should not be set by the user.
    #[serde(rename="versionId")]
    
    pub version_id: Option<String>,
}

impl client::RequestValue for GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage {}
impl client::ResponseResult for GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage {}


/// Response from searching fact-checked claims.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search claims](ClaimSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1FactCheckedClaimSearchResponse {
    /// The list of claims and all of their associated information.
    
    pub claims: Option<Vec<GoogleFactcheckingFactchecktoolsV1alpha1Claim>>,
    /// The next pagination token in the Search response. It should be used as the `page_token` for the following request. An empty value means no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleFactcheckingFactchecktoolsV1alpha1FactCheckedClaimSearchResponse {}


/// Response from listing `ClaimReview` markup.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list pages](PageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1ListClaimReviewMarkupPagesResponse {
    /// The result list of pages of `ClaimReview` markup.
    #[serde(rename="claimReviewMarkupPages")]
    
    pub claim_review_markup_pages: Option<Vec<GoogleFactcheckingFactchecktoolsV1alpha1ClaimReviewMarkupPage>>,
    /// The next pagination token in the Search response. It should be used as the `page_token` for the following request. An empty value means no more results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleFactcheckingFactchecktoolsV1alpha1ListClaimReviewMarkupPagesResponse {}


/// Information about the publisher.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFactcheckingFactchecktoolsV1alpha1Publisher {
    /// The name of this publisher. For instance, "Awesome Fact Checks".
    
    pub name: Option<String>,
    /// Host-level site name, without the protocol or "www" prefix. For instance, "awesomefactchecks.com". This value of this field is based purely on the claim review URL.
    
    pub site: Option<String>,
}

impl client::Part for GoogleFactcheckingFactchecktoolsV1alpha1Publisher {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete pages](PageDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleProtobufEmpty { _never_set: Option<bool> }

impl client::ResponseResult for GoogleProtobufEmpty {}


