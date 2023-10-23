use super::*;
/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get blogs](BlogGetCall) (response)
/// * [get by url blogs](BlogGetByUrlCall) (response)
/// * [list by user blogs](BlogListByUserCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Blog {
    /// The JSON custom meta-data for the Blog.
    #[serde(rename="customMetaData")]
    
    pub custom_meta_data: Option<String>,
    /// The description of this blog. This is displayed underneath the title.
    
    pub description: Option<String>,
    /// The identifier for this resource.
    
    pub id: Option<String>,
    /// The kind of this entry. Always blogger#blog.
    
    pub kind: Option<String>,
    /// The locale this Blog is set to.
    
    pub locale: Option<BlogLocale>,
    /// The name of this blog. This is displayed as the title.
    
    pub name: Option<String>,
    /// The container of pages in this blog.
    
    pub pages: Option<BlogPages>,
    /// The container of posts in this blog.
    
    pub posts: Option<BlogPosts>,
    /// RFC 3339 date-time when this blog was published.
    
    pub published: Option<String>,
    /// The API REST URL to fetch this resource from.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The status of the blog.
    
    pub status: Option<BlogStatusEnum>,
    /// RFC 3339 date-time when this blog was last updated.
    
    pub updated: Option<String>,
    /// The URL where this blog is published.
    
    pub url: Option<String>,
}

impl client::Resource for Blog {}
impl client::ResponseResult for Blog {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list by user blogs](BlogListByUserCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlogList {
    /// Admin level list of blog per-user information.
    #[serde(rename="blogUserInfos")]
    
    pub blog_user_infos: Option<Vec<BlogUserInfo>>,
    /// The list of Blogs this user has Authorship or Admin rights over.
    
    pub items: Option<Vec<Blog>>,
    /// The kind of this entity. Always blogger#blogList.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for BlogList {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlogPerUserInfo {
    /// ID of the Blog resource.
    #[serde(rename="blogId")]
    
    pub blog_id: Option<String>,
    /// True if the user has Admin level access to the blog.
    #[serde(rename="hasAdminAccess")]
    
    pub has_admin_access: Option<bool>,
    /// The kind of this entity. Always blogger#blogPerUserInfo.
    
    pub kind: Option<String>,
    /// The Photo Album Key for the user when adding photos to the blog.
    #[serde(rename="photosAlbumKey")]
    
    pub photos_album_key: Option<String>,
    /// Access permissions that the user has for the blog (ADMIN, AUTHOR, or READER).
    
    pub role: Option<BlogPerUserInfoRoleEnum>,
    /// ID of the User.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for BlogPerUserInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get blog user infos](BlogUserInfoGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlogUserInfo {
    /// The Blog resource.
    
    pub blog: Option<Blog>,
    /// Information about a User for the Blog.
    
    pub blog_user_info: Option<BlogPerUserInfo>,
    /// The kind of this entity. Always blogger#blogUserInfo.
    
    pub kind: Option<String>,
}

impl client::Resource for BlogUserInfo {}
impl client::ResponseResult for BlogUserInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [approve comments](CommentApproveCall) (response)
/// * [delete comments](CommentDeleteCall) (none)
/// * [get comments](CommentGetCall) (response)
/// * [list comments](CommentListCall) (none)
/// * [list by blog comments](CommentListByBlogCall) (none)
/// * [mark as spam comments](CommentMarkAsSpamCall) (response)
/// * [remove content comments](CommentRemoveContentCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    /// The author of this Comment.
    
    pub author: Option<CommentAuthor>,
    /// Data about the blog containing this comment.
    
    pub blog: Option<CommentBlog>,
    /// The actual content of the comment. May include HTML markup.
    
    pub content: Option<String>,
    /// The identifier for this resource.
    
    pub id: Option<String>,
    /// Data about the comment this is in reply to.
    #[serde(rename="inReplyTo")]
    
    pub in_reply_to: Option<CommentInReplyTo>,
    /// The kind of this entry. Always blogger#comment.
    
    pub kind: Option<String>,
    /// Data about the post containing this comment.
    
    pub post: Option<CommentPost>,
    /// RFC 3339 date-time when this comment was published.
    
    pub published: Option<String>,
    /// The API REST URL to fetch this resource from.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The status of the comment (only populated for admin users).
    
    pub status: Option<CommentStatusEnum>,
    /// RFC 3339 date-time when this comment was last updated.
    
    pub updated: Option<String>,
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
/// * [list by blog comments](CommentListByBlogCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentList {
    /// Etag of the response.
    
    pub etag: Option<String>,
    /// The List of Comments for a Post.
    
    pub items: Option<Vec<Comment>>,
    /// The kind of this entry. Always blogger#commentList.
    
    pub kind: Option<String>,
    /// Pagination token to fetch the next page, if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Pagination token to fetch the previous page, if one exists.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
}

impl client::ResponseResult for CommentList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete pages](PageDeleteCall) (none)
/// * [get pages](PageGetCall) (response)
/// * [insert pages](PageInsertCall) (request|response)
/// * [list pages](PageListCall) (none)
/// * [patch pages](PagePatchCall) (request|response)
/// * [publish pages](PagePublishCall) (response)
/// * [revert pages](PageRevertCall) (response)
/// * [update pages](PageUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    /// The author of this Page.
    
    pub author: Option<PageAuthor>,
    /// Data about the blog containing this Page.
    
    pub blog: Option<PageBlog>,
    /// The body content of this Page, in HTML.
    
    pub content: Option<String>,
    /// Etag of the resource.
    
    pub etag: Option<String>,
    /// The identifier for this resource.
    
    pub id: Option<String>,
    /// The kind of this entity. Always blogger#page.
    
    pub kind: Option<String>,
    /// RFC 3339 date-time when this Page was published.
    
    pub published: Option<String>,
    /// The API REST URL to fetch this resource from.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The status of the page for admin resources (either LIVE or DRAFT).
    
    pub status: Option<PageStatusEnum>,
    /// The title of this entity. This is the name displayed in the Admin user interface.
    
    pub title: Option<String>,
    /// RFC 3339 date-time when this Page was trashed.
    
    pub trashed: Option<String>,
    /// RFC 3339 date-time when this Page was last updated.
    
    pub updated: Option<String>,
    /// The URL that this Page is displayed at.
    
    pub url: Option<String>,
}

impl client::RequestValue for Page {}
impl client::Resource for Page {}
impl client::ResponseResult for Page {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list pages](PageListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageList {
    /// Etag of the response.
    
    pub etag: Option<String>,
    /// The list of Pages for a Blog.
    
    pub items: Option<Vec<Page>>,
    /// The kind of this entity. Always blogger#pageList.
    
    pub kind: Option<String>,
    /// Pagination token to fetch the next page, if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for PageList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get page views](PageViewGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Pageviews {
    /// Blog Id.
    #[serde(rename="blogId")]
    
    pub blog_id: Option<String>,
    /// The container of posts in this blog.
    
    pub counts: Option<Vec<PageviewsCounts>>,
    /// The kind of this entry. Always blogger#page_views.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Pageviews {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete posts](PostDeleteCall) (none)
/// * [get posts](PostGetCall) (response)
/// * [get by path posts](PostGetByPathCall) (response)
/// * [insert posts](PostInsertCall) (request|response)
/// * [list posts](PostListCall) (none)
/// * [patch posts](PostPatchCall) (request|response)
/// * [publish posts](PostPublishCall) (response)
/// * [revert posts](PostRevertCall) (response)
/// * [search posts](PostSearchCall) (none)
/// * [update posts](PostUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    /// The author of this Post.
    
    pub author: Option<PostAuthor>,
    /// Data about the blog containing this Post.
    
    pub blog: Option<PostBlog>,
    /// The content of the Post. May contain HTML markup.
    
    pub content: Option<String>,
    /// The JSON meta-data for the Post.
    #[serde(rename="customMetaData")]
    
    pub custom_meta_data: Option<String>,
    /// Etag of the resource.
    
    pub etag: Option<String>,
    /// The identifier of this Post.
    
    pub id: Option<String>,
    /// Display image for the Post.
    
    pub images: Option<Vec<PostImages>>,
    /// The kind of this entity. Always blogger#post.
    
    pub kind: Option<String>,
    /// The list of labels this Post was tagged with.
    
    pub labels: Option<Vec<String>>,
    /// The location for geotagged posts.
    
    pub location: Option<PostLocation>,
    /// RFC 3339 date-time when this Post was published.
    
    pub published: Option<String>,
    /// Comment control and display setting for readers of this post.
    #[serde(rename="readerComments")]
    
    pub reader_comments: Option<PostReaderCommentsEnum>,
    /// The container of comments on this Post.
    
    pub replies: Option<PostReplies>,
    /// The API REST URL to fetch this resource from.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Status of the post. Only set for admin-level requests.
    
    pub status: Option<PostStatusEnum>,
    /// The title of the Post.
    
    pub title: Option<String>,
    /// The title link URL, similar to atom's related link.
    #[serde(rename="titleLink")]
    
    pub title_link: Option<String>,
    /// RFC 3339 date-time when this Post was last trashed.
    
    pub trashed: Option<String>,
    /// RFC 3339 date-time when this Post was last updated.
    
    pub updated: Option<String>,
    /// The URL where this Post is displayed.
    
    pub url: Option<String>,
}

impl client::RequestValue for Post {}
impl client::Resource for Post {}
impl client::ResponseResult for Post {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list posts](PostListCall) (response)
/// * [search posts](PostSearchCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostList {
    /// Etag of the response.
    
    pub etag: Option<String>,
    /// The list of Posts for this Blog.
    
    pub items: Option<Vec<Post>>,
    /// The kind of this entity. Always blogger#postList.
    
    pub kind: Option<String>,
    /// Pagination token to fetch the next page, if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// Pagination token to fetch the previous page, if one exists.
    #[serde(rename="prevPageToken")]
    
    pub prev_page_token: Option<String>,
}

impl client::ResponseResult for PostList {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostPerUserInfo {
    /// ID of the Blog that the post resource belongs to.
    #[serde(rename="blogId")]
    
    pub blog_id: Option<String>,
    /// True if the user has Author level access to the post.
    #[serde(rename="hasEditAccess")]
    
    pub has_edit_access: Option<bool>,
    /// The kind of this entity. Always blogger#postPerUserInfo.
    
    pub kind: Option<String>,
    /// ID of the Post resource.
    #[serde(rename="postId")]
    
    pub post_id: Option<String>,
    /// ID of the User.
    #[serde(rename="userId")]
    
    pub user_id: Option<String>,
}

impl client::Part for PostPerUserInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get post user infos](PostUserInfoGetCall) (response)
/// * [list post user infos](PostUserInfoListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostUserInfo {
    /// The kind of this entity. Always blogger#postUserInfo.
    
    pub kind: Option<String>,
    /// The Post resource.
    
    pub post: Option<Post>,
    /// Information about a User for the Post.
    
    pub post_user_info: Option<PostPerUserInfo>,
}

impl client::Resource for PostUserInfo {}
impl client::ResponseResult for PostUserInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list post user infos](PostUserInfoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostUserInfosList {
    /// The list of Posts with User information for the post, for this Blog.
    
    pub items: Option<Vec<PostUserInfo>>,
    /// The kind of this entity. Always blogger#postList.
    
    pub kind: Option<String>,
    /// Pagination token to fetch the next page, if one exists.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for PostUserInfosList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get users](UserGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Profile summary information.
    
    pub about: Option<String>,
    /// The container of blogs for this user.
    
    pub blogs: Option<UserBlogs>,
    /// The timestamp of when this profile was created, in seconds since epoch.
    
    pub created: Option<String>,
    /// The display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The identifier for this User.
    
    pub id: Option<String>,
    /// The kind of this entity. Always blogger#user.
    
    pub kind: Option<String>,
    /// This user's locale
    
    pub locale: Option<UserLocale>,
    /// The API REST URL to fetch this resource from.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The user's profile page.
    
    pub url: Option<String>,
}

impl client::Resource for User {}
impl client::ResponseResult for User {}


/// The locale this Blog is set to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlogLocale {
    /// The country this blog's locale is set to.
    
    pub country: Option<String>,
    /// The language this blog is authored in.
    
    pub language: Option<String>,
    /// The language variant this blog is authored in.
    
    pub variant: Option<String>,
}

impl client::NestedType for BlogLocale {}
impl client::Part for BlogLocale {}


/// The container of pages in this blog.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlogPages {
    /// The URL of the container for pages in this blog.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The count of pages in this blog.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::NestedType for BlogPages {}
impl client::Part for BlogPages {}


/// The container of posts in this blog.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BlogPosts {
    /// The List of Posts for this Blog.
    
    pub items: Option<Vec<Post>>,
    /// The URL of the container for posts in this blog.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The count of posts in this blog.
    #[serde(rename="totalItems")]
    
    pub total_items: Option<i32>,
}

impl client::NestedType for BlogPosts {}
impl client::Part for BlogPosts {}


/// The author of this Comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentAuthor {
    /// The display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The identifier of the creator.
    
    pub id: Option<String>,
    /// The creator's avatar.
    
    pub image: Option<CommentAuthorImage>,
    /// The URL of the creator's Profile page.
    
    pub url: Option<String>,
}

impl client::NestedType for CommentAuthor {}
impl client::Part for CommentAuthor {}


/// The creator's avatar.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentAuthorImage {
    /// The creator's avatar URL.
    
    pub url: Option<String>,
}

impl client::NestedType for CommentAuthorImage {}
impl client::Part for CommentAuthorImage {}


/// Data about the blog containing this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentBlog {
    /// The identifier of the blog containing this comment.
    
    pub id: Option<String>,
}

impl client::NestedType for CommentBlog {}
impl client::Part for CommentBlog {}


/// Data about the comment this is in reply to.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentInReplyTo {
    /// The identified of the parent of this comment.
    
    pub id: Option<String>,
}

impl client::NestedType for CommentInReplyTo {}
impl client::Part for CommentInReplyTo {}


/// Data about the post containing this comment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommentPost {
    /// The identifier of the post containing this comment.
    
    pub id: Option<String>,
}

impl client::NestedType for CommentPost {}
impl client::Part for CommentPost {}


/// The author of this Page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageAuthor {
    /// The display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The identifier of the creator.
    
    pub id: Option<String>,
    /// The creator's avatar.
    
    pub image: Option<PageAuthorImage>,
    /// The URL of the creator's Profile page.
    
    pub url: Option<String>,
}

impl client::NestedType for PageAuthor {}
impl client::Part for PageAuthor {}


/// The creator's avatar.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageAuthorImage {
    /// The creator's avatar URL.
    
    pub url: Option<String>,
}

impl client::NestedType for PageAuthorImage {}
impl client::Part for PageAuthorImage {}


/// Data about the blog containing this Page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageBlog {
    /// The identifier of the blog containing this page.
    
    pub id: Option<String>,
}

impl client::NestedType for PageBlog {}
impl client::Part for PageBlog {}


/// The container of posts in this blog.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageviewsCounts {
    /// Count of page views for the given time range.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub count: Option<i64>,
    /// Time range the given count applies to.
    #[serde(rename="timeRange")]
    
    pub time_range: Option<PageviewsCountTimeRangeEnum>,
}

impl client::NestedType for PageviewsCounts {}
impl client::Part for PageviewsCounts {}


/// The author of this Post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostAuthor {
    /// The display name.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The identifier of the creator.
    
    pub id: Option<String>,
    /// The creator's avatar.
    
    pub image: Option<PostAuthorImage>,
    /// The URL of the creator's Profile page.
    
    pub url: Option<String>,
}

impl client::NestedType for PostAuthor {}
impl client::Part for PostAuthor {}


/// The creator's avatar.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostAuthorImage {
    /// The creator's avatar URL.
    
    pub url: Option<String>,
}

impl client::NestedType for PostAuthorImage {}
impl client::Part for PostAuthorImage {}


/// Data about the blog containing this Post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostBlog {
    /// The identifier of the Blog that contains this Post.
    
    pub id: Option<String>,
}

impl client::NestedType for PostBlog {}
impl client::Part for PostBlog {}


/// Display image for the Post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostImages {
    /// no description provided
    
    pub url: Option<String>,
}

impl client::NestedType for PostImages {}
impl client::Part for PostImages {}


/// The location for geotagged posts.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostLocation {
    /// Location's latitude.
    
    pub lat: Option<f64>,
    /// Location's longitude.
    
    pub lng: Option<f64>,
    /// Location name.
    
    pub name: Option<String>,
    /// Location's viewport span. Can be used when rendering a map preview.
    
    pub span: Option<String>,
}

impl client::NestedType for PostLocation {}
impl client::Part for PostLocation {}


/// The container of comments on this Post.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PostReplies {
    /// The List of Comments for this Post.
    
    pub items: Option<Vec<Comment>>,
    /// The URL of the comments on this post.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The count of comments on this post.
    #[serde(rename="totalItems")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_items: Option<i64>,
}

impl client::NestedType for PostReplies {}
impl client::Part for PostReplies {}


/// The container of blogs for this user.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserBlogs {
    /// The URL of the Blogs for this user.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::NestedType for UserBlogs {}
impl client::Part for UserBlogs {}


/// This user's locale
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserLocale {
    /// The country this blog's locale is set to.
    
    pub country: Option<String>,
    /// The language this blog is authored in.
    
    pub language: Option<String>,
    /// The language variant this blog is authored in.
    
    pub variant: Option<String>,
}

impl client::NestedType for UserLocale {}
impl client::Part for UserLocale {}


