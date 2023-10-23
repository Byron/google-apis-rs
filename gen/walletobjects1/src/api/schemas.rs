use super::*;
/// ActivationOptions for the class
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivationOptions {
    /// HTTPS URL that supports REST semantics. Would be used for requesting activation from partners for given valuable, triggered by the users.
    #[serde(rename="activationUrl")]
    
    pub activation_url: Option<String>,
    /// Flag to allow users to make activation call from different device. This allows client to render the activation button enabled even if the activationStatus is ACTIVATED but the requested device is different than the current device.
    #[serde(rename="allowReactivation")]
    
    pub allow_reactivation: Option<bool>,
}

impl client::Part for ActivationOptions {}


/// The activation status of the object. This field includes activation status if valuable supports activation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ActivationStatus {
    /// no description provided
    
    pub state: Option<ActivationStatuStateEnum>,
}

impl client::Part for ActivationStatus {}


/// Resource used when the AddMessage endpoints are called.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage eventticketclass](EventticketclasAddmessageCall) (request)
/// * [addmessage eventticketobject](EventticketobjectAddmessageCall) (request)
/// * [addmessage flightclass](FlightclasAddmessageCall) (request)
/// * [addmessage flightobject](FlightobjectAddmessageCall) (request)
/// * [addmessage giftcardclass](GiftcardclasAddmessageCall) (request)
/// * [addmessage giftcardobject](GiftcardobjectAddmessageCall) (request)
/// * [addmessage loyaltyclass](LoyaltyclasAddmessageCall) (request)
/// * [addmessage loyaltyobject](LoyaltyobjectAddmessageCall) (request)
/// * [addmessage offerclass](OfferclasAddmessageCall) (request)
/// * [addmessage offerobject](OfferobjectAddmessageCall) (request)
/// * [addmessage transitclass](TransitclasAddmessageCall) (request)
/// * [addmessage transitobject](TransitobjectAddmessageCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddMessageRequest {
    /// no description provided
    
    pub message: Option<Message>,
}

impl client::RequestValue for AddMessageRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AirportInfo {
    /// Three character IATA airport code. This is a required field for `origin` and `destination`. Eg: "SFO"
    #[serde(rename="airportIataCode")]
    
    pub airport_iata_code: Option<String>,
    /// Optional field that overrides the airport city name defined by IATA. By default, Google takes the `airportIataCode` provided and maps it to the official airport city name defined by IATA. Official IATA airport city names can be found at IATA airport city names website. For example, for the airport IATA code "LTN", IATA website tells us that the corresponding airport city is "London". If this field is not populated, Google would display "London". However, populating this field with a custom name (eg: "London Luton") would override it.
    #[serde(rename="airportNameOverride")]
    
    pub airport_name_override: Option<LocalizedString>,
    /// A name of the gate. Eg: "B59" or "59"
    
    pub gate: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#airportInfo"`.
    
    pub kind: Option<String>,
    /// Terminal name. Eg: "INTL" or "I"
    
    pub terminal: Option<String>,
}

impl client::Part for AirportInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppLinkData {
    /// Optional information about the partner app link. If included, the app link link module will be rendered on the valuable details on the android client.
    #[serde(rename="androidAppLinkInfo")]
    
    pub android_app_link_info: Option<AppLinkDataAppLinkInfo>,
    /// Optional information about the partner app link. If included, the app link link module will be rendered on the valuable details on the ios client.
    #[serde(rename="iosAppLinkInfo")]
    
    pub ios_app_link_info: Option<AppLinkDataAppLinkInfo>,
    /// Optional information about the partner app link. If included, the app link link module will be rendered on the valuable details on the web client.
    #[serde(rename="webAppLinkInfo")]
    
    pub web_app_link_info: Option<AppLinkDataAppLinkInfo>,
}

impl client::Part for AppLinkData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppLinkDataAppLinkInfo {
    /// Optional image to be displayed in the App Link Module
    #[serde(rename="appLogoImage")]
    
    pub app_logo_image: Option<Image>,
    /// Url to follow when opening the App Link Module on clients. It will be used by partners to open their webpage or deeplink into their app.
    #[serde(rename="appTarget")]
    
    pub app_target: Option<AppLinkDataAppLinkInfoAppTarget>,
    /// String to be displayed in the description of the App Link Module Required
    
    pub description: Option<LocalizedString>,
    /// String to be displayed in the title of the App Link Module Required
    
    pub title: Option<LocalizedString>,
}

impl client::Part for AppLinkDataAppLinkInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppLinkDataAppLinkInfoAppTarget {
    /// no description provided
    #[serde(rename="targetUri")]
    
    pub target_uri: Option<Uri>,
}

impl client::Part for AppLinkDataAppLinkInfoAppTarget {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationKey {
    /// Available only to Smart Tap enabled partners. Contact support for additional guidance.
    
    pub id: Option<i32>,
    /// Available only to Smart Tap enabled partners. Contact support for additional guidance.
    #[serde(rename="publicKeyPem")]
    
    pub public_key_pem: Option<String>,
}

impl client::Part for AuthenticationKey {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Barcode {
    /// An optional text that will override the default text that shows under the barcode. This field is intended for a human readable equivalent of the barcode value, used when the barcode cannot be scanned.
    #[serde(rename="alternateText")]
    
    pub alternate_text: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#barcode"`.
    
    pub kind: Option<String>,
    /// The render encoding for the barcode. When specified, barcode is rendered in the given encoding. Otherwise best known encoding is chosen by Google.
    #[serde(rename="renderEncoding")]
    
    pub render_encoding: Option<BarcodeRenderEncodingEnum>,
    /// Optional text that will be shown when the barcode is hidden behind a click action. This happens in cases where a pass has Smart Tap enabled. If not specified, a default is chosen by Google.
    #[serde(rename="showCodeText")]
    
    pub show_code_text: Option<LocalizedString>,
    /// The type of barcode.
    #[serde(rename="type")]
    
    pub type_: Option<BarcodeTypeEnum>,
    /// The value encoded in the barcode.
    
    pub value: Option<String>,
}

impl client::Part for Barcode {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BarcodeSectionDetail {
    /// A reference to an existing text-based or image field to display.
    #[serde(rename="fieldSelector")]
    
    pub field_selector: Option<FieldSelector>,
}

impl client::Part for BarcodeSectionDetail {}


/// Information to read/write to blobstore2.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Blobstore2Info {
    /// The blob generation id.
    #[serde(rename="blobGeneration")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub blob_generation: Option<i64>,
    /// The blob id, e.g., /blobstore/prod/playground/scotty
    #[serde(rename="blobId")]
    
    pub blob_id: Option<String>,
    /// Read handle passed from Bigstore -> Scotty for a GCS download. This is a signed, serialized blobstore2.ReadHandle proto which must never be set outside of Bigstore, and is not applicable to non-GCS media downloads.
    #[serde(rename="downloadReadHandle")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub download_read_handle: Option<Vec<u8>>,
    /// The blob read token. Needed to read blobs that have not been replicated. Might not be available until the final call.
    #[serde(rename="readToken")]
    
    pub read_token: Option<String>,
    /// Metadata passed from Blobstore -> Scotty for a new GCS upload. This is a signed, serialized blobstore2.BlobMetadataContainer proto which must never be consumed outside of Bigstore, and is not applicable to non-GCS media uploads.
    #[serde(rename="uploadMetadataContainer")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub upload_metadata_container: Option<Vec<u8>>,
}

impl client::Part for Blobstore2Info {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BoardingAndSeatingInfo {
    /// Set this field only if this flight boards through more than one door or bridge and you want to explicitly print the door location on the boarding pass. Most airlines route their passengers to the right door or bridge by refering to doors/bridges by the `seatClass`. In those cases `boardingDoor` should not be set.
    #[serde(rename="boardingDoor")]
    
    pub boarding_door: Option<BoardingAndSeatingInfoBoardingDoorEnum>,
    /// The value of boarding group (or zone) this passenger shall board with. eg: "B" The label for this value will be determined by the `boardingPolicy` field in the `flightClass` referenced by this object.
    #[serde(rename="boardingGroup")]
    
    pub boarding_group: Option<String>,
    /// The value of boarding position. eg: "76"
    #[serde(rename="boardingPosition")]
    
    pub boarding_position: Option<String>,
    /// A small image shown above the boarding barcode. Airlines can use it to communicate any special boarding privileges. In the event the security program logo is also set, this image might be rendered alongside the logo for that security program.
    #[serde(rename="boardingPrivilegeImage")]
    
    pub boarding_privilege_image: Option<Image>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#boardingAndSeatingInfo"`.
    
    pub kind: Option<String>,
    /// The passenger's seat assignment. To be used when there is no specific identifier to use in `seatNumber`. eg: "assigned at gate"
    #[serde(rename="seatAssignment")]
    
    pub seat_assignment: Option<LocalizedString>,
    /// The value of the seat class. eg: "Economy" or "Economy Plus"
    #[serde(rename="seatClass")]
    
    pub seat_class: Option<String>,
    /// The value of passenger seat. If there is no specific identifier, use `seatAssignment` instead. eg: "25A"
    #[serde(rename="seatNumber")]
    
    pub seat_number: Option<String>,
    /// The sequence number on the boarding pass. This usually matches the sequence in which the passengers checked in. Airline might use the number for manual boarding and bag tags. eg: "49"
    #[serde(rename="sequenceNumber")]
    
    pub sequence_number: Option<String>,
}

impl client::Part for BoardingAndSeatingInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BoardingAndSeatingPolicy {
    /// Indicates the policy the airline uses for boarding. If unset, Google will default to `zoneBased`.
    #[serde(rename="boardingPolicy")]
    
    pub boarding_policy: Option<BoardingAndSeatingPolicyBoardingPolicyEnum>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#boardingAndSeatingPolicy"`.
    
    pub kind: Option<String>,
    /// Seating policy which dictates how we display the seat class. If unset, Google will default to `cabinBased`.
    #[serde(rename="seatClassPolicy")]
    
    pub seat_class_policy: Option<BoardingAndSeatingPolicySeatClassPolicyEnum>,
}

impl client::Part for BoardingAndSeatingPolicy {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CallbackOptions {
    /// URL for the merchant endpoint that would be called to request updates. The URL should be hosted on HTTPS and robots.txt should allow the URL path to be accessible by UserAgent:Google-Valuables. Deprecated.
    #[serde(rename="updateRequestUrl")]
    
    pub update_request_url: Option<String>,
    /// The HTTPS url configured by the merchant. The URL should be hosted on HTTPS and robots.txt should allow the URL path to be accessible by UserAgent:Google-Valuables.
    
    pub url: Option<String>,
}

impl client::Part for CallbackOptions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CardBarcodeSectionDetails {
    /// Optional information to display below the barcode.
    #[serde(rename="firstBottomDetail")]
    
    pub first_bottom_detail: Option<BarcodeSectionDetail>,
    /// Optional information to display above the barcode. If `secondTopDetail` is defined, this will be displayed to the start side of this detail section.
    #[serde(rename="firstTopDetail")]
    
    pub first_top_detail: Option<BarcodeSectionDetail>,
    /// Optional second piece of information to display above the barcode. If `firstTopDetail` is defined, this will be displayed to the end side of this detail section.
    #[serde(rename="secondTopDetail")]
    
    pub second_top_detail: Option<BarcodeSectionDetail>,
}

impl client::Part for CardBarcodeSectionDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CardRowOneItem {
    /// The item to be displayed in the row. This item will be automatically centered.
    
    pub item: Option<TemplateItem>,
}

impl client::Part for CardRowOneItem {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CardRowTemplateInfo {
    /// Template for a row containing one item. Exactly one of "one_item", "two_items", "three_items" must be set.
    #[serde(rename="oneItem")]
    
    pub one_item: Option<CardRowOneItem>,
    /// Template for a row containing three items. Exactly one of "one_item", "two_items", "three_items" must be set.
    #[serde(rename="threeItems")]
    
    pub three_items: Option<CardRowThreeItems>,
    /// Template for a row containing two items. Exactly one of "one_item", "two_items", "three_items" must be set.
    #[serde(rename="twoItems")]
    
    pub two_items: Option<CardRowTwoItems>,
}

impl client::Part for CardRowTemplateInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CardRowThreeItems {
    /// The item to be displayed at the end of the row. This item will be aligned to the right.
    #[serde(rename="endItem")]
    
    pub end_item: Option<TemplateItem>,
    /// The item to be displayed in the middle of the row. This item will be centered between the start and end items.
    #[serde(rename="middleItem")]
    
    pub middle_item: Option<TemplateItem>,
    /// The item to be displayed at the start of the row. This item will be aligned to the left.
    #[serde(rename="startItem")]
    
    pub start_item: Option<TemplateItem>,
}

impl client::Part for CardRowThreeItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CardRowTwoItems {
    /// The item to be displayed at the end of the row. This item will be aligned to the right.
    #[serde(rename="endItem")]
    
    pub end_item: Option<TemplateItem>,
    /// The item to be displayed at the start of the row. This item will be aligned to the left.
    #[serde(rename="startItem")]
    
    pub start_item: Option<TemplateItem>,
}

impl client::Part for CardRowTwoItems {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CardTemplateOverride {
    /// Template information for rows in the card view. At most three rows are allowed to be specified.
    #[serde(rename="cardRowTemplateInfos")]
    
    pub card_row_template_infos: Option<Vec<CardRowTemplateInfo>>,
}

impl client::Part for CardTemplateOverride {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ClassTemplateInfo {
    /// Specifies extra information to be displayed above and below the barcode.
    #[serde(rename="cardBarcodeSectionDetails")]
    
    pub card_barcode_section_details: Option<CardBarcodeSectionDetails>,
    /// Override for the card view.
    #[serde(rename="cardTemplateOverride")]
    
    pub card_template_override: Option<CardTemplateOverride>,
    /// Override for the details view (beneath the card view).
    #[serde(rename="detailsTemplateOverride")]
    
    pub details_template_override: Option<DetailsTemplateOverride>,
    /// Override for the passes list view.
    #[serde(rename="listTemplateOverride")]
    
    pub list_template_override: Option<ListTemplateOverride>,
}

impl client::Part for ClassTemplateInfo {}


/// A sequence of media data references representing composite data. Introduced to support Bigstore composite objects. For details, visit http://go/bigstore-composites.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompositeMedia {
    /// Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef.
    #[serde(rename="blobRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob.
    #[serde(rename="blobstore2Info")]
    
    pub blobstore2_info: Option<Blobstore2Info>,
    /// A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field.
    #[serde(rename="cosmoBinaryReference")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// crc32.c hash for the payload.
    #[serde(rename="crc32cHash")]
    
    pub crc32c_hash: Option<u32>,
    /// Media data, set if reference_type is INLINE
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// Size of the data, in bytes
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// MD5 hash for the payload.
    #[serde(rename="md5Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// Reference to a TI Blob, set if reference_type is BIGSTORE_REF.
    #[serde(rename="objectId")]
    
    pub object_id: Option<ObjectId>,
    /// Path to the data, set if reference_type is PATH
    
    pub path: Option<String>,
    /// Describes what the field reference contains.
    #[serde(rename="referenceType")]
    
    pub reference_type: Option<CompositeMediaReferenceTypeEnum>,
    /// SHA-1 hash for the payload.
    #[serde(rename="sha1Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
}

impl client::Part for CompositeMedia {}


/// Detailed Content-Type information from Scotty. The Content-Type of the media will typically be filled in by the header or Scotty's best_guess, but this extended information provides the backend with more information so that it can make a better decision if needed. This is only used on media upload requests from Scotty.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentTypeInfo {
    /// Scotty's best guess of what the content type of the file is.
    #[serde(rename="bestGuess")]
    
    pub best_guess: Option<String>,
    /// The content type of the file derived by looking at specific bytes (i.e. "magic bytes") of the actual file.
    #[serde(rename="fromBytes")]
    
    pub from_bytes: Option<String>,
    /// The content type of the file derived from the file extension of the original file name used by the client.
    #[serde(rename="fromFileName")]
    
    pub from_file_name: Option<String>,
    /// The content type of the file as specified in the request headers, multipart headers, or RUPIO start request.
    #[serde(rename="fromHeader")]
    
    pub from_header: Option<String>,
    /// The content type of the file derived from the file extension of the URL path. The URL path is assumed to represent a file name (which is typically only true for agents that are providing a REST API).
    #[serde(rename="fromUrlPath")]
    
    pub from_url_path: Option<String>,
}

impl client::Part for ContentTypeInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateTime {
    /// An ISO 8601 extended format date/time. Offset may or may not be required (refer to the parent field's documentation). Time may be specified up to nanosecond precision. Offsets may be specified with seconds precision (even though offset seconds is not part of ISO 8601). For example: `1985-04-12T23:20:50.52Z` would be 20 minutes and 50.52 seconds after the 23rd hour of April 12th, 1985 in UTC. `1985-04-12T19:20:50.52-04:00` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985, 4 hours before UTC (same instant in time as the above example). If the date/time is intended for a physical location in New York, this would be the equivalent of Eastern Daylight Time (EDT). Remember that offset varies in regions that observe Daylight Saving Time (or Summer Time), depending on the time of the year. `1985-04-12T19:20:50.52` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985 with no offset information. Providing an offset makes this an absolute instant in time around the world. The date/time will be adjusted based on the user's time zone. For example, a time of `2018-06-19T18:30:00-04:00` will be 18:30:00 for a user in New York and 15:30:00 for a user in Los Angeles. Omitting the offset makes this a local date/time, representing several instants in time around the world. The date/time will always be in the user's current time zone. For example, a time of `2018-06-19T18:30:00` will be 18:30:00 for a user in New York and also 18:30:00 for a user in Los Angeles. This is useful when the same local date/time should apply to many physical locations across several time zones.
    
    pub date: Option<String>,
}

impl client::Part for DateTime {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetailsItemInfo {
    /// The item to be displayed in the details list.
    
    pub item: Option<TemplateItem>,
}

impl client::Part for DetailsItemInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetailsTemplateOverride {
    /// Information for the "nth" item displayed in the details list.
    #[serde(rename="detailsItemInfos")]
    
    pub details_item_infos: Option<Vec<DetailsItemInfo>>,
}

impl client::Part for DetailsTemplateOverride {}


/// Device context associated with the object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceContext {
    /// If set, redemption information will only be returned to the given device upon activation of the object. This should not be used as a stable identifier to trace a user's device. It can change across different passes for the same device or even across different activations for the same device. When setting this, callers must also set has_linked_device on the object being activated.
    #[serde(rename="deviceToken")]
    
    pub device_token: Option<String>,
}

impl client::Part for DeviceContext {}


/// Backend response for a Diff get checksums response. For details on the Scotty Diff protocol, visit http://go/scotty-diff-protocol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffChecksumsResponse {
    /// Exactly one of these fields must be populated. If checksums_location is filled, the server will return the corresponding contents to the user. If object_location is filled, the server will calculate the checksums based on the content there and return that to the user. For details on the format of the checksums, see http://go/scotty-diff-protocol.
    #[serde(rename="checksumsLocation")]
    
    pub checksums_location: Option<CompositeMedia>,
    /// The chunk size of checksums. Must be a multiple of 256KB.
    #[serde(rename="chunkSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub chunk_size_bytes: Option<i64>,
    /// If set, calculate the checksums based on the contents and return them to the caller.
    #[serde(rename="objectLocation")]
    
    pub object_location: Option<CompositeMedia>,
    /// The total size of the server object.
    #[serde(rename="objectSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// The object version of the object the checksums are being returned for.
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for DiffChecksumsResponse {}


/// Backend response for a Diff download response. For details on the Scotty Diff protocol, visit http://go/scotty-diff-protocol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffDownloadResponse {
    /// The original object location.
    #[serde(rename="objectLocation")]
    
    pub object_location: Option<CompositeMedia>,
}

impl client::Part for DiffDownloadResponse {}


/// A Diff upload request. For details on the Scotty Diff protocol, visit http://go/scotty-diff-protocol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffUploadRequest {
    /// The location of the checksums for the new object. Agents must clone the object located here, as the upload server will delete the contents once a response is received. For details on the format of the checksums, see http://go/scotty-diff-protocol.
    #[serde(rename="checksumsInfo")]
    
    pub checksums_info: Option<CompositeMedia>,
    /// The location of the new object. Agents must clone the object located here, as the upload server will delete the contents once a response is received.
    #[serde(rename="objectInfo")]
    
    pub object_info: Option<CompositeMedia>,
    /// The object version of the object that is the base version the incoming diff script will be applied to. This field will always be filled in.
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for DiffUploadRequest {}


/// Backend response for a Diff upload request. For details on the Scotty Diff protocol, visit http://go/scotty-diff-protocol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffUploadResponse {
    /// The object version of the object at the server. Must be included in the end notification response. The version in the end notification response must correspond to the new version of the object that is now stored at the server, after the upload.
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
    /// The location of the original file for a diff upload request. Must be filled in if responding to an upload start notification.
    #[serde(rename="originalObject")]
    
    pub original_object: Option<CompositeMedia>,
}

impl client::Part for DiffUploadResponse {}


/// Backend response for a Diff get version response. For details on the Scotty Diff protocol, visit http://go/scotty-diff-protocol.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiffVersionResponse {
    /// The total size of the server object.
    #[serde(rename="objectSizeBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_size_bytes: Option<i64>,
    /// The version of the object stored at the server.
    #[serde(rename="objectVersion")]
    
    pub object_version: Option<String>,
}

impl client::Part for DiffVersionResponse {}


/// Information about how a class may be discovered and instantiated from within the Android Pay app. This is done by searching for a loyalty or gift card program and scanning or manually entering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoverableProgram {
    /// Information about the ability to signin and add a valuable for this program through a merchant site. Used when MERCHANT_HOSTED_SIGNIN is enabled.
    #[serde(rename="merchantSigninInfo")]
    
    pub merchant_signin_info: Option<DiscoverableProgramMerchantSigninInfo>,
    /// Information about the ability to signup and add a valuable for this program through a merchant site. Used when MERCHANT_HOSTED_SIGNUP is enabled.
    #[serde(rename="merchantSignupInfo")]
    
    pub merchant_signup_info: Option<DiscoverableProgramMerchantSignupInfo>,
    /// Visibility state of the discoverable program.
    
    pub state: Option<DiscoverableProgramStateEnum>,
}

impl client::Part for DiscoverableProgram {}


/// Information about the merchant hosted signin flow for a program.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoverableProgramMerchantSigninInfo {
    /// The URL to direct the user to for the merchant's signin site.
    #[serde(rename="signinWebsite")]
    
    pub signin_website: Option<Uri>,
}

impl client::Part for DiscoverableProgramMerchantSigninInfo {}


/// Information about the merchant hosted signup flow for a program.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DiscoverableProgramMerchantSignupInfo {
    ///  User data that is sent in a POST request to the signup website URL. This information is encoded and then shared so that the merchant's website can prefill fields used to enroll the user for the discoverable program.
    #[serde(rename="signupSharedDatas")]
    
    pub signup_shared_datas: Option<Vec<DiscoverableProgramMerchantSignupInfoSignupSharedDatasEnum>>,
    /// The URL to direct the user to for the merchant's signup site.
    #[serde(rename="signupWebsite")]
    
    pub signup_website: Option<Uri>,
}

impl client::Part for DiscoverableProgramMerchantSignupInfo {}


/// Parameters specific to media downloads.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DownloadParameters {
    /// A boolean to be returned in the response to Scotty. Allows/disallows gzip encoding of the payload content when the server thinks it's advantageous (hence, does not guarantee compression) which allows Scotty to GZip the response to the client.
    #[serde(rename="allowGzipCompression")]
    
    pub allow_gzip_compression: Option<bool>,
    /// Determining whether or not Apiary should skip the inclusion of any Content-Range header on its response to Scotty.
    #[serde(rename="ignoreRange")]
    
    pub ignore_range: Option<bool>,
}

impl client::Part for DownloadParameters {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventDateTime {
    /// A custom label to use for the doors open value (`doorsOpen`) on the card detail view. This should only be used if the default "Doors Open" label or one of the `doorsOpenLabel` options is not sufficient. Both `doorsOpenLabel` and `customDoorsOpenLabel` may not be set. If neither is set, the label will default to "Doors Open", localized. If the doors open field is unset, this label will not be used.
    #[serde(rename="customDoorsOpenLabel")]
    
    pub custom_doors_open_label: Option<LocalizedString>,
    /// The date/time when the doors open at the venue. This is an ISO 8601 extended format date/time, with or without an offset. Time may be specified up to nanosecond precision. Offsets may be specified with seconds precision (even though offset seconds is not part of ISO 8601). For example: `1985-04-12T23:20:50.52Z` would be 20 minutes and 50.52 seconds after the 23rd hour of April 12th, 1985 in UTC. `1985-04-12T19:20:50.52-04:00` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985, 4 hours before UTC (same instant in time as the above example). If the event were in New York, this would be the equivalent of Eastern Daylight Time (EDT). Remember that offset varies in regions that observe Daylight Saving Time (or Summer Time), depending on the time of the year. `1985-04-12T19:20:50.52` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985 with no offset information. The portion of the date/time without the offset is considered the "local date/time". This should be the local date/time at the venue. For example, if the event occurs at the 20th hour of June 5th, 2018 at the venue, the local date/time portion should be `2018-06-05T20:00:00`. If the local date/time at the venue is 4 hours before UTC, an offset of `-04:00` may be appended. Without offset information, some rich features may not be available.
    #[serde(rename="doorsOpen")]
    
    pub doors_open: Option<String>,
    /// The label to use for the doors open value (`doorsOpen`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `doorsOpenLabel` and `customDoorsOpenLabel` may not be set. If neither is set, the label will default to "Doors Open", localized. If the doors open field is unset, this label will not be used.
    #[serde(rename="doorsOpenLabel")]
    
    pub doors_open_label: Option<EventDateTimeDoorsOpenLabelEnum>,
    /// The date/time when the event ends. If the event spans multiple days, it should be the end date/time on the last day. This is an ISO 8601 extended format date/time, with or without an offset. Time may be specified up to nanosecond precision. Offsets may be specified with seconds precision (even though offset seconds is not part of ISO 8601). For example: `1985-04-12T23:20:50.52Z` would be 20 minutes and 50.52 seconds after the 23rd hour of April 12th, 1985 in UTC. `1985-04-12T19:20:50.52-04:00` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985, 4 hours before UTC (same instant in time as the above example). If the event were in New York, this would be the equivalent of Eastern Daylight Time (EDT). Remember that offset varies in regions that observe Daylight Saving Time (or Summer Time), depending on the time of the year. `1985-04-12T19:20:50.52` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985 with no offset information. The portion of the date/time without the offset is considered the "local date/time". This should be the local date/time at the venue. For example, if the event occurs at the 20th hour of June 5th, 2018 at the venue, the local date/time portion should be `2018-06-05T20:00:00`. If the local date/time at the venue is 4 hours before UTC, an offset of `-04:00` may be appended. Without offset information, some rich features may not be available.
    
    pub end: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventDateTime"`.
    
    pub kind: Option<String>,
    /// The date/time when the event starts. If the event spans multiple days, it should be the start date/time on the first day. This is an ISO 8601 extended format date/time, with or without an offset. Time may be specified up to nanosecond precision. Offsets may be specified with seconds precision (even though offset seconds is not part of ISO 8601). For example: `1985-04-12T23:20:50.52Z` would be 20 minutes and 50.52 seconds after the 23rd hour of April 12th, 1985 in UTC. `1985-04-12T19:20:50.52-04:00` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985, 4 hours before UTC (same instant in time as the above example). If the event were in New York, this would be the equivalent of Eastern Daylight Time (EDT). Remember that offset varies in regions that observe Daylight Saving Time (or Summer Time), depending on the time of the year. `1985-04-12T19:20:50.52` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985 with no offset information. The portion of the date/time without the offset is considered the "local date/time". This should be the local date/time at the venue. For example, if the event occurs at the 20th hour of June 5th, 2018 at the venue, the local date/time portion should be `2018-06-05T20:00:00`. If the local date/time at the venue is 4 hours before UTC, an offset of `-04:00` may be appended. Without offset information, some rich features may not be available.
    
    pub start: Option<String>,
}

impl client::Part for EventDateTime {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventReservationInfo {
    /// The confirmation code of the event reservation. This may also take the form of an "order number", "confirmation number", "reservation number", or other equivalent.
    #[serde(rename="confirmationCode")]
    
    pub confirmation_code: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventReservationInfo"`.
    
    pub kind: Option<String>,
}

impl client::Part for EventReservationInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventSeat {
    /// The gate the ticket holder should enter to get to their seat, such as "A" or "West". This field is localizable so you may translate words or use different alphabets for the characters in an identifier.
    
    pub gate: Option<LocalizedString>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventSeat"`.
    
    pub kind: Option<String>,
    /// The row of the seat, such as "1", E", "BB", or "A5". This field is localizable so you may translate words or use different alphabets for the characters in an identifier.
    
    pub row: Option<LocalizedString>,
    /// The seat number, such as "1", "2", "3", or any other seat identifier. This field is localizable so you may translate words or use different alphabets for the characters in an identifier.
    
    pub seat: Option<LocalizedString>,
    /// The section of the seat, such as "121". This field is localizable so you may translate words or use different alphabets for the characters in an identifier.
    
    pub section: Option<LocalizedString>,
}

impl client::Part for EventSeat {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get eventticketclass](EventticketclasGetCall) (response)
/// * [insert eventticketclass](EventticketclasInsertCall) (request|response)
/// * [patch eventticketclass](EventticketclasPatchCall) (request|response)
/// * [update eventticketclass](EventticketclasUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTicketClass {
    /// Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead.
    #[serde(rename="allowMultipleUsersPerObject")]
    
    pub allow_multiple_users_per_object: Option<bool>,
    /// Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback.
    #[serde(rename="callbackOptions")]
    
    pub callback_options: Option<CallbackOptions>,
    /// Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display.
    #[serde(rename="classTemplateInfo")]
    
    pub class_template_info: Option<ClassTemplateInfo>,
    /// The label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used.
    #[serde(rename="confirmationCodeLabel")]
    
    pub confirmation_code_label: Option<EventTicketClasConfirmationCodeLabelEnum>,
    /// Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// A custom label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. This should only be used if the default "Confirmation Code" label or one of the `confirmationCodeLabel` options is not sufficient. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used.
    #[serde(rename="customConfirmationCodeLabel")]
    
    pub custom_confirmation_code_label: Option<LocalizedString>,
    /// A custom label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. This should only be used if the default "Gate" label or one of the `gateLabel` options is not sufficient. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used.
    #[serde(rename="customGateLabel")]
    
    pub custom_gate_label: Option<LocalizedString>,
    /// A custom label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. This should only be used if the default "Row" label or one of the `rowLabel` options is not sufficient. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used.
    #[serde(rename="customRowLabel")]
    
    pub custom_row_label: Option<LocalizedString>,
    /// A custom label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. This should only be used if the default "Seat" label or one of the `seatLabel` options is not sufficient. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used.
    #[serde(rename="customSeatLabel")]
    
    pub custom_seat_label: Option<LocalizedString>,
    /// A custom label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. This should only be used if the default "Section" label or one of the `sectionLabel` options is not sufficient. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used.
    #[serde(rename="customSectionLabel")]
    
    pub custom_section_label: Option<LocalizedString>,
    /// The date & time information of the event.
    #[serde(rename="dateTime")]
    
    pub date_time: Option<EventDateTime>,
    /// Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="enableSmartTap")]
    
    pub enable_smart_tap: Option<bool>,
    /// The ID of the event. This ID should be unique for every event in an account. It is used to group tickets together if the user has saved multiple tickets for the same event. It can be at most 64 characters. If provided, the grouping will be stable. Be wary of unintentional collision to avoid grouping tickets that should not be grouped. If you use only one class per event, you can simply set this to the `classId` (with or without the issuer ID portion). If not provided, the platform will attempt to use other data to group tickets (potentially unstable).
    #[serde(rename="eventId")]
    
    pub event_id: Option<String>,
    /// Required. The name of the event, such as "LA Dodgers at SF Giants".
    #[serde(rename="eventName")]
    
    pub event_name: Option<LocalizedString>,
    /// The fine print, terms, or conditions of the ticket.
    #[serde(rename="finePrint")]
    
    pub fine_print: Option<LocalizedString>,
    /// The label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used.
    #[serde(rename="gateLabel")]
    
    pub gate_label: Option<EventTicketClasGateLabelEnum>,
    /// Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object).
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<Uri>,
    /// Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="issuerName")]
    
    pub issuer_name: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketClass"`.
    
    pub kind: Option<String>,
    /// Links module data. If links module data is also defined on the object, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedIssuerName")]
    
    pub localized_issuer_name: Option<LocalizedString>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// The logo image of the ticket. This image is displayed in the card detail view of the app.
    
    pub logo: Option<Image>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Identifies whether multiple users and devices will save the same object referencing this class.
    #[serde(rename="multipleDevicesAndHoldersAllowedStatus")]
    
    pub multiple_devices_and_holders_allowed_status: Option<EventTicketClasMultipleDevicesAndHoldersAllowedStatusEnum>,
    /// Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="redemptionIssuers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub redemption_issuers: Option<Vec<i64>>,
    /// The review comments set by the platform when a class is marked `approved` or `rejected`.
    
    pub review: Option<Review>,
    /// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
    #[serde(rename="reviewStatus")]
    
    pub review_status: Option<EventTicketClasReviewStatusEnum>,
    /// The label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used.
    #[serde(rename="rowLabel")]
    
    pub row_label: Option<EventTicketClasRowLabelEnum>,
    /// The label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used.
    #[serde(rename="seatLabel")]
    
    pub seat_label: Option<EventTicketClasSeatLabelEnum>,
    /// The label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used.
    #[serde(rename="sectionLabel")]
    
    pub section_label: Option<EventTicketClasSectionLabelEnum>,
    /// Optional information about the security animation. If this is set a security animation will be rendered on pass details.
    #[serde(rename="securityAnimation")]
    
    pub security_animation: Option<SecurityAnimation>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// Event venue details.
    
    pub venue: Option<EventVenue>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// View Unlock Requirement options for the event ticket.
    #[serde(rename="viewUnlockRequirement")]
    
    pub view_unlock_requirement: Option<EventTicketClasViewUnlockRequirementEnum>,
    /// Deprecated.
    #[serde(rename="wordMark")]
    
    pub word_mark: Option<Image>,
}

impl client::RequestValue for EventTicketClass {}
impl client::ResponseResult for EventTicketClass {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage eventticketclass](EventticketclasAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTicketClassAddMessageResponse {
    /// The updated EventTicketClass resource.
    
    pub resource: Option<EventTicketClass>,
}

impl client::ResponseResult for EventTicketClassAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list eventticketclass](EventticketclasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTicketClassListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<EventTicketClass>>,
}

impl client::ResponseResult for EventTicketClassListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get eventticketobject](EventticketobjectGetCall) (response)
/// * [insert eventticketobject](EventticketobjectInsertCall) (request|response)
/// * [modifylinkedofferobjects eventticketobject](EventticketobjectModifylinkedofferobjectCall) (response)
/// * [patch eventticketobject](EventticketobjectPatchCall) (request|response)
/// * [update eventticketobject](EventticketobjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTicketObject {
    /// Optional information about the partner app link.
    #[serde(rename="appLinkData")]
    
    pub app_link_data: Option<AppLinkData>,
    /// The barcode type and value.
    
    pub barcode: Option<Barcode>,
    /// Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
    /// A copy of the inherited fields of the parent class. These fields are retrieved during a GET.
    #[serde(rename="classReference")]
    
    pub class_reference: Option<EventTicketClass>,
    /// Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers.
    #[serde(rename="disableExpirationNotification")]
    
    pub disable_expiration_notification: Option<bool>,
    /// The face value of the ticket, matching what would be printed on a physical version of the ticket.
    #[serde(rename="faceValue")]
    
    pub face_value: Option<Money>,
    /// Information that controls how passes are grouped together.
    #[serde(rename="groupingInfo")]
    
    pub grouping_info: Option<GroupingInfo>,
    /// Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information.
    #[serde(rename="hasLinkedDevice")]
    
    pub has_linked_device: Option<bool>,
    /// Indicates if the object has users. This field is set by the platform.
    #[serde(rename="hasUsers")]
    
    pub has_users: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketObject"`.
    
    pub kind: Option<String>,
    /// A list of offer objects linked to this event ticket. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="linkedOfferIds")]
    
    pub linked_offer_ids: Option<Vec<String>>,
    /// Links module data. If links module data is also defined on the class, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Reservation details for this ticket. This is expected to be shared amongst all tickets that were purchased in the same order.
    #[serde(rename="reservationInfo")]
    
    pub reservation_info: Option<EventReservationInfo>,
    /// The rotating barcode type and value.
    #[serde(rename="rotatingBarcode")]
    
    pub rotating_barcode: Option<RotatingBarcode>,
    /// Seating details for this ticket.
    #[serde(rename="seatInfo")]
    
    pub seat_info: Option<EventSeat>,
    /// The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported.
    #[serde(rename="smartTapRedemptionValue")]
    
    pub smart_tap_redemption_value: Option<String>,
    /// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
    
    pub state: Option<EventTicketObjectStateEnum>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// Name of the ticket holder, if the ticket is assigned to a person. E.g. "John Doe" or "Jane Doe".
    #[serde(rename="ticketHolderName")]
    
    pub ticket_holder_name: Option<String>,
    /// The number of the ticket. This can be a unique identifier across all tickets in an issuer's system, all tickets for the event (e.g. XYZ1234512345), or all tickets in the order (1, 2, 3, etc.).
    #[serde(rename="ticketNumber")]
    
    pub ticket_number: Option<String>,
    /// The type of the ticket, such as "Adult" or "Child", or "VIP" or "Standard".
    #[serde(rename="ticketType")]
    
    pub ticket_type: Option<LocalizedString>,
    /// The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed.
    #[serde(rename="validTimeInterval")]
    
    pub valid_time_interval: Option<TimeInterval>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::RequestValue for EventTicketObject {}
impl client::ResponseResult for EventTicketObject {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage eventticketobject](EventticketobjectAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTicketObjectAddMessageResponse {
    /// The updated EventTicketObject resource.
    
    pub resource: Option<EventTicketObject>,
}

impl client::ResponseResult for EventTicketObjectAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list eventticketobject](EventticketobjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventTicketObjectListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<EventTicketObject>>,
}

impl client::ResponseResult for EventTicketObjectListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventVenue {
    /// The address of the venue, such as "24 Willie Mays Plaza\nSan Francisco, CA 94107". Address lines are separated by line feed (`\n`) characters. This is required.
    
    pub address: Option<LocalizedString>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventVenue"`.
    
    pub kind: Option<String>,
    /// The name of the venue, such as "AT&T Park". This is required.
    
    pub name: Option<LocalizedString>,
}

impl client::Part for EventVenue {}


/// Indicates that the issuer would like GooglePay to send expiry notifications 2 days prior to the card expiration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExpiryNotification {
    /// Indicates if the object needs to have expiry notification enabled.
    #[serde(rename="enableNotification")]
    
    pub enable_notification: Option<bool>,
}

impl client::Part for ExpiryNotification {}


/// Reference definition to use with field overrides.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldReference {
    /// Only valid if the `fieldPath` references a date field. Chooses how the date field will be formatted and displayed in the UI.
    #[serde(rename="dateFormat")]
    
    pub date_format: Option<FieldReferenceDateFormatEnum>,
    /// Path to the field being referenced, prefixed with "object" or "class" and separated with dots. For example, it may be the string "object.purchaseDetails.purchasePrice".
    #[serde(rename="fieldPath")]
    
    pub field_path: Option<String>,
}

impl client::Part for FieldReference {}


/// Custom field selector to use with field overrides.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldSelector {
    /// If more than one reference is supplied, then the first one that references a non-empty field will be displayed.
    
    pub fields: Option<Vec<FieldReference>>,
}

impl client::Part for FieldSelector {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirstRowOption {
    /// A reference to the field to be displayed in the first row.
    #[serde(rename="fieldOption")]
    
    pub field_option: Option<FieldSelector>,
    /// no description provided
    #[serde(rename="transitOption")]
    
    pub transit_option: Option<FirstRowOptionTransitOptionEnum>,
}

impl client::Part for FirstRowOption {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightCarrier {
    /// A logo for the airline alliance, displayed above the QR code that the passenger scans to board.
    #[serde(rename="airlineAllianceLogo")]
    
    pub airline_alliance_logo: Option<Image>,
    /// A logo for the airline described by carrierIataCode and localizedAirlineName. This logo will be rendered at the top of the detailed card view.
    #[serde(rename="airlineLogo")]
    
    pub airline_logo: Option<Image>,
    /// A localized name of the airline specified by carrierIataCode. If unset, `issuer_name` or `localized_issuer_name` from `FlightClass` will be used for display purposes. eg: "Swiss Air" for "LX"
    #[serde(rename="airlineName")]
    
    pub airline_name: Option<LocalizedString>,
    /// Two character IATA airline code of the marketing carrier (as opposed to operating carrier). Exactly one of this or `carrierIcaoCode` needs to be provided for `carrier` and `operatingCarrier`. eg: "LX" for Swiss Air
    #[serde(rename="carrierIataCode")]
    
    pub carrier_iata_code: Option<String>,
    /// Three character ICAO airline code of the marketing carrier (as opposed to operating carrier). Exactly one of this or `carrierIataCode` needs to be provided for `carrier` and `operatingCarrier`. eg: "EZY" for Easy Jet
    #[serde(rename="carrierIcaoCode")]
    
    pub carrier_icao_code: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightCarrier"`.
    
    pub kind: Option<String>,
}

impl client::Part for FlightCarrier {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get flightclass](FlightclasGetCall) (response)
/// * [insert flightclass](FlightclasInsertCall) (request|response)
/// * [patch flightclass](FlightclasPatchCall) (request|response)
/// * [update flightclass](FlightclasUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightClass {
    /// Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead.
    #[serde(rename="allowMultipleUsersPerObject")]
    
    pub allow_multiple_users_per_object: Option<bool>,
    /// Policies for boarding and seating. These will inform which labels will be shown to users.
    #[serde(rename="boardingAndSeatingPolicy")]
    
    pub boarding_and_seating_policy: Option<BoardingAndSeatingPolicy>,
    /// Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback.
    #[serde(rename="callbackOptions")]
    
    pub callback_options: Option<CallbackOptions>,
    /// Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display.
    #[serde(rename="classTemplateInfo")]
    
    pub class_template_info: Option<ClassTemplateInfo>,
    /// Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// Required. Destination airport.
    
    pub destination: Option<AirportInfo>,
    /// Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="enableSmartTap")]
    
    pub enable_smart_tap: Option<bool>,
    /// Required. Information about the flight carrier and number.
    #[serde(rename="flightHeader")]
    
    pub flight_header: Option<FlightHeader>,
    /// Status of this flight. If unset, Google will compute status based on data from other sources, such as FlightStats, etc. Note: Google-computed status will not be returned in API responses.
    #[serde(rename="flightStatus")]
    
    pub flight_status: Option<FlightClasFlightStatusEnum>,
    /// Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object).
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<Uri>,
    /// Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="issuerName")]
    
    pub issuer_name: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightClass"`.
    
    pub kind: Option<String>,
    /// If this field is present, boarding passes served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT".
    #[serde(rename="languageOverride")]
    
    pub language_override: Option<String>,
    /// Links module data. If links module data is also defined on the object, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// The boarding time as it would be printed on the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. If this is not set, Google will set it based on data from other sources.
    #[serde(rename="localBoardingDateTime")]
    
    pub local_boarding_date_time: Option<String>,
    /// The estimated time the aircraft plans to reach the destination gate (not the runway) or the actual time it reached the gate. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already arrived at the gate. Google will use it to inform the user that the flight has arrived at the gate. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. If this is not set, Google will set it based on data from other sources.
    #[serde(rename="localEstimatedOrActualArrivalDateTime")]
    
    pub local_estimated_or_actual_arrival_date_time: Option<String>,
    /// The estimated time the aircraft plans to pull from the gate or the actual time the aircraft already pulled from the gate. Note: This is not the runway time. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already pulled from the gate. Google will use it to inform the user when the flight actually departed. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. If this is not set, Google will set it based on data from other sources.
    #[serde(rename="localEstimatedOrActualDepartureDateTime")]
    
    pub local_estimated_or_actual_departure_date_time: Option<String>,
    /// The gate closing time as it would be printed on the boarding pass. Do not set this field if you do not want to print it in the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport.
    #[serde(rename="localGateClosingDateTime")]
    
    pub local_gate_closing_date_time: Option<String>,
    /// The scheduled time the aircraft plans to reach the destination gate (not the runway). Note: This field should not change too close to the flight time. For updates to departure times (delays, etc), please set `localEstimatedOrActualArrivalDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. If this is not set, Google will set it based on data from other sources.
    #[serde(rename="localScheduledArrivalDateTime")]
    
    pub local_scheduled_arrival_date_time: Option<String>,
    /// Required. The scheduled date and time when the aircraft is expected to depart the gate (not the runway) Note: This field should not change too close to the departure time. For updates to departure times (delays, etc), please set `localEstimatedOrActualDepartureDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport.
    #[serde(rename="localScheduledDepartureDateTime")]
    
    pub local_scheduled_departure_date_time: Option<String>,
    /// Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedIssuerName")]
    
    pub localized_issuer_name: Option<LocalizedString>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Identifies whether multiple users and devices will save the same object referencing this class.
    #[serde(rename="multipleDevicesAndHoldersAllowedStatus")]
    
    pub multiple_devices_and_holders_allowed_status: Option<FlightClasMultipleDevicesAndHoldersAllowedStatusEnum>,
    /// Required. Origin airport.
    
    pub origin: Option<AirportInfo>,
    /// Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="redemptionIssuers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub redemption_issuers: Option<Vec<i64>>,
    /// The review comments set by the platform when a class is marked `approved` or `rejected`.
    
    pub review: Option<Review>,
    /// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
    #[serde(rename="reviewStatus")]
    
    pub review_status: Option<FlightClasReviewStatusEnum>,
    /// Optional information about the security animation. If this is set a security animation will be rendered on pass details.
    #[serde(rename="securityAnimation")]
    
    pub security_animation: Option<SecurityAnimation>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// View Unlock Requirement options for the boarding pass.
    #[serde(rename="viewUnlockRequirement")]
    
    pub view_unlock_requirement: Option<FlightClasViewUnlockRequirementEnum>,
    /// Deprecated.
    #[serde(rename="wordMark")]
    
    pub word_mark: Option<Image>,
}

impl client::RequestValue for FlightClass {}
impl client::ResponseResult for FlightClass {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage flightclass](FlightclasAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightClassAddMessageResponse {
    /// The updated FlightClass resource.
    
    pub resource: Option<FlightClass>,
}

impl client::ResponseResult for FlightClassAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list flightclass](FlightclasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightClassListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<FlightClass>>,
}

impl client::ResponseResult for FlightClassListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightHeader {
    /// Information about airline carrier. This is a required property of `flightHeader`.
    
    pub carrier: Option<FlightCarrier>,
    /// The flight number without IATA carrier code. This field should contain only digits. This is a required property of `flightHeader`. eg: "123"
    #[serde(rename="flightNumber")]
    
    pub flight_number: Option<String>,
    /// Override value to use for flight number. The default value used for display purposes is carrier + flight_number. If a different value needs to be shown to passengers, use this field to override the default behavior. eg: "XX1234 / YY576"
    #[serde(rename="flightNumberDisplayOverride")]
    
    pub flight_number_display_override: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightHeader"`.
    
    pub kind: Option<String>,
    /// Information about operating airline carrier.
    #[serde(rename="operatingCarrier")]
    
    pub operating_carrier: Option<FlightCarrier>,
    /// The flight number used by the operating carrier without IATA carrier code. This field should contain only digits. eg: "234"
    #[serde(rename="operatingFlightNumber")]
    
    pub operating_flight_number: Option<String>,
}

impl client::Part for FlightHeader {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get flightobject](FlightobjectGetCall) (response)
/// * [insert flightobject](FlightobjectInsertCall) (request|response)
/// * [patch flightobject](FlightobjectPatchCall) (request|response)
/// * [update flightobject](FlightobjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightObject {
    /// Optional information about the partner app link.
    #[serde(rename="appLinkData")]
    
    pub app_link_data: Option<AppLinkData>,
    /// The barcode type and value.
    
    pub barcode: Option<Barcode>,
    /// Passenger specific information about boarding and seating.
    #[serde(rename="boardingAndSeatingInfo")]
    
    pub boarding_and_seating_info: Option<BoardingAndSeatingInfo>,
    /// Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
    /// A copy of the inherited fields of the parent class. These fields are retrieved during a GET.
    #[serde(rename="classReference")]
    
    pub class_reference: Option<FlightClass>,
    /// Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for Flights.
    #[serde(rename="disableExpirationNotification")]
    
    pub disable_expiration_notification: Option<bool>,
    /// Information that controls how passes are grouped together.
    #[serde(rename="groupingInfo")]
    
    pub grouping_info: Option<GroupingInfo>,
    /// Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information.
    #[serde(rename="hasLinkedDevice")]
    
    pub has_linked_device: Option<bool>,
    /// Indicates if the object has users. This field is set by the platform.
    #[serde(rename="hasUsers")]
    
    pub has_users: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightObject"`.
    
    pub kind: Option<String>,
    /// Links module data. If links module data is also defined on the class, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Required. Passenger name as it would appear on the boarding pass. eg: "Dave M Gahan" or "Gahan/Dave" or "GAHAN/DAVEM"
    #[serde(rename="passengerName")]
    
    pub passenger_name: Option<String>,
    /// Required. Information about flight reservation.
    #[serde(rename="reservationInfo")]
    
    pub reservation_info: Option<ReservationInfo>,
    /// The rotating barcode type and value.
    #[serde(rename="rotatingBarcode")]
    
    pub rotating_barcode: Option<RotatingBarcode>,
    /// An image for the security program that applies to the passenger.
    #[serde(rename="securityProgramLogo")]
    
    pub security_program_logo: Option<Image>,
    /// The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported.
    #[serde(rename="smartTapRedemptionValue")]
    
    pub smart_tap_redemption_value: Option<String>,
    /// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
    
    pub state: Option<FlightObjectStateEnum>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed.
    #[serde(rename="validTimeInterval")]
    
    pub valid_time_interval: Option<TimeInterval>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::RequestValue for FlightObject {}
impl client::ResponseResult for FlightObject {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage flightobject](FlightobjectAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightObjectAddMessageResponse {
    /// The updated FlightObject resource.
    
    pub resource: Option<FlightObject>,
}

impl client::ResponseResult for FlightObjectAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list flightobject](FlightobjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FlightObjectListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<FlightObject>>,
}

impl client::ResponseResult for FlightObjectListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FrequentFlyerInfo {
    /// Frequent flyer number. Required for each nested object of kind `walletobjects#frequentFlyerInfo`.
    #[serde(rename="frequentFlyerNumber")]
    
    pub frequent_flyer_number: Option<String>,
    /// Frequent flyer program name. eg: "Lufthansa Miles & More"
    #[serde(rename="frequentFlyerProgramName")]
    
    pub frequent_flyer_program_name: Option<LocalizedString>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#frequentFlyerInfo"`.
    
    pub kind: Option<String>,
}

impl client::Part for FrequentFlyerInfo {}


/// Generic Class
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get genericclass](GenericclasGetCall) (response)
/// * [insert genericclass](GenericclasInsertCall) (request|response)
/// * [patch genericclass](GenericclasPatchCall) (request|response)
/// * [update genericclass](GenericclasUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenericClass {
    /// Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback.
    #[serde(rename="callbackOptions")]
    
    pub callback_options: Option<CallbackOptions>,
    /// Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display.
    #[serde(rename="classTemplateInfo")]
    
    pub class_template_info: Option<ClassTemplateInfo>,
    /// Available only to Smart Tap enabled partners. Contact support for additional guidance.
    #[serde(rename="enableSmartTap")]
    
    pub enable_smart_tap: Option<bool>,
    /// Required. The unique identifier for the class. This ID must be unique across all from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    
    pub id: Option<String>,
    /// Image module data. If `imageModulesData` is also defined on the object, both will be displayed. Only one of the image from class and one from object level will be rendered when both set.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Links module data. If `linksModuleData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Identifies whether multiple users and devices will save the same object referencing this class.
    #[serde(rename="multipleDevicesAndHoldersAllowedStatus")]
    
    pub multiple_devices_and_holders_allowed_status: Option<GenericClasMultipleDevicesAndHoldersAllowedStatusEnum>,
    /// Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="redemptionIssuers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub redemption_issuers: Option<Vec<i64>>,
    /// Optional information about the security animation. If this is set a security animation will be rendered on pass details.
    #[serde(rename="securityAnimation")]
    
    pub security_animation: Option<SecurityAnimation>,
    /// Text module data. If `textModulesData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// View Unlock Requirement options for the generic pass.
    #[serde(rename="viewUnlockRequirement")]
    
    pub view_unlock_requirement: Option<GenericClasViewUnlockRequirementEnum>,
}

impl client::RequestValue for GenericClass {}
impl client::ResponseResult for GenericClass {}


/// List response which contains the list of all generic classes for a given issuer ID.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list genericclass](GenericclasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenericClassListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<GenericClass>>,
}

impl client::ResponseResult for GenericClassListResponse {}


/// Generic Object Next ID: 119
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get genericobject](GenericobjectGetCall) (response)
/// * [insert genericobject](GenericobjectInsertCall) (request|response)
/// * [patch genericobject](GenericobjectPatchCall) (request|response)
/// * [update genericobject](GenericobjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenericObject {
    /// Information about the partner app link. The maximum number of these fields displayed is 10.
    #[serde(rename="appLinkData")]
    
    pub app_link_data: Option<AppLinkData>,
    /// The barcode type and value. If pass does not have a barcode, we can allow the issuer to set Barcode.alternate_text and display just that.
    
    pub barcode: Option<Barcode>,
    /// Required. The header of the pass. This is usually the Business name such as "XXX Gym", "AAA Insurance". This field is required and appears in the header row at the very top of the pass.
    #[serde(rename="cardTitle")]
    
    pub card_title: Option<LocalizedString>,
    /// Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
    /// Specify which `GenericType` the card belongs to. Deprecated.
    #[serde(rename="genericType")]
    
    pub generic_type: Option<GenericObjectGenericTypeEnum>,
    /// Information that controls how passes are grouped together.
    #[serde(rename="groupingInfo")]
    
    pub grouping_info: Option<GroupingInfo>,
    /// Indicates if the object has users. This field is set by the platform.
    #[serde(rename="hasUsers")]
    
    pub has_users: Option<bool>,
    /// Required. The title of the pass, such as "50% off coupon" or "Library card" or "Voucher". This field is required and appears in the title row of the pass detail view.
    
    pub header: Option<LocalizedString>,
    /// Banner image displayed on the front of the card if present. The image will be displayed at 100% width.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set, the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used and if logo is not set, a color would be chosen by Google.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`.
    
    pub id: Option<String>,
    /// Image module data. Only one of the image from class and one from object level will be rendered when both set.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Links module data. If `linksModuleData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// The logo image of the pass. This image is displayed in the card detail view in upper left, and also on the list/thumbnail view. If the logo is not present, the first letter of `cardTitle` would be shown as logo.
    
    pub logo: Option<Image>,
    /// The notification settings that are enabled for this object.
    
    pub notifications: Option<Notifications>,
    /// The rotating barcode settings/details.
    #[serde(rename="rotatingBarcode")]
    
    pub rotating_barcode: Option<RotatingBarcode>,
    /// The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported.
    #[serde(rename="smartTapRedemptionValue")]
    
    pub smart_tap_redemption_value: Option<String>,
    /// The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. If this is not provided, the object would be considered `ACTIVE`.
    
    pub state: Option<GenericObjectStateEnum>,
    /// The title label of the pass, such as location where this pass can be used. Appears right above the title in the title row in the pass detail view.
    
    pub subheader: Option<LocalizedString>,
    /// Text module data. If `textModulesData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// The time period this object will be considered valid or usable. When the time period is passed, the object will be considered expired, which will affect the rendering on user's devices.
    #[serde(rename="validTimeInterval")]
    
    pub valid_time_interval: Option<TimeInterval>,
}

impl client::RequestValue for GenericObject {}
impl client::ResponseResult for GenericObject {}


/// List response which contains the list of all generic objects for a given issuer ID.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list genericobject](GenericobjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenericObjectListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<GenericObject>>,
}

impl client::ResponseResult for GenericObjectListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get giftcardclass](GiftcardclasGetCall) (response)
/// * [insert giftcardclass](GiftcardclasInsertCall) (request|response)
/// * [patch giftcardclass](GiftcardclasPatchCall) (request|response)
/// * [update giftcardclass](GiftcardclasUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GiftCardClass {
    /// Determines whether the merchant supports gift card redemption using barcode. If true, app displays a barcode for the gift card on the Gift card details screen. If false, a barcode is not displayed.
    #[serde(rename="allowBarcodeRedemption")]
    
    pub allow_barcode_redemption: Option<bool>,
    /// Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead.
    #[serde(rename="allowMultipleUsersPerObject")]
    
    pub allow_multiple_users_per_object: Option<bool>,
    /// Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback.
    #[serde(rename="callbackOptions")]
    
    pub callback_options: Option<CallbackOptions>,
    /// The label to display for the card number, such as "Card Number".
    #[serde(rename="cardNumberLabel")]
    
    pub card_number_label: Option<String>,
    /// Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display.
    #[serde(rename="classTemplateInfo")]
    
    pub class_template_info: Option<ClassTemplateInfo>,
    /// Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="enableSmartTap")]
    
    pub enable_smart_tap: Option<bool>,
    /// The label to display for event number, such as "Target Event #".
    #[serde(rename="eventNumberLabel")]
    
    pub event_number_label: Option<String>,
    /// Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object).
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<Uri>,
    /// Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="issuerName")]
    
    pub issuer_name: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardClass"`.
    
    pub kind: Option<String>,
    /// Links module data. If links module data is also defined on the object, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Translated strings for the card_number_label.
    #[serde(rename="localizedCardNumberLabel")]
    
    pub localized_card_number_label: Option<LocalizedString>,
    /// Translated strings for the event_number_label.
    #[serde(rename="localizedEventNumberLabel")]
    
    pub localized_event_number_label: Option<LocalizedString>,
    /// Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedIssuerName")]
    
    pub localized_issuer_name: Option<LocalizedString>,
    /// Translated strings for the merchant_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedMerchantName")]
    
    pub localized_merchant_name: Option<LocalizedString>,
    /// Translated strings for the pin_label.
    #[serde(rename="localizedPinLabel")]
    
    pub localized_pin_label: Option<LocalizedString>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// Merchant name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="merchantName")]
    
    pub merchant_name: Option<String>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Identifies whether multiple users and devices will save the same object referencing this class.
    #[serde(rename="multipleDevicesAndHoldersAllowedStatus")]
    
    pub multiple_devices_and_holders_allowed_status: Option<GiftCardClasMultipleDevicesAndHoldersAllowedStatusEnum>,
    /// The label to display for the PIN, such as "4-digit PIN".
    #[serde(rename="pinLabel")]
    
    pub pin_label: Option<String>,
    /// The logo of the gift card program or company. This logo is displayed in both the details and list views of the app.
    #[serde(rename="programLogo")]
    
    pub program_logo: Option<Image>,
    /// Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="redemptionIssuers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub redemption_issuers: Option<Vec<i64>>,
    /// The review comments set by the platform when a class is marked `approved` or `rejected`.
    
    pub review: Option<Review>,
    /// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
    #[serde(rename="reviewStatus")]
    
    pub review_status: Option<GiftCardClasReviewStatusEnum>,
    /// Optional information about the security animation. If this is set a security animation will be rendered on pass details.
    #[serde(rename="securityAnimation")]
    
    pub security_animation: Option<SecurityAnimation>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// View Unlock Requirement options for the gift card.
    #[serde(rename="viewUnlockRequirement")]
    
    pub view_unlock_requirement: Option<GiftCardClasViewUnlockRequirementEnum>,
    /// Deprecated.
    #[serde(rename="wordMark")]
    
    pub word_mark: Option<Image>,
}

impl client::RequestValue for GiftCardClass {}
impl client::ResponseResult for GiftCardClass {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage giftcardclass](GiftcardclasAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GiftCardClassAddMessageResponse {
    /// The updated GiftCardClass resource.
    
    pub resource: Option<GiftCardClass>,
}

impl client::ResponseResult for GiftCardClassAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list giftcardclass](GiftcardclasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GiftCardClassListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<GiftCardClass>>,
}

impl client::ResponseResult for GiftCardClassListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get giftcardobject](GiftcardobjectGetCall) (response)
/// * [insert giftcardobject](GiftcardobjectInsertCall) (request|response)
/// * [patch giftcardobject](GiftcardobjectPatchCall) (request|response)
/// * [update giftcardobject](GiftcardobjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GiftCardObject {
    /// Optional information about the partner app link.
    #[serde(rename="appLinkData")]
    
    pub app_link_data: Option<AppLinkData>,
    /// The card's monetary balance.
    
    pub balance: Option<Money>,
    /// The date and time when the balance was last updated. Offset is required. If balance is updated and this property is not provided, system will default to the current time.
    #[serde(rename="balanceUpdateTime")]
    
    pub balance_update_time: Option<DateTime>,
    /// The barcode type and value.
    
    pub barcode: Option<Barcode>,
    /// Required. The card's number.
    #[serde(rename="cardNumber")]
    
    pub card_number: Option<String>,
    /// Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
    /// A copy of the inherited fields of the parent class. These fields are retrieved during a GET.
    #[serde(rename="classReference")]
    
    pub class_reference: Option<GiftCardClass>,
    /// Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers.
    #[serde(rename="disableExpirationNotification")]
    
    pub disable_expiration_notification: Option<bool>,
    /// The card's event number, an optional field used by some gift cards.
    #[serde(rename="eventNumber")]
    
    pub event_number: Option<String>,
    /// Information that controls how passes are grouped together.
    #[serde(rename="groupingInfo")]
    
    pub grouping_info: Option<GroupingInfo>,
    /// Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information.
    #[serde(rename="hasLinkedDevice")]
    
    pub has_linked_device: Option<bool>,
    /// Indicates if the object has users. This field is set by the platform.
    #[serde(rename="hasUsers")]
    
    pub has_users: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardObject"`.
    
    pub kind: Option<String>,
    /// Links module data. If links module data is also defined on the class, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// The card's PIN.
    
    pub pin: Option<String>,
    /// The rotating barcode type and value.
    #[serde(rename="rotatingBarcode")]
    
    pub rotating_barcode: Option<RotatingBarcode>,
    /// The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported.
    #[serde(rename="smartTapRedemptionValue")]
    
    pub smart_tap_redemption_value: Option<String>,
    /// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
    
    pub state: Option<GiftCardObjectStateEnum>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed.
    #[serde(rename="validTimeInterval")]
    
    pub valid_time_interval: Option<TimeInterval>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::RequestValue for GiftCardObject {}
impl client::ResponseResult for GiftCardObject {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage giftcardobject](GiftcardobjectAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GiftCardObjectAddMessageResponse {
    /// The updated GiftCardObject resource.
    
    pub resource: Option<GiftCardObject>,
}

impl client::ResponseResult for GiftCardObjectAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list giftcardobject](GiftcardobjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GiftCardObjectListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<GiftCardObject>>,
}

impl client::ResponseResult for GiftCardObjectListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GroupingInfo {
    /// Optional grouping ID for grouping the passes with the same ID visually together. Grouping with different types of passes is allowed.
    #[serde(rename="groupingId")]
    
    pub grouping_id: Option<String>,
    /// Optional index for sorting the passes when they are grouped with other passes. Passes with lower sort index are shown before passes with higher sort index. If unspecified, the value is assumed to be INT_MAX. For two passes with the same sort index, the sorting behavior is undefined.
    #[serde(rename="sortIndex")]
    
    pub sort_index: Option<i32>,
}

impl client::Part for GroupingInfo {}


/// Wrapping type for Google hosted images.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// Description of the image used for accessibility.
    #[serde(rename="contentDescription")]
    
    pub content_description: Option<LocalizedString>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#image"`.
    
    pub kind: Option<String>,
    /// The URI for the image.
    #[serde(rename="sourceUri")]
    
    pub source_uri: Option<ImageUri>,
}

impl client::Part for Image {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageModuleData {
    /// The ID associated with an image module. This field is here to enable ease of management of image modules.
    
    pub id: Option<String>,
    /// A 100% width image.
    #[serde(rename="mainImage")]
    
    pub main_image: Option<Image>,
}

impl client::Part for ImageModuleData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageUri {
    /// Additional information about the image, which is unused and retained only for backward compatibility.
    
    pub description: Option<String>,
    /// Translated strings for the description, which are unused and retained only for backward compatibility.
    #[serde(rename="localizedDescription")]
    
    pub localized_description: Option<LocalizedString>,
    /// The location of the image. URIs must have a scheme.
    
    pub uri: Option<String>,
}

impl client::Part for ImageUri {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InfoModuleData {
    /// A list of collections of labels and values. These will be displayed one after the other in a singular column.
    #[serde(rename="labelValueRows")]
    
    pub label_value_rows: Option<Vec<LabelValueRow>>,
    /// no description provided
    #[serde(rename="showLastUpdateTime")]
    
    pub show_last_update_time: Option<bool>,
}

impl client::Part for InfoModuleData {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get issuer](IssuerGetCall) (response)
/// * [insert issuer](IssuerInsertCall) (request|response)
/// * [patch issuer](IssuerPatchCall) (request|response)
/// * [update issuer](IssuerUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Issuer {
    /// Issuer contact information.
    #[serde(rename="contactInfo")]
    
    pub contact_info: Option<IssuerContactInfo>,
    /// URL for the issuer's home page.
    #[serde(rename="homepageUrl")]
    
    pub homepage_url: Option<String>,
    /// The unique identifier for an issuer account. This is automatically generated when the issuer is inserted.
    #[serde(rename="issuerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub issuer_id: Option<i64>,
    /// The account name of the issuer.
    
    pub name: Option<String>,
    /// Available only to Smart Tap enabled partners. Contact support for additional guidance.
    #[serde(rename="smartTapMerchantData")]
    
    pub smart_tap_merchant_data: Option<SmartTapMerchantData>,
}

impl client::RequestValue for Issuer {}
impl client::ResponseResult for Issuer {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IssuerContactInfo {
    /// Email addresses which will receive alerts.
    #[serde(rename="alertsEmails")]
    
    pub alerts_emails: Option<Vec<String>>,
    /// The primary contact email address.
    
    pub email: Option<String>,
    /// The primary contact name.
    
    pub name: Option<String>,
    /// The primary contact phone number.
    
    pub phone: Option<String>,
}

impl client::Part for IssuerContactInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list issuer](IssuerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IssuerListResponse {
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<Issuer>>,
}

impl client::ResponseResult for IssuerListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IssuerToUserInfo {
    /// no description provided
    
    pub action: Option<IssuerToUserInfoActionEnum>,
    /// no description provided
    #[serde(rename="signUpInfo")]
    
    pub sign_up_info: Option<SignUpInfo>,
    /// Currently not used, consider deprecating.
    
    pub url: Option<String>,
    /// JSON web token for action S2AP.
    
    pub value: Option<String>,
}

impl client::Part for IssuerToUserInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert jwt](JwtInsertCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JwtInsertResponse {
    /// Data that corresponds to the ids of the provided classes and objects in the JWT. resources will only include the non-empty arrays (i.e. if the JWT only includes eventTicketObjects, then that is the only field that will be present in resources).
    
    pub resources: Option<Resources>,
    /// A URI that, when opened, will allow the end user to save the object(s) identified in the JWT to their Google account.
    #[serde(rename="saveUri")]
    
    pub save_uri: Option<String>,
}

impl client::ResponseResult for JwtInsertResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert jwt](JwtInsertCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JwtResource {
    /// A string representing a JWT of the format described at https://developers.google.com/pay/passes/reference/s2w-reference#google-pay-api-for-passes-jwt
    
    pub jwt: Option<String>,
}

impl client::RequestValue for JwtResource {}


/// A pair of text strings to be displayed in the details view. Note we no longer display LabelValue/LabelValueRow as a table, instead a list of items.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelValue {
    /// The label for a specific row and column. Recommended maximum is 15 characters for a two-column layout and 30 characters for a one-column layout.
    
    pub label: Option<String>,
    /// Translated strings for the label. Recommended maximum is 15 characters for a two-column layout and 30 characters for a one-column layout.
    #[serde(rename="localizedLabel")]
    
    pub localized_label: Option<LocalizedString>,
    /// Translated strings for the value. Recommended maximum is 15 characters for a two-column layout and 30 characters for a one-column layout.
    #[serde(rename="localizedValue")]
    
    pub localized_value: Option<LocalizedString>,
    /// The value for a specific row and column. Recommended maximum is 15 characters for a two-column layout and 30 characters for a one-column layout.
    
    pub value: Option<String>,
}

impl client::Part for LabelValue {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelValueRow {
    /// A list of labels and values. These will be displayed in a singular column, one after the other, not in multiple columns, despite the field name.
    
    pub columns: Option<Vec<LabelValue>>,
}

impl client::Part for LabelValueRow {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLongPoint {
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#latLongPoint"`.
    
    pub kind: Option<String>,
    /// The latitude specified as any value in the range of -90.0 through +90.0, both inclusive. Values outside these bounds will be rejected.
    
    pub latitude: Option<f64>,
    /// The longitude specified in the range -180.0 through +180.0, both inclusive. Values outside these bounds will be rejected.
    
    pub longitude: Option<f64>,
}

impl client::Part for LatLongPoint {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LinksModuleData {
    /// The list of URIs.
    
    pub uris: Option<Vec<Uri>>,
}

impl client::Part for LinksModuleData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListTemplateOverride {
    /// Specifies from a predefined set of options or from a reference to the field what will be displayed in the first row.
    #[serde(rename="firstRowOption")]
    
    pub first_row_option: Option<FirstRowOption>,
    /// A reference to the field to be displayed in the second row. This option is only displayed if there are not multiple user objects in a group. If there is a group, the second row will always display a field shared by all objects.
    #[serde(rename="secondRowOption")]
    
    pub second_row_option: Option<FieldSelector>,
    /// A reference to the field to be displayed in the third row. This option is only displayed if there are not multiple user objects in a group. If there is a group, the third row will always display the number of objects in the group. Eg: "3 passes"
    #[serde(rename="thirdRowOption")]
    
    pub third_row_option: Option<FieldSelector>,
}

impl client::Part for ListTemplateOverride {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocalizedString {
    /// Contains the string to be displayed if no appropriate translation is available.
    #[serde(rename="defaultValue")]
    
    pub default_value: Option<TranslatedString>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#localizedString"`.
    
    pub kind: Option<String>,
    /// Contains the translations for the string.
    #[serde(rename="translatedValues")]
    
    pub translated_values: Option<Vec<TranslatedString>>,
}

impl client::Part for LocalizedString {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get loyaltyclass](LoyaltyclasGetCall) (response)
/// * [insert loyaltyclass](LoyaltyclasInsertCall) (request|response)
/// * [patch loyaltyclass](LoyaltyclasPatchCall) (request|response)
/// * [update loyaltyclass](LoyaltyclasUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyClass {
    /// The account ID label, such as "Member ID." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="accountIdLabel")]
    
    pub account_id_label: Option<String>,
    /// The account name label, such as "Member Name." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="accountNameLabel")]
    
    pub account_name_label: Option<String>,
    /// Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead.
    #[serde(rename="allowMultipleUsersPerObject")]
    
    pub allow_multiple_users_per_object: Option<bool>,
    /// Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback.
    #[serde(rename="callbackOptions")]
    
    pub callback_options: Option<CallbackOptions>,
    /// Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display.
    #[serde(rename="classTemplateInfo")]
    
    pub class_template_info: Option<ClassTemplateInfo>,
    /// Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// Information about how the class may be discovered and instantiated from within the Google Pay app.
    #[serde(rename="discoverableProgram")]
    
    pub discoverable_program: Option<DiscoverableProgram>,
    /// Identifies whether this class supports Smart Tap. The `redemptionIssuers` and one of object level `smartTapRedemptionLevel`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="enableSmartTap")]
    
    pub enable_smart_tap: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object).
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<Uri>,
    /// Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="issuerName")]
    
    pub issuer_name: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyClass"`.
    
    pub kind: Option<String>,
    /// Links module data. If links module data is also defined on the object, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Translated strings for the account_id_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedAccountIdLabel")]
    
    pub localized_account_id_label: Option<LocalizedString>,
    /// Translated strings for the account_name_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedAccountNameLabel")]
    
    pub localized_account_name_label: Option<LocalizedString>,
    /// Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedIssuerName")]
    
    pub localized_issuer_name: Option<LocalizedString>,
    /// Translated strings for the program_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedProgramName")]
    
    pub localized_program_name: Option<LocalizedString>,
    /// Translated strings for the rewards_tier. Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedRewardsTier")]
    
    pub localized_rewards_tier: Option<LocalizedString>,
    /// Translated strings for the rewards_tier_label. Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedRewardsTierLabel")]
    
    pub localized_rewards_tier_label: Option<LocalizedString>,
    /// Translated strings for the secondary_rewards_tier.
    #[serde(rename="localizedSecondaryRewardsTier")]
    
    pub localized_secondary_rewards_tier: Option<LocalizedString>,
    /// Translated strings for the secondary_rewards_tier_label.
    #[serde(rename="localizedSecondaryRewardsTierLabel")]
    
    pub localized_secondary_rewards_tier_label: Option<LocalizedString>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Identifies whether multiple users and devices will save the same object referencing this class.
    #[serde(rename="multipleDevicesAndHoldersAllowedStatus")]
    
    pub multiple_devices_and_holders_allowed_status: Option<LoyaltyClasMultipleDevicesAndHoldersAllowedStatusEnum>,
    /// Required. The logo of the loyalty program or company. This logo is displayed in both the details and list views of the app.
    #[serde(rename="programLogo")]
    
    pub program_logo: Option<Image>,
    /// Required. The program name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="programName")]
    
    pub program_name: Option<String>,
    /// Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and one of object level `smartTapRedemptionValue`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="redemptionIssuers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub redemption_issuers: Option<Vec<i64>>,
    /// The review comments set by the platform when a class is marked `approved` or `rejected`.
    
    pub review: Option<Review>,
    /// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
    #[serde(rename="reviewStatus")]
    
    pub review_status: Option<LoyaltyClasReviewStatusEnum>,
    /// The rewards tier, such as "Gold" or "Platinum." Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="rewardsTier")]
    
    pub rewards_tier: Option<String>,
    /// The rewards tier label, such as "Rewards Tier." Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="rewardsTierLabel")]
    
    pub rewards_tier_label: Option<String>,
    /// The secondary rewards tier, such as "Gold" or "Platinum."
    #[serde(rename="secondaryRewardsTier")]
    
    pub secondary_rewards_tier: Option<String>,
    /// The secondary rewards tier label, such as "Rewards Tier."
    #[serde(rename="secondaryRewardsTierLabel")]
    
    pub secondary_rewards_tier_label: Option<String>,
    /// Optional information about the security animation. If this is set a security animation will be rendered on pass details.
    #[serde(rename="securityAnimation")]
    
    pub security_animation: Option<SecurityAnimation>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// View Unlock Requirement options for the loyalty card.
    #[serde(rename="viewUnlockRequirement")]
    
    pub view_unlock_requirement: Option<LoyaltyClasViewUnlockRequirementEnum>,
    /// Deprecated.
    #[serde(rename="wordMark")]
    
    pub word_mark: Option<Image>,
}

impl client::RequestValue for LoyaltyClass {}
impl client::ResponseResult for LoyaltyClass {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage loyaltyclass](LoyaltyclasAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyClassAddMessageResponse {
    /// The updated LoyaltyClass resource.
    
    pub resource: Option<LoyaltyClass>,
}

impl client::ResponseResult for LoyaltyClassAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list loyaltyclass](LoyaltyclasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyClassListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<LoyaltyClass>>,
}

impl client::ResponseResult for LoyaltyClassListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get loyaltyobject](LoyaltyobjectGetCall) (response)
/// * [insert loyaltyobject](LoyaltyobjectInsertCall) (request|response)
/// * [modifylinkedofferobjects loyaltyobject](LoyaltyobjectModifylinkedofferobjectCall) (response)
/// * [patch loyaltyobject](LoyaltyobjectPatchCall) (request|response)
/// * [update loyaltyobject](LoyaltyobjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyObject {
    /// The loyalty account identifier. Recommended maximum length is 20 characters.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The loyalty account holder name, such as "John Smith." Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="accountName")]
    
    pub account_name: Option<String>,
    /// Optional information about the partner app link.
    #[serde(rename="appLinkData")]
    
    pub app_link_data: Option<AppLinkData>,
    /// The barcode type and value.
    
    pub barcode: Option<Barcode>,
    /// Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
    /// A copy of the inherited fields of the parent class. These fields are retrieved during a GET.
    #[serde(rename="classReference")]
    
    pub class_reference: Option<LoyaltyClass>,
    /// Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers.
    #[serde(rename="disableExpirationNotification")]
    
    pub disable_expiration_notification: Option<bool>,
    /// Information that controls how passes are grouped together.
    #[serde(rename="groupingInfo")]
    
    pub grouping_info: Option<GroupingInfo>,
    /// Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information.
    #[serde(rename="hasLinkedDevice")]
    
    pub has_linked_device: Option<bool>,
    /// Indicates if the object has users. This field is set by the platform.
    #[serde(rename="hasUsers")]
    
    pub has_users: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyObject"`.
    
    pub kind: Option<String>,
    /// A list of offer objects linked to this loyalty card. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="linkedOfferIds")]
    
    pub linked_offer_ids: Option<Vec<String>>,
    /// Links module data. If links module data is also defined on the class, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// The loyalty reward points label, balance, and type.
    #[serde(rename="loyaltyPoints")]
    
    pub loyalty_points: Option<LoyaltyPoints>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// The rotating barcode type and value.
    #[serde(rename="rotatingBarcode")]
    
    pub rotating_barcode: Option<RotatingBarcode>,
    /// The secondary loyalty reward points label, balance, and type. Shown in addition to the primary loyalty points.
    #[serde(rename="secondaryLoyaltyPoints")]
    
    pub secondary_loyalty_points: Option<LoyaltyPoints>,
    /// The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. If this value is not set but the class level fields `enableSmartTap` and `redemptionIssuers` are set up correctly, the `barcode.value` or the `accountId` fields are used as fallback if present.
    #[serde(rename="smartTapRedemptionValue")]
    
    pub smart_tap_redemption_value: Option<String>,
    /// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
    
    pub state: Option<LoyaltyObjectStateEnum>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed.
    #[serde(rename="validTimeInterval")]
    
    pub valid_time_interval: Option<TimeInterval>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::RequestValue for LoyaltyObject {}
impl client::ResponseResult for LoyaltyObject {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage loyaltyobject](LoyaltyobjectAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyObjectAddMessageResponse {
    /// The updated LoyaltyObject resource.
    
    pub resource: Option<LoyaltyObject>,
}

impl client::ResponseResult for LoyaltyObjectAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list loyaltyobject](LoyaltyobjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyObjectListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<LoyaltyObject>>,
}

impl client::ResponseResult for LoyaltyObjectListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyPoints {
    /// The account holder's loyalty point balance, such as "500" or "$10.00". Recommended maximum length is 7 characters. This is a required field of `loyaltyPoints` and `secondaryLoyaltyPoints`.
    
    pub balance: Option<LoyaltyPointsBalance>,
    /// The loyalty points label, such as "Points". Recommended maximum length is 9 characters.
    
    pub label: Option<String>,
    /// Translated strings for the label. Recommended maximum length is 9 characters.
    #[serde(rename="localizedLabel")]
    
    pub localized_label: Option<LocalizedString>,
}

impl client::Part for LoyaltyPoints {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LoyaltyPointsBalance {
    /// The double form of a balance. Only one of these subtypes (string, int, double, money) should be populated.
    
    pub double: Option<f64>,
    /// The integer form of a balance. Only one of these subtypes (string, int, double, money) should be populated.
    
    pub int: Option<i32>,
    /// The money form of a balance. Only one of these subtypes (string, int, double, money) should be populated.
    
    pub money: Option<Money>,
    /// The string form of a balance. Only one of these subtypes (string, int, double, money) should be populated.
    
    pub string: Option<String>,
}

impl client::Part for LoyaltyPointsBalance {}


/// A reference to data stored on the filesystem, on GFS or in blobstore.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    /// Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time.
    
    pub algorithm: Option<String>,
    /// Use object_id instead.
    #[serde(rename="bigstoreObjectRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub bigstore_object_ref: Option<Vec<u8>>,
    /// Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef.
    #[serde(rename="blobRef")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub blob_ref: Option<Vec<u8>>,
    /// Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob.
    #[serde(rename="blobstore2Info")]
    
    pub blobstore2_info: Option<Blobstore2Info>,
    /// A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified.
    #[serde(rename="compositeMedia")]
    
    pub composite_media: Option<Vec<CompositeMedia>>,
    /// MIME type of the data
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// Extended content type information provided for Scotty uploads.
    #[serde(rename="contentTypeInfo")]
    
    pub content_type_info: Option<ContentTypeInfo>,
    /// A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field.
    #[serde(rename="cosmoBinaryReference")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub cosmo_binary_reference: Option<Vec<u8>>,
    /// For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported.
    #[serde(rename="crc32cHash")]
    
    pub crc32c_hash: Option<u32>,
    /// Set if reference_type is DIFF_CHECKSUMS_RESPONSE.
    #[serde(rename="diffChecksumsResponse")]
    
    pub diff_checksums_response: Option<DiffChecksumsResponse>,
    /// Set if reference_type is DIFF_DOWNLOAD_RESPONSE.
    #[serde(rename="diffDownloadResponse")]
    
    pub diff_download_response: Option<DiffDownloadResponse>,
    /// Set if reference_type is DIFF_UPLOAD_REQUEST.
    #[serde(rename="diffUploadRequest")]
    
    pub diff_upload_request: Option<DiffUploadRequest>,
    /// Set if reference_type is DIFF_UPLOAD_RESPONSE.
    #[serde(rename="diffUploadResponse")]
    
    pub diff_upload_response: Option<DiffUploadResponse>,
    /// Set if reference_type is DIFF_VERSION_RESPONSE.
    #[serde(rename="diffVersionResponse")]
    
    pub diff_version_response: Option<DiffVersionResponse>,
    /// Parameters for a media download.
    #[serde(rename="downloadParameters")]
    
    pub download_parameters: Option<DownloadParameters>,
    /// Original file name
    
    pub filename: Option<String>,
    /// Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/uploader/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media.
    
    pub hash: Option<String>,
    /// For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification.
    #[serde(rename="hashVerified")]
    
    pub hash_verified: Option<bool>,
    /// Media data, set if reference_type is INLINE
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub inline: Option<Vec<u8>>,
    /// |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence.
    #[serde(rename="isPotentialRetry")]
    
    pub is_potential_retry: Option<bool>,
    /// Size of the data, in bytes
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub length: Option<i64>,
    /// Scotty-provided MD5 hash for an upload.
    #[serde(rename="md5Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub md5_hash: Option<Vec<u8>>,
    /// Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA.
    #[serde(rename="mediaId")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub media_id: Option<Vec<u8>>,
    /// Reference to a TI Blob, set if reference_type is BIGSTORE_REF.
    #[serde(rename="objectId")]
    
    pub object_id: Option<ObjectId>,
    /// Path to the data, set if reference_type is PATH
    
    pub path: Option<String>,
    /// Describes what the field reference contains.
    #[serde(rename="referenceType")]
    
    pub reference_type: Option<MediaReferenceTypeEnum>,
    /// Scotty-provided SHA1 hash for an upload.
    #[serde(rename="sha1Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha1_hash: Option<Vec<u8>>,
    /// Scotty-provided SHA256 hash for an upload.
    #[serde(rename="sha256Hash")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub sha256_hash: Option<Vec<u8>>,
    /// Time at which the media data was last updated, in milliseconds since UNIX epoch
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp: Option<u64>,
    /// A unique fingerprint/version id for the media data
    
    pub token: Option<String>,
}

impl client::Part for Media {}


/// Extra information added to operations that support Scotty media requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaRequestInfo {
    /// The number of current bytes uploaded or downloaded.
    #[serde(rename="currentBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub current_bytes: Option<i64>,
    /// Data to be copied to backend requests. Custom data is returned to Scotty in the agent_state field, which Scotty will then provide in subsequent upload notifications.
    #[serde(rename="customData")]
    
    pub custom_data: Option<String>,
    /// Set if the http request info is diff encoded. The value of this field is the version number of the base revision. This is corresponding to Apiary's mediaDiffObjectVersion (//depot/google3/java/com/google/api/server/media/variable/DiffObjectVersionVariable.java). See go/esf-scotty-diff-upload for more information.
    #[serde(rename="diffObjectVersion")]
    
    pub diff_object_version: Option<String>,
    /// The existence of the final_status field indicates that this is the last call to the agent for this request_id. http://google3/uploader/agent/scotty_agent.proto?l=737&rcl=347601929
    #[serde(rename="finalStatus")]
    
    pub final_status: Option<i32>,
    /// The type of notification received from Scotty.
    #[serde(rename="notificationType")]
    
    pub notification_type: Option<MediaRequestInfoNotificationTypeEnum>,
    /// The Scotty request ID.
    #[serde(rename="requestId")]
    
    pub request_id: Option<String>,
    /// The total size of the file.
    #[serde(rename="totalBytes")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes: Option<i64>,
    /// Whether the total bytes field contains an estimated data.
    #[serde(rename="totalBytesIsEstimated")]
    
    pub total_bytes_is_estimated: Option<bool>,
}

impl client::Part for MediaRequestInfo {}


/// A message that will be displayed with a Valuable
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// The message body.
    
    pub body: Option<String>,
    /// The period of time that the message will be displayed to users. You can define both a `startTime` and `endTime` for each message. A message is displayed immediately after a Wallet Object is inserted unless a `startTime` is set. The message will appear in a list of messages indefinitely if `endTime` is not provided.
    #[serde(rename="displayInterval")]
    
    pub display_interval: Option<TimeInterval>,
    /// The message header.
    
    pub header: Option<String>,
    /// The ID associated with a message. This field is here to enable ease of management of messages. Notice ID values could possibly duplicate across multiple messages in the same class/instance, and care must be taken to select a reasonable ID for each message.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#walletObjectMessage"`.
    
    pub kind: Option<String>,
    /// Translated strings for the message body.
    #[serde(rename="localizedBody")]
    
    pub localized_body: Option<LocalizedString>,
    /// Translated strings for the message header.
    #[serde(rename="localizedHeader")]
    
    pub localized_header: Option<LocalizedString>,
    /// The type of the message. Currently, this can only be set for offers.
    #[serde(rename="messageType")]
    
    pub message_type: Option<MessageMessageTypeEnum>,
}

impl client::Part for Message {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyLinkedOfferObjects {
    /// The linked offer object ids to add to the object.
    #[serde(rename="addLinkedOfferObjectIds")]
    
    pub add_linked_offer_object_ids: Option<Vec<String>>,
    /// The linked offer object ids to remove from the object.
    #[serde(rename="removeLinkedOfferObjectIds")]
    
    pub remove_linked_offer_object_ids: Option<Vec<String>>,
}

impl client::Part for ModifyLinkedOfferObjects {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [modifylinkedofferobjects eventticketobject](EventticketobjectModifylinkedofferobjectCall) (request)
/// * [modifylinkedofferobjects loyaltyobject](LoyaltyobjectModifylinkedofferobjectCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyLinkedOfferObjectsRequest {
    /// The linked offer object ids to add or remove from the object.
    #[serde(rename="linkedOfferObjectIds")]
    
    pub linked_offer_object_ids: Option<ModifyLinkedOfferObjects>,
}

impl client::RequestValue for ModifyLinkedOfferObjectsRequest {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Money {
    /// The currency code, such as "USD" or "EUR."
    #[serde(rename="currencyCode")]
    
    pub currency_code: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#money"`.
    
    pub kind: Option<String>,
    /// The unit of money amount in micros. For example, $1 USD would be represented as 1000000 micros.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub micros: Option<i64>,
}

impl client::Part for Money {}


/// Indicates if the object needs to have notification enabled. We support only one of ExpiryNotification/UpcomingNotification. `expiryNotification` takes precedence over `upcomingNotification`. In other words if `expiryNotification` is set, we ignore the `upcomingNotification` field.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notifications {
    /// A notification would be triggered at a specific time before the card expires.
    #[serde(rename="expiryNotification")]
    
    pub expiry_notification: Option<ExpiryNotification>,
    /// A notification would be triggered at a specific time before the card becomes usable.
    #[serde(rename="upcomingNotification")]
    
    pub upcoming_notification: Option<UpcomingNotification>,
}

impl client::Part for Notifications {}


/// This is a copy of the tech.blob.ObjectId proto, which could not be used directly here due to transitive closure issues with JavaScript support; see http://b/8801763.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectId {
    /// The name of the bucket to which this object belongs.
    #[serde(rename="bucketName")]
    
    pub bucket_name: Option<String>,
    /// Generation of the object. Generations are monotonically increasing across writes, allowing them to be be compared to determine which generation is newer. If this is omitted in a request, then you are requesting the live object. See http://go/bigstore-versions
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// The name of the object.
    #[serde(rename="objectName")]
    
    pub object_name: Option<String>,
}

impl client::Part for ObjectId {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get offerclass](OfferclasGetCall) (response)
/// * [insert offerclass](OfferclasInsertCall) (request|response)
/// * [patch offerclass](OfferclasPatchCall) (request|response)
/// * [update offerclass](OfferclasUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferClass {
    /// Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead.
    #[serde(rename="allowMultipleUsersPerObject")]
    
    pub allow_multiple_users_per_object: Option<bool>,
    /// Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback.
    #[serde(rename="callbackOptions")]
    
    pub callback_options: Option<CallbackOptions>,
    /// Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display.
    #[serde(rename="classTemplateInfo")]
    
    pub class_template_info: Option<ClassTemplateInfo>,
    /// Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// The details of the offer.
    
    pub details: Option<String>,
    /// Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="enableSmartTap")]
    
    pub enable_smart_tap: Option<bool>,
    /// The fine print or terms of the offer, such as "20% off any t-shirt at Adam's Apparel."
    #[serde(rename="finePrint")]
    
    pub fine_print: Option<String>,
    /// The help link for the offer, such as `http://myownpersonaldomain.com/help`
    #[serde(rename="helpUri")]
    
    pub help_uri: Option<Uri>,
    /// Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object).
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<Uri>,
    /// Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="issuerName")]
    
    pub issuer_name: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerClass"`.
    
    pub kind: Option<String>,
    /// Links module data. If links module data is also defined on the object, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Translated strings for the details.
    #[serde(rename="localizedDetails")]
    
    pub localized_details: Option<LocalizedString>,
    /// Translated strings for the fine_print.
    #[serde(rename="localizedFinePrint")]
    
    pub localized_fine_print: Option<LocalizedString>,
    /// Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedIssuerName")]
    
    pub localized_issuer_name: Option<LocalizedString>,
    /// Translated strings for the provider. Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedProvider")]
    
    pub localized_provider: Option<LocalizedString>,
    /// Translated strings for the short title. Recommended maximum length is 20 characters.
    #[serde(rename="localizedShortTitle")]
    
    pub localized_short_title: Option<LocalizedString>,
    /// Translated strings for the title. Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedTitle")]
    
    pub localized_title: Option<LocalizedString>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Identifies whether multiple users and devices will save the same object referencing this class.
    #[serde(rename="multipleDevicesAndHoldersAllowedStatus")]
    
    pub multiple_devices_and_holders_allowed_status: Option<OfferClasMultipleDevicesAndHoldersAllowedStatusEnum>,
    /// Required. The offer provider (either the aggregator name or merchant name). Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens.
    
    pub provider: Option<String>,
    /// Required. The redemption channels applicable to this offer.
    #[serde(rename="redemptionChannel")]
    
    pub redemption_channel: Option<OfferClasRedemptionChannelEnum>,
    /// Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="redemptionIssuers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub redemption_issuers: Option<Vec<i64>>,
    /// The review comments set by the platform when a class is marked `approved` or `rejected`.
    
    pub review: Option<Review>,
    /// Required. The status of the class. This field can be set to `draft` or The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
    #[serde(rename="reviewStatus")]
    
    pub review_status: Option<OfferClasReviewStatusEnum>,
    /// Optional information about the security animation. If this is set a security animation will be rendered on pass details.
    #[serde(rename="securityAnimation")]
    
    pub security_animation: Option<SecurityAnimation>,
    /// A shortened version of the title of the offer, such as "20% off," shown to users as a quick reference to the offer contents. Recommended maximum length is 20 characters.
    #[serde(rename="shortTitle")]
    
    pub short_title: Option<String>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// Required. The title of the offer, such as "20% off any t-shirt." Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens.
    
    pub title: Option<String>,
    /// The title image of the offer. This image is displayed in both the details and list views of the app.
    #[serde(rename="titleImage")]
    
    pub title_image: Option<Image>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// View Unlock Requirement options for the offer.
    #[serde(rename="viewUnlockRequirement")]
    
    pub view_unlock_requirement: Option<OfferClasViewUnlockRequirementEnum>,
    /// Deprecated.
    #[serde(rename="wordMark")]
    
    pub word_mark: Option<Image>,
}

impl client::RequestValue for OfferClass {}
impl client::ResponseResult for OfferClass {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage offerclass](OfferclasAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferClassAddMessageResponse {
    /// The updated OfferClass resource.
    
    pub resource: Option<OfferClass>,
}

impl client::ResponseResult for OfferClassAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list offerclass](OfferclasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferClassListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<OfferClass>>,
}

impl client::ResponseResult for OfferClassListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get offerobject](OfferobjectGetCall) (response)
/// * [insert offerobject](OfferobjectInsertCall) (request|response)
/// * [patch offerobject](OfferobjectPatchCall) (request|response)
/// * [update offerobject](OfferobjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferObject {
    /// Optional information about the partner app link.
    #[serde(rename="appLinkData")]
    
    pub app_link_data: Option<AppLinkData>,
    /// The barcode type and value.
    
    pub barcode: Option<Barcode>,
    /// Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
    /// A copy of the inherited fields of the parent class. These fields are retrieved during a GET.
    #[serde(rename="classReference")]
    
    pub class_reference: Option<OfferClass>,
    /// Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers.
    #[serde(rename="disableExpirationNotification")]
    
    pub disable_expiration_notification: Option<bool>,
    /// Information that controls how passes are grouped together.
    #[serde(rename="groupingInfo")]
    
    pub grouping_info: Option<GroupingInfo>,
    /// Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information.
    #[serde(rename="hasLinkedDevice")]
    
    pub has_linked_device: Option<bool>,
    /// Indicates if the object has users. This field is set by the platform.
    #[serde(rename="hasUsers")]
    
    pub has_users: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerObject"`.
    
    pub kind: Option<String>,
    /// Links module data. If links module data is also defined on the class, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// The rotating barcode type and value.
    #[serde(rename="rotatingBarcode")]
    
    pub rotating_barcode: Option<RotatingBarcode>,
    /// The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported.
    #[serde(rename="smartTapRedemptionValue")]
    
    pub smart_tap_redemption_value: Option<String>,
    /// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
    
    pub state: Option<OfferObjectStateEnum>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed.
    #[serde(rename="validTimeInterval")]
    
    pub valid_time_interval: Option<TimeInterval>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::RequestValue for OfferObject {}
impl client::ResponseResult for OfferObject {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage offerobject](OfferobjectAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferObjectAddMessageResponse {
    /// The updated OfferObject resource.
    
    pub resource: Option<OfferObject>,
}

impl client::ResponseResult for OfferObjectAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list offerobject](OfferobjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OfferObjectListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<OfferObject>>,
}

impl client::ResponseResult for OfferObjectListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#pagination"`.
    
    pub kind: Option<String>,
    /// Page token to send to fetch the next page.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Number of results returned in this page.
    #[serde(rename="resultsPerPage")]
    
    pub results_per_page: Option<i32>,
}

impl client::Part for Pagination {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get permissions](PermissionGetCall) (none)
/// * [update permissions](PermissionUpdateCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permission {
    /// The email address of the user, group, or service account to which this permission refers to.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// The role granted by this permission.
    
    pub role: Option<PermissionRoleEnum>,
}

impl client::Resource for Permission {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get permissions](PermissionGetCall) (response)
/// * [update permissions](PermissionUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Permissions {
    /// ID of the issuer the list of permissions refer to.
    #[serde(rename="issuerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub issuer_id: Option<i64>,
    /// The complete list of permissions for the issuer account.
    
    pub permissions: Option<Vec<Permission>>,
}

impl client::RequestValue for Permissions {}
impl client::ResponseResult for Permissions {}


/// Private data for TextModule. This data will be rendered as a TextModule for a pass.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateText {
    /// Translated strings for the body.
    
    pub body: Option<LocalizedString>,
    /// Translated strings for the header.
    
    pub header: Option<LocalizedString>,
}

impl client::Part for PrivateText {}


/// Private data for LinkModule. This data will be rendered as the LinkModule for a pass.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivateUri {
    /// The URI's title appearing in the app as text and its translated strings. Recommended maximum is 20 characters to ensure the full string is displayed on smaller screens.
    
    pub description: Option<LocalizedString>,
    /// The location of a web page, image, or other resource. URIs in the `LinksModuleData` can have different prefixes indicating the type of URI (a link to a web page, a link to a map, a telephone number, or an email address).
    
    pub uri: Option<String>,
}

impl client::Part for PrivateUri {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PurchaseDetails {
    /// ID of the account used to purchase the ticket.
    #[serde(rename="accountId")]
    
    pub account_id: Option<String>,
    /// The confirmation code for the purchase. This may be the same for multiple different tickets and is used to group tickets together.
    #[serde(rename="confirmationCode")]
    
    pub confirmation_code: Option<String>,
    /// The purchase date/time of the ticket. This is an ISO 8601 extended format date/time, with or without an offset. Time may be specified up to nanosecond precision. Offsets may be specified with seconds precision (even though offset seconds is not part of ISO 8601). For example: `1985-04-12T23:20:50.52Z` would be 20 minutes and 50.52 seconds after the 23rd hour of April 12th, 1985 in UTC. `1985-04-12T19:20:50.52-04:00` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985, 4 hours before UTC (same instant in time as the above example). If the event were in New York, this would be the equivalent of Eastern Daylight Time (EDT). Remember that offset varies in regions that observe Daylight Saving Time (or Summer Time), depending on the time of the year. `1985-04-12T19:20:50.52` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985 with no offset information. Without offset information, some rich features may not be available.
    #[serde(rename="purchaseDateTime")]
    
    pub purchase_date_time: Option<String>,
    /// Receipt number/identifier for tracking the ticket purchase via the body that sold the ticket.
    #[serde(rename="purchaseReceiptNumber")]
    
    pub purchase_receipt_number: Option<String>,
    /// The cost of the ticket.
    #[serde(rename="ticketCost")]
    
    pub ticket_cost: Option<TicketCost>,
}

impl client::Part for PurchaseDetails {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReservationInfo {
    /// Confirmation code needed to check into this flight. This is the number that the passenger would enter into a kiosk at the airport to look up the flight and print a boarding pass.
    #[serde(rename="confirmationCode")]
    
    pub confirmation_code: Option<String>,
    /// E-ticket number.
    #[serde(rename="eticketNumber")]
    
    pub eticket_number: Option<String>,
    /// Frequent flyer membership information.
    #[serde(rename="frequentFlyerInfo")]
    
    pub frequent_flyer_info: Option<FrequentFlyerInfo>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#reservationInfo"`.
    
    pub kind: Option<String>,
}

impl client::Part for ReservationInfo {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Resources {
    /// no description provided
    #[serde(rename="eventTicketClasses")]
    
    pub event_ticket_classes: Option<Vec<EventTicketClass>>,
    /// no description provided
    #[serde(rename="eventTicketObjects")]
    
    pub event_ticket_objects: Option<Vec<EventTicketObject>>,
    /// no description provided
    #[serde(rename="flightClasses")]
    
    pub flight_classes: Option<Vec<FlightClass>>,
    /// no description provided
    #[serde(rename="flightObjects")]
    
    pub flight_objects: Option<Vec<FlightObject>>,
    /// no description provided
    #[serde(rename="giftCardClasses")]
    
    pub gift_card_classes: Option<Vec<GiftCardClass>>,
    /// no description provided
    #[serde(rename="giftCardObjects")]
    
    pub gift_card_objects: Option<Vec<GiftCardObject>>,
    /// no description provided
    #[serde(rename="loyaltyClasses")]
    
    pub loyalty_classes: Option<Vec<LoyaltyClass>>,
    /// no description provided
    #[serde(rename="loyaltyObjects")]
    
    pub loyalty_objects: Option<Vec<LoyaltyObject>>,
    /// no description provided
    #[serde(rename="offerClasses")]
    
    pub offer_classes: Option<Vec<OfferClass>>,
    /// no description provided
    #[serde(rename="offerObjects")]
    
    pub offer_objects: Option<Vec<OfferObject>>,
    /// no description provided
    #[serde(rename="transitClasses")]
    
    pub transit_classes: Option<Vec<TransitClass>>,
    /// no description provided
    #[serde(rename="transitObjects")]
    
    pub transit_objects: Option<Vec<TransitObject>>,
}

impl client::Part for Resources {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Review {
    /// no description provided
    
    pub comments: Option<String>,
}

impl client::Part for Review {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RotatingBarcode {
    /// An optional text that will override the default text that shows under the barcode. This field is intended for a human readable equivalent of the barcode value, used when the barcode cannot be scanned.
    #[serde(rename="alternateText")]
    
    pub alternate_text: Option<String>,
    /// The render encoding for the barcode. When specified, barcode is rendered in the given encoding. Otherwise best known encoding is chosen by Google.
    #[serde(rename="renderEncoding")]
    
    pub render_encoding: Option<RotatingBarcodeRenderEncodingEnum>,
    /// Optional text that will be shown when the barcode is hidden behind a click action. This happens in cases where a pass has Smart Tap enabled. If not specified, a default is chosen by Google.
    #[serde(rename="showCodeText")]
    
    pub show_code_text: Option<LocalizedString>,
    /// Details used to evaluate the {totp_value_n} substitutions.
    #[serde(rename="totpDetails")]
    
    pub totp_details: Option<RotatingBarcodeTotpDetails>,
    /// The type of this barcode.
    #[serde(rename="type")]
    
    pub type_: Option<RotatingBarcodeTypeEnum>,
    /// String encoded barcode value. This string supports the following substitutions: * {totp_value_n}: Replaced with the TOTP value (see TotpDetails.parameters). * {totp_timestamp_millis}: Replaced with the timestamp (millis since epoch) at which the barcode was generated. * {totp_timestamp_seconds}: Replaced with the timestamp (seconds since epoch) at which the barcode was generated.
    #[serde(rename="valuePattern")]
    
    pub value_pattern: Option<String>,
}

impl client::Part for RotatingBarcode {}


/// Configuration for the time-based OTP substitutions. See https://tools.ietf.org/html/rfc6238
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RotatingBarcodeTotpDetails {
    /// The TOTP algorithm used to generate the OTP.
    
    pub algorithm: Option<RotatingBarcodeTotpDetailAlgorithmEnum>,
    /// The TOTP parameters for each of the {totp_value_*} substitutions. The TotpParameters at index n is used for the {totp_value_n} substitution.
    
    pub parameters: Option<Vec<RotatingBarcodeTotpDetailsTotpParameters>>,
    /// The time interval used for the TOTP value generation, in milliseconds.
    #[serde(rename="periodMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub period_millis: Option<i64>,
}

impl client::Part for RotatingBarcodeTotpDetails {}


/// Configuration for the key and value length. See https://www.rfc-editor.org/rfc/rfc4226#section-5.3
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RotatingBarcodeTotpDetailsTotpParameters {
    /// The secret key used for the TOTP value generation, encoded as a Base16 string.
    
    pub key: Option<String>,
    /// The length of the TOTP value in decimal digits.
    #[serde(rename="valueLength")]
    
    pub value_length: Option<i32>,
}

impl client::Part for RotatingBarcodeTotpDetailsTotpParameters {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecurityAnimation {
    /// Type of animation.
    #[serde(rename="animationType")]
    
    pub animation_type: Option<SecurityAnimationAnimationTypeEnum>,
}

impl client::Part for SecurityAnimation {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SignUpInfo {
    /// ID of the class the user can sign up for.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
}

impl client::Part for SignUpInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [insert smarttap](SmarttapInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SmartTap {
    /// The unique identifier for a smart tap. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is the Smart Tap id. The Smart Tap id is a Base64 encoded string which represents the id which was generated by the Google Pay app.
    
    pub id: Option<String>,
    /// Communication from merchant to user.
    
    pub infos: Option<Vec<IssuerToUserInfo>>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#smartTap"`.
    
    pub kind: Option<String>,
    /// Smart Tap merchant ID of who engaged in the Smart Tap interaction.
    #[serde(rename="merchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub merchant_id: Option<i64>,
}

impl client::RequestValue for SmartTap {}
impl client::ResponseResult for SmartTap {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SmartTapMerchantData {
    /// Available only to Smart Tap enabled partners. Contact support for additional guidance.
    #[serde(rename="authenticationKeys")]
    
    pub authentication_keys: Option<Vec<AuthenticationKey>>,
    /// Available only to Smart Tap enabled partners. Contact support for additional guidance.
    #[serde(rename="smartTapMerchantId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub smart_tap_merchant_id: Option<i64>,
}

impl client::Part for SmartTapMerchantData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TemplateItem {
    /// A reference to a field to display. If both `firstValue` and `secondValue` are populated, they will both appear as one item with a slash between them. For example, values A and B would be shown as "A / B".
    #[serde(rename="firstValue")]
    
    pub first_value: Option<FieldSelector>,
    /// A predefined item to display. Only one of `firstValue` or `predefinedItem` may be set.
    #[serde(rename="predefinedItem")]
    
    pub predefined_item: Option<TemplateItemPredefinedItemEnum>,
    /// A reference to a field to display. This may only be populated if the `firstValue` field is populated.
    #[serde(rename="secondValue")]
    
    pub second_value: Option<FieldSelector>,
}

impl client::Part for TemplateItem {}


/// Data for Text module. All fields are optional. Header will be displayed if available, different types of bodies will be concatenated if they are defined.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextModuleData {
    /// The body of the Text Module, which is defined as an uninterrupted string. Recommended maximum length is 500 characters to ensure full string is displayed on smaller screens.
    
    pub body: Option<String>,
    /// The header of the Text Module. Recommended maximum length is 35 characters to ensure full string is displayed on smaller screens.
    
    pub header: Option<String>,
    /// The ID associated with a text module. This field is here to enable ease of management of text modules.
    
    pub id: Option<String>,
    /// Translated strings for the body. Recommended maximum length is 500 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedBody")]
    
    pub localized_body: Option<LocalizedString>,
    /// Translated strings for the header. Recommended maximum length is 35 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedHeader")]
    
    pub localized_header: Option<LocalizedString>,
}

impl client::Part for TextModuleData {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TicketCost {
    /// A message describing any kind of discount that was applied.
    #[serde(rename="discountMessage")]
    
    pub discount_message: Option<LocalizedString>,
    /// The face value of the ticket.
    #[serde(rename="faceValue")]
    
    pub face_value: Option<Money>,
    /// The actual purchase price of the ticket, after tax and/or discounts.
    #[serde(rename="purchasePrice")]
    
    pub purchase_price: Option<Money>,
}

impl client::Part for TicketCost {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TicketLeg {
    /// The date/time of arrival. This is an ISO 8601 extended format date/time, with or without an offset. Time may be specified up to nanosecond precision. Offsets may be specified with seconds precision (even though offset seconds is not part of ISO 8601). For example: `1985-04-12T23:20:50.52Z` would be 20 minutes and 50.52 seconds after the 23rd hour of April 12th, 1985 in UTC. `1985-04-12T19:20:50.52-04:00` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985, 4 hours before UTC (same instant in time as the above example). If the event were in New York, this would be the equivalent of Eastern Daylight Time (EDT). Remember that offset varies in regions that observe Daylight Saving Time (or Summer Time), depending on the time of the year. `1985-04-12T19:20:50.52` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985 with no offset information. The portion of the date/time without the offset is considered the "local date/time". This should be the local date/time at the destination station. For example, if the event occurs at the 20th hour of June 5th, 2018 at the destination station, the local date/time portion should be `2018-06-05T20:00:00`. If the local date/time at the destination station is 4 hours before UTC, an offset of `-04:00` may be appended. Without offset information, some rich features may not be available.
    #[serde(rename="arrivalDateTime")]
    
    pub arrival_date_time: Option<String>,
    /// The train or ship name/number that the passsenger needs to board.
    
    pub carriage: Option<String>,
    /// The date/time of departure. This is required if there is no validity time interval set on the transit object. This is an ISO 8601 extended format date/time, with or without an offset. Time may be specified up to nanosecond precision. Offsets may be specified with seconds precision (even though offset seconds is not part of ISO 8601). For example: `1985-04-12T23:20:50.52Z` would be 20 minutes and 50.52 seconds after the 23rd hour of April 12th, 1985 in UTC. `1985-04-12T19:20:50.52-04:00` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985, 4 hours before UTC (same instant in time as the above example). If the event were in New York, this would be the equivalent of Eastern Daylight Time (EDT). Remember that offset varies in regions that observe Daylight Saving Time (or Summer Time), depending on the time of the year. `1985-04-12T19:20:50.52` would be 20 minutes and 50.52 seconds after the 19th hour of April 12th, 1985 with no offset information. The portion of the date/time without the offset is considered the "local date/time". This should be the local date/time at the origin station. For example, if the departure occurs at the 20th hour of June 5th, 2018 at the origin station, the local date/time portion should be `2018-06-05T20:00:00`. If the local date/time at the origin station is 4 hours before UTC, an offset of `-04:00` may be appended. Without offset information, some rich features may not be available.
    #[serde(rename="departureDateTime")]
    
    pub departure_date_time: Option<String>,
    /// The destination name.
    #[serde(rename="destinationName")]
    
    pub destination_name: Option<LocalizedString>,
    /// The destination station code.
    #[serde(rename="destinationStationCode")]
    
    pub destination_station_code: Option<String>,
    /// Short description/name of the fare for this leg of travel. Eg "Anytime Single Use".
    #[serde(rename="fareName")]
    
    pub fare_name: Option<LocalizedString>,
    /// The name of the origin station. This is required if `desinationName` is present or if `originStationCode` is not present.
    #[serde(rename="originName")]
    
    pub origin_name: Option<LocalizedString>,
    /// The origin station code. This is required if `destinationStationCode` is present or if `originName` is not present.
    #[serde(rename="originStationCode")]
    
    pub origin_station_code: Option<String>,
    /// The platform or gate where the passenger can board the carriage.
    
    pub platform: Option<String>,
    /// The reserved seat for the passenger(s). If more than one seat is to be specified then use the `ticketSeats` field instead. Both `ticketSeat` and `ticketSeats` may not be set.
    #[serde(rename="ticketSeat")]
    
    pub ticket_seat: Option<TicketSeat>,
    /// The reserved seat for the passenger(s). If only one seat is to be specified then use the `ticketSeat` field instead. Both `ticketSeat` and `ticketSeats` may not be set.
    #[serde(rename="ticketSeats")]
    
    pub ticket_seats: Option<Vec<TicketSeat>>,
    /// The name of the transit operator that is operating this leg of a trip.
    #[serde(rename="transitOperatorName")]
    
    pub transit_operator_name: Option<LocalizedString>,
    /// Terminus station or destination of the train/bus/etc.
    #[serde(rename="transitTerminusName")]
    
    pub transit_terminus_name: Option<LocalizedString>,
    /// The zone of boarding within the platform.
    
    pub zone: Option<String>,
}

impl client::Part for TicketLeg {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TicketRestrictions {
    /// Extra restrictions that don't fall under the "route" or "time" categories.
    #[serde(rename="otherRestrictions")]
    
    pub other_restrictions: Option<LocalizedString>,
    /// Restrictions about routes that may be taken. For example, this may be the string "Reserved CrossCountry trains only".
    #[serde(rename="routeRestrictions")]
    
    pub route_restrictions: Option<LocalizedString>,
    /// More details about the above `routeRestrictions`.
    #[serde(rename="routeRestrictionsDetails")]
    
    pub route_restrictions_details: Option<LocalizedString>,
    /// Restrictions about times this ticket may be used.
    #[serde(rename="timeRestrictions")]
    
    pub time_restrictions: Option<LocalizedString>,
}

impl client::Part for TicketRestrictions {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TicketSeat {
    /// The identifier of the train car or coach in which the ticketed seat is located. Eg. "10"
    
    pub coach: Option<String>,
    /// A custome fare class to be used if no `fareClass` applies. Both `fareClass` and `customFareClass` may not be set.
    #[serde(rename="customFareClass")]
    
    pub custom_fare_class: Option<LocalizedString>,
    /// The fare class of the ticketed seat.
    #[serde(rename="fareClass")]
    
    pub fare_class: Option<TicketSeatFareClassEnum>,
    /// The identifier of where the ticketed seat is located. Eg. "42". If there is no specific identifier, use `seatAssigment` instead.
    
    pub seat: Option<String>,
    /// The passenger's seat assignment. Eg. "no specific seat". To be used when there is no specific identifier to use in `seat`.
    #[serde(rename="seatAssignment")]
    
    pub seat_assignment: Option<LocalizedString>,
}

impl client::Part for TicketSeat {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeInterval {
    /// End time of the interval. Offset is not required. If an offset is provided and `start` time is set, `start` must also include an offset.
    
    pub end: Option<DateTime>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#timeInterval"`.
    
    pub kind: Option<String>,
    /// Start time of the interval. Offset is not required. If an offset is provided and `end` time is set, `end` must also include an offset.
    
    pub start: Option<DateTime>,
}

impl client::Part for TimeInterval {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get transitclass](TransitclasGetCall) (response)
/// * [insert transitclass](TransitclasInsertCall) (request|response)
/// * [patch transitclass](TransitclasPatchCall) (request|response)
/// * [update transitclass](TransitclasUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitClass {
    /// Activation options for an activatable ticket.
    #[serde(rename="activationOptions")]
    
    pub activation_options: Option<ActivationOptions>,
    /// Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead.
    #[serde(rename="allowMultipleUsersPerObject")]
    
    pub allow_multiple_users_per_object: Option<bool>,
    /// Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback.
    #[serde(rename="callbackOptions")]
    
    pub callback_options: Option<CallbackOptions>,
    /// Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display.
    #[serde(rename="classTemplateInfo")]
    
    pub class_template_info: Option<ClassTemplateInfo>,
    /// Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale.
    #[serde(rename="countryCode")]
    
    pub country_code: Option<String>,
    /// A custom label to use for the carriage value (`transitObject.ticketLeg.carriage`).
    #[serde(rename="customCarriageLabel")]
    
    pub custom_carriage_label: Option<LocalizedString>,
    /// A custom label to use for the coach value (`transitObject.ticketLeg.ticketSeat.coach`).
    #[serde(rename="customCoachLabel")]
    
    pub custom_coach_label: Option<LocalizedString>,
    /// A custom label to use for the transit concession category value (`transitObject.concessionCategory`).
    #[serde(rename="customConcessionCategoryLabel")]
    
    pub custom_concession_category_label: Option<LocalizedString>,
    /// A custom label to use for the confirmation code value (`transitObject.purchaseDetails.confirmationCode`).
    #[serde(rename="customConfirmationCodeLabel")]
    
    pub custom_confirmation_code_label: Option<LocalizedString>,
    /// A custom label to use for the transit discount message value (`transitObject.purchaseDetails.ticketCost.discountMessage`).
    #[serde(rename="customDiscountMessageLabel")]
    
    pub custom_discount_message_label: Option<LocalizedString>,
    /// A custom label to use for the fare class value (`transitObject.ticketLeg.ticketSeat.fareClass`).
    #[serde(rename="customFareClassLabel")]
    
    pub custom_fare_class_label: Option<LocalizedString>,
    /// A custom label to use for the transit fare name value (`transitObject.ticketLeg.fareName`).
    #[serde(rename="customFareNameLabel")]
    
    pub custom_fare_name_label: Option<LocalizedString>,
    /// A custom label to use for the other restrictions value (`transitObject.ticketRestrictions.otherRestrictions`).
    #[serde(rename="customOtherRestrictionsLabel")]
    
    pub custom_other_restrictions_label: Option<LocalizedString>,
    /// A custom label to use for the boarding platform value (`transitObject.ticketLeg.platform`).
    #[serde(rename="customPlatformLabel")]
    
    pub custom_platform_label: Option<LocalizedString>,
    /// A custom label to use for the purchase face value (`transitObject.purchaseDetails.ticketCost.faceValue`).
    #[serde(rename="customPurchaseFaceValueLabel")]
    
    pub custom_purchase_face_value_label: Option<LocalizedString>,
    /// A custom label to use for the purchase price value (`transitObject.purchaseDetails.ticketCost.purchasePrice`).
    #[serde(rename="customPurchasePriceLabel")]
    
    pub custom_purchase_price_label: Option<LocalizedString>,
    /// A custom label to use for the purchase receipt number value (`transitObject.purchaseDetails.purchaseReceiptNumber`).
    #[serde(rename="customPurchaseReceiptNumberLabel")]
    
    pub custom_purchase_receipt_number_label: Option<LocalizedString>,
    /// A custom label to use for the route restrictions details value (`transitObject.ticketRestrictions.routeRestrictionsDetails`).
    #[serde(rename="customRouteRestrictionsDetailsLabel")]
    
    pub custom_route_restrictions_details_label: Option<LocalizedString>,
    /// A custom label to use for the route restrictions value (`transitObject.ticketRestrictions.routeRestrictions`).
    #[serde(rename="customRouteRestrictionsLabel")]
    
    pub custom_route_restrictions_label: Option<LocalizedString>,
    /// A custom label to use for the seat location value (`transitObject.ticketLeg.ticketSeat.seat`).
    #[serde(rename="customSeatLabel")]
    
    pub custom_seat_label: Option<LocalizedString>,
    /// A custom label to use for the ticket number value (`transitObject.ticketNumber`).
    #[serde(rename="customTicketNumberLabel")]
    
    pub custom_ticket_number_label: Option<LocalizedString>,
    /// A custom label to use for the time restrictions details value (`transitObject.ticketRestrictions.timeRestrictions`).
    #[serde(rename="customTimeRestrictionsLabel")]
    
    pub custom_time_restrictions_label: Option<LocalizedString>,
    /// A custom label to use for the transit terminus name value (`transitObject.ticketLeg.transitTerminusName`).
    #[serde(rename="customTransitTerminusNameLabel")]
    
    pub custom_transit_terminus_name_label: Option<LocalizedString>,
    /// A custom label to use for the boarding zone value (`transitObject.ticketLeg.zone`).
    #[serde(rename="customZoneLabel")]
    
    pub custom_zone_label: Option<LocalizedString>,
    /// Controls the display of the single-leg itinerary for this class. By default, an itinerary will only display for multi-leg trips.
    #[serde(rename="enableSingleLegItinerary")]
    
    pub enable_single_leg_itinerary: Option<bool>,
    /// Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="enableSmartTap")]
    
    pub enable_smart_tap: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object).
    #[serde(rename="homepageUri")]
    
    pub homepage_uri: Option<Uri>,
    /// Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="issuerName")]
    
    pub issuer_name: Option<String>,
    /// If this field is present, transit tickets served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT".
    #[serde(rename="languageOverride")]
    
    pub language_override: Option<String>,
    /// Links module data. If links module data is also defined on the object, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedIssuerName")]
    
    pub localized_issuer_name: Option<LocalizedString>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// Required. The logo image of the ticket. This image is displayed in the card detail view of the app.
    
    pub logo: Option<Image>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// Identifies whether multiple users and devices will save the same object referencing this class.
    #[serde(rename="multipleDevicesAndHoldersAllowedStatus")]
    
    pub multiple_devices_and_holders_allowed_status: Option<TransitClasMultipleDevicesAndHoldersAllowedStatusEnum>,
    /// Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap.
    #[serde(rename="redemptionIssuers")]
    
    #[serde_as(as = "Option<Vec<::client::serde_with::DisplayFromStr>>")]
    pub redemption_issuers: Option<Vec<i64>>,
    /// The review comments set by the platform when a class is marked `approved` or `rejected`.
    
    pub review: Option<Review>,
    /// Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`.
    #[serde(rename="reviewStatus")]
    
    pub review_status: Option<TransitClasReviewStatusEnum>,
    /// Optional information about the security animation. If this is set a security animation will be rendered on pass details.
    #[serde(rename="securityAnimation")]
    
    pub security_animation: Option<SecurityAnimation>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// The name of the transit operator.
    #[serde(rename="transitOperatorName")]
    
    pub transit_operator_name: Option<LocalizedString>,
    /// Required. The type of transit this class represents, such as "bus".
    #[serde(rename="transitType")]
    
    pub transit_type: Option<TransitClasTransitTypeEnum>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
    /// View Unlock Requirement options for the transit ticket.
    #[serde(rename="viewUnlockRequirement")]
    
    pub view_unlock_requirement: Option<TransitClasViewUnlockRequirementEnum>,
    /// Watermark image to display on the user's device.
    
    pub watermark: Option<Image>,
    /// Deprecated.
    #[serde(rename="wordMark")]
    
    pub word_mark: Option<Image>,
}

impl client::RequestValue for TransitClass {}
impl client::ResponseResult for TransitClass {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage transitclass](TransitclasAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitClassAddMessageResponse {
    /// The updated TransitClass resource.
    
    pub resource: Option<TransitClass>,
}

impl client::ResponseResult for TransitClassAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transitclass](TransitclasListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitClassListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<TransitClass>>,
}

impl client::ResponseResult for TransitClassListResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get transitobject](TransitobjectGetCall) (response)
/// * [insert transitobject](TransitobjectInsertCall) (request|response)
/// * [patch transitobject](TransitobjectPatchCall) (request|response)
/// * [update transitobject](TransitobjectUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitObject {
    /// The activation status for the object. Required if the class has `activationOptions` set.
    #[serde(rename="activationStatus")]
    
    pub activation_status: Option<ActivationStatus>,
    /// Optional information about the partner app link.
    #[serde(rename="appLinkData")]
    
    pub app_link_data: Option<AppLinkData>,
    /// The barcode type and value.
    
    pub barcode: Option<Barcode>,
    /// Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you.
    #[serde(rename="classId")]
    
    pub class_id: Option<String>,
    /// A copy of the inherited fields of the parent class. These fields are retrieved during a GET.
    #[serde(rename="classReference")]
    
    pub class_reference: Option<TransitClass>,
    /// The concession category for the ticket.
    #[serde(rename="concessionCategory")]
    
    pub concession_category: Option<TransitObjectConcessionCategoryEnum>,
    /// A custom concession category to use when `concessionCategory` does not provide the right option. Both `concessionCategory` and `customConcessionCategory` may not be set.
    #[serde(rename="customConcessionCategory")]
    
    pub custom_concession_category: Option<LocalizedString>,
    /// A custom status to use for the ticket status value when `ticketStatus` does not provide the right option. Both `ticketStatus` and `customTicketStatus` may not be set.
    #[serde(rename="customTicketStatus")]
    
    pub custom_ticket_status: Option<LocalizedString>,
    /// Device context associated with the object.
    #[serde(rename="deviceContext")]
    
    pub device_context: Option<DeviceContext>,
    /// Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers.
    #[serde(rename="disableExpirationNotification")]
    
    pub disable_expiration_notification: Option<bool>,
    /// Information that controls how passes are grouped together.
    #[serde(rename="groupingInfo")]
    
    pub grouping_info: Option<GroupingInfo>,
    /// Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information.
    #[serde(rename="hasLinkedDevice")]
    
    pub has_linked_device: Option<bool>,
    /// Indicates if the object has users. This field is set by the platform.
    #[serde(rename="hasUsers")]
    
    pub has_users: Option<bool>,
    /// Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed.
    #[serde(rename="heroImage")]
    
    pub hero_image: Option<Image>,
    /// The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`.
    #[serde(rename="hexBackgroundColor")]
    
    pub hex_background_color: Option<String>,
    /// Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'.
    
    pub id: Option<String>,
    /// Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level.
    #[serde(rename="imageModulesData")]
    
    pub image_modules_data: Option<Vec<ImageModuleData>>,
    /// Deprecated. Use textModulesData instead.
    #[serde(rename="infoModuleData")]
    
    pub info_module_data: Option<InfoModuleData>,
    /// Links module data. If links module data is also defined on the class, both will be displayed.
    #[serde(rename="linksModuleData")]
    
    pub links_module_data: Option<LinksModuleData>,
    /// Note: This field is currently not supported to trigger geo notifications.
    
    pub locations: Option<Vec<LatLongPoint>>,
    /// An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10.
    
    pub messages: Option<Vec<Message>>,
    /// The name(s) of the passengers the ticket is assigned to. The above `passengerType` field is meant to give Google context on this field.
    #[serde(rename="passengerNames")]
    
    pub passenger_names: Option<String>,
    /// The number of passengers.
    #[serde(rename="passengerType")]
    
    pub passenger_type: Option<TransitObjectPassengerTypeEnum>,
    /// Purchase details for this ticket.
    #[serde(rename="purchaseDetails")]
    
    pub purchase_details: Option<PurchaseDetails>,
    /// The rotating barcode type and value.
    #[serde(rename="rotatingBarcode")]
    
    pub rotating_barcode: Option<RotatingBarcode>,
    /// The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported.
    #[serde(rename="smartTapRedemptionValue")]
    
    pub smart_tap_redemption_value: Option<String>,
    /// Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section.
    
    pub state: Option<TransitObjectStateEnum>,
    /// Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class.
    #[serde(rename="textModulesData")]
    
    pub text_modules_data: Option<Vec<TextModuleData>>,
    /// A single ticket leg contains departure and arrival information along with boarding and seating information. If more than one leg is to be specified then use the `ticketLegs` field instead. Both `ticketLeg` and `ticketLegs` may not be set.
    #[serde(rename="ticketLeg")]
    
    pub ticket_leg: Option<TicketLeg>,
    /// Each ticket may contain one or more legs. Each leg contains departure and arrival information along with boarding and seating information. If only one leg is to be specified then use the `ticketLeg` field instead. Both `ticketLeg` and `ticketLegs` may not be set.
    #[serde(rename="ticketLegs")]
    
    pub ticket_legs: Option<Vec<TicketLeg>>,
    /// The number of the ticket. This is a unique identifier for the ticket in the transit operator's system.
    #[serde(rename="ticketNumber")]
    
    pub ticket_number: Option<String>,
    /// Information about what kind of restrictions there are on using this ticket. For example, which days of the week it must be used, or which routes are allowed to be taken.
    #[serde(rename="ticketRestrictions")]
    
    pub ticket_restrictions: Option<TicketRestrictions>,
    /// The status of the ticket. For states which affect display, use the `state` field instead.
    #[serde(rename="ticketStatus")]
    
    pub ticket_status: Option<TransitObjectTicketStatusEnum>,
    /// This id is used to group tickets together if the user has saved multiple tickets for the same trip.
    #[serde(rename="tripId")]
    
    pub trip_id: Option<String>,
    /// Required. The type of trip this transit object represents. Used to determine the pass title and/or which symbol to use between the origin and destination.
    #[serde(rename="tripType")]
    
    pub trip_type: Option<TransitObjectTripTypeEnum>,
    /// The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed.
    #[serde(rename="validTimeInterval")]
    
    pub valid_time_interval: Option<TimeInterval>,
    /// Deprecated
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::RequestValue for TransitObject {}
impl client::ResponseResult for TransitObject {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [addmessage transitobject](TransitobjectAddmessageCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitObjectAddMessageResponse {
    /// The updated TransitObject resource.
    
    pub resource: Option<TransitObject>,
}

impl client::ResponseResult for TransitObjectAddMessageResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list transitobject](TransitobjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransitObjectListResponse {
    /// Pagination of the response.
    
    pub pagination: Option<Pagination>,
    /// Resources corresponding to the list request.
    
    pub resources: Option<Vec<TransitObject>>,
}

impl client::ResponseResult for TransitObjectListResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TranslatedString {
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#translatedString"`.
    
    pub kind: Option<String>,
    /// Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT".
    
    pub language: Option<String>,
    /// The UTF-8 encoded translated string.
    
    pub value: Option<String>,
}

impl client::Part for TranslatedString {}


/// Indicates that the issuer would like GooglePay send an upcoming card validity notification 1 day before card becomes valid/usable.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpcomingNotification {
    /// Indicates if the object needs to have upcoming notification enabled.
    #[serde(rename="enableNotification")]
    
    pub enable_notification: Option<bool>,
}

impl client::Part for UpcomingNotification {}


/// Request for sending user private Text or URI by the Issuer.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [v1 private content upload private data walletobjects](WalletobjectV1PrivateContentUploadPrivateDataCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadPrivateDataRequest {
    /// The ID of the issuer sending the data.
    #[serde(rename="issuerId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub issuer_id: Option<i64>,
    /// Private text data of the user.
    
    pub text: Option<PrivateText>,
    /// Private URIs of the user.
    
    pub uri: Option<PrivateUri>,
}

impl client::RequestValue for UploadPrivateDataRequest {}


/// Response for uploading user private data (text or URIs)
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [v1 private content upload private data walletobjects](WalletobjectV1PrivateContentUploadPrivateDataCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadPrivateDataResponse {
    /// A 64-bit content id for the private data that was uploaded by the Issuer.
    #[serde(rename="privateContentId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub private_content_id: Option<i64>,
}

impl client::ResponseResult for UploadPrivateDataResponse {}


/// Request to upload user’s private images by Issuers to be used in passes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload media](MediaUploadCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadPrivateImageRequest {
    /// A reference to the image payload that was uploaded by Scotty.
    
    pub blob: Option<Media>,
    /// Extra information about the uploaded media.
    #[serde(rename="mediaRequestInfo")]
    
    pub media_request_info: Option<MediaRequestInfo>,
}

impl client::RequestValue for UploadPrivateImageRequest {}


/// Response for uploading the private image
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [upload media](MediaUploadCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadPrivateImageResponse {
    /// A 64-bit content id for the image that was uploaded by the Issuer.
    #[serde(rename="privateContentId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub private_content_id: Option<i64>,
}

impl client::ResponseResult for UploadPrivateImageResponse {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Uri {
    /// The URI's title appearing in the app as text. Recommended maximum is 20 characters to ensure full string is displayed on smaller screens. Note that in some contexts this text is not used, such as when `description` is part of an image.
    
    pub description: Option<String>,
    /// The ID associated with a uri. This field is here to enable ease of management of uris.
    
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string `"walletobjects#uri"`.
    
    pub kind: Option<String>,
    /// Translated strings for the description. Recommended maximum is 20 characters to ensure full string is displayed on smaller screens.
    #[serde(rename="localizedDescription")]
    
    pub localized_description: Option<LocalizedString>,
    /// The location of a web page, image, or other resource. URIs in the `LinksModuleData` module can have different prefixes indicating the type of URI (a link to a web page, a link to a map, a telephone number, or an email address). URIs must have a scheme.
    
    pub uri: Option<String>,
}

impl client::Part for Uri {}


