<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
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

use crate::client;

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
    _base_url: String,
    _root_url: String,
}

impl<'a, ${', '.join(HUB_TYPE_PARAMETERS)}> client::Hub for ${hub_type}${ht_params} {}

impl<'a, ${', '.join(HUB_TYPE_PARAMETERS)}> ${hub_type}${ht_params}
    where  ${', '.join(hub_type_bounds())} {

    pub fn new(client: C, authenticator: A) -> ${hub_type}${ht_params} {
        ${hub_type} {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "${default_user_agent}".to_string(),
            _base_url: "${baseUrl}".to_string(),
            _root_url: "${rootUrl}".to_string(),
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
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `${baseUrl}`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `${rootUrl}`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
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

% for resource, methods in c.rta_map.items():
% for method in methods:
${mbuild.new(resource, method, c)}

% endfor ## method in methods
% endfor ## resource, methods
