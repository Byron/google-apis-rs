use super::*;
/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`MyBusiness`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusiness4 as mybusiness4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `admins_create(...)`, `admins_delete(...)`, `admins_list(...)`, `admins_patch(...)`, `create(...)`, `delete_notifications(...)`, `generate_account_number(...)`, `get(...)`, `get_notifications(...)`, `invitations_accept(...)`, `invitations_decline(...)`, `invitations_list(...)`, `list(...)`, `list_recommend_google_locations(...)`, `locations_admins_create(...)`, `locations_admins_delete(...)`, `locations_admins_list(...)`, `locations_admins_patch(...)`, `locations_associate(...)`, `locations_batch_get(...)`, `locations_batch_get_reviews(...)`, `locations_clear_association(...)`, `locations_create(...)`, `locations_delete(...)`, `locations_fetch_verification_options(...)`, `locations_find_matches(...)`, `locations_followers_get_metadata(...)`, `locations_get(...)`, `locations_get_google_updated(...)`, `locations_list(...)`, `locations_local_posts_create(...)`, `locations_local_posts_delete(...)`, `locations_local_posts_get(...)`, `locations_local_posts_list(...)`, `locations_local_posts_patch(...)`, `locations_local_posts_report_insights(...)`, `locations_media_create(...)`, `locations_media_customers_get(...)`, `locations_media_customers_list(...)`, `locations_media_delete(...)`, `locations_media_get(...)`, `locations_media_list(...)`, `locations_media_patch(...)`, `locations_media_start_upload(...)`, `locations_patch(...)`, `locations_questions_answers_delete(...)`, `locations_questions_answers_list(...)`, `locations_questions_answers_upsert(...)`, `locations_questions_create(...)`, `locations_questions_delete(...)`, `locations_questions_list(...)`, `locations_questions_patch(...)`, `locations_report_insights(...)`, `locations_reviews_delete_reply(...)`, `locations_reviews_get(...)`, `locations_reviews_list(...)`, `locations_reviews_update_reply(...)`, `locations_transfer(...)`, `locations_verifications_complete(...)`, `locations_verifications_list(...)`, `locations_verify(...)`, `update(...)` and `update_notifications(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusiness<S>,
}

impl<'a, S> client::MethodsBuilder for AccountMethods<'a, S> {}

impl<'a, S> AccountMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Invites the specified user to become an administrator for the specified
    /// account. The invitee must accept the invitation in order to be granted
    /// access to the account. See AcceptInvitation to programmatically accept an
    /// invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the account this admin is created for.
    pub fn admins_create(&self, request: Admin, parent: &str) -> AccountAdminCreateCall<'a, S> {
        AccountAdminCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified admin from the specified account.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the admin to remove from the account.
    pub fn admins_delete(&self, name: &str) -> AccountAdminDeleteCall<'a, S> {
        AccountAdminDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the admins for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the account from which to retrieve a list of admins.
    pub fn admins_list(&self, parent: &str) -> AccountAdminListCall<'a, S> {
        AccountAdminListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Admin for the specified Account Admin. Only the AdminRole of
    /// the Admin can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the admin to update.
    pub fn admins_patch(&self, request: Admin, name: &str) -> AccountAdminPatchCall<'a, S> {
        AccountAdminPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Accepts the specified invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the invitation that is being accepted.
    pub fn invitations_accept(&self, request: AcceptInvitationRequest, name: &str) -> AccountInvitationAcceptCall<'a, S> {
        AccountInvitationAcceptCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Declines the specified invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the account invitation that is being declined.
    pub fn invitations_decline(&self, request: DeclineInvitationRequest, name: &str) -> AccountInvitationDeclineCall<'a, S> {
        AccountInvitationDeclineCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists pending invitations for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the account from which the list of invitations is being
    ///              retrieved.
    pub fn invitations_list(&self, parent: &str) -> AccountInvitationListCall<'a, S> {
        AccountInvitationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _target_type: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Invites the specified user to become an administrator for the specified
    /// location. The invitee must accept the invitation in order to be granted
    /// access to the location. See AcceptInvitation to programmatically accept an
    /// invitation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the location this admin is created for.
    pub fn locations_admins_create(&self, request: Admin, parent: &str) -> AccountLocationAdminCreateCall<'a, S> {
        AccountLocationAdminCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified admin as a manager of the specified location.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the admin to remove from the location.
    pub fn locations_admins_delete(&self, name: &str) -> AccountLocationAdminDeleteCall<'a, S> {
        AccountLocationAdminDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the admins for the specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the location to list admins of.
    pub fn locations_admins_list(&self, parent: &str) -> AccountLocationAdminListCall<'a, S> {
        AccountLocationAdminListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the Admin for the specified Location Admin. Only the AdminRole of
    /// the Admin can be updated.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the admin to update.
    pub fn locations_admins_patch(&self, request: Admin, name: &str) -> AccountLocationAdminPatchCall<'a, S> {
        AccountLocationAdminPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the followers settings for a location.
    /// 
    /// NOT_FOUND is returned if either the account or the location doesn't exist.
    /// PRECONDITION_FAILED is returned if the location is not verified nor
    /// connected to Maps.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the location's followers metadata.
    ///            accounts/{account_id}/locations/{location_id}/followers/metadata
    pub fn locations_followers_get_metadata(&self, name: &str) -> AccountLocationFollowerGetMetadataCall<'a, S> {
        AccountLocationFollowerGetMetadataCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new local post associated with the specified location, and
    /// returns it.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the location in which to create this local post.
    pub fn locations_local_posts_create(&self, request: LocalPost, parent: &str) -> AccountLocationLocalPostCreateCall<'a, S> {
        AccountLocationLocalPostCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a local post.
    /// Returns `NOT_FOUND` if the local post does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the local post to delete.
    pub fn locations_local_posts_delete(&self, name: &str) -> AccountLocationLocalPostDeleteCall<'a, S> {
        AccountLocationLocalPostDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified local post.
    /// Returns `NOT_FOUND` if the local post does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the local post to fetch.
    pub fn locations_local_posts_get(&self, name: &str) -> AccountLocationLocalPostGetCall<'a, S> {
        AccountLocationLocalPostGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of local posts associated with a location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the location whose local posts will be listed.
    pub fn locations_local_posts_list(&self, parent: &str) -> AccountLocationLocalPostListCall<'a, S> {
        AccountLocationLocalPostListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified local post and returns the updated local post.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the local post to update.
    pub fn locations_local_posts_patch(&self, request: LocalPost, name: &str) -> AccountLocationLocalPostPatchCall<'a, S> {
        AccountLocationLocalPostPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns insights for a set of local posts associated with a single listing.
    /// Which metrics and how they are reported are options specified in the
    /// request proto.
    /// <aside class="note"><b>Note:</b> Insight reports are limited
    /// to 100 `local_post_names` per call.</aside>
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The name of the location for which to fetch insights.
    pub fn locations_local_posts_report_insights(&self, request: ReportLocalPostInsightsRequest, name: &str) -> AccountLocationLocalPostReportInsightCall<'a, S> {
        AccountLocationLocalPostReportInsightCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for the requested customer media item.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name of the requested customer media item.
    pub fn locations_media_customers_get(&self, name: &str) -> AccountLocationMediaCustomerGetCall<'a, S> {
        AccountLocationMediaCustomerGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of media items associated with a location that have been
    /// contributed by customers.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the location whose customer media items will be listed.
    pub fn locations_media_customers_list(&self, parent: &str) -> AccountLocationMediaCustomerListCall<'a, S> {
        AccountLocationMediaCustomerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new media item for the location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the location where this media item will be created.
    pub fn locations_media_create(&self, request: MediaItem, parent: &str) -> AccountLocationMediaCreateCall<'a, S> {
        AccountLocationMediaCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified media item.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the media item to be deleted.
    pub fn locations_media_delete(&self, name: &str) -> AccountLocationMediaDeleteCall<'a, S> {
        AccountLocationMediaDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for the requested media item.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the requested media item.
    pub fn locations_media_get(&self, name: &str) -> AccountLocationMediaGetCall<'a, S> {
        AccountLocationMediaGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of media items associated with a location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the location whose media items will be listed.
    pub fn locations_media_list(&self, parent: &str) -> AccountLocationMediaListCall<'a, S> {
        AccountLocationMediaListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates metadata of the specified media item.
    /// This can only be used to update the Category of a media item,
    /// with the exception that the new category cannot be COVER or PROFILE.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the media item to be updated.
    pub fn locations_media_patch(&self, request: MediaItem, name: &str) -> AccountLocationMediaPatchCall<'a, S> {
        AccountLocationMediaPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a `MediaItemDataRef` for media item uploading.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The resource name of the location this media item is to be added to.
    pub fn locations_media_start_upload(&self, request: StartUploadMediaItemDataRequest, parent: &str) -> AccountLocationMediaStartUploadCall<'a, S> {
        AccountLocationMediaStartUploadCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the answer written by the current user to a question.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the question to delete an answer for.
    pub fn locations_questions_answers_delete(&self, parent: &str) -> AccountLocationQuestionAnswerDeleteCall<'a, S> {
        AccountLocationQuestionAnswerDeleteCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the paginated list of answers for a specified question.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the question to fetch answers for.
    pub fn locations_questions_answers_list(&self, parent: &str) -> AccountLocationQuestionAnswerListCall<'a, S> {
        AccountLocationQuestionAnswerListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an answer or updates the existing answer written by the user for
    /// the specified question. A user can only create one answer per question.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the question to write an answer for.
    pub fn locations_questions_answers_upsert(&self, request: UpsertAnswerRequest, parent: &str) -> AccountLocationQuestionAnswerUpsertCall<'a, S> {
        AccountLocationQuestionAnswerUpsertCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a question for the specified location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the location to write a question for.
    pub fn locations_questions_create(&self, request: Question, parent: &str) -> AccountLocationQuestionCreateCall<'a, S> {
        AccountLocationQuestionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a specific question written by the current user.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the question to delete.
    pub fn locations_questions_delete(&self, name: &str) -> AccountLocationQuestionDeleteCall<'a, S> {
        AccountLocationQuestionDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the paginated list of questions and some of its answers for a
    /// specified location.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the location to fetch questions for.
    pub fn locations_questions_list(&self, parent: &str) -> AccountLocationQuestionListCall<'a, S> {
        AccountLocationQuestionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _answers_per_question: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a specific question written by the current user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the question to update.
    pub fn locations_questions_patch(&self, request: Question, name: &str) -> AccountLocationQuestionPatchCall<'a, S> {
        AccountLocationQuestionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the response to the specified review.
    /// This operation is only valid if the specified location is verified.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the review reply to delete.
    pub fn locations_reviews_delete_reply(&self, name: &str) -> AccountLocationReviewDeleteReplyCall<'a, S> {
        AccountLocationReviewDeleteReplyCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified review.
    /// This operation is only valid if the specified location is verified.
    /// Returns `NOT_FOUND` if the review does not exist, or has been deleted.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the review to fetch.
    pub fn locations_reviews_get(&self, name: &str) -> AccountLocationReviewGetCall<'a, S> {
        AccountLocationReviewGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the paginated list of reviews for the specified location.
    /// This operation is only valid if the specified location is verified.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the location to fetch reviews for.
    pub fn locations_reviews_list(&self, parent: &str) -> AccountLocationReviewListCall<'a, S> {
        AccountLocationReviewListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the reply to the specified review.
    /// A reply is created if one does not exist.
    /// This operation is only valid if the specified location is verified.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the review to respond to.
    pub fn locations_reviews_update_reply(&self, request: ReviewReply, name: &str) -> AccountLocationReviewUpdateReplyCall<'a, S> {
        AccountLocationReviewUpdateReplyCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Completes a `PENDING` verification.
    /// 
    /// It is only necessary for non `AUTO` verification methods. `AUTO`
    /// verification request is instantly `VERIFIED` upon creation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the verification to complete.
    pub fn locations_verifications_complete(&self, request: CompleteVerificationRequest, name: &str) -> AccountLocationVerificationCompleteCall<'a, S> {
        AccountLocationVerificationCompleteCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List verifications of a location, ordered by create time.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Resource name of the location that verification requests belong to.
    pub fn locations_verifications_list(&self, parent: &str) -> AccountLocationVerificationListCall<'a, S> {
        AccountLocationVerificationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Associates a location to a place ID. Any previous association is
    /// overwritten. This operation is only valid if the location is unverified.
    /// The association must be valid, that is, it appears in the list of
    /// `FindMatchingLocations`.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the location to associate.
    pub fn locations_associate(&self, request: AssociateLocationRequest, name: &str) -> AccountLocationAssociateCall<'a, S> {
        AccountLocationAssociateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets all of the specified locations in the given account.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the account from which to fetch locations.
    pub fn locations_batch_get(&self, request: BatchGetLocationsRequest, name: &str) -> AccountLocationBatchGetCall<'a, S> {
        AccountLocationBatchGetCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the paginated list of reviews for all specified locations.
    /// This operation is only valid if the specified locations are verified.
    /// <aside class="note"><b>Note:</b> Reviews are limited
    /// to a batch size of 200 `location_names` per call.</aside>
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the account from which to retrieve a list of reviews across
    ///            multiple locations.
    pub fn locations_batch_get_reviews(&self, request: BatchGetReviewsRequest, name: &str) -> AccountLocationBatchGetReviewCall<'a, S> {
        AccountLocationBatchGetReviewCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears an association between a location and its place ID. This
    /// operation is only valid if the location is unverified.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the location to disassociate.
    pub fn locations_clear_association(&self, request: ClearLocationAssociationRequest, name: &str) -> AccountLocationClearAssociationCall<'a, S> {
        AccountLocationClearAssociationCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new location owned by the
    /// specified account, and returns it.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - The name of the account in which to create this location.
    pub fn locations_create(&self, request: Location, parent: &str) -> AccountLocationCreateCall<'a, S> {
        AccountLocationCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _validate_only: Default::default(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a location.
    /// 
    /// <aside class="note"><b>Note:</b> If this location cannot be deleted using
    /// the API as marked in the
    /// LocationState, use the [Google My
    /// Business](https://business.google.com/manage/) website.
    /// 
    /// Returns `NOT_FOUND` if the location does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the location to delete.
    pub fn locations_delete(&self, name: &str) -> AccountLocationDeleteCall<'a, S> {
        AccountLocationDeleteCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reports all eligible verification options for a location in a specific
    /// language.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the location to verify.
    pub fn locations_fetch_verification_options(&self, request: FetchVerificationOptionsRequest, name: &str) -> AccountLocationFetchVerificationOptionCall<'a, S> {
        AccountLocationFetchVerificationOptionCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Finds all of the possible locations that are a match to the specified
    /// location. This operation is only valid if the location is unverified.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The resource name of the location to find matches for.
    pub fn locations_find_matches(&self, request: FindMatchingLocationsRequest, name: &str) -> AccountLocationFindMatchCall<'a, S> {
        AccountLocationFindMatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified location. Returns `NOT_FOUND` if the
    /// location does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the location to fetch.
    pub fn locations_get(&self, name: &str) -> AccountLocationGetCall<'a, S> {
        AccountLocationGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the Google-updated version of the specified location.
    /// Returns `NOT_FOUND` if the location does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the location to fetch.
    pub fn locations_get_google_updated(&self, name: &str) -> AccountLocationGetGoogleUpdatedCall<'a, S> {
        AccountLocationGetGoogleUpdatedCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the locations for the specified account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - The name of the account to fetch locations from. If the Account is of AccountType PERSONAL, only Locations that are
    ///              directly owned by the Account are returned, otherwise it will return all
    ///              accessible locations from the Account, either directly or indirectly.
    pub fn locations_list(&self, parent: &str) -> AccountLocationListCall<'a, S> {
        AccountLocationListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _language_code: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified location.
    /// 
    /// Photos are only allowed on a location that has a Google+ page.
    /// 
    /// Returns `NOT_FOUND` if the location does not exist.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the location to update.
    pub fn locations_patch(&self, request: Location, name: &str) -> AccountLocationPatchCall<'a, S> {
        AccountLocationPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _update_mask: Default::default(),
            _attribute_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a report containing insights on one or more metrics by location.
    /// 
    /// <aside class="note"><b>Note:</b> Insight reports are limited
    /// to a batch size of 10 `location_names` per call.</aside>
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The account resource name.
    pub fn locations_report_insights(&self, request: ReportLocationInsightsRequest, name: &str) -> AccountLocationReportInsightCall<'a, S> {
        AccountLocationReportInsightCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Moves a location from an account that the user owns to another account
    /// that the same user administers. The user must be an owner of the account
    /// the location is currently associated with and must also be at least a
    /// manager of the destination account. Returns the Location with its new
    /// resource name.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the location to transfer.
    pub fn locations_transfer(&self, request: TransferLocationRequest, name: &str) -> AccountLocationTransferCall<'a, S> {
        AccountLocationTransferCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Starts the verification process for a location.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of the location to verify.
    pub fn locations_verify(&self, request: VerifyLocationRequest, name: &str) -> AccountLocationVerifyCall<'a, S> {
        AccountLocationVerifyCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an account with the specified name and type under the given parent.
    /// <ul>
    /// <li> Personal accounts and Organizations cannot be created. </li>
    /// <li> User Groups cannot be created with a Personal account as primary
    /// owner. </li>
    /// <li> Location Groups cannot be created with a primary owner of a
    /// Personal account if the Personal account is in an Organization. </li>
    /// <li> Location Groups cannot own Location Groups. </li>
    /// </ul>
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: Account) -> AccountCreateCall<'a, S> {
        AccountCreateCall {
            hub: self.hub,
            _request: request,
            _primary_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Clears the pubsub notification settings for the account.
    /// 
    /// # Arguments
    ///
    /// * `name` - The resource name for the notification settings to be cleared.
    pub fn delete_notifications(&self, name: &str) -> AccountDeleteNotificationCall<'a, S> {
        AccountDeleteNotificationCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates an account number for this account. The account number is not
    /// provisioned when an account is created. Use this request to create an
    /// account number when it is required.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the account to generate an account number for.
    pub fn generate_account_number(&self, request: GenerateAccountNumberRequest, name: &str) -> AccountGenerateAccountNumberCall<'a, S> {
        AccountGenerateAccountNumberCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified account. Returns `NOT_FOUND` if the
    /// account does not exist or if the caller does not have access rights to it.
    /// 
    /// # Arguments
    ///
    /// * `name` - The name of the account to fetch.
    pub fn get(&self, name: &str) -> AccountGetCall<'a, S> {
        AccountGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the pubsub notification settings for the account.
    /// 
    /// # Arguments
    ///
    /// * `name` - The notification settings resource name.
    pub fn get_notifications(&self, name: &str) -> AccountGetNotificationCall<'a, S> {
        AccountGetNotificationCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all of the accounts for the authenticated user. This includes all
    /// accounts that the user owns, as well as any accounts for which the user
    /// has management rights.
    pub fn list(&self) -> AccountListCall<'a, S> {
        AccountListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all the GoogleLocations that
    /// have been recommended to the specified GMB account.
    /// Recommendations are provided for personal accounts and location groups
    /// only, requests for all other account types will result in an error.
    /// The recommendations for location groups are based on the locations in that
    /// group.
    /// 
    /// The recommendations for personal accounts are based on all of
    /// the locations that the user has access to on Google My Business (which
    /// includes locations they can access through location groups), and is a
    /// superset of all recommendations generated for the user.
    /// 
    /// # Arguments
    ///
    /// * `name` - Name of the account resource to fetch recommended Google locations for.
    pub fn list_recommend_google_locations(&self, name: &str) -> AccountListRecommendGoogleLocationCall<'a, S> {
        AccountListRecommendGoogleLocationCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the specified business account. Personal accounts cannot be
    /// updated using this method.
    /// <aside class="note"><b>Note:</b> The only editable field for an account is
    /// `account_name`.
    /// Any other fields passed in (such as `type` or `role`) are
    /// ignored.</aside>
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The name of the account to update.
    pub fn update(&self, request: Account, name: &str) -> AccountUpdateCall<'a, S> {
        AccountUpdateCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _validate_only: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the pubsub notification settings for the account informing My Business
    /// which topic to send pubsub notifications for:
    /// 
    /// - New reviews for locations administered by the account.
    /// - Updated reviews for locations administered by the account.
    /// - New `GoogleUpdates` for locations administered by the account.
    /// 
    /// An account will only have one notification settings resource, and only one
    /// pubsub topic can be set.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - The notification settings resource name.
    pub fn update_notifications(&self, request: Notifications, name: &str) -> AccountUpdateNotificationCall<'a, S> {
        AccountUpdateNotificationCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *attribute* resources.
/// It is not used directly, but through the [`MyBusiness`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusiness4 as mybusiness4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.attributes();
/// # }
/// ```
pub struct AttributeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusiness<S>,
}

impl<'a, S> client::MethodsBuilder for AttributeMethods<'a, S> {}

impl<'a, S> AttributeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the list of available attributes that would be available for a
    /// location with the given primary category and country.
    pub fn list(&self) -> AttributeListCall<'a, S> {
        AttributeListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _name: Default::default(),
            _language_code: Default::default(),
            _country: Default::default(),
            _category_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *category* resources.
/// It is not used directly, but through the [`MyBusiness`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusiness4 as mybusiness4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.categories();
/// # }
/// ```
pub struct CategoryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusiness<S>,
}

impl<'a, S> client::MethodsBuilder for CategoryMethods<'a, S> {}

impl<'a, S> CategoryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns a list of business categories. Search will match the category name
    /// but not the category ID.
    /// 
    /// <aside class="note"><b>Note:</b> Search only matches the front of
    /// a category name (that is, 'food' may return 'Food Court' but not 'Fast Food
    /// Restaurant').</aside>
    pub fn list(&self) -> CategoryListCall<'a, S> {
        CategoryListCall {
            hub: self.hub,
            _search_term: Default::default(),
            _region_code: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _language_code: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *chain* resources.
/// It is not used directly, but through the [`MyBusiness`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusiness4 as mybusiness4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.chains();
/// # }
/// ```
pub struct ChainMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusiness<S>,
}

impl<'a, S> client::MethodsBuilder for ChainMethods<'a, S> {}

impl<'a, S> ChainMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the specified chain. Returns `NOT_FOUND` if the
    /// chain does not exist.
    /// 
    /// # Arguments
    ///
    /// * `name` - The chain's resource name, in the format `chains/{chain_place_id}`.
    pub fn get(&self, name: &str) -> ChainGetCall<'a, S> {
        ChainGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches the chain based on chain name.
    pub fn search(&self) -> ChainSearchCall<'a, S> {
        ChainSearchCall {
            hub: self.hub,
            _result_count: Default::default(),
            _chain_display_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *googleLocation* resources.
/// It is not used directly, but through the [`MyBusiness`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusiness4 as mybusiness4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `report(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.google_locations();
/// # }
/// ```
pub struct GoogleLocationMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusiness<S>,
}

impl<'a, S> client::MethodsBuilder for GoogleLocationMethods<'a, S> {}

impl<'a, S> GoogleLocationMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Report a GoogleLocation.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Resource name of a [GoogleLocation], in the format
    ///            `googleLocations/{googleLocationId}`.
    pub fn report(&self, request: ReportGoogleLocationRequest, name: &str) -> GoogleLocationReportCall<'a, S> {
        GoogleLocationReportCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Search all of the possible locations that are a match to the specified
    /// request.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: SearchGoogleLocationsRequest) -> GoogleLocationSearchCall<'a, S> {
        GoogleLocationSearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *verificationToken* resources.
/// It is not used directly, but through the [`MyBusiness`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusiness4 as mybusiness4;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use mybusiness4::{MyBusiness, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = MyBusiness::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `generate(...)`
/// // to build up your call.
/// let rb = hub.verification_tokens();
/// # }
/// ```
pub struct VerificationTokenMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a MyBusiness<S>,
}

impl<'a, S> client::MethodsBuilder for VerificationTokenMethods<'a, S> {}

impl<'a, S> VerificationTokenMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generates a token for the provided location data as a vetted partner.
    /// 
    /// Throws PERMISSION_DENIED if the caller is not a vetted partner account.
    /// Throws FAILED_PRECONDITION if the caller's VettedStatus is INVALID.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn generate(&self, request: GenerateVerificationTokenRequest) -> VerificationTokenGenerateCall<'a, S> {
        VerificationTokenGenerateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



