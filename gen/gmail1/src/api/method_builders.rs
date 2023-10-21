use super::*;
/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`Gmail`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_gmail1 as gmail1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use gmail1::{Gmail, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Gmail::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `drafts_create(...)`, `drafts_delete(...)`, `drafts_get(...)`, `drafts_list(...)`, `drafts_send(...)`, `drafts_update(...)`, `get_profile(...)`, `history_list(...)`, `labels_create(...)`, `labels_delete(...)`, `labels_get(...)`, `labels_list(...)`, `labels_patch(...)`, `labels_update(...)`, `messages_attachments_get(...)`, `messages_batch_delete(...)`, `messages_batch_modify(...)`, `messages_delete(...)`, `messages_get(...)`, `messages_import(...)`, `messages_insert(...)`, `messages_list(...)`, `messages_modify(...)`, `messages_send(...)`, `messages_trash(...)`, `messages_untrash(...)`, `settings_cse_identities_create(...)`, `settings_cse_identities_delete(...)`, `settings_cse_identities_get(...)`, `settings_cse_identities_list(...)`, `settings_cse_identities_patch(...)`, `settings_cse_keypairs_create(...)`, `settings_cse_keypairs_disable(...)`, `settings_cse_keypairs_enable(...)`, `settings_cse_keypairs_get(...)`, `settings_cse_keypairs_list(...)`, `settings_cse_keypairs_obliterate(...)`, `settings_delegates_create(...)`, `settings_delegates_delete(...)`, `settings_delegates_get(...)`, `settings_delegates_list(...)`, `settings_filters_create(...)`, `settings_filters_delete(...)`, `settings_filters_get(...)`, `settings_filters_list(...)`, `settings_forwarding_addresses_create(...)`, `settings_forwarding_addresses_delete(...)`, `settings_forwarding_addresses_get(...)`, `settings_forwarding_addresses_list(...)`, `settings_get_auto_forwarding(...)`, `settings_get_imap(...)`, `settings_get_language(...)`, `settings_get_pop(...)`, `settings_get_vacation(...)`, `settings_send_as_create(...)`, `settings_send_as_delete(...)`, `settings_send_as_get(...)`, `settings_send_as_list(...)`, `settings_send_as_patch(...)`, `settings_send_as_smime_info_delete(...)`, `settings_send_as_smime_info_get(...)`, `settings_send_as_smime_info_insert(...)`, `settings_send_as_smime_info_list(...)`, `settings_send_as_smime_info_set_default(...)`, `settings_send_as_update(...)`, `settings_send_as_verify(...)`, `settings_update_auto_forwarding(...)`, `settings_update_imap(...)`, `settings_update_language(...)`, `settings_update_pop(...)`, `settings_update_vacation(...)`, `stop(...)`, `threads_delete(...)`, `threads_get(...)`, `threads_list(...)`, `threads_modify(...)`, `threads_trash(...)`, `threads_untrash(...)` and `watch(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gmail<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new draft with the `DRAFT` label.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn drafts_create(&self, request: Draft, user_id: &str) -> UserDraftCreateCall<'a, S> {
        UserDraftCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Immediately and permanently deletes the specified draft. Does not simply trash it.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the draft to delete.
    pub fn drafts_delete(&self, user_id: &str, id: &str) -> UserDraftDeleteCall<'a, S> {
        UserDraftDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified draft.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the draft to retrieve.
    pub fn drafts_get(&self, user_id: &str, id: &str) -> UserDraftGetCall<'a, S> {
        UserDraftGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the drafts in the user's mailbox.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn drafts_list(&self, user_id: &str) -> UserDraftListCall<'a, S> {
        UserDraftListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _q: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_spam_trash: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sends the specified, existing draft to the recipients in the `To`, `Cc`, and `Bcc` headers.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn drafts_send(&self, request: Draft, user_id: &str) -> UserDraftSendCall<'a, S> {
        UserDraftSendCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Replaces a draft's content.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the draft to update.
    pub fn drafts_update(&self, request: Draft, user_id: &str, id: &str) -> UserDraftUpdateCall<'a, S> {
        UserDraftUpdateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing `historyId`).
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn history_list(&self, user_id: &str) -> UserHistoryListCall<'a, S> {
        UserHistoryListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _start_history_id: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _label_id: Default::default(),
            _history_types: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new label.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn labels_create(&self, request: Label, user_id: &str) -> UserLabelCreateCall<'a, S> {
        UserLabelCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Immediately and permanently deletes the specified label and removes it from any messages and threads that it is applied to.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the label to delete.
    pub fn labels_delete(&self, user_id: &str, id: &str) -> UserLabelDeleteCall<'a, S> {
        UserLabelDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified label.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the label to retrieve.
    pub fn labels_get(&self, user_id: &str, id: &str) -> UserLabelGetCall<'a, S> {
        UserLabelGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all labels in the user's mailbox.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn labels_list(&self, user_id: &str) -> UserLabelListCall<'a, S> {
        UserLabelListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patch the specified label.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the label to update.
    pub fn labels_patch(&self, request: Label, user_id: &str, id: &str) -> UserLabelPatchCall<'a, S> {
        UserLabelPatchCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified label.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the label to update.
    pub fn labels_update(&self, request: Label, user_id: &str, id: &str) -> UserLabelUpdateCall<'a, S> {
        UserLabelUpdateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified message attachment.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `messageId` - The ID of the message containing the attachment.
    /// * `id` - The ID of the attachment.
    pub fn messages_attachments_get(&self, user_id: &str, message_id: &str, id: &str) -> UserMessageAttachmentGetCall<'a, S> {
        UserMessageAttachmentGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _message_id: message_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes many messages by message ID. Provides no guarantees that messages were not already deleted or even existed at all.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn messages_batch_delete(&self, request: BatchDeleteMessagesRequest, user_id: &str) -> UserMessageBatchDeleteCall<'a, S> {
        UserMessageBatchDeleteCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the labels on the specified messages.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn messages_batch_modify(&self, request: BatchModifyMessagesRequest, user_id: &str) -> UserMessageBatchModifyCall<'a, S> {
        UserMessageBatchModifyCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Immediately and permanently deletes the specified message. This operation cannot be undone. Prefer `messages.trash` instead.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the message to delete.
    pub fn messages_delete(&self, user_id: &str, id: &str) -> UserMessageDeleteCall<'a, S> {
        UserMessageDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified message.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the message to retrieve. This ID is usually retrieved using `messages.list`. The ID is also contained in the result when a message is inserted (`messages.insert`) or imported (`messages.import`).
    pub fn messages_get(&self, user_id: &str, id: &str) -> UserMessageGetCall<'a, S> {
        UserMessageGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _metadata_headers: Default::default(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Imports a message into only this user's mailbox, with standard email delivery scanning and classification similar to receiving via SMTP. This method doesn't perform SPF checks, so it might not work for some spam messages, such as those attempting to perform domain spoofing. This method does not send a message. Note: This function doesn't trigger forwarding rules or filters set up by the user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn messages_import(&self, request: Message, user_id: &str) -> UserMessageImportCall<'a, S> {
        UserMessageImportCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _process_for_calendar: Default::default(),
            _never_mark_spam: Default::default(),
            _internal_date_source: Default::default(),
            _deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn messages_insert(&self, request: Message, user_id: &str) -> UserMessageInsertCall<'a, S> {
        UserMessageInsertCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _internal_date_source: Default::default(),
            _deleted: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the messages in the user's mailbox.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn messages_list(&self, user_id: &str) -> UserMessageListCall<'a, S> {
        UserMessageListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _q: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _label_ids: Default::default(),
            _include_spam_trash: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the labels on the specified message.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the message to modify.
    pub fn messages_modify(&self, request: ModifyMessageRequest, user_id: &str, id: &str) -> UserMessageModifyCall<'a, S> {
        UserMessageModifyCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sends the specified message to the recipients in the `To`, `Cc`, and `Bcc` headers.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn messages_send(&self, request: Message, user_id: &str) -> UserMessageSendCall<'a, S> {
        UserMessageSendCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves the specified message to the trash.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the message to Trash.
    pub fn messages_trash(&self, user_id: &str, id: &str) -> UserMessageTrashCall<'a, S> {
        UserMessageTrashCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified message from the trash.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the message to remove from Trash.
    pub fn messages_untrash(&self, user_id: &str, id: &str) -> UserMessageUntrashCall<'a, S> {
        UserMessageUntrashCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates and configures a client-side encryption identity that's authorized to send mail from the user account. Google publishes the S/MIME certificate to a shared domain-wide directory so that people within a Google Workspace organization can encrypt and send mail to the identity.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    pub fn settings_cse_identities_create(&self, request: CseIdentity, user_id: &str) -> UserSettingCseIdentityCreateCall<'a, S> {
        UserSettingCseIdentityCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a client-side encryption identity. The authenticated user can no longer use the identity to send encrypted messages. You cannot restore the identity after you delete it. Instead, use the CreateCseIdentity method to create another identity with the same configuration.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    /// * `cseEmailAddress` - The primary email address associated with the client-side encryption identity configuration that's removed.
    pub fn settings_cse_identities_delete(&self, user_id: &str, cse_email_address: &str) -> UserSettingCseIdentityDeleteCall<'a, S> {
        UserSettingCseIdentityDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _cse_email_address: cse_email_address.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a client-side encryption identity configuration.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    /// * `cseEmailAddress` - The primary email address associated with the client-side encryption identity configuration that's retrieved.
    pub fn settings_cse_identities_get(&self, user_id: &str, cse_email_address: &str) -> UserSettingCseIdentityGetCall<'a, S> {
        UserSettingCseIdentityGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _cse_email_address: cse_email_address.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the client-side encrypted identities for an authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    pub fn settings_cse_identities_list(&self, user_id: &str) -> UserSettingCseIdentityListCall<'a, S> {
        UserSettingCseIdentityListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associates a different key pair with an existing client-side encryption identity. The updated key pair must validate against Google's [S/MIME certificate profiles](https://support.google.com/a/answer/7300887).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    /// * `emailAddress` - The email address of the client-side encryption identity to update.
    pub fn settings_cse_identities_patch(&self, request: CseIdentity, user_id: &str, email_address: &str) -> UserSettingCseIdentityPatchCall<'a, S> {
        UserSettingCseIdentityPatchCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _email_address: email_address.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates and uploads a client-side encryption S/MIME public key certificate chain and private key metadata for the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    pub fn settings_cse_keypairs_create(&self, request: CseKeyPair, user_id: &str) -> UserSettingCseKeypairCreateCall<'a, S> {
        UserSettingCseKeypairCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Turns off a client-side encryption key pair. The authenticated user can no longer use the key pair to decrypt incoming CSE message texts or sign outgoing CSE mail. To regain access, use the EnableCseKeyPair to turn on the key pair. After 30 days, you can permanently delete the key pair by using the ObliterateCseKeyPair method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    /// * `keyPairId` - The identifier of the key pair to turn off.
    pub fn settings_cse_keypairs_disable(&self, request: DisableCseKeyPairRequest, user_id: &str, key_pair_id: &str) -> UserSettingCseKeypairDisableCall<'a, S> {
        UserSettingCseKeypairDisableCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _key_pair_id: key_pair_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Turns on a client-side encryption key pair that was turned off. The key pair becomes active again for any associated client-side encryption identities.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    /// * `keyPairId` - The identifier of the key pair to turn on.
    pub fn settings_cse_keypairs_enable(&self, request: EnableCseKeyPairRequest, user_id: &str, key_pair_id: &str) -> UserSettingCseKeypairEnableCall<'a, S> {
        UserSettingCseKeypairEnableCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _key_pair_id: key_pair_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an existing client-side encryption key pair.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    /// * `keyPairId` - The identifier of the key pair to retrieve.
    pub fn settings_cse_keypairs_get(&self, user_id: &str, key_pair_id: &str) -> UserSettingCseKeypairGetCall<'a, S> {
        UserSettingCseKeypairGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _key_pair_id: key_pair_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists client-side encryption key pairs for an authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    pub fn settings_cse_keypairs_list(&self, user_id: &str) -> UserSettingCseKeypairListCall<'a, S> {
        UserSettingCseKeypairListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a client-side encryption key pair permanently and immediately. You can only permanently delete key pairs that have been turned off for more than 30 days. To turn off a key pair, use the DisableCseKeyPair method. Gmail can't restore or decrypt any messages that were encrypted by an obliterated key. Authenticated users and Google Workspace administrators lose access to reading the encrypted messages.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
    /// * `keyPairId` - The identifier of the key pair to obliterate.
    pub fn settings_cse_keypairs_obliterate(&self, request: ObliterateCseKeyPairRequest, user_id: &str, key_pair_id: &str) -> UserSettingCseKeypairObliterateCall<'a, S> {
        UserSettingCseKeypairObliterateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _key_pair_id: key_pair_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a delegate with its verification status set directly to `accepted`, without sending any verification email. The delegate user must be a member of the same G Suite organization as the delegator user. Gmail imposes limitations on the number of delegates and delegators each user in a G Suite organization can have. These limits depend on your organization, but in general each user can have up to 25 delegates and up to 10 delegators. Note that a delegate user must be referred to by their primary email address, and not an email alias. Also note that when a new delegate is created, there may be up to a one minute delay before the new delegate is available for use. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_delegates_create(&self, request: Delegate, user_id: &str) -> UserSettingDelegateCreateCall<'a, S> {
        UserSettingDelegateCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified delegate (which can be of any verification status), and revokes any verification that may have been required for using it. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `delegateEmail` - The email address of the user to be removed as a delegate.
    pub fn settings_delegates_delete(&self, user_id: &str, delegate_email: &str) -> UserSettingDelegateDeleteCall<'a, S> {
        UserSettingDelegateDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate_email: delegate_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified delegate. Note that a delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `delegateEmail` - The email address of the user whose delegate relationship is to be retrieved.
    pub fn settings_delegates_get(&self, user_id: &str, delegate_email: &str) -> UserSettingDelegateGetCall<'a, S> {
        UserSettingDelegateGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate_email: delegate_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the delegates for the specified account. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_delegates_list(&self, user_id: &str) -> UserSettingDelegateListCall<'a, S> {
        UserSettingDelegateListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a filter. Note: you can only create a maximum of 1,000 filters.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_filters_create(&self, request: Filter, user_id: &str) -> UserSettingFilterCreateCall<'a, S> {
        UserSettingFilterCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Immediately and permanently deletes the specified filter.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `id` - The ID of the filter to be deleted.
    pub fn settings_filters_delete(&self, user_id: &str, id: &str) -> UserSettingFilterDeleteCall<'a, S> {
        UserSettingFilterDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a filter.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `id` - The ID of the filter to be fetched.
    pub fn settings_filters_get(&self, user_id: &str, id: &str) -> UserSettingFilterGetCall<'a, S> {
        UserSettingFilterGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the message filters of a Gmail user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_filters_list(&self, user_id: &str) -> UserSettingFilterListCall<'a, S> {
        UserSettingFilterListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a forwarding address. If ownership verification is required, a message will be sent to the recipient and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_forwarding_addresses_create(&self, request: ForwardingAddress, user_id: &str) -> UserSettingForwardingAddressCreateCall<'a, S> {
        UserSettingForwardingAddressCreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified forwarding address and revokes any verification that may have been required. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `forwardingEmail` - The forwarding address to be deleted.
    pub fn settings_forwarding_addresses_delete(&self, user_id: &str, forwarding_email: &str) -> UserSettingForwardingAddressDeleteCall<'a, S> {
        UserSettingForwardingAddressDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _forwarding_email: forwarding_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified forwarding address.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `forwardingEmail` - The forwarding address to be retrieved.
    pub fn settings_forwarding_addresses_get(&self, user_id: &str, forwarding_email: &str) -> UserSettingForwardingAddressGetCall<'a, S> {
        UserSettingForwardingAddressGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _forwarding_email: forwarding_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the forwarding addresses for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_forwarding_addresses_list(&self, user_id: &str) -> UserSettingForwardingAddressListCall<'a, S> {
        UserSettingForwardingAddressListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified S/MIME config for the specified send-as alias.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The email address that appears in the "From:" header for mail sent using this alias.
    /// * `id` - The immutable ID for the SmimeInfo.
    pub fn settings_send_as_smime_info_delete(&self, user_id: &str, send_as_email: &str, id: &str) -> UserSettingSendASmimeInfoDeleteCall<'a, S> {
        UserSettingSendASmimeInfoDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified S/MIME config for the specified send-as alias.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The email address that appears in the "From:" header for mail sent using this alias.
    /// * `id` - The immutable ID for the SmimeInfo.
    pub fn settings_send_as_smime_info_get(&self, user_id: &str, send_as_email: &str, id: &str) -> UserSettingSendASmimeInfoGetCall<'a, S> {
        UserSettingSendASmimeInfoGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Insert (upload) the given S/MIME config for the specified send-as alias. Note that pkcs12 format is required for the key.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The email address that appears in the "From:" header for mail sent using this alias.
    pub fn settings_send_as_smime_info_insert(&self, request: SmimeInfo, user_id: &str, send_as_email: &str) -> UserSettingSendASmimeInfoInsertCall<'a, S> {
        UserSettingSendASmimeInfoInsertCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists S/MIME configs for the specified send-as alias.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The email address that appears in the "From:" header for mail sent using this alias.
    pub fn settings_send_as_smime_info_list(&self, user_id: &str, send_as_email: &str) -> UserSettingSendASmimeInfoListCall<'a, S> {
        UserSettingSendASmimeInfoListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the default S/MIME config for the specified send-as alias.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The email address that appears in the "From:" header for mail sent using this alias.
    /// * `id` - The immutable ID for the SmimeInfo.
    pub fn settings_send_as_smime_info_set_default(&self, user_id: &str, send_as_email: &str, id: &str) -> UserSettingSendASmimeInfoSetDefaultCall<'a, S> {
        UserSettingSendASmimeInfoSetDefaultCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a custom "from" send-as alias. If an SMTP MSA is specified, Gmail will attempt to connect to the SMTP service to validate the configuration before creating the alias. If ownership verification is required for the alias, a message will be sent to the email address and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_send_as_create(&self, request: SendAs, user_id: &str) -> UserSettingSendACreateCall<'a, S> {
        UserSettingSendACreateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified send-as alias. Revokes any verification that may have been required for using it. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The send-as alias to be deleted.
    pub fn settings_send_as_delete(&self, user_id: &str, send_as_email: &str) -> UserSettingSendADeleteCall<'a, S> {
        UserSettingSendADeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified send-as alias. Fails with an HTTP 404 error if the specified address is not a member of the collection.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The send-as alias to be retrieved.
    pub fn settings_send_as_get(&self, user_id: &str, send_as_email: &str) -> UserSettingSendAGetCall<'a, S> {
        UserSettingSendAGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the send-as aliases for the specified account. The result includes the primary send-as address associated with the account as well as any custom "from" aliases.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_send_as_list(&self, user_id: &str) -> UserSettingSendAListCall<'a, S> {
        UserSettingSendAListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patch the specified send-as alias.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The send-as alias to be updated.
    pub fn settings_send_as_patch(&self, request: SendAs, user_id: &str, send_as_email: &str) -> UserSettingSendAPatchCall<'a, S> {
        UserSettingSendAPatchCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a send-as alias. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. Addresses other than the primary address for the account can only be updated by service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The send-as alias to be updated.
    pub fn settings_send_as_update(&self, request: SendAs, user_id: &str, send_as_email: &str) -> UserSettingSendAUpdateCall<'a, S> {
        UserSettingSendAUpdateCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sends a verification email to the specified send-as alias address. The verification status must be `pending`. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    /// * `sendAsEmail` - The send-as alias to be verified.
    pub fn settings_send_as_verify(&self, user_id: &str, send_as_email: &str) -> UserSettingSendAVerifyCall<'a, S> {
        UserSettingSendAVerifyCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _send_as_email: send_as_email.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the auto-forwarding setting for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_get_auto_forwarding(&self, user_id: &str) -> UserSettingGetAutoForwardingCall<'a, S> {
        UserSettingGetAutoForwardingCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets IMAP settings.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_get_imap(&self, user_id: &str) -> UserSettingGetImapCall<'a, S> {
        UserSettingGetImapCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets language settings.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_get_language(&self, user_id: &str) -> UserSettingGetLanguageCall<'a, S> {
        UserSettingGetLanguageCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets POP settings.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_get_pop(&self, user_id: &str) -> UserSettingGetPopCall<'a, S> {
        UserSettingGetPopCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets vacation responder settings.
    /// 
    /// # Arguments
    ///
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_get_vacation(&self, user_id: &str) -> UserSettingGetVacationCall<'a, S> {
        UserSettingGetVacationCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the auto-forwarding setting for the specified account. A verified forwarding address must be specified when auto-forwarding is enabled. This method is only available to service account clients that have been delegated domain-wide authority.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_update_auto_forwarding(&self, request: AutoForwarding, user_id: &str) -> UserSettingUpdateAutoForwardingCall<'a, S> {
        UserSettingUpdateAutoForwardingCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates IMAP settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_update_imap(&self, request: ImapSettings, user_id: &str) -> UserSettingUpdateImapCall<'a, S> {
        UserSettingUpdateImapCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates language settings. If successful, the return object contains the `displayLanguage` that was saved for the user, which may differ from the value passed into the request. This is because the requested `displayLanguage` may not be directly supported by Gmail but have a close variant that is, and so the variant may be chosen and saved instead.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_update_language(&self, request: LanguageSettings, user_id: &str) -> UserSettingUpdateLanguageCall<'a, S> {
        UserSettingUpdateLanguageCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates POP settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_update_pop(&self, request: PopSettings, user_id: &str) -> UserSettingUpdatePopCall<'a, S> {
        UserSettingUpdatePopCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates vacation responder settings.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - User's email address. The special value "me" can be used to indicate the authenticated user.
    pub fn settings_update_vacation(&self, request: VacationSettings, user_id: &str) -> UserSettingUpdateVacationCall<'a, S> {
        UserSettingUpdateVacationCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Immediately and permanently deletes the specified thread. Any messages that belong to the thread are also deleted. This operation cannot be undone. Prefer `threads.trash` instead.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - ID of the Thread to delete.
    pub fn threads_delete(&self, user_id: &str, id: &str) -> UserThreadDeleteCall<'a, S> {
        UserThreadDeleteCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified thread.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the thread to retrieve.
    pub fn threads_get(&self, user_id: &str, id: &str) -> UserThreadGetCall<'a, S> {
        UserThreadGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _metadata_headers: Default::default(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the threads in the user's mailbox.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn threads_list(&self, user_id: &str) -> UserThreadListCall<'a, S> {
        UserThreadListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _q: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _label_ids: Default::default(),
            _include_spam_trash: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the labels applied to the thread. This applies to all messages in the thread.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the thread to modify.
    pub fn threads_modify(&self, request: ModifyThreadRequest, user_id: &str, id: &str) -> UserThreadModifyCall<'a, S> {
        UserThreadModifyCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves the specified thread to the trash. Any messages that belong to the thread are also moved to the trash.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the thread to Trash.
    pub fn threads_trash(&self, user_id: &str, id: &str) -> UserThreadTrashCall<'a, S> {
        UserThreadTrashCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified thread from the trash. Any messages that belong to the thread are also removed from the trash.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    /// * `id` - The ID of the thread to remove from Trash.
    pub fn threads_untrash(&self, user_id: &str, id: &str) -> UserThreadUntrashCall<'a, S> {
        UserThreadUntrashCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the current user's Gmail profile.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn get_profile(&self, user_id: &str) -> UserGetProfileCall<'a, S> {
        UserGetProfileCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop receiving push notifications for the given user mailbox.
    /// 
    /// # Arguments
    ///
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn stop(&self, user_id: &str) -> UserStopCall<'a, S> {
        UserStopCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Set up or update a push notification watch on the given user mailbox.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `userId` - The user's email address. The special value `me` can be used to indicate the authenticated user.
    pub fn watch(&self, request: WatchRequest, user_id: &str) -> UserWatchCall<'a, S> {
        UserWatchCall {
            hub: self.hub,
            _request: request,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



