use super::*;
/// A builder providing access to all methods supported on *blogUserInfo* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.blog_user_infos();
/// # }
/// ```
pub struct BlogUserInfoMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for BlogUserInfoMethods<'a, S> {}

impl<'a, S> BlogUserInfoMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one blog and user info pair by blog id and user id.
    /// 
    /// # Arguments
    ///
    /// * `userId` - No description provided.
    /// * `blogId` - No description provided.
    pub fn get(&self, user_id: &str, blog_id: &str) -> BlogUserInfoGetCall<'a, S> {
        BlogUserInfoGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _blog_id: blog_id.to_string(),
            _max_posts: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *blog* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`, `get_by_url(...)` and `list_by_user(...)`
/// // to build up your call.
/// let rb = hub.blogs();
/// # }
/// ```
pub struct BlogMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for BlogMethods<'a, S> {}

impl<'a, S> BlogMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a blog by id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    pub fn get(&self, blog_id: &str) -> BlogGetCall<'a, S> {
        BlogGetCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _view: Default::default(),
            _max_posts: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a blog by url.
    /// 
    /// # Arguments
    ///
    /// * `url` - No description provided.
    pub fn get_by_url(&self, url: &str) -> BlogGetByUrlCall<'a, S> {
        BlogGetByUrlCall {
            hub: self.hub,
            _url: url.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists blogs by user.
    /// 
    /// # Arguments
    ///
    /// * `userId` - No description provided.
    pub fn list_by_user(&self, user_id: &str) -> BlogListByUserCall<'a, S> {
        BlogListByUserCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _view: Default::default(),
            _status: Default::default(),
            _role: Default::default(),
            _fetch_user_info: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *comment* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `approve(...)`, `delete(...)`, `get(...)`, `list(...)`, `list_by_blog(...)`, `mark_as_spam(...)` and `remove_content(...)`
/// // to build up your call.
/// let rb = hub.comments();
/// # }
/// ```
pub struct CommentMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for CommentMethods<'a, S> {}

impl<'a, S> CommentMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks a comment as not spam by blog id, post id and comment id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    /// * `commentId` - No description provided.
    pub fn approve(&self, blog_id: &str, post_id: &str, comment_id: &str) -> CommentApproveCall<'a, S> {
        CommentApproveCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a comment by blog id, post id and comment id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    /// * `commentId` - No description provided.
    pub fn delete(&self, blog_id: &str, post_id: &str, comment_id: &str) -> CommentDeleteCall<'a, S> {
        CommentDeleteCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a comment by id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    /// * `commentId` - No description provided.
    pub fn get(&self, blog_id: &str, post_id: &str, comment_id: &str) -> CommentGetCall<'a, S> {
        CommentGetCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _comment_id: comment_id.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists comments.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn list(&self, blog_id: &str, post_id: &str) -> CommentListCall<'a, S> {
        CommentListCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _view: Default::default(),
            _status: Default::default(),
            _start_date: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _fetch_bodies: Default::default(),
            _end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists comments by blog.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    pub fn list_by_blog(&self, blog_id: &str) -> CommentListByBlogCall<'a, S> {
        CommentListByBlogCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _status: Default::default(),
            _start_date: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _fetch_bodies: Default::default(),
            _end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Marks a comment as spam by blog id, post id and comment id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    /// * `commentId` - No description provided.
    pub fn mark_as_spam(&self, blog_id: &str, post_id: &str, comment_id: &str) -> CommentMarkAsSpamCall<'a, S> {
        CommentMarkAsSpamCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the content of a comment by blog id, post id and comment id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    /// * `commentId` - No description provided.
    pub fn remove_content(&self, blog_id: &str, post_id: &str, comment_id: &str) -> CommentRemoveContentCall<'a, S> {
        CommentRemoveContentCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _comment_id: comment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *pageView* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.page_views();
/// # }
/// ```
pub struct PageViewMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for PageViewMethods<'a, S> {}

impl<'a, S> PageViewMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets page views by blog id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    pub fn get(&self, blog_id: &str) -> PageViewGetCall<'a, S> {
        PageViewGetCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _range: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *page* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)`, `publish(...)`, `revert(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.pages();
/// # }
/// ```
pub struct PageMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for PageMethods<'a, S> {}

impl<'a, S> PageMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a page by blog id and page id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `pageId` - No description provided.
    pub fn delete(&self, blog_id: &str, page_id: &str) -> PageDeleteCall<'a, S> {
        PageDeleteCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _page_id: page_id.to_string(),
            _use_trash: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a page by blog id and page id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `pageId` - No description provided.
    pub fn get(&self, blog_id: &str, page_id: &str) -> PageGetCall<'a, S> {
        PageGetCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _page_id: page_id.to_string(),
            _view: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `blogId` - No description provided.
    pub fn insert(&self, request: Page, blog_id: &str) -> PageInsertCall<'a, S> {
        PageInsertCall {
            hub: self.hub,
            _request: request,
            _blog_id: blog_id.to_string(),
            _is_draft: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists pages.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    pub fn list(&self, blog_id: &str) -> PageListCall<'a, S> {
        PageListCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _view: Default::default(),
            _status: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _fetch_bodies: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a page.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `blogId` - No description provided.
    /// * `pageId` - No description provided.
    pub fn patch(&self, request: Page, blog_id: &str, page_id: &str) -> PagePatchCall<'a, S> {
        PagePatchCall {
            hub: self.hub,
            _request: request,
            _blog_id: blog_id.to_string(),
            _page_id: page_id.to_string(),
            _revert: Default::default(),
            _publish: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Publishes a page.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `pageId` - No description provided.
    pub fn publish(&self, blog_id: &str, page_id: &str) -> PagePublishCall<'a, S> {
        PagePublishCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _page_id: page_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts a published or scheduled page to draft state.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `pageId` - No description provided.
    pub fn revert(&self, blog_id: &str, page_id: &str) -> PageRevertCall<'a, S> {
        PageRevertCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _page_id: page_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a page by blog id and page id.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `blogId` - No description provided.
    /// * `pageId` - No description provided.
    pub fn update(&self, request: Page, blog_id: &str, page_id: &str) -> PageUpdateCall<'a, S> {
        PageUpdateCall {
            hub: self.hub,
            _request: request,
            _blog_id: blog_id.to_string(),
            _page_id: page_id.to_string(),
            _revert: Default::default(),
            _publish: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *postUserInfo* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.post_user_infos();
/// # }
/// ```
pub struct PostUserInfoMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for PostUserInfoMethods<'a, S> {}

impl<'a, S> PostUserInfoMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one post and user info pair, by post_id and user_id.
    /// 
    /// # Arguments
    ///
    /// * `userId` - No description provided.
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn get(&self, user_id: &str, blog_id: &str, post_id: &str) -> PostUserInfoGetCall<'a, S> {
        PostUserInfoGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _max_comments: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists post and user info pairs.
    /// 
    /// # Arguments
    ///
    /// * `userId` - No description provided.
    /// * `blogId` - No description provided.
    pub fn list(&self, user_id: &str, blog_id: &str) -> PostUserInfoListCall<'a, S> {
        PostUserInfoListCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _blog_id: blog_id.to_string(),
            _view: Default::default(),
            _status: Default::default(),
            _start_date: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _labels: Default::default(),
            _fetch_bodies: Default::default(),
            _end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *post* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_by_path(...)`, `insert(...)`, `list(...)`, `patch(...)`, `publish(...)`, `revert(...)`, `search(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.posts();
/// # }
/// ```
pub struct PostMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for PostMethods<'a, S> {}

impl<'a, S> PostMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a post by blog id and post id.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn delete(&self, blog_id: &str, post_id: &str) -> PostDeleteCall<'a, S> {
        PostDeleteCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _use_trash: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a post by blog id and post id
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn get(&self, blog_id: &str, post_id: &str) -> PostGetCall<'a, S> {
        PostGetCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _view: Default::default(),
            _max_comments: Default::default(),
            _fetch_images: Default::default(),
            _fetch_body: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets a post by path.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `path` - No description provided.
    pub fn get_by_path(&self, blog_id: &str, path: &str) -> PostGetByPathCall<'a, S> {
        PostGetByPathCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _path: path.to_string(),
            _view: Default::default(),
            _max_comments: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts a post.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `blogId` - No description provided.
    pub fn insert(&self, request: Post, blog_id: &str) -> PostInsertCall<'a, S> {
        PostInsertCall {
            hub: self.hub,
            _request: request,
            _blog_id: blog_id.to_string(),
            _is_draft: Default::default(),
            _fetch_images: Default::default(),
            _fetch_body: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists posts.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    pub fn list(&self, blog_id: &str) -> PostListCall<'a, S> {
        PostListCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _view: Default::default(),
            _status: Default::default(),
            _start_date: Default::default(),
            _sort_option: Default::default(),
            _page_token: Default::default(),
            _order_by: Default::default(),
            _max_results: Default::default(),
            _labels: Default::default(),
            _fetch_images: Default::default(),
            _fetch_bodies: Default::default(),
            _end_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a post.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn patch(&self, request: Post, blog_id: &str, post_id: &str) -> PostPatchCall<'a, S> {
        PostPatchCall {
            hub: self.hub,
            _request: request,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _revert: Default::default(),
            _publish: Default::default(),
            _max_comments: Default::default(),
            _fetch_images: Default::default(),
            _fetch_body: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Publishes a post.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn publish(&self, blog_id: &str, post_id: &str) -> PostPublishCall<'a, S> {
        PostPublishCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _publish_date: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Reverts a published or scheduled post to draft state.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn revert(&self, blog_id: &str, post_id: &str) -> PostRevertCall<'a, S> {
        PostRevertCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for posts matching given query terms in the specified blog.
    /// 
    /// # Arguments
    ///
    /// * `blogId` - No description provided.
    /// * `q` - No description provided.
    pub fn search(&self, blog_id: &str, q: &str) -> PostSearchCall<'a, S> {
        PostSearchCall {
            hub: self.hub,
            _blog_id: blog_id.to_string(),
            _q: q.to_string(),
            _order_by: Default::default(),
            _fetch_bodies: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a post by blog id and post id.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `blogId` - No description provided.
    /// * `postId` - No description provided.
    pub fn update(&self, request: Post, blog_id: &str, post_id: &str) -> PostUpdateCall<'a, S> {
        PostUpdateCall {
            hub: self.hub,
            _request: request,
            _blog_id: blog_id.to_string(),
            _post_id: post_id.to_string(),
            _revert: Default::default(),
            _publish: Default::default(),
            _max_comments: Default::default(),
            _fetch_images: Default::default(),
            _fetch_body: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the [`Blogger`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_blogger3 as blogger3;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use blogger3::{Blogger, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Blogger::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a Blogger<S>,
}

impl<'a, S> client::MethodsBuilder for UserMethods<'a, S> {}

impl<'a, S> UserMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets one user by user_id.
    /// 
    /// # Arguments
    ///
    /// * `userId` - No description provided.
    pub fn get(&self, user_id: &str) -> UserGetCall<'a, S> {
        UserGetCall {
            hub: self.hub,
            _user_id: user_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



