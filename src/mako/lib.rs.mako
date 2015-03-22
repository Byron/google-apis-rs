<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="lib/util.mako"/>\
<%namespace name="rbuild" file="lib/rbuild.mako"/>\
<%namespace name="mbuild" file="lib/mbuild.mako"/>\
<%namespace name="schema" file="lib/schema.mako"/>\
<%  
    from util import (new_context, rust_comment, rust_doc_comment, rust_module_doc_comment, 
                      rb_type, hub_type, mangle_ident, hub_type_params_s, hub_type_bounds, 
                      rb_type_params_s, find_fattest_resource, HUB_TYPE_PARAMETERS, METHODS_RESOURCE,
                      UNUSED_TYPE_MARKER, schema_markers)

    c = new_context(schemas, resources, context.get('methods'))
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()

    default_user_agent = "google-api-rust-client/" + cargo.build_version
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>

<%block filter="rust_module_doc_comment">\
${lib.docs(c)}
</%block>
#![feature(core,io,thread_sleep)]
// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut)]
// Required for serde annotations
#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate hyper;
extern crate serde;
extern crate "yup-oauth2" as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::json;
use std::io;
use std::fs;
use std::thread::sleep;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, CallBuilder, Hub, ReadSeek, Part, ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, ResourceMethodsBuilder, Resource};


// ##############
// UTILITIES ###
// ############

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
${lib.hub_usage_example(c)}\
</%block>
pub struct ${hub_type}${ht_params} {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,

    _m: PhantomData<NC>
}

impl<'a, ${', '.join(HUB_TYPE_PARAMETERS)}> Hub for ${hub_type}${ht_params} {}

impl<'a, ${', '.join(HUB_TYPE_PARAMETERS)}> ${hub_type}${ht_params}
    where  ${', '.join(hub_type_bounds())} {

    pub fn new(client: C, authenticator: A) -> ${hub_type}${ht_params} {
        ${hub_type} {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "${default_user_agent}".to_string(),
            _m: PhantomData
        }
    }

    % for resource in sorted(c.rta_map.keys()):
    pub fn ${mangle_ident(resource)}(&'a self) -> ${rb_type(resource)}${rb_type_params_s(resource, c)} {
        ${rb_type(resource)} { hub: &self }
    }
    % endfor

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `${default_user_agent}`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        let prev = self._user_agent.clone();
        self._user_agent = agent_name;
        prev
    }
}


% if c.schemas:
// ############
// SCHEMAS ###
// ##########
% for s in c.schemas.values():
% if UNUSED_TYPE_MARKER not in schema_markers(s, c, transitive=True):
${schema.new(s, c)}
% endif
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