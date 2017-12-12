// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Poly Service* crate version *1.0.6+20171209*, where *20171209* is the exact revision of the *poly:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.6*.
//! 
//! Everything else about the *Poly Service* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/poly/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/poly1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.PolyService.html) ... 
//! 
//! * [assets](struct.Asset.html)
//!  * [*get*](struct.AssetGetCall.html) and [*list*](struct.AssetListCall.html)
//! * users
//!  * [*assets list*](struct.UserAssetListCall.html) and [*likedassets list*](struct.UserLikedassetListCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.PolyService.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.assets().get(...).doit()
//! let r = hub.assets().list(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-poly1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_poly1 as poly1;
//! use poly1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use poly1::PolyService;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.assets().list()
//!              .page_token("takimata")
//!              .page_size(-70)
//!              .order_by("amet.")
//!              .max_complexity("erat")
//!              .keywords("labore")
//!              .format("sea")
//!              .curated(false)
//!              .category("dolores")
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ##############
// UTILITIES ###
// ############




// ########
// HUB ###
// ######

/// Central instance to access all PolyService related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_poly1 as poly1;
/// use poly1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use poly1::PolyService;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.assets().list()
///              .page_token("gubergren")
///              .page_size(-95)
///              .order_by("aliquyam")
///              .max_complexity("ea")
///              .keywords("no")
///              .format("justo")
///              .curated(true)
///              .category("et")
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
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
pub struct PolyService<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for PolyService<C, A> {}

impl<'a, C, A> PolyService<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> PolyService<C, A> {
        PolyService {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.6".to_string(),
            _base_url: "https://poly.googleapis.com/".to_string(),
            _root_url: "https://poly.googleapis.com/".to_string(),
        }
    }

    pub fn assets(&'a self) -> AssetMethods<'a, C, A> {
        AssetMethods { hub: &self }
    }
    pub fn users(&'a self) -> UserMethods<'a, C, A> {
        UserMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.6`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://poly.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://poly.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The same asset can be represented in different formats, for example,
/// a [WaveFront .obj](//en.wikipedia.org/wiki/Wavefront_.obj_file) file with its
/// corresponding .mtl file or a [Khronos glTF](//www.khronos.org/gltf) file
/// with its corresponding .glb binary data. A format refers to a specific
/// representation of an asset and contains all information needed to
/// retrieve and describe this representation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Format {
    /// A short string that identifies the format type of this representation.
    /// Possible values are: `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`.
    #[serde(rename="formatType")]
    pub format_type: Option<String>,
    /// The root of the file hierarchy. This will always be populated.
    /// For some format_types - such as `TILT`, which are self-contained -
    /// this is all of the data.
    /// 
    /// Other types - such as `OBJ` - often reference other data elements.
    /// These are contained in the resources field.
    pub root: Option<File>,
    /// Complexity stats about this representation of the asset.
    #[serde(rename="formatComplexity")]
    pub format_complexity: Option<FormatComplexity>,
    /// A list of dependencies of the root element. May include, but is not
    /// limited to, materials, textures, and shader programs.
    pub resources: Option<Vec<File>>,
}

impl Part for Format {}


/// A response message from a request to list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [assets list users](struct.UserAssetListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListUserAssetsResponse {
    /// The continuation token for retrieving the next page. If empty,
    /// indicates that there are no more pages. To get the next page, submit the
    /// same request specifying this value as the
    /// page_token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The total number of assets in the list, without pagination.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// A list of UserAssets matching the request.
    #[serde(rename="userAssets")]
    pub user_assets: Option<Vec<UserAsset>>,
}

impl ResponseResult for ListUserAssetsResponse {}


/// A response message from a request to list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [likedassets list users](struct.UserLikedassetListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLikedAssetsResponse {
    /// The continuation token for retrieving the next page. If empty,
    /// indicates that there are no more pages. To get the next page, submit the
    /// same request specifying this value as the
    /// page_token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The total number of assets in the list, without pagination.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// A list of assets that match the criteria specified in the request.
    pub assets: Option<Vec<Asset>>,
}

impl ResponseResult for ListLikedAssetsResponse {}


/// Information on the complexity of this Format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FormatComplexity {
    /// The estimated number of triangles.
    #[serde(rename="triangleCount")]
    pub triangle_count: Option<i64>,
    /// A non-negative integer that represents the level of detail (LOD) of this
    /// format relative to other formats of the same asset with the same
    /// format_type.
    /// This hint allows you to sort formats from the most-detailed (0) to
    /// least-detailed (integers greater than 0).
    #[serde(rename="lodHint")]
    pub lod_hint: Option<i32>,
}

impl Part for FormatComplexity {}


/// Data about the user's asset.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserAsset {
    /// An Asset.
    pub asset: Option<Asset>,
}

impl Part for UserAsset {}


/// Hints for displaying the asset, based on information available when the asset
/// was uploaded.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PresentationParams {
    /// A rotation that should be applied to the object root to make it upright.
    /// More precisely, this quaternion transforms from "object space" (the space
    /// in which the object is defined) to "presentation space", a coordinate
    /// system where +Y is up, +X is right, -Z is forward. For example, if
    /// the object is the Eiffel Tower, in its local coordinate system the
    /// object might be laid out such that the base of the tower is on the
    /// YZ plane and the tip of the tower is towards positive X. In this case
    /// this quaternion would specify a rotation (of 90 degrees about the Z
    /// axis) such that in the presentation space the base of the tower is
    /// aligned with the XZ plane, and the tip of the tower lies towards +Y.
    /// 
    /// This rotation is unrelated to the object's pose in the web preview,
    /// which is just a camera position setting and is *not* reflected in this
    /// rotation.
    /// 
    /// Please note: this is applicable only to the gLTF.
    #[serde(rename="orientingRotation")]
    pub orienting_rotation: Option<Quaternion>,
    /// The materials' diffuse/albedo color. This does not apply to vertex colors
    /// or texture maps.
    #[serde(rename="colorSpace")]
    pub color_space: Option<String>,
}

impl Part for PresentationParams {}


/// A [Quaternion](//en.wikipedia.org/wiki/Quaternion). Please note: if in the
/// response you see "w: 1" and nothing else this is the default value of
/// [0, 0, 0, 1] where x,y, and z are 0.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Quaternion {
    /// The y component.
    pub y: Option<f64>,
    /// The x component.
    pub x: Option<f64>,
    /// The z component.
    pub z: Option<f64>,
    /// The scalar component.
    pub w: Option<f64>,
}

impl Part for Quaternion {}


/// Represents and describes an asset in the Poly library. An asset is a 3D model
/// or scene created using [Tilt Brush](//www.tiltbrush.com),
/// [Blocks](//vr.google.com/blocks/), or any 3D program that produces a file
/// that can be upload to Poly.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get assets](struct.AssetGetCall.html) (response)
/// * [list assets](struct.AssetListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    /// The time when the asset was last modified. For published assets, whose
    /// contents are immutable, the update time changes only when metadata
    /// properties, such as visibility, are updated.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// The human-readable name, set by the asset's author.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// The human-readable description, set by the asset's author.
    pub description: Option<String>,
    /// The license under which the author has made the asset available
    /// for use, if any.
    pub license: Option<String>,
    /// The visibility of the asset and who can access it.
    pub visibility: Option<String>,
    /// The thumbnail image for the asset.
    pub thumbnail: Option<File>,
    /// Hints for displaying the asset. Note that these parameters are not
    /// immutable; the author of an asset may change them post-publication.
    #[serde(rename="presentationParams")]
    pub presentation_params: Option<PresentationParams>,
    /// The author's publicly visible name. Use this name when giving credit to the
    /// author. For more information, see [Licensing](/poly/discover/licensing).
    #[serde(rename="authorName")]
    pub author_name: Option<String>,
    /// Whether this asset has been curated by the Poly team.
    #[serde(rename="isCurated")]
    pub is_curated: Option<bool>,
    /// A list of Formats where each
    /// format describes one representation of the asset.
    pub formats: Option<Vec<Format>>,
    /// For published assets, the time when the asset was published.
    /// For unpublished assets, the time when the asset was created.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// The unique identifier for the asset in the form:
    /// `assets/{ASSET_ID}`.
    pub name: Option<String>,
}

impl Resource for Asset {}
impl ResponseResult for Asset {}


/// Represents a file in Poly, which can be a root,
/// resource, or thumbnail file.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    /// The URL where the file data can be retrieved.
    pub url: Option<String>,
    /// The path of the resource file relative to the root file.
    /// For root or thumbnail files, this is just the filename.
    #[serde(rename="relativePath")]
    pub relative_path: Option<String>,
    /// The MIME content-type, such as `image/png`.
    /// For more information, see
    /// [MIME types](//developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).
    #[serde(rename="contentType")]
    pub content_type: Option<String>,
}

impl Part for File {}


/// A response message from a request to list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list assets](struct.AssetListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAssetsResponse {
    /// The continuation token for retrieving the next page. If empty,
    /// indicates that there are no more pages. To get the next page, submit the
    /// same request specifying this value as the
    /// page_token.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// The total number of assets in the list, without pagination.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
    /// A list of assets that match the criteria specified in the request.
    pub assets: Option<Vec<Asset>>,
}

impl ResponseResult for ListAssetsResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *asset* resources.
/// It is not used directly, but through the `PolyService` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_poly1 as poly1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use poly1::PolyService;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.assets();
/// # }
/// ```
pub struct AssetMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PolyService<C, A>,
}

impl<'a, C, A> MethodsBuilder for AssetMethods<'a, C, A> {}

impl<'a, C, A> AssetMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all public, remixable assets. These are assets with an access level of
    /// PUBLIC and published under the
    /// CC-By license.
    pub fn list(&self) -> AssetListCall<'a, C, A> {
        AssetListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _max_complexity: Default::default(),
            _keywords: Default::default(),
            _format: Default::default(),
            _curated: Default::default(),
            _category: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns detailed information about an asset given its name.
    /// PRIVATE assets are returned only if
    ///  the currently authenticated user (via OAuth token) is the author of the asset.
    /// 
    /// # Arguments
    ///
    /// * `name` - Required. An asset's name in the form `assets/{ASSET_ID}`.
    pub fn get(&self, name: &str) -> AssetGetCall<'a, C, A> {
        AssetGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *user* resources.
/// It is not used directly, but through the `PolyService` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_poly1 as poly1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use poly1::PolyService;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `assets_list(...)` and `likedassets_list(...)`
/// // to build up your call.
/// let rb = hub.users();
/// # }
/// ```
pub struct UserMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PolyService<C, A>,
}

impl<'a, C, A> MethodsBuilder for UserMethods<'a, C, A> {}

impl<'a, C, A> UserMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assets that the user has liked. Only the value 'me', representing
    /// the currently-authenticated user, is supported. May include assets with an
    /// access level of UNLISTED.
    /// 
    /// # Arguments
    ///
    /// * `name` - A valid user id. Currently, only the special value 'me', representing the
    ///            currently-authenticated user is supported. To use 'me', you must pass
    ///            an OAuth token with the request.
    pub fn likedassets_list(&self, name: &str) -> UserLikedassetListCall<'a, C, A> {
        UserLikedassetListCall {
            hub: self.hub,
            _name: name.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists assets authored by the given user. Only the value 'me', representing
    /// the currently-authenticated user, is supported. May include assets with an
    /// access level of PRIVATE or
    /// UNLISTED and assets which are
    /// All Rights Reserved for the
    /// currently-authenticated user.
    /// 
    /// # Arguments
    ///
    /// * `name` - A valid user id. Currently, only the special value 'me', representing the
    ///            currently-authenticated user is supported. To use 'me', you must pass
    ///            an OAuth token with the request.
    pub fn assets_list(&self, name: &str) -> UserAssetListCall<'a, C, A> {
        UserAssetListCall {
            hub: self.hub,
            _name: name.to_string(),
            _visibility: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _order_by: Default::default(),
            _format: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Lists all public, remixable assets. These are assets with an access level of
/// PUBLIC and published under the
/// CC-By license.
///
/// A builder for the *list* method supported by a *asset* resource.
/// It is not used directly, but through a `AssetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_poly1 as poly1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use poly1::PolyService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.assets().list()
///              .page_token("et")
///              .page_size(-41)
///              .order_by("ipsum")
///              .max_complexity("Lorem")
///              .keywords("et")
///              .format("duo")
///              .curated(true)
///              .category("sea")
///              .doit();
/// # }
/// ```
pub struct AssetListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PolyService<C, A>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _max_complexity: Option<String>,
    _keywords: Option<String>,
    _format: Option<String>,
    _curated: Option<bool>,
    _category: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for AssetListCall<'a, C, A> {}

impl<'a, C, A> AssetListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListAssetsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "poly.assets.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((10 + self._additional_params.len()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._max_complexity {
            params.push(("maxComplexity", value.to_string()));
        }
        if let Some(value) = self._keywords {
            params.push(("keywords", value.to_string()));
        }
        if let Some(value) = self._format {
            params.push(("format", value.to_string()));
        }
        if let Some(value) = self._curated {
            params.push(("curated", value.to_string()));
        }
        if let Some(value) = self._category {
            params.push(("category", value.to_string()));
        }
        for &field in ["alt", "pageToken", "pageSize", "orderBy", "maxComplexity", "keywords", "format", "curated", "category"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/assets";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Specifies a continuation token from a previous search whose results were
    /// split into multiple pages. To get the next page, submit the same request
    /// specifying the value from next_page_token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AssetListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of assets to be returned. This value must be between `1`
    /// and `100`. Defaults to `20`.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AssetListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Specifies an ordering for assets. Acceptable values are:
    /// `BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets
    /// based on a combination of popularity and other features.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> AssetListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Returns assets that are of the specified complexity or less. Defaults to
    /// COMPLEX. For example, a request for
    /// MEDIUM assets also includes
    /// SIMPLE assets.
    ///
    /// Sets the *max complexity* query property to the given value.
    pub fn max_complexity(mut self, new_value: &str) -> AssetListCall<'a, C, A> {
        self._max_complexity = Some(new_value.to_string());
        self
    }
    /// One or more search terms to be matched against all text that Poly has
    /// indexed for assets, which includes display_name,
    /// description, and tags. Multiple keywords should be
    /// separated by spaces.
    ///
    /// Sets the *keywords* query property to the given value.
    pub fn keywords(mut self, new_value: &str) -> AssetListCall<'a, C, A> {
        self._keywords = Some(new_value.to_string());
        self
    }
    /// Return only assets with the matching format. Acceptable values are:
    /// `BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`.
    ///
    /// Sets the *format* query property to the given value.
    pub fn format(mut self, new_value: &str) -> AssetListCall<'a, C, A> {
        self._format = Some(new_value.to_string());
        self
    }
    /// Return only assets that have been curated by the Poly team.
    ///
    /// Sets the *curated* query property to the given value.
    pub fn curated(mut self, new_value: bool) -> AssetListCall<'a, C, A> {
        self._curated = Some(new_value);
        self
    }
    /// Filter assets based on the specified category. Supported values are:
    /// `animals`, `architecture`, `art`, `food`, `nature`, `objects`, `people`, `scenes`,
    /// `technology`, and `transport`.
    ///
    /// Sets the *category* query property to the given value.
    pub fn category(mut self, new_value: &str) -> AssetListCall<'a, C, A> {
        self._category = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AssetListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AssetListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns detailed information about an asset given its name.
/// PRIVATE assets are returned only if
///  the currently authenticated user (via OAuth token) is the author of the asset.
///
/// A builder for the *get* method supported by a *asset* resource.
/// It is not used directly, but through a `AssetMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_poly1 as poly1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use poly1::PolyService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.assets().get("name")
///              .doit();
/// # }
/// ```
pub struct AssetGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PolyService<C, A>,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for AssetGetCall<'a, C, A> {}

impl<'a, C, A> AssetGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Asset)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "poly.assets.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Required. An asset's name in the form `assets/{ASSET_ID}`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AssetGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AssetGetCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> AssetGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists assets that the user has liked. Only the value 'me', representing
/// the currently-authenticated user, is supported. May include assets with an
/// access level of UNLISTED.
///
/// A builder for the *likedassets.list* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_poly1 as poly1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use poly1::PolyService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().likedassets_list("name")
///              .page_token("erat")
///              .page_size(-95)
///              .order_by("dolor")
///              .format("eirmod")
///              .doit();
/// # }
/// ```
pub struct UserLikedassetListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PolyService<C, A>,
    _name: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _format: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserLikedassetListCall<'a, C, A> {}

impl<'a, C, A> UserLikedassetListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListLikedAssetsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "poly.users.likedassets.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((7 + self._additional_params.len()));
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._format {
            params.push(("format", value.to_string()));
        }
        for &field in ["alt", "name", "pageToken", "pageSize", "orderBy", "format"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}/likedassets";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// A valid user id. Currently, only the special value 'me', representing the
    /// currently-authenticated user is supported. To use 'me', you must pass
    /// an OAuth token with the request.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> UserLikedassetListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// Specifies a continuation token from a previous search whose results were
    /// split into multiple pages. To get the next page, submit the same request
    /// specifying the value from
    /// next_page_token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserLikedassetListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of assets to be returned. This value must be between `1`
    /// and `100`. Defaults to `20`.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> UserLikedassetListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Specifies an ordering for assets. Acceptable values are:
    /// `BEST`, `NEWEST`, `OLDEST`, 'LIKED_TIME'. Defaults to `LIKED_TIME`, which
    /// ranks assets based on how recently they were liked.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> UserLikedassetListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Return only assets with the matching format. Acceptable values are:
    /// `BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`.
    ///
    /// Sets the *format* query property to the given value.
    pub fn format(mut self, new_value: &str) -> UserLikedassetListCall<'a, C, A> {
        self._format = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserLikedassetListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserLikedassetListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists assets authored by the given user. Only the value 'me', representing
/// the currently-authenticated user, is supported. May include assets with an
/// access level of PRIVATE or
/// UNLISTED and assets which are
/// All Rights Reserved for the
/// currently-authenticated user.
///
/// A builder for the *assets.list* method supported by a *user* resource.
/// It is not used directly, but through a `UserMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_poly1 as poly1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use poly1::PolyService;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = PolyService::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().assets_list("name")
///              .visibility("amet")
///              .page_token("no")
///              .page_size(-36)
///              .order_by("eirmod")
///              .format("dolore")
///              .doit();
/// # }
/// ```
pub struct UserAssetListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PolyService<C, A>,
    _name: String,
    _visibility: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _order_by: Option<String>,
    _format: Option<String>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C, A> CallBuilder for UserAssetListCall<'a, C, A> {}

impl<'a, C, A> UserAssetListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListUserAssetsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "poly.users.assets.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((8 + self._additional_params.len()));
        params.push(("name", self._name.to_string()));
        if let Some(value) = self._visibility {
            params.push(("visibility", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._format {
            params.push(("format", value.to_string()));
        }
        for &field in ["alt", "name", "visibility", "pageToken", "pageSize", "orderBy", "format"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+name}/assets";
        
        let mut key = self.hub.auth.borrow_mut().api_key();
        if key.is_none() {
            key = dlg.api_key();
        }
        match key {
            Some(value) => params.push(("key", value)),
            None => {
                dlg.finished(false);
                return Err(Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET);
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["name"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }



        loop {
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()));

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// A valid user id. Currently, only the special value 'me', representing the
    /// currently-authenticated user is supported. To use 'me', you must pass
    /// an OAuth token with the request.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> UserAssetListCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The visibility of the assets to be returned.
    /// Defaults to VISIBILITY_UNSPECIFIED which returns all assets.
    ///
    /// Sets the *visibility* query property to the given value.
    pub fn visibility(mut self, new_value: &str) -> UserAssetListCall<'a, C, A> {
        self._visibility = Some(new_value.to_string());
        self
    }
    /// Specifies a continuation token from a previous search whose results were
    /// split into multiple pages. To get the next page, submit the same request
    /// specifying the value from
    /// next_page_token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> UserAssetListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of assets to be returned. This value must be between `1`
    /// and `100`. Defaults to `20`.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> UserAssetListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// Specifies an ordering for assets. Acceptable values are:
    /// `BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets
    /// based on a combination of popularity and other features.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> UserAssetListCall<'a, C, A> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Return only assets with the matching format. Acceptable values are:
    /// `BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`.
    ///
    /// Sets the *format* query property to the given value.
    pub fn format(mut self, new_value: &str) -> UserAssetListCall<'a, C, A> {
        self._format = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> UserAssetListCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> UserAssetListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


