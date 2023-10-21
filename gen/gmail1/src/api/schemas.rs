use super::*;
/// Auto-forwarding settings for an account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings get auto forwarding users](UserSettingGetAutoForwardingCall) (response)
/// * [settings update auto forwarding users](UserSettingUpdateAutoForwardingCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AutoForwarding {
    /// The state that a message should be left in after it has been forwarded.
    
    pub disposition: Option<String>,
    /// Email address to which all incoming messages are forwarded. This email address must be a verified member of the forwarding addresses.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// Whether all incoming mail is automatically forwarded to another address.
    
    pub enabled: Option<bool>,
}

impl client::RequestValue for AutoForwarding {}
impl client::ResponseResult for AutoForwarding {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages batch delete users](UserMessageBatchDeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchDeleteMessagesRequest {
    /// The IDs of the messages to delete.
    
    pub ids: Option<Vec<String>>,
}

impl client::RequestValue for BatchDeleteMessagesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages batch modify users](UserMessageBatchModifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchModifyMessagesRequest {
    /// A list of label IDs to add to messages.
    #[serde(rename="addLabelIds")]
    
    pub add_label_ids: Option<Vec<String>>,
    /// The IDs of the messages to modify. There is a limit of 1000 ids per request.
    
    pub ids: Option<Vec<String>>,
    /// A list of label IDs to remove from messages.
    #[serde(rename="removeLabelIds")]
    
    pub remove_label_ids: Option<Vec<String>>,
}

impl client::RequestValue for BatchModifyMessagesRequest {}


/// The client-side encryption (CSE) configuration for the email address of an authenticated user. Gmail uses CSE configurations to save drafts of client-side encrypted email messages, and to sign and send encrypted email messages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings cse identities create users](UserSettingCseIdentityCreateCall) (request|response)
/// * [settings cse identities get users](UserSettingCseIdentityGetCall) (response)
/// * [settings cse identities patch users](UserSettingCseIdentityPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CseIdentity {
    /// The email address for the sending identity. The email address must be the primary email address of the authenticated user.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// If a key pair is associated, the identifier of the key pair, CseKeyPair.
    #[serde(rename="primaryKeyPairId")]
    
    pub primary_key_pair_id: Option<String>,
}

impl client::RequestValue for CseIdentity {}
impl client::ResponseResult for CseIdentity {}


/// A client-side encryption S/MIME key pair, which is comprised of a public key, its certificate chain, and metadata for its paired private key. Gmail uses the key pair to complete the following tasks: - Sign outgoing client-side encrypted messages. - Save and reopen drafts of client-side encrypted messages. - Save and reopen sent messages. - Decrypt incoming or archived S/MIME messages.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings cse keypairs create users](UserSettingCseKeypairCreateCall) (request|response)
/// * [settings cse keypairs disable users](UserSettingCseKeypairDisableCall) (response)
/// * [settings cse keypairs enable users](UserSettingCseKeypairEnableCall) (response)
/// * [settings cse keypairs get users](UserSettingCseKeypairGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CseKeyPair {
    /// Output only. If a key pair is set to `DISABLED`, the time that the key pair's state changed from `ENABLED` to `DISABLED`. This field is present only when the key pair is in state `DISABLED`.
    #[serde(rename="disableTime")]
    
    pub disable_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The current state of the key pair.
    #[serde(rename="enablementState")]
    
    pub enablement_state: Option<String>,
    /// Output only. The immutable ID for the client-side encryption S/MIME key pair.
    #[serde(rename="keyPairId")]
    
    pub key_pair_id: Option<String>,
    /// Output only. The public key and its certificate chain, in [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format.
    
    pub pem: Option<String>,
    /// Input only. The public key and its certificate chain. The chain must be in [PKCS#7](https://en.wikipedia.org/wiki/PKCS_7) format and use PEM encoding and ASCII armor.
    
    pub pkcs7: Option<String>,
    /// Metadata for instances of this key pair's private key.
    #[serde(rename="privateKeyMetadata")]
    
    pub private_key_metadata: Option<Vec<CsePrivateKeyMetadata>>,
    /// Output only. The email address identities that are specified on the leaf certificate.
    #[serde(rename="subjectEmailAddresses")]
    
    pub subject_email_addresses: Option<Vec<String>>,
}

impl client::RequestValue for CseKeyPair {}
impl client::ResponseResult for CseKeyPair {}


/// Metadata for a private key instance.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CsePrivateKeyMetadata {
    /// Metadata for a private key instance managed by an external key access control list service.
    #[serde(rename="kaclsKeyMetadata")]
    
    pub kacls_key_metadata: Option<KaclsKeyMetadata>,
    /// Output only. The immutable ID for the private key metadata instance.
    #[serde(rename="privateKeyMetadataId")]
    
    pub private_key_metadata_id: Option<String>,
}

impl client::Part for CsePrivateKeyMetadata {}


/// Settings for a delegate. Delegates can read, send, and delete messages, as well as view and add contacts, for the delegator’s account. See “Set up mail delegation” for more information about delegates.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings delegates create users](UserSettingDelegateCreateCall) (request|response)
/// * [settings delegates get users](UserSettingDelegateGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Delegate {
    /// The email address of the delegate.
    #[serde(rename="delegateEmail")]
    
    pub delegate_email: Option<String>,
    /// Indicates whether this address has been verified and can act as a delegate for the account. Read-only.
    #[serde(rename="verificationStatus")]
    
    pub verification_status: Option<String>,
}

impl client::RequestValue for Delegate {}
impl client::ResponseResult for Delegate {}


/// Requests to turn off a client-side encryption key pair.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings cse keypairs disable users](UserSettingCseKeypairDisableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DisableCseKeyPairRequest { _never_set: Option<bool> }

impl client::RequestValue for DisableCseKeyPairRequest {}


/// A draft email in the user’s mailbox.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [drafts create users](UserDraftCreateCall) (request|response)
/// * [drafts get users](UserDraftGetCall) (response)
/// * [drafts send users](UserDraftSendCall) (request)
/// * [drafts update users](UserDraftUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Draft {
    /// The immutable ID of the draft.
    
    pub id: Option<String>,
    /// The message content of the draft.
    
    pub message: Option<Message>,
}

impl client::RequestValue for Draft {}
impl client::ResponseResult for Draft {}


/// Requests to turn on a client-side encryption key pair.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings cse keypairs enable users](UserSettingCseKeypairEnableCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnableCseKeyPairRequest { _never_set: Option<bool> }

impl client::RequestValue for EnableCseKeyPairRequest {}


/// Resource definition for Gmail filters. Filters apply to specific messages instead of an entire email thread.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings filters create users](UserSettingFilterCreateCall) (request|response)
/// * [settings filters get users](UserSettingFilterGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// Action that the filter performs.
    
    pub action: Option<FilterAction>,
    /// Matching criteria for the filter.
    
    pub criteria: Option<FilterCriteria>,
    /// The server assigned ID of the filter.
    
    pub id: Option<String>,
}

impl client::RequestValue for Filter {}
impl client::ResponseResult for Filter {}


/// A set of actions to perform on a message.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterAction {
    /// List of labels to add to the message.
    #[serde(rename="addLabelIds")]
    
    pub add_label_ids: Option<Vec<String>>,
    /// Email address that the message should be forwarded to.
    
    pub forward: Option<String>,
    /// List of labels to remove from the message.
    #[serde(rename="removeLabelIds")]
    
    pub remove_label_ids: Option<Vec<String>>,
}

impl client::Part for FilterAction {}


/// Message matching criteria.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FilterCriteria {
    /// Whether the response should exclude chats.
    #[serde(rename="excludeChats")]
    
    pub exclude_chats: Option<bool>,
    /// The sender's display name or email address.
    
    pub from: Option<String>,
    /// Whether the message has any attachment.
    #[serde(rename="hasAttachment")]
    
    pub has_attachment: Option<bool>,
    /// Only return messages not matching the specified query. Supports the same query format as the Gmail search box. For example, `"from:someuser@example.com rfc822msgid: is:unread"`.
    #[serde(rename="negatedQuery")]
    
    pub negated_query: Option<String>,
    /// Only return messages matching the specified query. Supports the same query format as the Gmail search box. For example, `"from:someuser@example.com rfc822msgid: is:unread"`.
    
    pub query: Option<String>,
    /// The size of the entire RFC822 message in bytes, including all headers and attachments.
    
    pub size: Option<i32>,
    /// How the message size in bytes should be in relation to the size field.
    #[serde(rename="sizeComparison")]
    
    pub size_comparison: Option<String>,
    /// Case-insensitive phrase found in the message's subject. Trailing and leading whitespace are be trimmed and adjacent spaces are collapsed.
    
    pub subject: Option<String>,
    /// The recipient's display name or email address. Includes recipients in the "to", "cc", and "bcc" header fields. You can use simply the local part of the email address. For example, "example" and "example@" both match "example@gmail.com". This field is case-insensitive.
    
    pub to: Option<String>,
}

impl client::Part for FilterCriteria {}


/// Settings for a forwarding address.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings forwarding addresses create users](UserSettingForwardingAddressCreateCall) (request|response)
/// * [settings forwarding addresses get users](UserSettingForwardingAddressGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ForwardingAddress {
    /// An email address to which messages can be forwarded.
    #[serde(rename="forwardingEmail")]
    
    pub forwarding_email: Option<String>,
    /// Indicates whether this address has been verified and is usable for forwarding. Read-only.
    #[serde(rename="verificationStatus")]
    
    pub verification_status: Option<String>,
}

impl client::RequestValue for ForwardingAddress {}
impl client::ResponseResult for ForwardingAddress {}


/// A record of a change to the user's mailbox. Each history change may affect multiple messages in multiple ways.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct History {
    /// The mailbox sequence ID.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub id: Option<u64>,
    /// Labels added to messages in this history record.
    #[serde(rename="labelsAdded")]
    
    pub labels_added: Option<Vec<HistoryLabelAdded>>,
    /// Labels removed from messages in this history record.
    #[serde(rename="labelsRemoved")]
    
    pub labels_removed: Option<Vec<HistoryLabelRemoved>>,
    /// List of messages changed in this history record. The fields for specific change types, such as `messagesAdded` may duplicate messages in this field. We recommend using the specific change-type fields instead of this.
    
    pub messages: Option<Vec<Message>>,
    /// Messages added to the mailbox in this history record.
    #[serde(rename="messagesAdded")]
    
    pub messages_added: Option<Vec<HistoryMessageAdded>>,
    /// Messages deleted (not Trashed) from the mailbox in this history record.
    #[serde(rename="messagesDeleted")]
    
    pub messages_deleted: Option<Vec<HistoryMessageDeleted>>,
}

impl client::Part for History {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistoryLabelAdded {
    /// Label IDs added to the message.
    #[serde(rename="labelIds")]
    
    pub label_ids: Option<Vec<String>>,
    /// no description provided
    
    pub message: Option<Message>,
}

impl client::Part for HistoryLabelAdded {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistoryLabelRemoved {
    /// Label IDs removed from the message.
    #[serde(rename="labelIds")]
    
    pub label_ids: Option<Vec<String>>,
    /// no description provided
    
    pub message: Option<Message>,
}

impl client::Part for HistoryLabelRemoved {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistoryMessageAdded {
    /// no description provided
    
    pub message: Option<Message>,
}

impl client::Part for HistoryMessageAdded {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HistoryMessageDeleted {
    /// no description provided
    
    pub message: Option<Message>,
}

impl client::Part for HistoryMessageDeleted {}


/// IMAP settings for an account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings get imap users](UserSettingGetImapCall) (response)
/// * [settings update imap users](UserSettingUpdateImapCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImapSettings {
    /// If this value is true, Gmail will immediately expunge a message when it is marked as deleted in IMAP. Otherwise, Gmail will wait for an update from the client before expunging messages marked as deleted.
    #[serde(rename="autoExpunge")]
    
    pub auto_expunge: Option<bool>,
    /// Whether IMAP is enabled for the account.
    
    pub enabled: Option<bool>,
    /// The action that will be executed on a message when it is marked as deleted and expunged from the last visible IMAP folder.
    #[serde(rename="expungeBehavior")]
    
    pub expunge_behavior: Option<String>,
    /// An optional limit on the number of messages that an IMAP folder may contain. Legal values are 0, 1000, 2000, 5000 or 10000. A value of zero is interpreted to mean that there is no limit.
    #[serde(rename="maxFolderSize")]
    
    pub max_folder_size: Option<i32>,
}

impl client::RequestValue for ImapSettings {}
impl client::ResponseResult for ImapSettings {}


/// Metadata for private keys managed by an external key access control list service. For details about managing key access, see [Google Workspace CSE API Reference](https://developers.google.com/workspace/cse/reference).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KaclsKeyMetadata {
    /// Opaque data generated and used by the key access control list service. Maximum size: 8 KiB.
    #[serde(rename="kaclsData")]
    
    pub kacls_data: Option<String>,
    /// The URI of the key access control list service that manages the private key.
    #[serde(rename="kaclsUri")]
    
    pub kacls_uri: Option<String>,
}

impl client::Part for KaclsKeyMetadata {}


/// Labels are used to categorize messages and threads within the user’s mailbox. The maximum number of labels supported for a user’s mailbox is 10,000.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [labels create users](UserLabelCreateCall) (request|response)
/// * [labels get users](UserLabelGetCall) (response)
/// * [labels patch users](UserLabelPatchCall) (request|response)
/// * [labels update users](UserLabelUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Label {
    /// The color to assign to the label. Color is only available for labels that have their `type` set to `user`.
    
    pub color: Option<LabelColor>,
    /// The immutable ID of the label.
    
    pub id: Option<String>,
    /// The visibility of the label in the label list in the Gmail web interface.
    #[serde(rename="labelListVisibility")]
    
    pub label_list_visibility: Option<String>,
    /// The visibility of messages with this label in the message list in the Gmail web interface.
    #[serde(rename="messageListVisibility")]
    
    pub message_list_visibility: Option<String>,
    /// The total number of messages with the label.
    #[serde(rename="messagesTotal")]
    
    pub messages_total: Option<i32>,
    /// The number of unread messages with the label.
    #[serde(rename="messagesUnread")]
    
    pub messages_unread: Option<i32>,
    /// The display name of the label.
    
    pub name: Option<String>,
    /// The total number of threads with the label.
    #[serde(rename="threadsTotal")]
    
    pub threads_total: Option<i32>,
    /// The number of unread threads with the label.
    #[serde(rename="threadsUnread")]
    
    pub threads_unread: Option<i32>,
    /// The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Label {}
impl client::ResponseResult for Label {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LabelColor {
    /// The background color represented as hex string #RRGGBB (ex #000000). This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765
    #[serde(rename="backgroundColor")]
    
    pub background_color: Option<String>,
    /// The text color of the label, represented as hex string. This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765
    #[serde(rename="textColor")]
    
    pub text_color: Option<String>,
}

impl client::Part for LabelColor {}


/// Language settings for an account. These settings correspond to the “Language settings” feature in the web interface.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings get language users](UserSettingGetLanguageCall) (response)
/// * [settings update language users](UserSettingUpdateLanguageCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LanguageSettings {
    /// The language to display Gmail in, formatted as an RFC 3066 Language Tag (for example `en-GB`, `fr` or `ja` for British English, French, or Japanese respectively). The set of languages supported by Gmail evolves over time, so please refer to the "Language" dropdown in the Gmail settings for all available options, as described in the language settings help article. A table of sample values is also provided in the Managing Language Settings guide Not all Gmail clients can display the same set of languages. In the case that a user's display language is not available for use on a particular client, said client automatically chooses to display in the closest supported variant (or a reasonable default).
    #[serde(rename="displayLanguage")]
    
    pub display_language: Option<String>,
}

impl client::RequestValue for LanguageSettings {}
impl client::ResponseResult for LanguageSettings {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings cse identities list users](UserSettingCseIdentityListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCseIdentitiesResponse {
    /// One page of the list of CSE identities configured for the user.
    #[serde(rename="cseIdentities")]
    
    pub cse_identities: Option<Vec<CseIdentity>>,
    /// Pagination token to be passed to a subsequent ListCseIdentities call in order to retrieve the next page of identities. If this value is not returned or is the empty string, then no further pages remain.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCseIdentitiesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings cse keypairs list users](UserSettingCseKeypairListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListCseKeyPairsResponse {
    /// One page of the list of CSE key pairs installed for the user.
    #[serde(rename="cseKeyPairs")]
    
    pub cse_key_pairs: Option<Vec<CseKeyPair>>,
    /// Pagination token to be passed to a subsequent ListCseKeyPairs call in order to retrieve the next page of key pairs. If this value is not returned, then no further pages remain.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListCseKeyPairsResponse {}


/// Response for the ListDelegates method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings delegates list users](UserSettingDelegateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDelegatesResponse {
    /// List of the user's delegates (with any verification status). If an account doesn't have delegates, this field doesn't appear.
    
    pub delegates: Option<Vec<Delegate>>,
}

impl client::ResponseResult for ListDelegatesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [drafts list users](UserDraftListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDraftsResponse {
    /// List of drafts. Note that the `Message` property in each `Draft` resource only contains an `id` and a `threadId`. The messages.get method can fetch additional message details.
    
    pub drafts: Option<Vec<Draft>>,
    /// Token to retrieve the next page of results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Estimated total number of results.
    #[serde(rename="resultSizeEstimate")]
    
    pub result_size_estimate: Option<u32>,
}

impl client::ResponseResult for ListDraftsResponse {}


/// Response for the ListFilters method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings filters list users](UserSettingFilterListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFiltersResponse {
    /// List of a user's filters.
    
    pub filter: Option<Vec<Filter>>,
}

impl client::ResponseResult for ListFiltersResponse {}


/// Response for the ListForwardingAddresses method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings forwarding addresses list users](UserSettingForwardingAddressListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListForwardingAddressesResponse {
    /// List of addresses that may be used for forwarding.
    #[serde(rename="forwardingAddresses")]
    
    pub forwarding_addresses: Option<Vec<ForwardingAddress>>,
}

impl client::ResponseResult for ListForwardingAddressesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [history list users](UserHistoryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListHistoryResponse {
    /// List of history records. Any `messages` contained in the response will typically only have `id` and `threadId` fields populated.
    
    pub history: Option<Vec<History>>,
    /// The ID of the mailbox's current history record.
    #[serde(rename="historyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub history_id: Option<u64>,
    /// Page token to retrieve the next page of results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListHistoryResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [labels list users](UserLabelListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLabelsResponse {
    /// List of labels. Note that each label resource only contains an `id`, `name`, `messageListVisibility`, `labelListVisibility`, and `type`. The labels.get method can fetch additional label details.
    
    pub labels: Option<Vec<Label>>,
}

impl client::ResponseResult for ListLabelsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages list users](UserMessageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMessagesResponse {
    /// List of messages. Note that each message resource contains only an `id` and a `threadId`. Additional message details can be fetched using the messages.get method.
    
    pub messages: Option<Vec<Message>>,
    /// Token to retrieve the next page of results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Estimated total number of results.
    #[serde(rename="resultSizeEstimate")]
    
    pub result_size_estimate: Option<u32>,
}

impl client::ResponseResult for ListMessagesResponse {}


/// Response for the ListSendAs method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings send as list users](UserSettingSendAListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSendAsResponse {
    /// List of send-as aliases.
    #[serde(rename="sendAs")]
    
    pub send_as: Option<Vec<SendAs>>,
}

impl client::ResponseResult for ListSendAsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings send as smime info list users](UserSettingSendASmimeInfoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSmimeInfoResponse {
    /// List of SmimeInfo.
    #[serde(rename="smimeInfo")]
    
    pub smime_info: Option<Vec<SmimeInfo>>,
}

impl client::ResponseResult for ListSmimeInfoResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [threads list users](UserThreadListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListThreadsResponse {
    /// Page token to retrieve the next page of results in the list.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Estimated total number of results.
    #[serde(rename="resultSizeEstimate")]
    
    pub result_size_estimate: Option<u32>,
    /// List of threads. Note that each thread resource does not contain a list of `messages`. The list of `messages` for a given thread can be fetched using the threads.get method.
    
    pub threads: Option<Vec<Thread>>,
}

impl client::ResponseResult for ListThreadsResponse {}


/// An email message.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [drafts send users](UserDraftSendCall) (response)
/// * [messages get users](UserMessageGetCall) (response)
/// * [messages import users](UserMessageImportCall) (request|response)
/// * [messages insert users](UserMessageInsertCall) (request|response)
/// * [messages modify users](UserMessageModifyCall) (response)
/// * [messages send users](UserMessageSendCall) (request|response)
/// * [messages trash users](UserMessageTrashCall) (response)
/// * [messages untrash users](UserMessageUntrashCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// The ID of the last history record that modified this message.
    #[serde(rename="historyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub history_id: Option<u64>,
    /// The immutable ID of the message.
    
    pub id: Option<String>,
    /// The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.
    #[serde(rename="internalDate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub internal_date: Option<i64>,
    /// List of IDs of labels applied to this message.
    #[serde(rename="labelIds")]
    
    pub label_ids: Option<Vec<String>>,
    /// The parsed email structure in the message parts.
    
    pub payload: Option<MessagePart>,
    /// The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub raw: Option<Vec<u8>>,
    /// Estimated size in bytes of the message.
    #[serde(rename="sizeEstimate")]
    
    pub size_estimate: Option<i32>,
    /// A short part of the message text.
    
    pub snippet: Option<String>,
    /// The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match. 
    #[serde(rename="threadId")]
    
    pub thread_id: Option<String>,
}

impl client::RequestValue for Message {}
impl client::ResponseResult for Message {}


/// A single MIME message part.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MessagePart {
    /// The message part body for this part, which may be empty for container MIME message parts.
    
    pub body: Option<MessagePartBody>,
    /// The filename of the attachment. Only present if this message part represents an attachment.
    
    pub filename: Option<String>,
    /// List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.
    
    pub headers: Option<Vec<MessagePartHeader>>,
    /// The MIME type of the message part.
    #[serde(rename="mimeType")]
    
    pub mime_type: Option<String>,
    /// The immutable ID of the message part.
    #[serde(rename="partId")]
    
    pub part_id: Option<String>,
    /// The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.
    
    pub parts: Option<Vec<MessagePart>>,
}

impl client::Part for MessagePart {}


/// The body of a single MIME message part.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages attachments get users](UserMessageAttachmentGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MessagePartBody {
    /// When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.
    #[serde(rename="attachmentId")]
    
    pub attachment_id: Option<String>,
    /// The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Number of bytes for the message part data (encoding notwithstanding).
    
    pub size: Option<i32>,
}

impl client::ResponseResult for MessagePartBody {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MessagePartHeader {
    /// The name of the header before the `:` separator. For example, `To`.
    
    pub name: Option<String>,
    /// The value of the header after the `:` separator. For example, `someuser@example.com`.
    
    pub value: Option<String>,
}

impl client::Part for MessagePartHeader {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [messages modify users](UserMessageModifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyMessageRequest {
    /// A list of IDs of labels to add to this message. You can add up to 100 labels with each update.
    #[serde(rename="addLabelIds")]
    
    pub add_label_ids: Option<Vec<String>>,
    /// A list IDs of labels to remove from this message. You can remove up to 100 labels with each update.
    #[serde(rename="removeLabelIds")]
    
    pub remove_label_ids: Option<Vec<String>>,
}

impl client::RequestValue for ModifyMessageRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [threads modify users](UserThreadModifyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyThreadRequest {
    /// A list of IDs of labels to add to this thread. You can add up to 100 labels with each update.
    #[serde(rename="addLabelIds")]
    
    pub add_label_ids: Option<Vec<String>>,
    /// A list of IDs of labels to remove from this thread. You can remove up to 100 labels with each update.
    #[serde(rename="removeLabelIds")]
    
    pub remove_label_ids: Option<Vec<String>>,
}

impl client::RequestValue for ModifyThreadRequest {}


/// Request to obliterate a CSE key pair.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings cse keypairs obliterate users](UserSettingCseKeypairObliterateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObliterateCseKeyPairRequest { _never_set: Option<bool> }

impl client::RequestValue for ObliterateCseKeyPairRequest {}


/// POP settings for an account.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings get pop users](UserSettingGetPopCall) (response)
/// * [settings update pop users](UserSettingUpdatePopCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PopSettings {
    /// The range of messages which are accessible via POP.
    #[serde(rename="accessWindow")]
    
    pub access_window: Option<String>,
    /// The action that will be executed on a message after it has been fetched via POP.
    
    pub disposition: Option<String>,
}

impl client::RequestValue for PopSettings {}
impl client::ResponseResult for PopSettings {}


/// Profile for a Gmail user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get profile users](UserGetProfileCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    /// The user's email address.
    #[serde(rename="emailAddress")]
    
    pub email_address: Option<String>,
    /// The ID of the mailbox's current history record.
    #[serde(rename="historyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub history_id: Option<u64>,
    /// The total number of messages in the mailbox.
    #[serde(rename="messagesTotal")]
    
    pub messages_total: Option<i32>,
    /// The total number of threads in the mailbox.
    #[serde(rename="threadsTotal")]
    
    pub threads_total: Option<i32>,
}

impl client::ResponseResult for Profile {}


/// Settings associated with a send-as alias, which can be either the primary login address associated with the account or a custom “from” address. Send-as aliases correspond to the “Send Mail As” feature in the web interface.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings send as create users](UserSettingSendACreateCall) (request|response)
/// * [settings send as get users](UserSettingSendAGetCall) (response)
/// * [settings send as patch users](UserSettingSendAPatchCall) (request|response)
/// * [settings send as update users](UserSettingSendAUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendAs {
    /// A name that appears in the "From:" header for mail sent using this alias. For custom "from" addresses, when this is empty, Gmail will populate the "From:" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Whether this address is selected as the default "From:" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address.
    #[serde(rename="isDefault")]
    
    pub is_default: Option<bool>,
    /// Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only.
    #[serde(rename="isPrimary")]
    
    pub is_primary: Option<bool>,
    /// An optional email address that is included in a "Reply-To:" header for mail sent using this alias. If this is empty, Gmail will not generate a "Reply-To:" header.
    #[serde(rename="replyToAddress")]
    
    pub reply_to_address: Option<String>,
    /// The email address that appears in the "From:" header for mail sent using this alias. This is read-only for all operations except create.
    #[serde(rename="sendAsEmail")]
    
    pub send_as_email: Option<String>,
    /// An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only.
    
    pub signature: Option<String>,
    /// An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom "from" aliases.
    #[serde(rename="smtpMsa")]
    
    pub smtp_msa: Option<SmtpMsa>,
    /// Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom "from" aliases.
    #[serde(rename="treatAsAlias")]
    
    pub treat_as_alias: Option<bool>,
    /// Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases.
    #[serde(rename="verificationStatus")]
    
    pub verification_status: Option<String>,
}

impl client::RequestValue for SendAs {}
impl client::ResponseResult for SendAs {}


/// An S/MIME email config.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings send as smime info get users](UserSettingSendASmimeInfoGetCall) (response)
/// * [settings send as smime info insert users](UserSettingSendASmimeInfoInsertCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SmimeInfo {
    /// Encrypted key password, when key is encrypted.
    #[serde(rename="encryptedKeyPassword")]
    
    pub encrypted_key_password: Option<String>,
    /// When the certificate expires (in milliseconds since epoch).
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration: Option<i64>,
    /// The immutable ID for the SmimeInfo.
    
    pub id: Option<String>,
    /// Whether this SmimeInfo is the default one for this user's send-as address.
    #[serde(rename="isDefault")]
    
    pub is_default: Option<bool>,
    /// The S/MIME certificate issuer's common name.
    #[serde(rename="issuerCn")]
    
    pub issuer_cn: Option<String>,
    /// PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key).
    
    pub pem: Option<String>,
    /// PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub pkcs12: Option<Vec<u8>>,
}

impl client::RequestValue for SmimeInfo {}
impl client::ResponseResult for SmimeInfo {}


/// Configuration for communication with an SMTP service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SmtpMsa {
    /// The hostname of the SMTP service. Required.
    
    pub host: Option<String>,
    /// The password that will be used for authentication with the SMTP service. This is a write-only field that can be specified in requests to create or update SendAs settings; it is never populated in responses.
    
    pub password: Option<String>,
    /// The port of the SMTP service. Required.
    
    pub port: Option<i32>,
    /// The protocol that will be used to secure communication with the SMTP service. Required.
    #[serde(rename="securityMode")]
    
    pub security_mode: Option<String>,
    /// The username that will be used for authentication with the SMTP service. This is a write-only field that can be specified in requests to create or update SendAs settings; it is never populated in responses.
    
    pub username: Option<String>,
}

impl client::Part for SmtpMsa {}


/// A collection of messages representing a conversation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [threads get users](UserThreadGetCall) (response)
/// * [threads modify users](UserThreadModifyCall) (response)
/// * [threads trash users](UserThreadTrashCall) (response)
/// * [threads untrash users](UserThreadUntrashCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Thread {
    /// The ID of the last history record that modified this thread.
    #[serde(rename="historyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub history_id: Option<u64>,
    /// The unique ID of the thread.
    
    pub id: Option<String>,
    /// The list of messages in the thread.
    
    pub messages: Option<Vec<Message>>,
    /// A short part of the message text.
    
    pub snippet: Option<String>,
}

impl client::ResponseResult for Thread {}


/// Vacation auto-reply settings for an account. These settings correspond to the “Vacation responder” feature in the web interface.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [settings get vacation users](UserSettingGetVacationCall) (response)
/// * [settings update vacation users](UserSettingUpdateVacationCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct VacationSettings {
    /// Flag that controls whether Gmail automatically replies to messages.
    #[serde(rename="enableAutoReply")]
    
    pub enable_auto_reply: Option<bool>,
    /// An optional end time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives before the end time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`.
    #[serde(rename="endTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub end_time: Option<i64>,
    /// Response body in HTML format. Gmail will sanitize the HTML before storing it. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used.
    #[serde(rename="responseBodyHtml")]
    
    pub response_body_html: Option<String>,
    /// Response body in plain text format. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used.
    #[serde(rename="responseBodyPlainText")]
    
    pub response_body_plain_text: Option<String>,
    /// Optional text to prepend to the subject line in vacation responses. In order to enable auto-replies, either the response subject or the response body must be nonempty.
    #[serde(rename="responseSubject")]
    
    pub response_subject: Option<String>,
    /// Flag that determines whether responses are sent to recipients who are not in the user's list of contacts.
    #[serde(rename="restrictToContacts")]
    
    pub restrict_to_contacts: Option<bool>,
    /// Flag that determines whether responses are sent to recipients who are outside of the user's domain. This feature is only available for G Suite users.
    #[serde(rename="restrictToDomain")]
    
    pub restrict_to_domain: Option<bool>,
    /// An optional start time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives after the start time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`.
    #[serde(rename="startTime")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub start_time: Option<i64>,
}

impl client::RequestValue for VacationSettings {}
impl client::ResponseResult for VacationSettings {}


/// Set up or update a new push notification watch on this user’s mailbox.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [watch users](UserWatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WatchRequest {
    /// Filtering behavior of labelIds list specified.
    #[serde(rename="labelFilterAction")]
    
    pub label_filter_action: Option<String>,
    /// List of label_ids to restrict notifications about. By default, if unspecified, all changes are pushed out. If specified then dictates which labels are required for a push notification to be generated.
    #[serde(rename="labelIds")]
    
    pub label_ids: Option<Vec<String>>,
    /// A fully qualified Google Cloud Pub/Sub API topic name to publish the events to. This topic name **must** already exist in Cloud Pub/Sub and you **must** have already granted gmail "publish" permission on it. For example, "projects/my-project-identifier/topics/my-topic-name" (using the Cloud Pub/Sub "v1" topic naming format). Note that the "my-project-identifier" portion must exactly match your Google developer project id (the one executing this watch request).
    #[serde(rename="topicName")]
    
    pub topic_name: Option<String>,
}

impl client::RequestValue for WatchRequest {}


/// Push notification watch response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [watch users](UserWatchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WatchResponse {
    /// When Gmail will stop sending notifications for mailbox updates (epoch millis). Call `watch` again before this time to renew the watch.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration: Option<i64>,
    /// The ID of the mailbox's current history record.
    #[serde(rename="historyId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub history_id: Option<u64>,
}

impl client::ResponseResult for WatchResponse {}


