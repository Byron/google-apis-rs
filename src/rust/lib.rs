#![feature(core)]
//! library with code shared by all generated implementations
extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

// just pull it in the check if it compiles
mod cmn;

/// This module is for testing only, its code is used in mako templates
#[cfg(test)]
mod dev;