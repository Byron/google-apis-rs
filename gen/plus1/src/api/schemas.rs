use super::*;
/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Acl {
    /// Description of the access granted, suitable for display.
    
    pub description: Option<String>,
    /// The list of access entries.
    
    pub items: Option<Vec<PlusAclentryResource>>,
    /// Identifies this resource as a collection of access controls. Value: "plus#acl".
    
    pub kind: Option<String>,
}

impl client::Part for Acl {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get activities](ActivityGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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
    #[serde(rename="crosspostSource")]
    
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
    #[serde(rename="placeId")]
    
    pub place_id: Option<String>,
    /// Name of the place where this activity occurred.
    #[serde(rename="placeName")]
    
    pub place_name: Option<String>,
    /// The service provider that initially published this activity.
    
    pub provider: Option<ActivityProvider>,
    /// The time at which this activity was initially published. Formatted as an RFC 3339 timestamp.
    
    pub published: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Radius, in meters, of the region where this activity occurred, centered at the latitude and longitude identified in geocode.
    
    pub radius: Option<String>,
    /// Title of this activity.
    
    pub title: Option<String>,
    /// The time at which this activity was last updated. Formatted as an RFC 3339 timestamp.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The link to this activity.
    
    pub url: Option<String>,
    /// This activity's verb, which indicates the action that was performed. Possible values include, but are not limited to, the following values:  
    /// - "post" - Publish content to the stream. 
    /// - "share" - Reshare an activity.
    
    pub verb: Option<String>,
}

impl client::ResponseResult for Activity {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list activities](ActivityListCall) (response)
/// * [search activities](ActivitySearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Link to this activity resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The title of this collection of activities, which is a truncated portion of the content.
    
    pub title: Option<String>,
    /// The time at which this collection of activities was last updated. Formatted as an RFC 3339 timestamp.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for ActivityFeed {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get comments](CommentGetCall) (response)
/// * [list comments](CommentListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// The person who posted this comment.
    
    pub actor: Option<CommentActor>,
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The ID of this comment.
    
    pub id: Option<String>,
    /// The activity this comment replied to.
    #[serde(rename="inReplyTo")]
    
    pub in_reply_to: Option<Vec<CommentInReplyTo>>,
    /// Identifies this resource as a comment. Value: "plus#comment".
    
    pub kind: Option<String>,
    /// The object of this comment.
    
    pub object: Option<CommentObject>,
    /// People who +1'd this comment.
    
    pub plusoners: Option<CommentPlusoners>,
    /// The time at which this comment was initially published. Formatted as an RFC 3339 timestamp.
    
    pub published: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Link to this comment resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The time at which this comment was last updated. Formatted as an RFC 3339 timestamp.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// This comment's verb, indicating what action was performed. Possible values are:  
    /// - "post" - Publish content to the stream.
    
    pub verb: Option<String>,
}

impl client::Resource for Comment {}
impl client::ResponseResult for Comment {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list comments](CommentListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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
    #[serde(rename="nextLink")]
    
    pub next_link: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The title of this collection of comments.
    
    pub title: Option<String>,
    /// The time at which this collection of comments was last updated. Formatted as an RFC 3339 timestamp.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::ResponseResult for CommentFeed {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list people](PersonListCall) (response)
/// * [list by activity people](PersonListByActivityCall) (response)
/// * [search people](PersonSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PeopleFeed {
    /// ETag of this response for caching purposes.
    
    pub etag: Option<String>,
    /// The people in this page of results. Each item includes the id, displayName, image, and url for the person. To retrieve additional profile data, see the people.get method.
    
    pub items: Option<Vec<Person>>,
    /// Identifies this resource as a collection of people. Value: "plus#peopleFeed".
    
    pub kind: Option<String>,
    /// The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Link to this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The title of this collection of people.
    
    pub title: Option<String>,
    /// The total number of people available in this list. The number of people in a response might be smaller due to paging. This might not be set for all collections.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::ResponseResult for PeopleFeed {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get people](PersonGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    /// A short biography for this person.
    #[serde(rename="aboutMe")]
    
    pub about_me: Option<String>,
    /// The age range of the person. Valid ranges are 17 or younger, 18 to 20, and 21 or older. Age is determined from the user's birthday using Western age reckoning.
    #[serde(rename="ageRange")]
    
    pub age_range: Option<PersonAgeRange>,
    /// The person's date of birth, represented as YYYY-MM-DD.
    
    pub birthday: Option<String>,
    /// The "bragging rights" line of this person.
    #[serde(rename="braggingRights")]
    
    pub bragging_rights: Option<String>,
    /// For followers who are visible, the number of people who have added this person or page to a circle.
    #[serde(rename="circledByCount")]
    
    pub circled_by_count: Option<i32>,
    /// The cover photo content.
    
    pub cover: Option<PersonCover>,
    /// (this field is not currently used)
    #[serde(rename="currentLocation")]
    
    pub current_location: Option<String>,
    /// The name of this person, which is suitable for display.
    #[serde(rename="displayName")]
    
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
    #[serde(rename="isPlusUser")]
    
    pub is_plus_user: Option<bool>,
    /// Identifies this resource as a person. Value: "plus#person".
    
    pub kind: Option<String>,
    /// The user's preferred language for rendering.
    
    pub language: Option<String>,
    /// An object representation of the individual components of a person's name.
    
    pub name: Option<PersonName>,
    /// The nickname of this person.
    
    pub nickname: Option<String>,
    /// Type of person within Google+. Possible values include, but are not limited to, the following values:  
    /// - "person" - represents an actual person. 
    /// - "page" - represents a page.
    #[serde(rename="objectType")]
    
    pub object_type: Option<String>,
    /// The occupation of this person.
    
    pub occupation: Option<String>,
    /// A list of current or past organizations with which this person is associated.
    
    pub organizations: Option<Vec<PersonOrganizations>>,
    /// A list of places where this person has lived.
    #[serde(rename="placesLived")]
    
    pub places_lived: Option<Vec<PersonPlacesLived>>,
    /// If a Google+ Page, the number of people who have +1'd this page.
    #[serde(rename="plusOneCount")]
    
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
    #[serde(rename="relationshipStatus")]
    
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

impl client::ResponseResult for Person {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Place {
    /// The physical address of the place.
    
    pub address: Option<PlaceAddress>,
    /// The display name of the place.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The id of the place.
    
    pub id: Option<String>,
    /// Identifies this resource as a place. Value: "plus#place".
    
    pub kind: Option<String>,
    /// The position of the place.
    
    pub position: Option<PlacePosition>,
}

impl client::Part for Place {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlusAclentryResource {
    /// A descriptive name for this entry. Suitable for display.
    #[serde(rename="displayName")]
    
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
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::Part for PlusAclentryResource {}


/// The person who performed this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActor {
    /// Actor info specific to particular clients.
    #[serde(rename="clientSpecificActorInfo")]
    
    pub client_specific_actor_info: Option<ActivityActorClientSpecificActorInfo>,
    /// The name of the actor, suitable for display.
    #[serde(rename="displayName")]
    
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

impl client::NestedType for ActivityActor {}
impl client::Part for ActivityActor {}


/// Actor info specific to particular clients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActorClientSpecificActorInfo {
    /// Actor info specific to YouTube clients.
    #[serde(rename="youtubeActorInfo")]
    
    pub youtube_actor_info: Option<ActivityActorClientSpecificActorInfoYoutubeActorInfo>,
}

impl client::NestedType for ActivityActorClientSpecificActorInfo {}
impl client::Part for ActivityActorClientSpecificActorInfo {}


/// Actor info specific to YouTube clients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActorClientSpecificActorInfoYoutubeActorInfo {
    /// ID of the YouTube channel owned by the Actor.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
}

impl client::NestedType for ActivityActorClientSpecificActorInfoYoutubeActorInfo {}
impl client::Part for ActivityActorClientSpecificActorInfoYoutubeActorInfo {}


/// The image representation of the actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActorImage {
    /// The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    
    pub url: Option<String>,
}

impl client::NestedType for ActivityActorImage {}
impl client::Part for ActivityActorImage {}


/// An object representation of the individual components of name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActorName {
    /// The family name ("last name") of the actor.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The given name ("first name") of the actor.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
}

impl client::NestedType for ActivityActorName {}
impl client::Part for ActivityActorName {}


/// Verification status of actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityActorVerification {
    /// Verification for one-time or manual processes.
    #[serde(rename="adHocVerified")]
    
    pub ad_hoc_verified: Option<String>,
}

impl client::NestedType for ActivityActorVerification {}
impl client::Part for ActivityActorVerification {}


/// The object of this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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
    #[serde(rename="objectType")]
    
    pub object_type: Option<String>,
    /// The content (text) as provided by the author, which is stored without any HTML formatting. When creating or updating an activity, this value must be supplied as plain text in the request.
    #[serde(rename="originalContent")]
    
    pub original_content: Option<String>,
    /// People who +1'd this activity.
    
    pub plusoners: Option<ActivityObjectPlusoners>,
    /// Comments in reply to this activity.
    
    pub replies: Option<ActivityObjectReplies>,
    /// People who reshared this activity.
    
    pub resharers: Option<ActivityObjectResharers>,
    /// The URL that points to the linked resource.
    
    pub url: Option<String>,
}

impl client::NestedType for ActivityObject {}
impl client::Part for ActivityObject {}


/// If this activity's object is itself another activity, such as when a person reshares an activity, this property specifies the original activity's actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectActor {
    /// Actor info specific to particular clients.
    #[serde(rename="clientSpecificActorInfo")]
    
    pub client_specific_actor_info: Option<ActivityObjectActorClientSpecificActorInfo>,
    /// The original actor's name, which is suitable for display.
    #[serde(rename="displayName")]
    
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

impl client::NestedType for ActivityObjectActor {}
impl client::Part for ActivityObjectActor {}


/// Actor info specific to particular clients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectActorClientSpecificActorInfo {
    /// Actor info specific to YouTube clients.
    #[serde(rename="youtubeActorInfo")]
    
    pub youtube_actor_info: Option<ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo>,
}

impl client::NestedType for ActivityObjectActorClientSpecificActorInfo {}
impl client::Part for ActivityObjectActorClientSpecificActorInfo {}


/// Actor info specific to YouTube clients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo {
    /// ID of the YouTube channel owned by the Actor.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
}

impl client::NestedType for ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo {}
impl client::Part for ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo {}


/// The image representation of the original actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectActorImage {
    /// A URL that points to a thumbnail photo of the original actor.
    
    pub url: Option<String>,
}

impl client::NestedType for ActivityObjectActorImage {}
impl client::Part for ActivityObjectActorImage {}


/// Verification status of actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectActorVerification {
    /// Verification for one-time or manual processes.
    #[serde(rename="adHocVerified")]
    
    pub ad_hoc_verified: Option<String>,
}

impl client::NestedType for ActivityObjectActorVerification {}
impl client::Part for ActivityObjectActorVerification {}


/// The media objects attached to this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachments {
    /// If the attachment is an article, this property contains a snippet of text from the article. It can also include descriptions for other types.
    
    pub content: Option<String>,
    /// The title of the attachment, such as a photo caption or an article title.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// If the attachment is a video, the embeddable link.
    
    pub embed: Option<ActivityObjectAttachmentsEmbed>,
    /// The full image URL for photo attachments.
    #[serde(rename="fullImage")]
    
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
    #[serde(rename="objectType")]
    
    pub object_type: Option<String>,
    /// If the attachment is an album, this property is a list of potential additional thumbnails from the album.
    
    pub thumbnails: Option<Vec<ActivityObjectAttachmentsThumbnails>>,
    /// The link to the attachment, which should be of type text/html.
    
    pub url: Option<String>,
}

impl client::NestedType for ActivityObjectAttachments {}
impl client::Part for ActivityObjectAttachments {}


/// If the attachment is a video, the embeddable link.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsEmbed {
    /// Media type of the link.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// URL of the link.
    
    pub url: Option<String>,
}

impl client::NestedType for ActivityObjectAttachmentsEmbed {}
impl client::Part for ActivityObjectAttachmentsEmbed {}


/// The full image URL for photo attachments.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsFullImage {
    /// The height, in pixels, of the linked resource.
    
    pub height: Option<u32>,
    /// Media type of the link.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// URL of the image.
    
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    
    pub width: Option<u32>,
}

impl client::NestedType for ActivityObjectAttachmentsFullImage {}
impl client::Part for ActivityObjectAttachmentsFullImage {}


/// The preview image for photos or videos.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsImage {
    /// The height, in pixels, of the linked resource.
    
    pub height: Option<u32>,
    /// Media type of the link.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Image URL.
    
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    
    pub width: Option<u32>,
}

impl client::NestedType for ActivityObjectAttachmentsImage {}
impl client::Part for ActivityObjectAttachmentsImage {}


/// If the attachment is an album, this property is a list of potential additional thumbnails from the album.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsThumbnails {
    /// Potential name of the thumbnail.
    
    pub description: Option<String>,
    /// Image resource.
    
    pub image: Option<ActivityObjectAttachmentsThumbnailsImage>,
    /// URL of the webpage containing the image.
    
    pub url: Option<String>,
}

impl client::NestedType for ActivityObjectAttachmentsThumbnails {}
impl client::Part for ActivityObjectAttachmentsThumbnails {}


/// Image resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectAttachmentsThumbnailsImage {
    /// The height, in pixels, of the linked resource.
    
    pub height: Option<u32>,
    /// Media type of the link.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Image url.
    
    pub url: Option<String>,
    /// The width, in pixels, of the linked resource.
    
    pub width: Option<u32>,
}

impl client::NestedType for ActivityObjectAttachmentsThumbnailsImage {}
impl client::Part for ActivityObjectAttachmentsThumbnailsImage {}


/// People who +1'd this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectPlusoners {
    /// The URL for the collection of people who +1'd this activity.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Total number of people who +1'd this activity.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<u32>,
}

impl client::NestedType for ActivityObjectPlusoners {}
impl client::Part for ActivityObjectPlusoners {}


/// Comments in reply to this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectReplies {
    /// The URL for the collection of comments in reply to this activity.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Total number of comments on this activity.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<u32>,
}

impl client::NestedType for ActivityObjectReplies {}
impl client::Part for ActivityObjectReplies {}


/// People who reshared this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityObjectResharers {
    /// The URL for the collection of resharers.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Total number of people who reshared this activity.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<u32>,
}

impl client::NestedType for ActivityObjectResharers {}
impl client::Part for ActivityObjectResharers {}


/// The service provider that initially published this activity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivityProvider {
    /// Name of the service provider.
    
    pub title: Option<String>,
}

impl client::NestedType for ActivityProvider {}
impl client::Part for ActivityProvider {}


/// The person who posted this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentActor {
    /// Actor info specific to particular clients.
    #[serde(rename="clientSpecificActorInfo")]
    
    pub client_specific_actor_info: Option<CommentActorClientSpecificActorInfo>,
    /// The name of this actor, suitable for display.
    #[serde(rename="displayName")]
    
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

impl client::NestedType for CommentActor {}
impl client::Part for CommentActor {}


/// Actor info specific to particular clients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentActorClientSpecificActorInfo {
    /// Actor info specific to YouTube clients.
    #[serde(rename="youtubeActorInfo")]
    
    pub youtube_actor_info: Option<CommentActorClientSpecificActorInfoYoutubeActorInfo>,
}

impl client::NestedType for CommentActorClientSpecificActorInfo {}
impl client::Part for CommentActorClientSpecificActorInfo {}


/// Actor info specific to YouTube clients.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentActorClientSpecificActorInfoYoutubeActorInfo {
    /// ID of the YouTube channel owned by the Actor.
    #[serde(rename="channelId")]
    
    pub channel_id: Option<String>,
}

impl client::NestedType for CommentActorClientSpecificActorInfoYoutubeActorInfo {}
impl client::Part for CommentActorClientSpecificActorInfoYoutubeActorInfo {}


/// The image representation of this actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentActorImage {
    /// The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    
    pub url: Option<String>,
}

impl client::NestedType for CommentActorImage {}
impl client::Part for CommentActorImage {}


/// Verification status of actor.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentActorVerification {
    /// Verification for one-time or manual processes.
    #[serde(rename="adHocVerified")]
    
    pub ad_hoc_verified: Option<String>,
}

impl client::NestedType for CommentActorVerification {}
impl client::Part for CommentActorVerification {}


/// The activity this comment replied to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentInReplyTo {
    /// The ID of the activity.
    
    pub id: Option<String>,
    /// The URL of the activity.
    
    pub url: Option<String>,
}

impl client::NestedType for CommentInReplyTo {}
impl client::Part for CommentInReplyTo {}


/// The object of this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentObject {
    /// The HTML-formatted content, suitable for display.
    
    pub content: Option<String>,
    /// The object type of this comment. Possible values are:  
    /// - "comment" - A comment in reply to an activity.
    #[serde(rename="objectType")]
    
    pub object_type: Option<String>,
    /// The content (text) as provided by the author, stored without any HTML formatting. When creating or updating a comment, this value must be supplied as plain text in the request.
    #[serde(rename="originalContent")]
    
    pub original_content: Option<String>,
}

impl client::NestedType for CommentObject {}
impl client::Part for CommentObject {}


/// People who +1'd this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentPlusoners {
    /// Total number of people who +1'd this comment.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<u32>,
}

impl client::NestedType for CommentPlusoners {}
impl client::Part for CommentPlusoners {}


/// The age range of the person. Valid ranges are 17 or younger, 18 to 20, and 21 or older. Age is determined from the user's birthday using Western age reckoning.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonAgeRange {
    /// The age range's upper bound, if any. Possible values include, but are not limited to, the following:  
    /// - "17" - for age 17 
    /// - "20" - for age 20
    
    pub max: Option<i32>,
    /// The age range's lower bound, if any. Possible values include, but are not limited to, the following:  
    /// - "21" - for age 21 
    /// - "18" - for age 18
    
    pub min: Option<i32>,
}

impl client::NestedType for PersonAgeRange {}
impl client::Part for PersonAgeRange {}


/// The cover photo content.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonCover {
    /// Extra information about the cover photo.
    #[serde(rename="coverInfo")]
    
    pub cover_info: Option<PersonCoverCoverInfo>,
    /// The person's primary cover image.
    #[serde(rename="coverPhoto")]
    
    pub cover_photo: Option<PersonCoverCoverPhoto>,
    /// The layout of the cover art. Possible values include, but are not limited to, the following values:  
    /// - "banner" - One large image banner.
    
    pub layout: Option<String>,
}

impl client::NestedType for PersonCover {}
impl client::Part for PersonCover {}


/// Extra information about the cover photo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonCoverCoverInfo {
    /// The difference between the left position of the cover image and the actual displayed cover image. Only valid for banner layout.
    #[serde(rename="leftImageOffset")]
    
    pub left_image_offset: Option<i32>,
    /// The difference between the top position of the cover image and the actual displayed cover image. Only valid for banner layout.
    #[serde(rename="topImageOffset")]
    
    pub top_image_offset: Option<i32>,
}

impl client::NestedType for PersonCoverCoverInfo {}
impl client::Part for PersonCoverCoverInfo {}


/// The person's primary cover image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonCoverCoverPhoto {
    /// The height of the image.
    
    pub height: Option<i32>,
    /// The URL of the image.
    
    pub url: Option<String>,
    /// The width of the image.
    
    pub width: Option<i32>,
}

impl client::NestedType for PersonCoverCoverPhoto {}
impl client::Part for PersonCoverCoverPhoto {}


/// A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonEmails {
    /// The type of address. Possible values include, but are not limited to, the following values:  
    /// - "account" - Google account email address. 
    /// - "home" - Home email address. 
    /// - "work" - Work email address. 
    /// - "other" - Other.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The email address.
    
    pub value: Option<String>,
}

impl client::NestedType for PersonEmails {}
impl client::Part for PersonEmails {}


/// The representation of the person's profile photo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonImage {
    /// Whether the person's profile photo is the default one
    #[serde(rename="isDefault")]
    
    pub is_default: Option<bool>,
    /// The URL of the person's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side.
    
    pub url: Option<String>,
}

impl client::NestedType for PersonImage {}
impl client::Part for PersonImage {}


/// An object representation of the individual components of a person's name.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonName {
    /// The family name (last name) of this person.
    #[serde(rename="familyName")]
    
    pub family_name: Option<String>,
    /// The full name of this person, including middle names, suffixes, etc.
    
    pub formatted: Option<String>,
    /// The given name (first name) of this person.
    #[serde(rename="givenName")]
    
    pub given_name: Option<String>,
    /// The honorific prefixes (such as "Dr." or "Mrs.") for this person.
    #[serde(rename="honorificPrefix")]
    
    pub honorific_prefix: Option<String>,
    /// The honorific suffixes (such as "Jr.") for this person.
    #[serde(rename="honorificSuffix")]
    
    pub honorific_suffix: Option<String>,
    /// The middle name of this person.
    #[serde(rename="middleName")]
    
    pub middle_name: Option<String>,
}

impl client::NestedType for PersonName {}
impl client::Part for PersonName {}


/// A list of current or past organizations with which this person is associated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonOrganizations {
    /// The department within the organization. Deprecated.
    
    pub department: Option<String>,
    /// A short description of the person's role in this organization. Deprecated.
    
    pub description: Option<String>,
    /// The date that the person left this organization.
    #[serde(rename="endDate")]
    
    pub end_date: Option<String>,
    /// The location of this organization. Deprecated.
    
    pub location: Option<String>,
    /// The name of the organization.
    
    pub name: Option<String>,
    /// If "true", indicates this organization is the person's primary one, which is typically interpreted as the current one.
    
    pub primary: Option<bool>,
    /// The date that the person joined this organization.
    #[serde(rename="startDate")]
    
    pub start_date: Option<String>,
    /// The person's job title or role within the organization.
    
    pub title: Option<String>,
    /// The type of organization. Possible values include, but are not limited to, the following values:  
    /// - "work" - Work. 
    /// - "school" - School.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for PersonOrganizations {}
impl client::Part for PersonOrganizations {}


/// A list of places where this person has lived.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonPlacesLived {
    /// If "true", this place of residence is this person's primary residence.
    
    pub primary: Option<bool>,
    /// A place where this person has lived. For example: "Seattle, WA", "Near Toronto".
    
    pub value: Option<String>,
}

impl client::NestedType for PersonPlacesLived {}
impl client::Part for PersonPlacesLived {}


/// A list of URLs for this person.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PersonUrls {
    /// The label of the URL.
    
    pub label: Option<String>,
    /// The type of URL. Possible values include, but are not limited to, the following values:  
    /// - "otherProfile" - URL for another profile. 
    /// - "contributor" - URL to a site for which this person is a contributor. 
    /// - "website" - URL for this Google+ Page's primary website. 
    /// - "other" - Other URL.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// The URL value.
    
    pub value: Option<String>,
}

impl client::NestedType for PersonUrls {}
impl client::Part for PersonUrls {}


/// The physical address of the place.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlaceAddress {
    /// The formatted address for display.
    
    pub formatted: Option<String>,
}

impl client::NestedType for PlaceAddress {}
impl client::Part for PlaceAddress {}


/// The position of the place.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlacePosition {
    /// The latitude of this position.
    
    pub latitude: Option<f64>,
    /// The longitude of this position.
    
    pub longitude: Option<f64>,
}

impl client::NestedType for PlacePosition {}
impl client::Part for PlacePosition {}


