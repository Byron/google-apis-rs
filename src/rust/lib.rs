//! library with code shared by all generated implementations
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

use std::marker::MarkerTrait;

/// Identifies types which can be inserted and deleted.
/// Types with this trait are most commonly used by clients of this API.
pub trait Resource: MarkerTrait {}

/// Identifies types which are used in API responses.
pub trait ResponseResult: MarkerTrait {}

/// Identifies types which are used in API requests.
pub trait RequestResult: MarkerTrait {}

/// Identifies types which are only used as part of other types, which 
/// usually are carrying the `Resource` trait.
pub trait Part: MarkerTrait {}

/// Identifies types which are only used by other types internally.
/// They have no special meaning, this trait just marks them for completeness.
pub trait NestedType: MarkerTrait {}

/// This module is for testing only, its code is used in mako templates
#[cfg(test)]
mod dev;