#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// View your circles and the people and pages in them
    PluCircleRead,

    /// View your basic profile info, including your age range and language
    PluLogin,

    /// Associate you with your personal info on Google
    PluMe,

    /// Send your photos and videos to Google+
    PluMediaUpload,

    /// View your own Google+ profile and profiles visible to you
    PluProfileRead,

    /// View your Google+ posts, comments, and stream
    PluStreamRead,

    /// View your email address
    UserinfoEmail,

    /// See your personal info, including any personal info you've made publicly available
    UserinfoProfile,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PluCircleRead => "https://www.googleapis.com/auth/plus.circles.read",
            Scope::PluLogin => "https://www.googleapis.com/auth/plus.login",
            Scope::PluMe => "https://www.googleapis.com/auth/plus.me",
            Scope::PluMediaUpload => "https://www.googleapis.com/auth/plus.media.upload",
            Scope::PluProfileRead => "https://www.googleapis.com/auth/plus.profiles.read",
            Scope::PluStreamRead => "https://www.googleapis.com/auth/plus.stream.read",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            Scope::UserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::PluMe
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all PlusDomains related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
/// use plusdomains1::{Result, Error};
/// # async fn dox() {
/// use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().list("activityId")
///              .sort_order("takimata")
///              .page_token("amet.")
///              .max_results(81)
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
pub struct PlusDomains<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for PlusDomains<C> {}

impl<'a, C> PlusDomains<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> PlusDomains<C> {
        PlusDomains {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://www.googleapis.com/plusDomains/v1/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn activities(&'a self) -> ActivityMethods<'a, C> {
        ActivityMethods { hub: self }
    }
    pub fn audiences(&'a self) -> AudienceMethods<'a, C> {
        AudienceMethods { hub: self }
    }
    pub fn circles(&'a self) -> CircleMethods<'a, C> {
        CircleMethods { hub: self }
    }
    pub fn comments(&'a self) -> CommentMethods<'a, C> {
        CommentMethods { hub: self }
    }
    pub fn media(&'a self) -> MediaMethods<'a, C> {
        MediaMethods { hub: self }
    }
    pub fn people(&'a self) -> PersonMethods<'a, C> {
        PersonMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/plusDomains/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Acl {
    /// Description of the access granted, suitable for display.
    pub description: Option<String>,
    /// Whether access is restricted to the domain.
    #[serde(rename = "domainRestricted")]
    pub domain_restricted: Option<bool>,
    /// The list of access entries.
    pub items: Option<Vec<PlusDomainsAclentryResource>>,
    /// Identifies this resource as a collection of access controls. Value: "plus#acl".
    pub kind: Option<String>,
}

impl common::Part for Acl {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get activities](ActivityGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Activity {
    /// Identifies who has access to see this activity.
    pub access: Option<Acl>,
    /// The person who performed this activity.
    pub actor: Option<ActivityActor>,
    /// Street address where this activity occurred.
    pub address: Option<String>,
    /// Additional content added by the person who shared this activity, applicable only when resharing an activity.
    pub annotation: Option<String>,
    /// If this activity is a crosspost from another system, this property specifies the ID of the original activity.
    #[serde(rename = "crosspostSource")]
    pub crosspost_source: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// Latitude and longitude where this activity occurred. Format is latitude followed by longitude, space separated.
    pub geocode: Option<String>,
    /// The ID of this activity.
    pub id: Option<String>,
    /// Identifies this resource as an activity. Value: "plus#activity".
    pub kind: Option<String>,
    /// The location where this activity occurred.
    pub location: Option<Place>,
    /// The object of this activity.
    pub object: Option<ActivityObject>,
    /// ID of the place where this activity occurred.
    #[serde(rename = "placeId")]
    pub place_id: Option<String>,
    /// Name of the place where this activity occurred.
    #[serde(rename = "placeName")]
    pub place_name: Option<String>,
    /// The service provider that initially published this activity.
    pub provider: Option<ActivityProvider>,
    /// The time at which this activity was initially published. Formatted as an RFC 3339 timestamp.
    pub published: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Radius, in meters, of the region where this activity occurred, centered at the latitude and longitude identified in geocode.
    pub radius: Option<String>,
    /// Title of this activity.
    pub title: Option<String>,
    /// The time at which this activity was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The link to this activity.
    pub url: Option<String>,
    /// This activity's verb, which indicates the action that was performed. Possible values include, but are not limited to, the following values:  
    /// - "post" - Publish content to the stream.
    /// - "share" - Reshare an activity.
    pub verb: Option<String>,
}

impl common::ResponseResult for Activity {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list activities](ActivityListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityFeed {
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The ID of this collection of activities. Deprecated.
    pub id: Option<String>,
    /// The activities in this page of results.
    pub items: Option<Vec<Activity>>,
    /// Identifies this resource as a collection of activities. Value: "plus#activityFeed".
    pub kind: Option<String>,
    /// Link to the next page of activities.
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// Link to this activity resource.
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    /// The title of this collection of activities, which is a truncated portion of the content.
    pub title: Option<String>,
    /// The time at which this collection of activities was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::ResponseResult for ActivityFeed {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list audiences](AudienceListCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Audience {
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The access control list entry.
    pub item: Option<PlusDomainsAclentryResource>,
    /// Identifies this resource as an audience. Value: "plus#audience".
    pub kind: Option<String>,
    /// The number of people in this circle. This only applies if entity_type is CIRCLE.
    #[serde(rename = "memberCount")]
    pub member_count: Option<u32>,
    /// The circle members' visibility as chosen by the owner of the circle. This only applies for items with "item.type" equals "circle". Possible values are:  
    /// - "public" - Members are visible to the public.
    /// - "limited" - Members are visible to a limited audience.
    /// - "private" - Members are visible to the owner only.
    pub visibility: Option<String>,
}

impl common::Resource for Audience {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list audiences](AudienceListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AudiencesFeed {
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The audiences in this result.
    pub items: Option<Vec<Audience>>,
    /// Identifies this resource as a collection of audiences. Value: "plus#audienceFeed".
    pub kind: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// The total number of ACL entries. The number of entries in this response may be smaller due to paging.
    #[serde(rename = "totalItems")]
    pub total_items: Option<i32>,
}

impl common::ResponseResult for AudiencesFeed {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list circles](CircleListCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Circle {
    /// The description of this circle.
    pub description: Option<String>,
    /// The circle name.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The ID of the circle.
    pub id: Option<String>,
    /// Identifies this resource as a circle. Value: "plus#circle".
    pub kind: Option<String>,
    /// The people in this circle.
    pub people: Option<CirclePeople>,
    /// Link to this circle resource
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
}

impl common::Resource for Circle {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list circles](CircleListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CircleFeed {
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The circles in this page of results.
    pub items: Option<Vec<Circle>>,
    /// Identifies this resource as a collection of circles. Value: "plus#circleFeed".
    pub kind: Option<String>,
    /// Link to the next page of circles.
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// Link to this page of circles.
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    /// The title of this list of resources.
    pub title: Option<String>,
    /// The total number of circles. The number of circles in this response may be smaller due to paging.
    #[serde(rename = "totalItems")]
    pub total_items: Option<i32>,
}

impl common::ResponseResult for CircleFeed {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get comments](CommentGetCall) (response)
/// * [list comments](CommentListCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Comment {
    /// The person who posted this comment.
    pub actor: Option<CommentActor>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The ID of this comment.
    pub id: Option<String>,
    /// The activity this comment replied to.
    #[serde(rename = "inReplyTo")]
    pub in_reply_to: Option<Vec<CommentInReplyTo>>,
    /// Identifies this resource as a comment. Value: "plus#comment".
    pub kind: Option<String>,
    /// The object of this comment.
    pub object: Option<CommentObject>,
    /// People who +1'd this comment.
    pub plusoners: Option<CommentPlusoners>,
    /// The time at which this comment was initially published. Formatted as an RFC 3339 timestamp.
    pub published: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Link to this comment resource.
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    /// The time at which this comment was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// This comment's verb, indicating what action was performed. Possible values are:  
    /// - "post" - Publish content to the stream.
    pub verb: Option<String>,
}

impl common::Resource for Comment {}
impl common::ResponseResult for Comment {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list comments](CommentListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentFeed {
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The ID of this collection of comments.
    pub id: Option<String>,
    /// The comments in this page of results.
    pub items: Option<Vec<Comment>>,
    /// Identifies this resource as a collection of comments. Value: "plus#commentFeed".
    pub kind: Option<String>,
    /// Link to the next page of activities.
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// The title of this collection of comments.
    pub title: Option<String>,
    /// The time at which this collection of comments was last updated. Formatted as an RFC 3339 timestamp.
    pub updated: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::ResponseResult for CommentFeed {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [insert media](MediaInsertCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Media {
    /// The person who uploaded this media.
    pub author: Option<MediaAuthor>,
    /// The display name for this media.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// Exif information of the media item.
    pub exif: Option<MediaExif>,
    /// The height in pixels of the original image.
    pub height: Option<u32>,
    /// ID of this media, which is generated by the API.
    pub id: Option<String>,
    /// The type of resource.
    pub kind: Option<String>,
    /// The time at which this media was originally created in UTC. Formatted as an RFC 3339 timestamp that matches this example: 2010-11-25T14:30:27.655Z
    #[serde(rename = "mediaCreatedTime")]
    pub media_created_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The URL of this photo or video's still image.
    #[serde(rename = "mediaUrl")]
    pub media_url: Option<String>,
    /// The time at which this media was uploaded. Formatted as an RFC 3339 timestamp.
    pub published: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The size in bytes of this video.
    #[serde(rename = "sizeBytes")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub size_bytes: Option<i64>,
    /// The list of video streams for this video. There might be several different streams available for a single video, either Flash or MPEG, of various sizes
    pub streams: Option<Vec<Videostream>>,
    /// A description, or caption, for this media.
    pub summary: Option<String>,
    /// The time at which this media was last updated. This includes changes to media metadata. Formatted as an RFC 3339 timestamp.
    pub updated: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The URL for the page that hosts this media.
    pub url: Option<String>,
    /// The duration in milliseconds of this video.
    #[serde(rename = "videoDuration")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub video_duration: Option<i64>,
    /// The encoding status of this video. Possible values are:  
    /// - "UPLOADING" - Not all the video bytes have been received.
    /// - "PENDING" - Video not yet processed.
    /// - "FAILED" - Video processing failed.
    /// - "READY" - A single video stream is playable.
    /// - "FINAL" - All video streams are playable.
    #[serde(rename = "videoStatus")]
    pub video_status: Option<String>,
    /// The width in pixels of the original image.
    pub width: Option<u32>,
}

impl common::RequestValue for Media {}
impl common::ResponseResult for Media {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list people](PersonListCall) (response)
/// * [list by activity people](PersonListByActivityCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PeopleFeed {
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The people in this page of results. Each item includes the id, displayName, image, and url for the person. To retrieve additional profile data, see the people.get method.
    pub items: Option<Vec<Person>>,
    /// Identifies this resource as a collection of people. Value: "plus#peopleFeed".
    pub kind: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// Link to this resource.
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    /// The title of this collection of people.
    pub title: Option<String>,
    /// The total number of people available in this list. The number of people in a response might be smaller due to paging. This might not be set for all collections.
    #[serde(rename = "totalItems")]
    pub total_items: Option<i32>,
}

impl common::ResponseResult for PeopleFeed {}

/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get people](PersonGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Person {
    /// A short biography for this person.
    #[serde(rename = "aboutMe")]
    pub about_me: Option<String>,
    /// The person's date of birth, represented as YYYY-MM-DD.
    pub birthday: Option<String>,
    /// The "bragging rights" line of this person.
    #[serde(rename = "braggingRights")]
    pub bragging_rights: Option<String>,
    /// For followers who are visible, the number of people who have added this person or page to a circle.
    #[serde(rename = "circledByCount")]
    pub circled_by_count: Option<i32>,
    /// The cover photo content.
    pub cover: Option<PersonCover>,
    /// (this field is not currently used)
    #[serde(rename = "currentLocation")]
    pub current_location: Option<String>,
    /// The name of this person, which is suitable for display.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The hosted domain name for the user's Google Apps account. For instance, example.com. The plus.profile.emails.read or email scope is needed to get this domain name.
    pub domain: Option<String>,
    /// A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address.
    pub emails: Option<Vec<PersonEmails>>,
    /// ETag of this response for caching purposes.
    pub etag: Option<String>,
    /// The person's gender. Possible values include, but are not limited to, the following values:  
    /// - "male" - Male gender.
    /// - "female" - Female gender.
    /// - "other" - Other.
    pub gender: Option<String>,
    /// The ID of this person.
    pub id: Option<String>,
    /// The representation of the person's profile photo.
    pub image: Option<PersonImage>,
    /// Whether this user has signed up for Google+.
    #[serde(rename = "isPlusUser")]
    pub is_plus_user: Option<bool>,
    /// Identifies this resource as a person. Value: "plus#person".
    pub kind: Option<String>,
    /// An object representation of the individual components of a person's name.
    pub name: Option<PersonName>,
    /// The nickname of this person.
    pub nickname: Option<String>,
    /// Type of person within Google+. Possible values include, but are not limited to, the following values:  
    /// - "person" - represents an actual person.
    /// - "page" - represents a page.
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    /// The occupation of this person.
    pub occupation: Option<String>,
    /// A list of current or past organizations with which this person is associated.
    pub organizations: Option<Vec<PersonOrganizations>>,
    /// A list of places where this person has lived.
    #[serde(rename = "placesLived")]
    pub places_lived: Option<Vec<PersonPlacesLived>>,
    /// If a Google+ Page, the number of people who have +1'd this page.
    #[serde(rename = "plusOneCount")]
    pub plus_one_count: Option<i32>,
    /// The person's relationship status. Possible values include, but are not limited to, the following values:  
    /// - "single" - Person is single.
    /// - "in_a_relationship" - Person is in a relationship.
    /// - "engaged" - Person is engaged.
    /// - "married" - Person is married.
    /// - "its_complicated" - The relationship is complicated.
    /// - "open_relationship" - Person is in an open relationship.
    /// - "widowed" - Person is widowed.
    /// - "in_domestic_partnership" - Person is in a domestic partnership.
    /// - "in_civil_union" - Person is in a civil union.
    #[serde(rename = "relationshipStatus")]
    pub relationship_status: Option<String>,
    /// The person's skills.
    pub skills: Option<String>,
    /// The brief description (tagline) of this person.
    pub tagline: Option<String>,
    /// The URL of this person's profile.
    pub url: Option<String>,
    /// A list of URLs for this person.
    pub urls: Option<Vec<PersonUrls>>,
    /// Whether the person or Google+ Page has been verified.
    pub verified: Option<bool>,
}

impl common::ResponseResult for Person {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Place {
    /// The physical address of the place.
    pub address: Option<PlaceAddress>,
    /// The display name of the place.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The id of the place.
    pub id: Option<String>,
    /// Identifies this resource as a place. Value: "plus#place".
    pub kind: Option<String>,
    /// The position of the place.
    pub position: Option<PlacePosition>,
}

impl common::Part for Place {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlusDomainsAclentryResource {
    /// A descriptive name for this entry. Suitable for display.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The ID of the entry. For entries of type "person" or "circle", this is the ID of the resource. For other types, this property is not set.
    pub id: Option<String>,
    /// The type of entry describing to whom access is granted. Possible values are:  
    /// - "person" - Access to an individual.
    /// - "circle" - Access to members of a circle.
    /// - "myCircles" - Access to members of all the person's circles.
    /// - "extendedCircles" - Access to members of all the person's circles, plus all of the people in their circles.
    /// - "domain" - Access to members of the person's Google Apps domain.
    /// - "public" - Access to anyone on the web.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl common::Part for PlusDomainsAclentryResource {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Videostream {
    /// The height, in pixels, of the video resource.
    pub height: Option<i32>,
    /// MIME type of the video stream.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// URL of the video stream.
    pub url: Option<String>,
    /// The width, in pixels, of the video resource.
    pub width: Option<i32>,
}

impl common::Part for Videostream {}

/// The person who performed this activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityActor {
    /// Actor info specific to particular clients.
    #[serde(rename = "clientSpecificActorInfo")]
    pub client_specific_actor_info: Option<ActivityActorClientSpecificActorInfo>,
    /// The name of the actor, suitable for display.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The ID of the actor's Person resource.
    pub id: Option<String>,
    /// The image representation of the actor.
    pub image: Option<ActivityActorImage>,
    /// An object representation of the individual components of name.
    pub name: Option<ActivityActorName>,
    /// The link to the actor's Google profile.
    pub url: Option<String>,
    /// Verification status of actor.
    pub verification: Option<ActivityActorVerification>,
}

impl common::NestedType for ActivityActor {}
impl common::Part for ActivityActor {}

/// Actor info specific to particular clients.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityActorClientSpecificActorInfo {
    /// Actor info specific to YouTube clients.
    #[serde(rename = "youtubeActorInfo")]
    pub youtube_actor_info: Option<ActivityActorClientSpecificActorInfoYoutubeActorInfo>,
}

impl common::NestedType for ActivityActorClientSpecificActorInfo {}
impl common::Part for ActivityActorClientSpecificActorInfo {}

/// Actor info specific to YouTube clients.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityActorClientSpecificActorInfoYoutubeActorInfo {
    /// ID of the YouTube channel owned by the Actor.
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
}

impl common::NestedType for ActivityActorClientSpecificActorInfoYoutubeActorInfo {}
impl common::Part for ActivityActorClientSpecificActorInfoYoutubeActorInfo {}

/// The image representation of the actor.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityActorImage {
    /// The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    pub url: Option<String>,
}

impl common::NestedType for ActivityActorImage {}
impl common::Part for ActivityActorImage {}

/// An object representation of the individual components of name.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityActorName {
    /// The family name ("last name") of the actor.
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
    /// The given name ("first name") of the actor.
    #[serde(rename = "givenName")]
    pub given_name: Option<String>,
}

impl common::NestedType for ActivityActorName {}
impl common::Part for ActivityActorName {}

/// Verification status of actor.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityActorVerification {
    /// Verification for one-time or manual processes.
    #[serde(rename = "adHocVerified")]
    pub ad_hoc_verified: Option<String>,
}

impl common::NestedType for ActivityActorVerification {}
impl common::Part for ActivityActorVerification {}

/// The object of this activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObject {
    /// If this activity's object is itself another activity, such as when a person reshares an activity, this property specifies the original activity's actor.
    pub actor: Option<ActivityObjectActor>,
    /// The media objects attached to this activity.
    pub attachments: Option<Vec<ActivityObjectAttachments>>,
    /// The HTML-formatted content, which is suitable for display.
    pub content: Option<String>,
    /// The ID of the object. When resharing an activity, this is the ID of the activity that is being reshared.
    pub id: Option<String>,
    /// The type of the object. Possible values include, but are not limited to, the following values:  
    /// - "note" - Textual content.
    /// - "activity" - A Google+ activity.
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    /// The content (text) as provided by the author, which is stored without any HTML formatting. When creating or updating an activity, this value must be supplied as plain text in the request.
    #[serde(rename = "originalContent")]
    pub original_content: Option<String>,
    /// People who +1'd this activity.
    pub plusoners: Option<ActivityObjectPlusoners>,
    /// Comments in reply to this activity.
    pub replies: Option<ActivityObjectReplies>,
    /// People who reshared this activity.
    pub resharers: Option<ActivityObjectResharers>,
    /// Status of the activity as seen by the viewer.
    #[serde(rename = "statusForViewer")]
    pub status_for_viewer: Option<ActivityObjectStatusForViewer>,
    /// The URL that points to the linked resource.
    pub url: Option<String>,
}

impl common::NestedType for ActivityObject {}
impl common::Part for ActivityObject {}

/// If this activity's object is itself another activity, such as when a person reshares an activity, this property specifies the original activity's actor.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectActor {
    /// Actor info specific to particular clients.
    #[serde(rename = "clientSpecificActorInfo")]
    pub client_specific_actor_info: Option<ActivityObjectActorClientSpecificActorInfo>,
    /// The original actor's name, which is suitable for display.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// ID of the original actor.
    pub id: Option<String>,
    /// The image representation of the original actor.
    pub image: Option<ActivityObjectActorImage>,
    /// A link to the original actor's Google profile.
    pub url: Option<String>,
    /// Verification status of actor.
    pub verification: Option<ActivityObjectActorVerification>,
}

impl common::NestedType for ActivityObjectActor {}
impl common::Part for ActivityObjectActor {}

/// Actor info specific to particular clients.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectActorClientSpecificActorInfo {
    /// Actor info specific to YouTube clients.
    #[serde(rename = "youtubeActorInfo")]
    pub youtube_actor_info: Option<ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo>,
}

impl common::NestedType for ActivityObjectActorClientSpecificActorInfo {}
impl common::Part for ActivityObjectActorClientSpecificActorInfo {}

/// Actor info specific to YouTube clients.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo {
    /// ID of the YouTube channel owned by the Actor.
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
}

impl common::NestedType for ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo {}
impl common::Part for ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo {}

/// The image representation of the original actor.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectActorImage {
    /// A URL that points to a thumbnail photo of the original actor.
    pub url: Option<String>,
}

impl common::NestedType for ActivityObjectActorImage {}
impl common::Part for ActivityObjectActorImage {}

/// Verification status of actor.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectActorVerification {
    /// Verification for one-time or manual processes.
    #[serde(rename = "adHocVerified")]
    pub ad_hoc_verified: Option<String>,
}

impl common::NestedType for ActivityObjectActorVerification {}
impl common::Part for ActivityObjectActorVerification {}

/// The media objects attached to this activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectAttachments {
    /// If the attachment is an article, this property contains a snippet of text from the article. It can also include descriptions for other types.
    pub content: Option<String>,
    /// The title of the attachment, such as a photo caption or an article title.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// If the attachment is a video, the embeddable link.
    pub embed: Option<ActivityObjectAttachmentsEmbed>,
    /// The full image URL for photo attachments.
    #[serde(rename = "fullImage")]
    pub full_image: Option<ActivityObjectAttachmentsFullImage>,
    /// The ID of the attachment.
    pub id: Option<String>,
    /// The preview image for photos or videos.
    pub image: Option<ActivityObjectAttachmentsImage>,
    /// The type of media object. Possible values include, but are not limited to, the following values:  
    /// - "photo" - A photo.
    /// - "album" - A photo album.
    /// - "video" - A video.
    /// - "article" - An article, specified by a link.
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    /// When previewing, these are the optional thumbnails for the post. When posting an article, choose one by setting the attachment.image.url property. If you don't choose one, one will be chosen for you.
    #[serde(rename = "previewThumbnails")]
    pub preview_thumbnails: Option<Vec<ActivityObjectAttachmentsPreviewThumbnails>>,
    /// If the attachment is an album, this property is a list of potential additional thumbnails from the album.
    pub thumbnails: Option<Vec<ActivityObjectAttachmentsThumbnails>>,
    /// The link to the attachment, which should be of type text/html.
    pub url: Option<String>,
}

impl common::NestedType for ActivityObjectAttachments {}
impl common::Part for ActivityObjectAttachments {}

/// If the attachment is a video, the embeddable link.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectAttachmentsEmbed {
    /// Media type of the link.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// URL of the link.
    pub url: Option<String>,
}

impl common::NestedType for ActivityObjectAttachmentsEmbed {}
impl common::Part for ActivityObjectAttachmentsEmbed {}

/// The full image URL for photo attachments.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectAttachmentsFullImage {
    /// The height, in pixels, of the linked resource.
    pub height: Option<u32>,
    /// Media type of the link.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// URL of the image.
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    pub width: Option<u32>,
}

impl common::NestedType for ActivityObjectAttachmentsFullImage {}
impl common::Part for ActivityObjectAttachmentsFullImage {}

/// The preview image for photos or videos.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectAttachmentsImage {
    /// The height, in pixels, of the linked resource.
    pub height: Option<u32>,
    /// Media type of the link.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Image URL.
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    pub width: Option<u32>,
}

impl common::NestedType for ActivityObjectAttachmentsImage {}
impl common::Part for ActivityObjectAttachmentsImage {}

/// When previewing, these are the optional thumbnails for the post. When posting an article, choose one by setting the attachment.image.url property. If you don't choose one, one will be chosen for you.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectAttachmentsPreviewThumbnails {
    /// URL of the thumbnail image.
    pub url: Option<String>,
}

impl common::NestedType for ActivityObjectAttachmentsPreviewThumbnails {}
impl common::Part for ActivityObjectAttachmentsPreviewThumbnails {}

/// If the attachment is an album, this property is a list of potential additional thumbnails from the album.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectAttachmentsThumbnails {
    /// Potential name of the thumbnail.
    pub description: Option<String>,
    /// Image resource.
    pub image: Option<ActivityObjectAttachmentsThumbnailsImage>,
    /// URL of the webpage containing the image.
    pub url: Option<String>,
}

impl common::NestedType for ActivityObjectAttachmentsThumbnails {}
impl common::Part for ActivityObjectAttachmentsThumbnails {}

/// Image resource.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectAttachmentsThumbnailsImage {
    /// The height, in pixels, of the linked resource.
    pub height: Option<u32>,
    /// Media type of the link.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Image url.
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    pub width: Option<u32>,
}

impl common::NestedType for ActivityObjectAttachmentsThumbnailsImage {}
impl common::Part for ActivityObjectAttachmentsThumbnailsImage {}

/// People who +1'd this activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectPlusoners {
    /// The URL for the collection of people who +1'd this activity.
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    /// Total number of people who +1'd this activity.
    #[serde(rename = "totalItems")]
    pub total_items: Option<u32>,
}

impl common::NestedType for ActivityObjectPlusoners {}
impl common::Part for ActivityObjectPlusoners {}

/// Comments in reply to this activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectReplies {
    /// The URL for the collection of comments in reply to this activity.
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    /// Total number of comments on this activity.
    #[serde(rename = "totalItems")]
    pub total_items: Option<u32>,
}

impl common::NestedType for ActivityObjectReplies {}
impl common::Part for ActivityObjectReplies {}

/// People who reshared this activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectResharers {
    /// The URL for the collection of resharers.
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
    /// Total number of people who reshared this activity.
    #[serde(rename = "totalItems")]
    pub total_items: Option<u32>,
}

impl common::NestedType for ActivityObjectResharers {}
impl common::Part for ActivityObjectResharers {}

/// Status of the activity as seen by the viewer.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityObjectStatusForViewer {
    /// Whether the viewer can comment on the activity.
    #[serde(rename = "canComment")]
    pub can_comment: Option<bool>,
    /// Whether the viewer can +1 the activity.
    #[serde(rename = "canPlusone")]
    pub can_plusone: Option<bool>,
    /// Whether the viewer can edit or delete the activity.
    #[serde(rename = "canUpdate")]
    pub can_update: Option<bool>,
    /// Whether the viewer has +1'd the activity.
    #[serde(rename = "isPlusOned")]
    pub is_plus_oned: Option<bool>,
    /// Whether reshares are disabled for the activity.
    #[serde(rename = "resharingDisabled")]
    pub resharing_disabled: Option<bool>,
}

impl common::NestedType for ActivityObjectStatusForViewer {}
impl common::Part for ActivityObjectStatusForViewer {}

/// The service provider that initially published this activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActivityProvider {
    /// Name of the service provider.
    pub title: Option<String>,
}

impl common::NestedType for ActivityProvider {}
impl common::Part for ActivityProvider {}

/// The people in this circle.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CirclePeople {
    /// The total number of people in this circle.
    #[serde(rename = "totalItems")]
    pub total_items: Option<u32>,
}

impl common::NestedType for CirclePeople {}
impl common::Part for CirclePeople {}

/// The person who posted this comment.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentActor {
    /// Actor info specific to particular clients.
    #[serde(rename = "clientSpecificActorInfo")]
    pub client_specific_actor_info: Option<CommentActorClientSpecificActorInfo>,
    /// The name of this actor, suitable for display.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// The ID of the actor.
    pub id: Option<String>,
    /// The image representation of this actor.
    pub image: Option<CommentActorImage>,
    /// A link to the Person resource for this actor.
    pub url: Option<String>,
    /// Verification status of actor.
    pub verification: Option<CommentActorVerification>,
}

impl common::NestedType for CommentActor {}
impl common::Part for CommentActor {}

/// Actor info specific to particular clients.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentActorClientSpecificActorInfo {
    /// Actor info specific to YouTube clients.
    #[serde(rename = "youtubeActorInfo")]
    pub youtube_actor_info: Option<CommentActorClientSpecificActorInfoYoutubeActorInfo>,
}

impl common::NestedType for CommentActorClientSpecificActorInfo {}
impl common::Part for CommentActorClientSpecificActorInfo {}

/// Actor info specific to YouTube clients.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentActorClientSpecificActorInfoYoutubeActorInfo {
    /// ID of the YouTube channel owned by the Actor.
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
}

impl common::NestedType for CommentActorClientSpecificActorInfoYoutubeActorInfo {}
impl common::Part for CommentActorClientSpecificActorInfoYoutubeActorInfo {}

/// The image representation of this actor.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentActorImage {
    /// The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    pub url: Option<String>,
}

impl common::NestedType for CommentActorImage {}
impl common::Part for CommentActorImage {}

/// Verification status of actor.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentActorVerification {
    /// Verification for one-time or manual processes.
    #[serde(rename = "adHocVerified")]
    pub ad_hoc_verified: Option<String>,
}

impl common::NestedType for CommentActorVerification {}
impl common::Part for CommentActorVerification {}

/// The activity this comment replied to.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentInReplyTo {
    /// The ID of the activity.
    pub id: Option<String>,
    /// The URL of the activity.
    pub url: Option<String>,
}

impl common::NestedType for CommentInReplyTo {}
impl common::Part for CommentInReplyTo {}

/// The object of this comment.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentObject {
    /// The HTML-formatted content, suitable for display.
    pub content: Option<String>,
    /// The object type of this comment. Possible values are:  
    /// - "comment" - A comment in reply to an activity.
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    /// The content (text) as provided by the author, stored without any HTML formatting. When creating or updating a comment, this value must be supplied as plain text in the request.
    #[serde(rename = "originalContent")]
    pub original_content: Option<String>,
}

impl common::NestedType for CommentObject {}
impl common::Part for CommentObject {}

/// People who +1'd this comment.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CommentPlusoners {
    /// Total number of people who +1'd this comment.
    #[serde(rename = "totalItems")]
    pub total_items: Option<u32>,
}

impl common::NestedType for CommentPlusoners {}
impl common::Part for CommentPlusoners {}

/// The person who uploaded this media.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MediaAuthor {
    /// The author's name.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// ID of the author.
    pub id: Option<String>,
    /// The author's Google profile image.
    pub image: Option<MediaAuthorImage>,
    /// A link to the author's Google profile.
    pub url: Option<String>,
}

impl common::NestedType for MediaAuthor {}
impl common::Part for MediaAuthor {}

/// The author's Google profile image.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MediaAuthorImage {
    /// The URL of the author's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    pub url: Option<String>,
}

impl common::NestedType for MediaAuthorImage {}
impl common::Part for MediaAuthorImage {}

/// Exif information of the media item.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MediaExif {
    /// The time the media was captured. Formatted as an RFC 3339 timestamp.
    pub time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::NestedType for MediaExif {}
impl common::Part for MediaExif {}

/// The cover photo content.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonCover {
    /// Extra information about the cover photo.
    #[serde(rename = "coverInfo")]
    pub cover_info: Option<PersonCoverCoverInfo>,
    /// The person's primary cover image.
    #[serde(rename = "coverPhoto")]
    pub cover_photo: Option<PersonCoverCoverPhoto>,
    /// The layout of the cover art. Possible values include, but are not limited to, the following values:  
    /// - "banner" - One large image banner.
    pub layout: Option<String>,
}

impl common::NestedType for PersonCover {}
impl common::Part for PersonCover {}

/// Extra information about the cover photo.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonCoverCoverInfo {
    /// The difference between the left position of the cover image and the actual displayed cover image. Only valid for banner layout.
    #[serde(rename = "leftImageOffset")]
    pub left_image_offset: Option<i32>,
    /// The difference between the top position of the cover image and the actual displayed cover image. Only valid for banner layout.
    #[serde(rename = "topImageOffset")]
    pub top_image_offset: Option<i32>,
}

impl common::NestedType for PersonCoverCoverInfo {}
impl common::Part for PersonCoverCoverInfo {}

/// The person's primary cover image.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonCoverCoverPhoto {
    /// The height of the image.
    pub height: Option<i32>,
    /// The URL of the image.
    pub url: Option<String>,
    /// The width of the image.
    pub width: Option<i32>,
}

impl common::NestedType for PersonCoverCoverPhoto {}
impl common::Part for PersonCoverCoverPhoto {}

/// A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonEmails {
    /// The type of address. Possible values include, but are not limited to, the following values:  
    /// - "account" - Google account email address.
    /// - "home" - Home email address.
    /// - "work" - Work email address.
    /// - "other" - Other.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The email address.
    pub value: Option<String>,
}

impl common::NestedType for PersonEmails {}
impl common::Part for PersonEmails {}

/// The representation of the person's profile photo.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonImage {
    /// Whether the person's profile photo is the default one
    #[serde(rename = "isDefault")]
    pub is_default: Option<bool>,
    /// The URL of the person's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    pub url: Option<String>,
}

impl common::NestedType for PersonImage {}
impl common::Part for PersonImage {}

/// An object representation of the individual components of a person's name.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonName {
    /// The family name (last name) of this person.
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
    /// The full name of this person, including middle names, suffixes, etc.
    pub formatted: Option<String>,
    /// The given name (first name) of this person.
    #[serde(rename = "givenName")]
    pub given_name: Option<String>,
    /// The honorific prefixes (such as "Dr." or "Mrs.") for this person.
    #[serde(rename = "honorificPrefix")]
    pub honorific_prefix: Option<String>,
    /// The honorific suffixes (such as "Jr.") for this person.
    #[serde(rename = "honorificSuffix")]
    pub honorific_suffix: Option<String>,
    /// The middle name of this person.
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
}

impl common::NestedType for PersonName {}
impl common::Part for PersonName {}

/// A list of current or past organizations with which this person is associated.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonOrganizations {
    /// The department within the organization. Deprecated.
    pub department: Option<String>,
    /// A short description of the person's role in this organization. Deprecated.
    pub description: Option<String>,
    /// The date that the person left this organization.
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    /// The location of this organization. Deprecated.
    pub location: Option<String>,
    /// The name of the organization.
    pub name: Option<String>,
    /// If "true", indicates this organization is the person's primary one, which is typically interpreted as the current one.
    pub primary: Option<bool>,
    /// The date that the person joined this organization.
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    /// The person's job title or role within the organization.
    pub title: Option<String>,
    /// The type of organization. Possible values include, but are not limited to, the following values:  
    /// - "work" - Work.
    /// - "school" - School.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl common::NestedType for PersonOrganizations {}
impl common::Part for PersonOrganizations {}

/// A list of places where this person has lived.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonPlacesLived {
    /// If "true", this place of residence is this person's primary residence.
    pub primary: Option<bool>,
    /// A place where this person has lived. For example: "Seattle, WA", "Near Toronto".
    pub value: Option<String>,
}

impl common::NestedType for PersonPlacesLived {}
impl common::Part for PersonPlacesLived {}

/// A list of URLs for this person.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PersonUrls {
    /// The label of the URL.
    pub label: Option<String>,
    /// The type of URL. Possible values include, but are not limited to, the following values:  
    /// - "otherProfile" - URL for another profile.
    /// - "contributor" - URL to a site for which this person is a contributor.
    /// - "website" - URL for this Google+ Page's primary website.
    /// - "other" - Other URL.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// The URL value.
    pub value: Option<String>,
}

impl common::NestedType for PersonUrls {}
impl common::Part for PersonUrls {}

/// The physical address of the place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlaceAddress {
    /// The formatted address for display.
    pub formatted: Option<String>,
}

impl common::NestedType for PlaceAddress {}
impl common::Part for PlaceAddress {}

/// The position of the place.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlacePosition {
    /// The latitude of this position.
    pub latitude: Option<f64>,
    /// The longitude of this position.
    pub longitude: Option<f64>,
}

impl common::NestedType for PlacePosition {}
impl common::Part for PlacePosition {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
///
/// # async fn dox() {
/// use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlusDomains::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
}

impl<'a, C> common::MethodsBuilder for ActivityMethods<'a, C> {}

impl<'a, C> ActivityMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get.
    pub fn get(&self, activity_id: &str) -> ActivityGetCall<'a, C> {
        ActivityGetCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get activities for. The special value "me" can be used to indicate the authenticated user.
    /// * `collection` - The collection of activities to list.
    pub fn list(&self, user_id: &str, collection: &str) -> ActivityListCall<'a, C> {
        ActivityListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *audience* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
///
/// # async fn dox() {
/// use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlusDomains::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.audiences();
/// # }
/// ```
pub struct AudienceMethods<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
}

impl<'a, C> common::MethodsBuilder for AudienceMethods<'a, C> {}

impl<'a, C> AudienceMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get audiences for. The special value "me" can be used to indicate the authenticated user.
    pub fn list(&self, user_id: &str) -> AudienceListCall<'a, C> {
        AudienceListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *circle* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
///
/// # async fn dox() {
/// use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlusDomains::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.circles();
/// # }
/// ```
pub struct CircleMethods<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
}

impl<'a, C> common::MethodsBuilder for CircleMethods<'a, C> {}

impl<'a, C> CircleMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `userId` - The ID of the user to get circles for. The special value "me" can be used to indicate the authenticated user.
    pub fn list(&self, user_id: &str) -> CircleListCall<'a, C> {
        CircleListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
///
/// # async fn dox() {
/// use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlusDomains::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
}

impl<'a, C> common::MethodsBuilder for CommentMethods<'a, C> {}

impl<'a, C> CommentMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `commentId` - The ID of the comment to get.
    pub fn get(&self, comment_id: &str) -> CommentGetCall<'a, C> {
        CommentGetCall {
            hub: self.hub,
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get comments for.
    pub fn list(&self, activity_id: &str) -> CommentListCall<'a, C> {
        CommentListCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _sort_order: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *media* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
///
/// # async fn dox() {
/// use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlusDomains::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.media();
/// # }
/// ```
pub struct MediaMethods<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
}

impl<'a, C> common::MethodsBuilder for MediaMethods<'a, C> {}

impl<'a, C> MediaMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The ID of the user to create the activity on behalf of.
    /// * `collection` - No description provided.
    pub fn insert(
        &self,
        request: Media,
        user_id: &str,
        collection: &str,
    ) -> MediaInsertCall<'a, C> {
        MediaInsertCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _collection: collection.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *person* resources.
/// It is not used directly, but through the [`PlusDomains`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_plusdomains1 as plusdomains1;
///
/// # async fn dox() {
/// use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http1()
///         .build()
/// );
/// let mut hub = PlusDomains::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `list(...)` and `list_by_activity(...)`
/// // to build up your call.
/// let rb = hub.people();
/// # }
/// ```
pub struct PersonMethods<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
}

impl<'a, C> common::MethodsBuilder for PersonMethods<'a, C> {}

impl<'a, C> PersonMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Get a person's profile.
    ///
    /// # Arguments
    ///
    /// * `userId` - The ID of the person to get the profile for. The special value "me" can be used to indicate the authenticated user.
    pub fn get(&self, user_id: &str) -> PersonGetCall<'a, C> {
        PersonGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// List all of the people in the specified collection.
    ///
    /// # Arguments
    ///
    /// * `userId` - Get the collection of people for the person identified. Use "me" to indicate the authenticated user.
    /// * `collection` - The collection of people to list.
    pub fn list(&self, user_id: &str, collection: &str) -> PersonListCall<'a, C> {
        PersonListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Shut down. See https://developers.google.com/+/api-shutdown for more details.
    ///
    /// # Arguments
    ///
    /// * `activityId` - The ID of the activity to get the list of people for.
    /// * `collection` - The collection of people to list.
    pub fn list_by_activity(
        &self,
        activity_id: &str,
        collection: &str,
    ) -> PersonListByActivityCall<'a, C> {
        PersonListByActivityCall {
            hub: self.hub,
            _activity_id: activity_id.to_string(),
            _collection: collection.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *get* method supported by a *activity* resource.
/// It is not used directly, but through a [`ActivityMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().get("activityId")
///              .doit().await;
/// # }
/// ```
pub struct ActivityGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _activity_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ActivityGetCall<'a, C> {}

impl<'a, C> ActivityGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Activity)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.activities.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "activityId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("activityId", self._activity_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "activities/{activityId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PluLogin.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{activityId}", "activityId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["activityId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the activity to get.
    ///
    /// Sets the *activity id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn activity_id(mut self, new_value: &str) -> ActivityGetCall<'a, C> {
        self._activity_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ActivityGetCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluLogin`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ActivityGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ActivityGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ActivityGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *list* method supported by a *activity* resource.
/// It is not used directly, but through a [`ActivityMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.activities().list("userId", "collection")
///              .page_token("gubergren")
///              .max_results(26)
///              .doit().await;
/// # }
/// ```
pub struct ActivityListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _user_id: String,
    _collection: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ActivityListCall<'a, C> {}

impl<'a, C> ActivityListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ActivityFeed)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.activities.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "userId", "collection", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("userId", self._user_id);
        params.push("collection", self._collection);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "people/{userId}/activities/{collection}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PluLogin.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{userId}", "userId"), ("{collection}", "collection")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["collection", "userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the user to get activities for. The special value "me" can be used to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> ActivityListCall<'a, C> {
        self._user_id = new_value.to_string();
        self
    }
    /// The collection of activities to list.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> ActivityListCall<'a, C> {
        self._collection = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ActivityListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of activities to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ActivityListCall<'a, C> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ActivityListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ActivityListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluLogin`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ActivityListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ActivityListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ActivityListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *list* method supported by a *audience* resource.
/// It is not used directly, but through a [`AudienceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.audiences().list("userId")
///              .page_token("ea")
///              .max_results(46)
///              .doit().await;
/// # }
/// ```
pub struct AudienceListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _user_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AudienceListCall<'a, C> {}

impl<'a, C> AudienceListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, AudiencesFeed)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.audiences.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "userId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("userId", self._user_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "people/{userId}/audiences";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PluCircleRead.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the user to get audiences for. The special value "me" can be used to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> AudienceListCall<'a, C> {
        self._user_id = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AudienceListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of circles to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> AudienceListCall<'a, C> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> AudienceListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> AudienceListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluCircleRead`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AudienceListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AudienceListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AudienceListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *list* method supported by a *circle* resource.
/// It is not used directly, but through a [`CircleMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.circles().list("userId")
///              .page_token("amet")
///              .max_results(81)
///              .doit().await;
/// # }
/// ```
pub struct CircleListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _user_id: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for CircleListCall<'a, C> {}

impl<'a, C> CircleListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, CircleFeed)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.circles.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "userId", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("userId", self._user_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "people/{userId}/circles";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PluCircleRead.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the user to get circles for. The special value "me" can be used to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> CircleListCall<'a, C> {
        self._user_id = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CircleListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of circles to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> CircleListCall<'a, C> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> CircleListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CircleListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluCircleRead`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CircleListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CircleListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CircleListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *get* method supported by a *comment* resource.
/// It is not used directly, but through a [`CommentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().get("commentId")
///              .doit().await;
/// # }
/// ```
pub struct CommentGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _comment_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for CommentGetCall<'a, C> {}

impl<'a, C> CommentGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Comment)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.comments.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "commentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("commentId", self._comment_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "comments/{commentId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PluLogin.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{commentId}", "commentId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["commentId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the comment to get.
    ///
    /// Sets the *comment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn comment_id(mut self, new_value: &str) -> CommentGetCall<'a, C> {
        self._comment_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> CommentGetCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CommentGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluLogin`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CommentGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CommentGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CommentGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *list* method supported by a *comment* resource.
/// It is not used directly, but through a [`CommentMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.comments().list("activityId")
///              .sort_order("ut")
///              .page_token("gubergren")
///              .max_results(85)
///              .doit().await;
/// # }
/// ```
pub struct CommentListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _activity_id: String,
    _sort_order: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for CommentListCall<'a, C> {}

impl<'a, C> CommentListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, CommentFeed)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.comments.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "activityId", "sortOrder", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("activityId", self._activity_id);
        if let Some(value) = self._sort_order.as_ref() {
            params.push("sortOrder", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "activities/{activityId}/comments";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PluLogin.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{activityId}", "activityId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["activityId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the activity to get comments for.
    ///
    /// Sets the *activity id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn activity_id(mut self, new_value: &str) -> CommentListCall<'a, C> {
        self._activity_id = new_value.to_string();
        self
    }
    /// The order in which to sort the list of comments.
    ///
    /// Sets the *sort order* query property to the given value.
    pub fn sort_order(mut self, new_value: &str) -> CommentListCall<'a, C> {
        self._sort_order = Some(new_value.to_string());
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CommentListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of comments to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> CommentListCall<'a, C> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> CommentListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> CommentListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluLogin`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> CommentListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> CommentListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> CommentListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *insert* method supported by a *media* resource.
/// It is not used directly, but through a [`MediaMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// use plusdomains1::api::Media;
/// use std::fs;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Media::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.media().insert(req, "userId", "collection")
///              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct MediaInsertCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _request: Media,
    _user_id: String,
    _collection: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for MediaInsertCall<'a, C> {}

impl<'a, C> MediaInsertCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    async fn doit<RS>(
        mut self,
        mut reader: RS,
        reader_mime_type: mime::Mime,
        protocol: common::UploadProtocol,
    ) -> common::Result<(common::Response, Media)>
    where
        RS: common::ReadSeek,
    {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.media.insert",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "userId", "collection"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("userId", self._user_id);
        params.push("collection", self._collection);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let (mut url, upload_type) = if protocol == common::UploadProtocol::Resumable {
            (
                self.hub._root_url.clone()
                    + "resumable/upload/plusDomains/v1/people/{userId}/media/{collection}",
                "resumable",
            )
        } else if protocol == common::UploadProtocol::Simple {
            (
                self.hub._root_url.clone()
                    + "upload/plusDomains/v1/people/{userId}/media/{collection}",
                "multipart",
            )
        } else {
            unreachable!()
        };
        params.push("uploadType", upload_type);
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PluLogin.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{userId}", "userId"), ("{collection}", "collection")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["collection", "userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        let mut upload_url_from_server;

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let mut mp_reader: common::MultiPartReader = Default::default();
                let (mut body_reader, content_type) = match protocol {
                    common::UploadProtocol::Simple => {
                        mp_reader.reserve_exact(2);
                        let size = reader.seek(std::io::SeekFrom::End(0)).unwrap();
                        reader.seek(std::io::SeekFrom::Start(0)).unwrap();

                        mp_reader
                            .add_part(
                                &mut request_value_reader,
                                request_size,
                                json_mime_type.clone(),
                            )
                            .add_part(&mut reader, size, reader_mime_type.clone());
                        (
                            &mut mp_reader as &mut (dyn std::io::Read + Send),
                            common::MultiPartReader::mime_type(),
                        )
                    }
                    _ => (
                        &mut request_value_reader as &mut (dyn std::io::Read + Send),
                        json_mime_type.clone(),
                    ),
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
                if protocol == common::UploadProtocol::Resumable {
                    req_builder = req_builder
                        .header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                }

                let mut body_reader_bytes = vec![];
                body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                let request = req_builder
                    .header(CONTENT_TYPE, content_type.to_string())
                    .body(common::to_body(body_reader_bytes.into()));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    if protocol == common::UploadProtocol::Resumable {
                        let size = reader.seek(std::io::SeekFrom::End(0)).unwrap();
                        reader.seek(std::io::SeekFrom::Start(0)).unwrap();

                        let upload_result = {
                            let url_str = &parts
                                .headers
                                .get("Location")
                                .expect("LOCATION header is part of protocol")
                                .to_str()
                                .unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            common::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server {
                                    Some(0)
                                } else {
                                    None
                                },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!(
                                    "Bearer {}",
                                    token
                                        .ok_or_else(|| common::Error::MissingToken(
                                            "resumable upload requires token".into()
                                        ))?
                                        .as_str()
                                ),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size,
                            }
                            .upload()
                            .await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(common::Error::Cancelled);
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(common::Error::HttpError(err));
                            }
                            Some(Ok(response)) => {
                                (parts, body) = response.into_parts();
                                if !parts.status.is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(common::Error::Failure(
                                        common::Response::from_parts(parts, body),
                                    ));
                                }
                            }
                        }
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
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
    /// * *valid mime types*: 'image/*' and 'video/*'
    pub async fn upload_resumable<RS>(
        self,
        resumeable_stream: RS,
        mime_type: mime::Mime,
    ) -> common::Result<(common::Response, Media)>
    where
        RS: common::ReadSeek,
    {
        self.doit(
            resumeable_stream,
            mime_type,
            common::UploadProtocol::Resumable,
        )
        .await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 0kb
    /// * *valid mime types*: 'image/*' and 'video/*'
    pub async fn upload<RS>(
        self,
        stream: RS,
        mime_type: mime::Mime,
    ) -> common::Result<(common::Response, Media)>
    where
        RS: common::ReadSeek,
    {
        self.doit(stream, mime_type, common::UploadProtocol::Simple)
            .await
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Media) -> MediaInsertCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The ID of the user to create the activity on behalf of.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> MediaInsertCall<'a, C> {
        self._user_id = new_value.to_string();
        self
    }
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> MediaInsertCall<'a, C> {
        self._collection = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> MediaInsertCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> MediaInsertCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluLogin`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MediaInsertCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MediaInsertCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MediaInsertCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Get a person's profile.
///
/// A builder for the *get* method supported by a *person* resource.
/// It is not used directly, but through a [`PersonMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().get("userId")
///              .doit().await;
/// # }
/// ```
pub struct PersonGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _user_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PersonGetCall<'a, C> {}

impl<'a, C> PersonGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Person)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.people.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "userId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("userId", self._user_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "people/{userId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PluLogin.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the person to get the profile for. The special value "me" can be used to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> PersonGetCall<'a, C> {
        self._user_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> PersonGetCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PersonGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluLogin`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PersonGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PersonGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PersonGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// List all of the people in the specified collection.
///
/// A builder for the *list* method supported by a *person* resource.
/// It is not used directly, but through a [`PersonMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().list("userId", "collection")
///              .page_token("ea")
///              .order_by("dolor")
///              .max_results(45)
///              .doit().await;
/// # }
/// ```
pub struct PersonListCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _user_id: String,
    _collection: String,
    _page_token: Option<String>,
    _order_by: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PersonListCall<'a, C> {}

impl<'a, C> PersonListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PeopleFeed)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.people.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "userId",
            "collection",
            "pageToken",
            "orderBy",
            "maxResults",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        params.push("userId", self._user_id);
        params.push("collection", self._collection);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "people/{userId}/people/{collection}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::PluCircleRead.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in
            [("{userId}", "userId"), ("{collection}", "collection")].iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["collection", "userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Get the collection of people for the person identified. Use "me" to indicate the authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> PersonListCall<'a, C> {
        self._user_id = new_value.to_string();
        self
    }
    /// The collection of people to list.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> PersonListCall<'a, C> {
        self._collection = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PersonListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The order to return people in.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> PersonListCall<'a, C> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The maximum number of people to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PersonListCall<'a, C> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> PersonListCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PersonListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluCircleRead`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PersonListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PersonListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PersonListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Shut down. See https://developers.google.com/+/api-shutdown for more details.
///
/// A builder for the *listByActivity* method supported by a *person* resource.
/// It is not used directly, but through a [`PersonMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_plusdomains1 as plusdomains1;
/// # async fn dox() {
/// # use plusdomains1::{PlusDomains, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = PlusDomains::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.people().list_by_activity("activityId", "collection")
///              .page_token("sed")
///              .max_results(31)
///              .doit().await;
/// # }
/// ```
pub struct PersonListByActivityCall<'a, C>
where
    C: 'a,
{
    hub: &'a PlusDomains<C>,
    _activity_id: String,
    _collection: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for PersonListByActivityCall<'a, C> {}

impl<'a, C> PersonListByActivityCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, PeopleFeed)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "plusDomains.people.listByActivity",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "activityId", "collection", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("activityId", self._activity_id);
        params.push("collection", self._collection);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "activities/{activityId}/people/{collection}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PluLogin.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{activityId}", "activityId"),
            ("{collection}", "collection"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["collection", "activityId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
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
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The ID of the activity to get the list of people for.
    ///
    /// Sets the *activity id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn activity_id(mut self, new_value: &str) -> PersonListByActivityCall<'a, C> {
        self._activity_id = new_value.to_string();
        self
    }
    /// The collection of people to list.
    ///
    /// Sets the *collection* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn collection(mut self, new_value: &str) -> PersonListByActivityCall<'a, C> {
        self._collection = new_value.to_string();
        self
    }
    /// The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of "nextPageToken" from the previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PersonListByActivityCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of people to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PersonListByActivityCall<'a, C> {
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
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> PersonListByActivityCall<'a, C> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> PersonListByActivityCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PluLogin`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PersonListByActivityCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PersonListByActivityCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PersonListByActivityCall<'a, C> {
        self._scopes.clear();
        self
    }
}
