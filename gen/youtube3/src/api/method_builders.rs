use super::*;
/// A builder providing access to all methods supported on *abuseReport* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.abuse_reports();
/// # }
/// ```
pub struct AbuseReportMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for AbuseReportMethods<'a, S> {}

impl<'a, S> AbuseReportMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: AbuseReport) -> AbuseReportInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        AbuseReportInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *activity* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.activities();
/// # }
/// ```
pub struct ActivityMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for ActivityMethods<'a, S> {}

impl<'a, S> ActivityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more activity resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in an activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set *part=snippet*, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &Vec<String>) -> ActivityListCall<'a, S> {
        ActivityListCall {
            hub: self.hub,
            _part: part.clone(),
            _region_code: Default::default(),
            _published_before: Default::default(),
            _published_after: Default::default(),
            _page_token: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _home: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *caption* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `download(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.captions();
/// # }
/// ```
pub struct CaptionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for CaptionMethods<'a, S> {}

impl<'a, S> CaptionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> CaptionDeleteCall<'a, S> {
        CaptionDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _on_behalf_of: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Downloads a caption track.
    /// 
    /// # Arguments
    ///
    /// * `id` - The ID of the caption track to download, required for One Platform.
    pub fn download(&self, id: &str) -> CaptionDownloadCall<'a, S> {
        CaptionDownloadCall {
            hub: self.hub,
            _id: id.to_string(),
            _tlang: Default::default(),
            _tfmt: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _on_behalf_of: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Caption) -> CaptionInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        CaptionInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _sync: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _on_behalf_of: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more caption resource parts that the API response will include. The part names that you can include in the parameter value are id and snippet.
    /// * `videoId` - Returns the captions for the specified video.
    pub fn list(&self, part: &Vec<String>, video_id: &str) -> CaptionListCall<'a, S> {
        CaptionListCall {
            hub: self.hub,
            _part: part.clone(),
            _video_id: video_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _on_behalf_of: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: Caption) -> CaptionUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        CaptionUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _sync: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _on_behalf_of: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channelBanner* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.channel_banners();
/// # }
/// ```
pub struct ChannelBannerMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for ChannelBannerMethods<'a, S> {}

impl<'a, S> ChannelBannerMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: ChannelBannerResource) -> ChannelBannerInsertCall<'a, S> {
        ChannelBannerInsertCall {
            hub: self.hub,
            _request: request,
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channelSection* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.channel_sections();
/// # }
/// ```
pub struct ChannelSectionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for ChannelSectionMethods<'a, S> {}

impl<'a, S> ChannelSectionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> ChannelSectionDeleteCall<'a, S> {
        ChannelSectionDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: ChannelSection) -> ChannelSectionInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        ChannelSectionInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more channelSection resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channelSection resource, the snippet property contains other properties, such as a display title for the channelSection. If you set *part=snippet*, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &Vec<String>) -> ChannelSectionListCall<'a, S> {
        ChannelSectionListCall {
            hub: self.hub,
            _part: part.clone(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: ChannelSection) -> ChannelSectionUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        ChannelSectionUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for ChannelMethods<'a, S> {}

impl<'a, S> ChannelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more channel resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channel resource, the contentDetails property contains other properties, such as the uploads properties. As such, if you set *part=contentDetails*, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &Vec<String>) -> ChannelListCall<'a, S> {
        ChannelListCall {
            hub: self.hub,
            _part: part.clone(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _my_subscribers: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _managed_by_me: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _for_username: Default::default(),
            _category_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: Channel) -> ChannelUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        ChannelUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *commentThread* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.comment_threads();
/// # }
/// ```
pub struct CommentThreadMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for CommentThreadMethods<'a, S> {}

impl<'a, S> CommentThreadMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: CommentThread) -> CommentThreadInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        CommentThreadInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more commentThread resource properties that the API response will include.
    pub fn list(&self, part: &Vec<String>) -> CommentThreadListCall<'a, S> {
        CommentThreadListCall {
            hub: self.hub,
            _part: part.clone(),
            _video_id: Default::default(),
            _text_format: Default::default(),
            _search_terms: Default::default(),
            _page_token: Default::default(),
            _order: Default::default(),
            _moderation_status: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _channel_id: Default::default(),
            _all_threads_related_to_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)`, `mark_as_spam(...)`, `set_moderation_status(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for CommentMethods<'a, S> {}

impl<'a, S> CommentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> CommentDeleteCall<'a, S> {
        CommentDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Comment) -> CommentInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        CommentInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more comment resource properties that the API response will include.
    pub fn list(&self, part: &Vec<String>) -> CommentListCall<'a, S> {
        CommentListCall {
            hub: self.hub,
            _part: part.clone(),
            _text_format: Default::default(),
            _parent_id: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Expresses the caller's opinion that one or more comments should be flagged as spam.
    /// 
    /// # Arguments
    ///
    /// * `id` - Flags the comments with the given IDs as spam in the caller's opinion.
    pub fn mark_as_spam(&self, id: &Vec<String>) -> CommentMarkAsSpamCall<'a, S> {
        CommentMarkAsSpamCall {
            hub: self.hub,
            _id: id.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the moderation status of one or more comments.
    /// 
    /// # Arguments
    ///
    /// * `id` - Modifies the moderation status of the comments with the given IDs
    /// * `moderationStatus` - Specifies the requested moderation status. Note, comments can be in statuses, which are not available through this call. For example, this call does not allow to mark a comment as 'likely spam'. Valid values: MODERATION_STATUS_PUBLISHED, MODERATION_STATUS_HELD_FOR_REVIEW, MODERATION_STATUS_REJECTED.
    pub fn set_moderation_status(&self, id: &Vec<String>, moderation_status: &str) -> CommentSetModerationStatuCall<'a, S> {
        CommentSetModerationStatuCall {
            hub: self.hub,
            _id: id.clone(),
            _moderation_status: moderation_status.to_string(),
            _ban_author: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: Comment) -> CommentUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        CommentUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *i18nLanguage* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.i18n_languages();
/// # }
/// ```
pub struct I18nLanguageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for I18nLanguageMethods<'a, S> {}

impl<'a, S> I18nLanguageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the i18nLanguage resource properties that the API response will include. Set the parameter value to snippet.
    pub fn list(&self, part: &Vec<String>) -> I18nLanguageListCall<'a, S> {
        I18nLanguageListCall {
            hub: self.hub,
            _part: part.clone(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *i18nRegion* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.i18n_regions();
/// # }
/// ```
pub struct I18nRegionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for I18nRegionMethods<'a, S> {}

impl<'a, S> I18nRegionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the i18nRegion resource properties that the API response will include. Set the parameter value to snippet.
    pub fn list(&self, part: &Vec<String>) -> I18nRegionListCall<'a, S> {
        I18nRegionListCall {
            hub: self.hub,
            _part: part.clone(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liveBroadcast* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `bind(...)`, `delete(...)`, `insert(...)`, `insert_cuepoint(...)`, `list(...)`, `transition(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.live_broadcasts();
/// # }
/// ```
pub struct LiveBroadcastMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for LiveBroadcastMethods<'a, S> {}

impl<'a, S> LiveBroadcastMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Bind a broadcast to a stream.
    /// 
    /// # Arguments
    ///
    /// * `id` - Broadcast to bind to the stream
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn bind(&self, id: &str, part: &Vec<String>) -> LiveBroadcastBindCall<'a, S> {
        LiveBroadcastBindCall {
            hub: self.hub,
            _id: id.to_string(),
            _part: part.clone(),
            _stream_id: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Delete a given broadcast.
    /// 
    /// # Arguments
    ///
    /// * `id` - Broadcast to delete.
    pub fn delete(&self, id: &str) -> LiveBroadcastDeleteCall<'a, S> {
        LiveBroadcastDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new stream for the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: LiveBroadcast) -> LiveBroadcastInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        LiveBroadcastInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Insert cuepoints in a broadcast
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert_cuepoint(&self, request: Cuepoint) -> LiveBroadcastInsertCuepointCall<'a, S> {
        LiveBroadcastInsertCuepointCall {
            hub: self.hub,
            _request: request,
            _part: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the list of broadcasts associated with the given channel.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, status and statistics.
    pub fn list(&self, part: &Vec<String>) -> LiveBroadcastListCall<'a, S> {
        LiveBroadcastListCall {
            hub: self.hub,
            _part: part.clone(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _broadcast_type: Default::default(),
            _broadcast_status: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Transition a broadcast to a given status.
    /// 
    /// # Arguments
    ///
    /// * `broadcastStatus` - The status to which the broadcast is going to transition.
    /// * `id` - Broadcast to transition.
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status.
    pub fn transition(&self, broadcast_status: &str, id: &str, part: &Vec<String>) -> LiveBroadcastTransitionCall<'a, S> {
        LiveBroadcastTransitionCall {
            hub: self.hub,
            _broadcast_status: broadcast_status.to_string(),
            _id: id.to_string(),
            _part: part.clone(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing broadcast for the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: LiveBroadcast) -> LiveBroadcastUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        LiveBroadcastUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liveChatBan* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)` and `insert(...)`
/// // to build up your call.
/// let rb = hub.live_chat_bans();
/// # }
/// ```
pub struct LiveChatBanMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for LiveChatBanMethods<'a, S> {}

impl<'a, S> LiveChatBanMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a chat ban.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> LiveChatBanDeleteCall<'a, S> {
        LiveChatBanDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: LiveChatBan) -> LiveChatBanInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        LiveChatBanInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liveChatMessage* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.live_chat_messages();
/// # }
/// ```
pub struct LiveChatMessageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for LiveChatMessageMethods<'a, S> {}

impl<'a, S> LiveChatMessageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a chat message.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> LiveChatMessageDeleteCall<'a, S> {
        LiveChatMessageDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: LiveChatMessage) -> LiveChatMessageInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        LiveChatMessageInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `liveChatId` - The id of the live chat for which comments should be returned.
    /// * `part` - The *part* parameter specifies the liveChatComment resource parts that the API response will include. Supported values are id and snippet.
    pub fn list(&self, live_chat_id: &str, part: &Vec<String>) -> LiveChatMessageListCall<'a, S> {
        LiveChatMessageListCall {
            hub: self.hub,
            _live_chat_id: live_chat_id.to_string(),
            _part: part.clone(),
            _profile_image_size: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liveChatModerator* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.live_chat_moderators();
/// # }
/// ```
pub struct LiveChatModeratorMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for LiveChatModeratorMethods<'a, S> {}

impl<'a, S> LiveChatModeratorMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a chat moderator.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> LiveChatModeratorDeleteCall<'a, S> {
        LiveChatModeratorDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: LiveChatModerator) -> LiveChatModeratorInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        LiveChatModeratorInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `liveChatId` - The id of the live chat for which moderators should be returned.
    /// * `part` - The *part* parameter specifies the liveChatModerator resource parts that the API response will include. Supported values are id and snippet.
    pub fn list(&self, live_chat_id: &str, part: &Vec<String>) -> LiveChatModeratorListCall<'a, S> {
        LiveChatModeratorListCall {
            hub: self.hub,
            _live_chat_id: live_chat_id.to_string(),
            _part: part.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *liveStream* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.live_streams();
/// # }
/// ```
pub struct LiveStreamMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for LiveStreamMethods<'a, S> {}

impl<'a, S> LiveStreamMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an existing stream for the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> LiveStreamDeleteCall<'a, S> {
        LiveStreamDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new stream for the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: LiveStream) -> LiveStreamInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        LiveStreamInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieve the list of streams associated with the given channel. --
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more liveStream resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, cdn, and status.
    pub fn list(&self, part: &Vec<String>) -> LiveStreamListCall<'a, S> {
        LiveStreamListCall {
            hub: self.hub,
            _part: part.clone(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing stream for the authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: LiveStream) -> LiveStreamUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        LiveStreamUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *member* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.members();
/// # }
/// ```
pub struct MemberMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for MemberMethods<'a, S> {}

impl<'a, S> MemberMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of members that match the request criteria for a channel.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the member resource parts that the API response will include. Set the parameter value to snippet.
    pub fn list(&self, part: &Vec<String>) -> MemberListCall<'a, S> {
        MemberListCall {
            hub: self.hub,
            _part: part.clone(),
            _page_token: Default::default(),
            _mode: Default::default(),
            _max_results: Default::default(),
            _has_access_to_level: Default::default(),
            _filter_by_member_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *membershipsLevel* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.memberships_levels();
/// # }
/// ```
pub struct MembershipsLevelMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for MembershipsLevelMethods<'a, S> {}

impl<'a, S> MembershipsLevelMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of all pricing levels offered by a creator to the fans.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the membershipsLevel resource parts that the API response will include. Supported values are id and snippet.
    pub fn list(&self, part: &Vec<String>) -> MembershipsLevelListCall<'a, S> {
        MembershipsLevelListCall {
            hub: self.hub,
            _part: part.clone(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *playlistItem* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.playlist_items();
/// # }
/// ```
pub struct PlaylistItemMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for PlaylistItemMethods<'a, S> {}

impl<'a, S> PlaylistItemMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> PlaylistItemDeleteCall<'a, S> {
        PlaylistItemDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: PlaylistItem) -> PlaylistItemInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        PlaylistItemInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlistItem resource, the snippet property contains numerous fields, including the title, description, position, and resourceId properties. As such, if you set *part=snippet*, the API response will contain all of those properties.
    pub fn list(&self, part: &Vec<String>) -> PlaylistItemListCall<'a, S> {
        PlaylistItemListCall {
            hub: self.hub,
            _part: part.clone(),
            _video_id: Default::default(),
            _playlist_id: Default::default(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: PlaylistItem) -> PlaylistItemUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        PlaylistItemUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *playlist* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.playlists();
/// # }
/// ```
pub struct PlaylistMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for PlaylistMethods<'a, S> {}

impl<'a, S> PlaylistMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> PlaylistDeleteCall<'a, S> {
        PlaylistDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Playlist) -> PlaylistInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        PlaylistInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlist resource, the snippet property contains properties like author, title, description, tags, and timeCreated. As such, if you set *part=snippet*, the API response will contain all of those properties.
    pub fn list(&self, part: &Vec<String>) -> PlaylistListCall<'a, S> {
        PlaylistListCall {
            hub: self.hub,
            _part: part.clone(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: Playlist) -> PlaylistUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        PlaylistUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *search* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.search();
/// # }
/// ```
pub struct SearchMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for SearchMethods<'a, S> {}

impl<'a, S> SearchMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of search resources
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more search resource properties that the API response will include. Set the parameter value to snippet.
    pub fn list(&self, part: &Vec<String>) -> SearchListCall<'a, S> {
        SearchListCall {
            hub: self.hub,
            _part: part.clone(),
            _video_type: Default::default(),
            _video_syndicated: Default::default(),
            _video_license: Default::default(),
            _video_embeddable: Default::default(),
            _video_duration: Default::default(),
            _video_dimension: Default::default(),
            _video_definition: Default::default(),
            _video_category_id: Default::default(),
            _video_caption: Default::default(),
            _type_: Default::default(),
            _topic_id: Default::default(),
            _safe_search: Default::default(),
            _relevance_language: Default::default(),
            _related_to_video_id: Default::default(),
            _region_code: Default::default(),
            _q: Default::default(),
            _published_before: Default::default(),
            _published_after: Default::default(),
            _page_token: Default::default(),
            _order: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _max_results: Default::default(),
            _location_radius: Default::default(),
            _location: Default::default(),
            _for_mine: Default::default(),
            _for_developer: Default::default(),
            _for_content_owner: Default::default(),
            _event_type: Default::default(),
            _channel_type: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *subscription* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.subscriptions();
/// # }
/// ```
pub struct SubscriptionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for SubscriptionMethods<'a, S> {}

impl<'a, S> SubscriptionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> SubscriptionDeleteCall<'a, S> {
        SubscriptionDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Subscription) -> SubscriptionInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        SubscriptionInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a subscription resource, the snippet property contains other properties, such as a display title for the subscription. If you set *part=snippet*, the API response will also contain all of those nested properties.
    pub fn list(&self, part: &Vec<String>) -> SubscriptionListCall<'a, S> {
        SubscriptionListCall {
            hub: self.hub,
            _part: part.clone(),
            _page_token: Default::default(),
            _order: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _my_subscribers: Default::default(),
            _my_recent_subscribers: Default::default(),
            _mine: Default::default(),
            _max_results: Default::default(),
            _id: Default::default(),
            _for_channel_id: Default::default(),
            _channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *superChatEvent* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.super_chat_events();
/// # }
/// ```
pub struct SuperChatEventMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for SuperChatEventMethods<'a, S> {}

impl<'a, S> SuperChatEventMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the superChatEvent resource parts that the API response will include. This parameter is currently not supported.
    pub fn list(&self, part: &Vec<String>) -> SuperChatEventListCall<'a, S> {
        SuperChatEventListCall {
            hub: self.hub,
            _part: part.clone(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *test* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `insert(...)`
/// // to build up your call.
/// let rb = hub.tests();
/// # }
/// ```
pub struct TestMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for TestMethods<'a, S> {}

impl<'a, S> TestMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// POST method.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: TestItem) -> TestInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        TestInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _external_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *thirdPartyLink* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `insert(...)`, `list(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.third_party_links();
/// # }
/// ```
pub struct ThirdPartyLinkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for ThirdPartyLinkMethods<'a, S> {}

impl<'a, S> ThirdPartyLinkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `linkingToken` - Delete the partner links with the given linking token.
    /// * `type` - Type of the link to be deleted.
    pub fn delete(&self, linking_token: &str, type_: &str) -> ThirdPartyLinkDeleteCall<'a, S> {
        ThirdPartyLinkDeleteCall {
            hub: self.hub,
            _linking_token: linking_token.to_string(),
            _type_: type_.to_string(),
            _part: Default::default(),
            _external_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: ThirdPartyLink) -> ThirdPartyLinkInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        ThirdPartyLinkInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _external_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the thirdPartyLink resource parts that the API response will include. Supported values are linkingToken, status, and snippet.
    pub fn list(&self, part: &Vec<String>) -> ThirdPartyLinkListCall<'a, S> {
        ThirdPartyLinkListCall {
            hub: self.hub,
            _part: part.clone(),
            _type_: Default::default(),
            _linking_token: Default::default(),
            _external_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: ThirdPartyLink) -> ThirdPartyLinkUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        ThirdPartyLinkUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _external_channel_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *thumbnail* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `set(...)`
/// // to build up your call.
/// let rb = hub.thumbnails();
/// # }
/// ```
pub struct ThumbnailMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for ThumbnailMethods<'a, S> {}

impl<'a, S> ThumbnailMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// As this is not an insert in a strict sense (it supports uploading/setting of a thumbnail for multiple videos, which doesn't result in creation of a single resource), I use a custom verb here.
    /// 
    /// # Arguments
    ///
    /// * `videoId` - Returns the Thumbnail with the given video IDs for Stubby or Apiary.
    pub fn set(&self, video_id: &str) -> ThumbnailSetCall<'a, S> {
        ThumbnailSetCall {
            hub: self.hub,
            _video_id: video_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *videoAbuseReportReason* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.video_abuse_report_reasons();
/// # }
/// ```
pub struct VideoAbuseReportReasonMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for VideoAbuseReportReasonMethods<'a, S> {}

impl<'a, S> VideoAbuseReportReasonMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the videoCategory resource parts that the API response will include. Supported values are id and snippet.
    pub fn list(&self, part: &Vec<String>) -> VideoAbuseReportReasonListCall<'a, S> {
        VideoAbuseReportReasonListCall {
            hub: self.hub,
            _part: part.clone(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *videoCategory* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.video_categories();
/// # }
/// ```
pub struct VideoCategoryMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for VideoCategoryMethods<'a, S> {}

impl<'a, S> VideoCategoryMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies the videoCategory resource properties that the API response will include. Set the parameter value to snippet.
    pub fn list(&self, part: &Vec<String>) -> VideoCategoryListCall<'a, S> {
        VideoCategoryListCall {
            hub: self.hub,
            _part: part.clone(),
            _region_code: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *video* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get_rating(...)`, `insert(...)`, `list(...)`, `rate(...)`, `report_abuse(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.videos();
/// # }
/// ```
pub struct VideoMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for VideoMethods<'a, S> {}

impl<'a, S> VideoMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a resource.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn delete(&self, id: &str) -> VideoDeleteCall<'a, S> {
        VideoDeleteCall {
            hub: self.hub,
            _id: id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the ratings that the authorized user gave to a list of specified videos.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    pub fn get_rating(&self, id: &Vec<String>) -> VideoGetRatingCall<'a, S> {
        VideoGetRatingCall {
            hub: self.hub,
            _id: id.clone(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a new resource into this collection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn insert(&self, request: Video) -> VideoInsertCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        VideoInsertCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _stabilize: Default::default(),
            _on_behalf_of_content_owner_channel: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _notify_subscribers: Default::default(),
            _auto_levels: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of resources, possibly filtered.
    /// 
    /// # Arguments
    ///
    /// * `part` - The *part* parameter specifies a comma-separated list of one or more video resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set *part=snippet*, the API response will contain all of those properties.
    pub fn list(&self, part: &Vec<String>) -> VideoListCall<'a, S> {
        VideoListCall {
            hub: self.hub,
            _part: part.clone(),
            _video_category_id: Default::default(),
            _region_code: Default::default(),
            _page_token: Default::default(),
            _on_behalf_of_content_owner: Default::default(),
            _my_rating: Default::default(),
            _max_width: Default::default(),
            _max_results: Default::default(),
            _max_height: Default::default(),
            _locale: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _chart: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Adds a like or dislike rating to a video or removes a rating from a video.
    /// 
    /// # Arguments
    ///
    /// * `id` - No description provided.
    /// * `rating` - No description provided.
    pub fn rate(&self, id: &str, rating: &str) -> VideoRateCall<'a, S> {
        VideoRateCall {
            hub: self.hub,
            _id: id.to_string(),
            _rating: rating.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Report abuse for a video.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn report_abuse(&self, request: VideoAbuseReport) -> VideoReportAbuseCall<'a, S> {
        VideoReportAbuseCall {
            hub: self.hub,
            _request: request,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn update(&self, request: Video) -> VideoUpdateCall<'a, S> {
        use client::ToParts;
            let parts = vec![request.to_parts()];
        VideoUpdateCall {
            hub: self.hub,
            _request: request,
            _part: parts,
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *watermark* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `set(...)` and `unset(...)`
/// // to build up your call.
/// let rb = hub.watermarks();
/// # }
/// ```
pub struct WatermarkMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for WatermarkMethods<'a, S> {}

impl<'a, S> WatermarkMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Allows upload of watermark image and setting it for a channel.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `channelId` - No description provided.
    pub fn set(&self, request: InvideoBranding, channel_id: &str) -> WatermarkSetCall<'a, S> {
        WatermarkSetCall {
            hub: self.hub,
            _request: request,
            _channel_id: channel_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Allows removal of channel watermark.
    /// 
    /// # Arguments
    ///
    /// * `channelId` - No description provided.
    pub fn unset(&self, channel_id: &str) -> WatermarkUnsetCall<'a, S> {
        WatermarkUnsetCall {
            hub: self.hub,
            _channel_id: channel_id.to_string(),
            _on_behalf_of_content_owner: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *youtube* resources.
/// It is not used directly, but through the [`YouTube`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_youtube3 as youtube3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use youtube3::{YouTube, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `v3_update_comment_threads(...)`
/// // to build up your call.
/// let rb = hub.youtube();
/// # }
/// ```
pub struct YoutubeMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a YouTube<S>,
}

impl<'a, S> client::MethodsBuilder for YoutubeMethods<'a, S> {}

impl<'a, S> YoutubeMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an existing resource.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn v3_update_comment_threads(&self, request: CommentThread) -> YoutubeV3UpdateCommentThreadCall<'a, S> {
        YoutubeV3UpdateCommentThreadCall {
            hub: self.hub,
            _request: request,
            _part: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



