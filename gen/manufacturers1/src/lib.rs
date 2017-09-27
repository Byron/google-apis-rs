// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Manufacturer Center* crate version *1.0.6+20170808*, where *20170808* is the exact revision of the *manufacturers:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.6*.
//! 
//! Everything else about the *Manufacturer Center* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/manufacturers/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/manufacturers1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.ManufacturerCenter.html) ... 
//! 
//! * accounts
//!  * [*products delete*](struct.AccountProductDeleteCall.html), [*products get*](struct.AccountProductGetCall.html), [*products list*](struct.AccountProductListCall.html) and [*products update*](struct.AccountProductUpdateCall.html)
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
//! * **[Hub](struct.ManufacturerCenter.html)**
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
//! let r = hub.accounts().products_update(...).doit()
//! let r = hub.accounts().products_get(...).doit()
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
//! google-manufacturers1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_manufacturers1 as manufacturers1;
//! use manufacturers1::Product;
//! use manufacturers1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use manufacturers1::ManufacturerCenter;
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
//! let mut hub = ManufacturerCenter::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = Product::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.accounts().products_update(req, "parent", "name")
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

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Manage your product listings for Google Manufacturer Center
    Manufacturercenter,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Manufacturercenter => "https://www.googleapis.com/auth/manufacturercenter",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Manufacturercenter
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all ManufacturerCenter related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_manufacturers1 as manufacturers1;
/// use manufacturers1::Product;
/// use manufacturers1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use manufacturers1::ManufacturerCenter;
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
/// let mut hub = ManufacturerCenter::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Product::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().products_update(req, "parent", "name")
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
pub struct ManufacturerCenter<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for ManufacturerCenter<C, A> {}

impl<'a, C, A> ManufacturerCenter<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> ManufacturerCenter<C, A> {
        ManufacturerCenter {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.6".to_string(),
            _base_url: "https://manufacturers.googleapis.com/".to_string(),
            _root_url: "https://manufacturers.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C, A> {
        AccountMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.6`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://manufacturers.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://manufacturers.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// The number of products in a single package. For more information, see
/// https://support.google.com/manufacturers/answer/6124116#count.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Count {
    /// The unit in which these products are counted.
    pub unit: Option<String>,
    /// The numeric value of the number of products in a package.
    pub value: Option<String>,
}

impl Part for Count {}


/// Product data.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products update accounts](struct.AccountProductUpdateCall.html) (request|response)
/// * [products get accounts](struct.AccountProductGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    /// The target country of the product as a CLDR territory code (for example,
    /// US).
    /// @OutputOnly
    #[serde(rename="targetCountry")]
    pub target_country: Option<i64>,
    /// Names of the attributes of the product deleted manually via the
    /// Manufacturer Center UI.
    /// @OutputOnly
    #[serde(rename="manuallyDeletedAttributes")]
    pub manually_deleted_attributes: Option<Vec<String>>,
    /// Name in the format `{target_country}:{content_language}:{product_id}`.
    /// 
    /// `target_country`   - The target country of the product as a CLDR territory
    ///                      code (for example, US).
    /// 
    /// `content_language` - The content language of the product as a two-letter
    ///                      ISO 639-1 language code (for example, en).
    /// 
    /// `product_id`     -   The ID of the product. For more information, see
    ///                      https://support.google.com/manufacturers/answer/6124116#id.
    /// @OutputOnly
    pub name: Option<String>,
    /// Parent ID in the format `accounts/{account_id}`.
    /// 
    /// `account_id` - The ID of the Manufacturer Center account.
    /// @OutputOnly
    pub parent: Option<String>,
    /// Attributes of the product provided manually via the Manufacturer Center UI.
    /// @OutputOnly
    #[serde(rename="manuallyProvidedAttributes")]
    pub manually_provided_attributes: Option<Attributes>,
    /// Final attributes of the product. The final attributes are obtained by
    /// overriding the uploaded attributes with the manually provided and deleted
    /// attributes. Google systems only process, evaluate, review, and/or use final
    /// attributes.
    /// @OutputOnly
    #[serde(rename="finalAttributes")]
    pub final_attributes: Option<Attributes>,
    /// Attributes of the product uploaded via the Manufacturer Center API or via
    /// feeds.
    #[serde(rename="uploadedAttributes")]
    pub uploaded_attributes: Option<Attributes>,
    /// The content language of the product as a two-letter ISO 639-1 language code
    /// (for example, en).
    /// @OutputOnly
    #[serde(rename="contentLanguage")]
    pub content_language: Option<String>,
    /// A server-generated list of issues associated with the product.
    /// @OutputOnly
    pub issues: Option<Vec<Issue>>,
    /// The ID of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#id.
    /// @OutputOnly
    #[serde(rename="productId")]
    pub product_id: Option<String>,
}

impl RequestValue for Product {}
impl ResponseResult for Product {}


/// The capacity of a product. For more information, see
/// https://support.google.com/manufacturers/answer/6124116#capacity.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Capacity {
    /// The unit of the capacity, i.e., MB, GB, or TB.
    pub unit: Option<String>,
    /// The numeric value of the capacity.
    pub value: Option<String>,
}

impl Part for Capacity {}


/// An image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// The status of the image.
    /// @OutputOnly
    pub status: Option<String>,
    /// The URL of the image. For crawled images, this is the provided URL. For
    /// uploaded images, this is a serving URL from Google if the image has been
    /// processed successfully.
    #[serde(rename="imageUrl")]
    pub image_url: Option<String>,
    /// The type of the image, i.e., crawled or uploaded.
    /// @OutputOnly
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for Image {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products list accounts](struct.AccountProductListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListProductsResponse {
    /// The token for the retrieval of the next page of product statuses.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// List of the products.
    pub products: Option<Vec<Product>>,
}

impl ResponseResult for ListProductsResponse {}


/// A feature description of the product. For more information, see
/// https://support.google.com/manufacturers/answer/6124116#featuredesc.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureDescription {
    /// A short description of the feature.
    pub headline: Option<String>,
    /// A detailed description of the feature.
    pub text: Option<String>,
    /// An optional image describing the feature.
    pub image: Option<Image>,
}

impl Part for FeatureDescription {}


/// A product detail of the product. For more information, see
/// https://support.google.com/manufacturers/answer/6124116#productdetail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProductDetail {
    /// The name of the attribute.
    #[serde(rename="attributeName")]
    pub attribute_name: Option<String>,
    /// The value of the attribute.
    #[serde(rename="attributeValue")]
    pub attribute_value: Option<String>,
    /// A short section name that can be reused between multiple product details.
    #[serde(rename="sectionName")]
    pub section_name: Option<String>,
}

impl Part for ProductDetail {}


/// Attributes of the product. For more information, see
/// https://support.google.com/manufacturers/answer/6124116.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    /// The name of the group of products related to the product. For more
    /// information, see
    /// https://support.google.com/manufacturers/answer/6124116#productline.
    #[serde(rename="productLine")]
    pub product_line: Option<String>,
    /// The color of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#color.
    pub color: Option<String>,
    /// The release date of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#release.
    #[serde(rename="releaseDate")]
    pub release_date: Option<String>,
    /// The item group id of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#itemgroupid.
    #[serde(rename="itemGroupId")]
    pub item_group_id: Option<String>,
    /// The videos of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#video.
    #[serde(rename="videoLink")]
    pub video_link: Option<Vec<String>>,
    /// The flavor of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#flavor.
    pub flavor: Option<String>,
    /// The scent of the product. For more information, see
    ///  https://support.google.com/manufacturers/answer/6124116#scent.
    pub scent: Option<String>,
    /// The size of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#size.
    pub size: Option<String>,
    /// The capacity of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#capacity.
    pub capacity: Option<Capacity>,
    /// The title of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#title.
    pub title: Option<String>,
    /// The pattern of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#pattern.
    pub pattern: Option<String>,
    /// The disclosure date of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#disclosure.
    #[serde(rename="disclosureDate")]
    pub disclosure_date: Option<String>,
    /// The theme of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#theme.
    pub theme: Option<String>,
    /// The suggested retail price (MSRP) of the product. For more information,
    /// see https://support.google.com/manufacturers/answer/6124116#price.
    #[serde(rename="suggestedRetailPrice")]
    pub suggested_retail_price: Option<Price>,
    /// The material of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#material.
    pub material: Option<String>,
    /// The description of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#description.
    pub description: Option<String>,
    /// The format of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#format.
    pub format: Option<String>,
    /// The Manufacturer Part Number (MPN) of the product. For more information,
    /// see https://support.google.com/manufacturers/answer/6124116#mpn.
    pub mpn: Option<String>,
    /// The brand name of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#brand.
    pub brand: Option<String>,
    /// The details of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#productdetail.
    #[serde(rename="productDetail")]
    pub product_detail: Option<Vec<ProductDetail>>,
    /// The canonical name of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#productname.
    #[serde(rename="productName")]
    pub product_name: Option<String>,
    /// The rich format description of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#featuredesc.
    #[serde(rename="featureDescription")]
    pub feature_description: Option<Vec<FeatureDescription>>,
    /// The size system of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#sizesystem.
    #[serde(rename="sizeSystem")]
    pub size_system: Option<String>,
    /// The size type of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#sizetype.
    #[serde(rename="sizeType")]
    pub size_type: Option<String>,
    /// The additional images of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#addlimage.
    #[serde(rename="additionalImageLink")]
    pub additional_image_link: Option<Vec<Image>>,
    /// The count of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#count.
    pub count: Option<Count>,
    /// The target gender of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#gender.
    pub gender: Option<String>,
    /// The URL of the detail page of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#productpage.
    #[serde(rename="productPageUrl")]
    pub product_page_url: Option<String>,
    /// The image of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#image.
    #[serde(rename="imageLink")]
    pub image_link: Option<Image>,
    /// The category of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#producttype.
    #[serde(rename="productType")]
    pub product_type: Option<Vec<String>>,
    /// The target account id. Should only be used in the accounts of the data
    /// partners.
    #[serde(rename="targetAccountId")]
    pub target_account_id: Option<String>,
    /// The Global Trade Item Number (GTIN) of the product. For more information,
    /// see https://support.google.com/manufacturers/answer/6124116#gtin.
    pub gtin: Option<Vec<String>>,
    /// The target age group of the product. For more information, see
    /// https://support.google.com/manufacturers/answer/6124116#agegroup.
    #[serde(rename="ageGroup")]
    pub age_group: Option<String>,
}

impl Part for Attributes {}


/// Product issue.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Issue {
    /// If present, the attribute that triggered the issue. For more information
    /// about attributes, see
    /// https://support.google.com/manufacturers/answer/6124116.
    pub attribute: Option<String>,
    /// The server-generated type of the issue, for example,
    /// “INCORRECT_TEXT_FORMATTING”, “IMAGE_NOT_SERVEABLE”, etc.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Description of the issue.
    pub description: Option<String>,
    /// The severity of the issue.
    pub severity: Option<String>,
    /// The timestamp when this issue appeared.
    pub timestamp: Option<String>,
}

impl Part for Issue {}


/// A price.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    /// The currency in which the price is denoted.
    pub currency: Option<String>,
    /// The numeric value of the price.
    pub amount: Option<String>,
}

impl Part for Price {}


/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
/// 
///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }
/// 
/// The JSON representation for `Empty` is empty JSON object `{}`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [products delete accounts](struct.AccountProductDeleteCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the `ManufacturerCenter` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_manufacturers1 as manufacturers1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use manufacturers1::ManufacturerCenter;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = ManufacturerCenter::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `products_delete(...)`, `products_get(...)`, `products_list(...)` and `products_update(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ManufacturerCenter<C, A>,
}

impl<'a, C, A> MethodsBuilder for AccountMethods<'a, C, A> {}

impl<'a, C, A> AccountMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists all the products in a Manufacturer Center account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Parent ID in the format `accounts/{account_id}`.
    ///              `account_id` - The ID of the Manufacturer Center account.
    pub fn products_list(&self, parent: &str) -> AccountProductListCall<'a, C, A> {
        AccountProductListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Inserts or updates the product in a Manufacturer Center account.
    /// 
    /// The checks at upload time are minimal. All required attributes need to be
    /// present for a product to be valid. Issues may show up later
    /// after the API has accepted an update for a product and it is possible to
    /// overwrite an existing valid product with an invalid product. To detect
    /// this, you should retrieve the product and check it for issues once the
    /// updated version is available.
    /// 
    /// Inserted or updated products first need to be processed before they can be
    /// retrieved. Until then, new products will be unavailable, and retrieval
    /// of updated products will return the original state of the product.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Parent ID in the format `accounts/{account_id}`.
    ///              `account_id` - The ID of the Manufacturer Center account.
    /// * `name` - Name in the format `{target_country}:{content_language}:{product_id}`.
    ///            `target_country`   - The target country of the product as a CLDR territory
    ///                                 code (for example, US).
    ///            `content_language` - The content language of the product as a two-letter
    ///                                 ISO 639-1 language code (for example, en).
    ///            `product_id`     -   The ID of the product. For more information, see
    ///                                 https://support.google.com/manufacturers/answer/6124116#id.
    pub fn products_update(&self, request: Product, parent: &str, name: &str) -> AccountProductUpdateCall<'a, C, A> {
        AccountProductUpdateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the product from a Manufacturer Center account.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Parent ID in the format `accounts/{account_id}`.
    ///              `account_id` - The ID of the Manufacturer Center account.
    /// * `name` - Name in the format `{target_country}:{content_language}:{product_id}`.
    ///            `target_country`   - The target country of the product as a CLDR territory
    ///                                 code (for example, US).
    ///            `content_language` - The content language of the product as a two-letter
    ///                                 ISO 639-1 language code (for example, en).
    ///            `product_id`     -   The ID of the product. For more information, see
    ///                                 https://support.google.com/manufacturers/answer/6124116#id.
    pub fn products_delete(&self, parent: &str, name: &str) -> AccountProductDeleteCall<'a, C, A> {
        AccountProductDeleteCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the product from a Manufacturer Center account, including product
    /// issues.
    /// 
    /// A recently updated product takes around 15 minutes to process. Changes are
    /// only visible after it has been processed. While some issues may be
    /// available once the product has been processed, other issues may take days
    /// to appear.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Parent ID in the format `accounts/{account_id}`.
    ///              `account_id` - The ID of the Manufacturer Center account.
    /// * `name` - Name in the format `{target_country}:{content_language}:{product_id}`.
    ///            `target_country`   - The target country of the product as a CLDR territory
    ///                                 code (for example, US).
    ///            `content_language` - The content language of the product as a two-letter
    ///                                 ISO 639-1 language code (for example, en).
    ///            `product_id`     -   The ID of the product. For more information, see
    ///                                 https://support.google.com/manufacturers/answer/6124116#id.
    pub fn products_get(&self, parent: &str, name: &str) -> AccountProductGetCall<'a, C, A> {
        AccountProductGetCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _name: name.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Lists all the products in a Manufacturer Center account.
///
/// A builder for the *products.list* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_manufacturers1 as manufacturers1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manufacturers1::ManufacturerCenter;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ManufacturerCenter::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().products_list("parent")
///              .page_token("accusam")
///              .page_size(-8)
///              .doit();
/// # }
/// ```
pub struct AccountProductListCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ManufacturerCenter<C, A>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountProductListCall<'a, C, A> {}

impl<'a, C, A> AccountProductListCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, ListProductsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manufacturers.accounts.products.list",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/products";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Manufacturercenter.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
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
            for param_name in ["parent"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// Parent ID in the format `accounts/{account_id}`.
    /// 
    /// `account_id` - The ID of the Manufacturer Center account.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountProductListCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// The token returned by the previous request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountProductListCall<'a, C, A> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of product statuses to return in the response, used for
    /// paging.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountProductListCall<'a, C, A> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountProductListCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountProductListCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Manufacturercenter`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountProductListCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Inserts or updates the product in a Manufacturer Center account.
/// 
/// The checks at upload time are minimal. All required attributes need to be
/// present for a product to be valid. Issues may show up later
/// after the API has accepted an update for a product and it is possible to
/// overwrite an existing valid product with an invalid product. To detect
/// this, you should retrieve the product and check it for issues once the
/// updated version is available.
/// 
/// Inserted or updated products first need to be processed before they can be
/// retrieved. Until then, new products will be unavailable, and retrieval
/// of updated products will return the original state of the product.
///
/// A builder for the *products.update* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_manufacturers1 as manufacturers1;
/// use manufacturers1::Product;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manufacturers1::ManufacturerCenter;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ManufacturerCenter::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Product::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().products_update(req, "parent", "name")
///              .doit();
/// # }
/// ```
pub struct AccountProductUpdateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ManufacturerCenter<C, A>,
    _request: Product,
    _parent: String,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountProductUpdateCall<'a, C, A> {}

impl<'a, C, A> AccountProductUpdateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Product)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manufacturers.accounts.products.update",
                               http_method: hyper::method::Method::Put });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((5 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "parent", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/products/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Manufacturercenter.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent"), ("{+name}", "name")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["name", "parent"].iter() {
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

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Put, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

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


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Product) -> AccountProductUpdateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// Parent ID in the format `accounts/{account_id}`.
    /// 
    /// `account_id` - The ID of the Manufacturer Center account.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountProductUpdateCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Name in the format `{target_country}:{content_language}:{product_id}`.
    /// 
    /// `target_country`   - The target country of the product as a CLDR territory
    ///                      code (for example, US).
    /// 
    /// `content_language` - The content language of the product as a two-letter
    ///                      ISO 639-1 language code (for example, en).
    /// 
    /// `product_id`     -   The ID of the product. For more information, see
    ///                      https://support.google.com/manufacturers/answer/6124116#id.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountProductUpdateCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountProductUpdateCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountProductUpdateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Manufacturercenter`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountProductUpdateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the product from a Manufacturer Center account.
///
/// A builder for the *products.delete* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_manufacturers1 as manufacturers1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manufacturers1::ManufacturerCenter;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ManufacturerCenter::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().products_delete("parent", "name")
///              .doit();
/// # }
/// ```
pub struct AccountProductDeleteCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ManufacturerCenter<C, A>,
    _parent: String,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountProductDeleteCall<'a, C, A> {}

impl<'a, C, A> AccountProductDeleteCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Empty)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manufacturers.accounts.products.delete",
                               http_method: hyper::method::Method::Delete });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "parent", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/products/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Manufacturercenter.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent"), ("{+name}", "name")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["name", "parent"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Delete, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// Parent ID in the format `accounts/{account_id}`.
    /// 
    /// `account_id` - The ID of the Manufacturer Center account.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountProductDeleteCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Name in the format `{target_country}:{content_language}:{product_id}`.
    /// 
    /// `target_country`   - The target country of the product as a CLDR territory
    ///                      code (for example, US).
    /// 
    /// `content_language` - The content language of the product as a two-letter
    ///                      ISO 639-1 language code (for example, en).
    /// 
    /// `product_id`     -   The ID of the product. For more information, see
    ///                      https://support.google.com/manufacturers/answer/6124116#id.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountProductDeleteCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountProductDeleteCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountProductDeleteCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Manufacturercenter`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountProductDeleteCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Gets the product from a Manufacturer Center account, including product
/// issues.
/// 
/// A recently updated product takes around 15 minutes to process. Changes are
/// only visible after it has been processed. While some issues may be
/// available once the product has been processed, other issues may take days
/// to appear.
///
/// A builder for the *products.get* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_manufacturers1 as manufacturers1;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use manufacturers1::ManufacturerCenter;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = ManufacturerCenter::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().products_get("parent", "name")
///              .doit();
/// # }
/// ```
pub struct AccountProductGetCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a ManufacturerCenter<C, A>,
    _parent: String,
    _name: String,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for AccountProductGetCall<'a, C, A> {}

impl<'a, C, A> AccountProductGetCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, Product)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "manufacturers.accounts.products.get",
                               http_method: hyper::method::Method::Get });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((4 + self._additional_params.len()));
        params.push(("parent", self._parent.to_string()));
        params.push(("name", self._name.to_string()));
        for &field in ["alt", "parent", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+parent}/products/{+name}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Manufacturercenter.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent"), ("{+name}", "name")].iter() {
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
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["name", "parent"].iter() {
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
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Get, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone());

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


    /// Parent ID in the format `accounts/{account_id}`.
    /// 
    /// `account_id` - The ID of the Manufacturer Center account.
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> AccountProductGetCall<'a, C, A> {
        self._parent = new_value.to_string();
        self
    }
    /// Name in the format `{target_country}:{content_language}:{product_id}`.
    /// 
    /// `target_country`   - The target country of the product as a CLDR territory
    ///                      code (for example, US).
    /// 
    /// `content_language` - The content language of the product as a two-letter
    ///                      ISO 639-1 language code (for example, en).
    /// 
    /// `product_id`     -   The ID of the product. For more information, see
    ///                      https://support.google.com/manufacturers/answer/6124116#id.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountProductGetCall<'a, C, A> {
        self._name = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> AccountProductGetCall<'a, C, A> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountProductGetCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Manufacturercenter`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountProductGetCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


