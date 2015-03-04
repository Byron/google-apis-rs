<% 
	from util import (iter_nested_types, new_context, rust_comment, rust_doc_comment,
                      rust_module_doc_comment, rust_doc_test_norun, canonical_type_name,
                      rust_test_fn_invisible)
	nested_schemas = list(iter_nested_types(schemas))

 	c = new_context(resources)

	hub_type = canonical_type_name(canonicalName)
%>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="mutil" file="lib/util.mako"/>\
<%namespace name="schema" file="lib/schema.mako"/>\
<%block filter="rust_comment">\
<%mutil:gen_info source="${self.uri}" />\
</%block>

<%block filter="rust_module_doc_comment">\
${lib.docs(c)}
</%block>
#![feature(core)]
#![allow(non_snake_case)]

extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

mod cmn;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::cell::RefCell;

pub use cmn::{Hub, Resource, Part, ResponseResult, RequestResult, NestedType};

// ########
// HUB ###
// ######

/// Central instance to access all ${hub_type} related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
<%block filter="rust_doc_comment">\
${lib.hub_usage_example()}\
</%block>
pub struct ${hub_type}<C, NC, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for ${hub_type}<C, NC, A> {}

impl<'a, C, NC, A> ${hub_type}<C, NC, A>
    where  NC: hyper::net::NetworkConnector,
            C: BorrowMut<hyper::Client<NC>> + 'a,
            A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> ${hub_type}<C, NC, A> {
        ${hub_type} {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _m: PhantomData,
        }
    }
}


// ############
// SCHEMAS ###
// ##########
% for s in schemas.values():
${schema.new(s, c)}
% endfor

// ###################
// NESTED SCHEMAS ###
// #################
## some schemas are only used once and basically internal types.
## We have to find them and process them as normal types
% for s in nested_schemas:
${schema.new(s, c)}
% endfor