<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="lib/util.mako"/>\
<%namespace name="rbuild" file="lib/rbuild.mako"/>\
<%namespace name="mbuild" file="lib/mbuild.mako"/>\
<%namespace name="schema" file="lib/schema.mako"/>\
<%  
    from util import (new_context, rust_comment, rust_doc_comment,
                      rust_module_doc_comment, rb_type, hub_type, mangle_ident, hub_type_params_s,
                      hub_type_bounds, rb_type_params_s)

    c = new_context(schemas, resources)
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>

<%block filter="rust_module_doc_comment">\
${lib.docs(c)}
</%block>
#![feature(core,io)]
// DEBUG !! TODO: Remove this
#![allow(dead_code)]
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut)]


extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use std::io;
use std::fs;

pub use cmn::{Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate};


// ##############
// UTILITIES ###
// ############

/// This macro is advertised in the documentation, which is why we deliver it as well
#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

${lib.scope_enum()}


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
<%lib:hub_usage_example/>\
</%block>
pub struct ${hub_type}${ht_params} {
    client: RefCell<C>,
    auth: RefCell<A>,
    _m: PhantomData<NC>
}

impl<'a, C, NC, A> Hub for ${hub_type}${ht_params} {}

impl<'a, C, NC, A> ${hub_type}${ht_params}
    where  ${', '.join(hub_type_bounds())} {

    pub fn new(client: C, authenticator: A) -> ${hub_type}${ht_params} {
        ${hub_type} {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _m: PhantomData,
        }
    }

    % for resource in sorted(c.rta_map.keys()):
    pub fn ${mangle_ident(resource)}(&'a self) -> ${rb_type(resource)}${rb_type_params_s(resource, c)} {
        ${rb_type(resource)} { hub: &self }
    }
    % endfor
}


% if c.schemas:
// ############
// SCHEMAS ###
// ##########
% for s in c.schemas.values():
${schema.new(s, c)}
% endfor
% endif

// ###################
// MethodBuilders ###
// #################

% for resource in c.rta_map:
${rbuild.new(resource, c)}


% endfor


// ###################
// CallBuilders   ###
// #################

% for resource, methods in c.rta_map.iteritems():
% for method in methods:
${mbuild.new(resource, method, c)}

% endfor ## method in methods
% endfor ## resource, methods