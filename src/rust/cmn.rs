use std::marker::MarkerTrait;
use std::io::{Read, Seek};
use std::borrow::BorrowMut;

/// Identifies the Hub. There is only one per library, this trait is supposed
/// to make intended use more explicit.
/// The hub allows to access all resource methods more easily.
pub trait Hub: MarkerTrait {}

/// Identifies types for building methods of a particular resource type
pub trait ResourceMethodsBuilder: MarkerTrait {}

/// Identifies types which represent builders for a particular resource method
pub trait MethodBuilder: MarkerTrait {}

/// Identifies types which can be inserted and deleted.
/// Types with this trait are most commonly used by clients of this API.
pub trait Resource: MarkerTrait {}

/// Identifies types which are used in API responses.
pub trait ResponseResult: MarkerTrait {}

/// Identifies types which are used in API requests.
pub trait RequestValue: MarkerTrait {}

/// Identifies types which are only used as part of other types, which 
/// usually are carrying the `Resource` trait.
pub trait Part: MarkerTrait {}

/// Identifies types which are only used by other types internally.
/// They have no special meaning, this trait just marks them for completeness.
pub trait NestedType: MarkerTrait {}

/// A utility to specify reader types which provide seeking capabilities too
pub trait ReadSeek: Seek + Read {}
impl<T: Seek + Read> ReadSeek for T {}


/// A utility type which can decode a server response that indicates error
#[derive(RustcDecodable)]
struct JsonServerError {
    error: String,
    error_description: Option<String>
}