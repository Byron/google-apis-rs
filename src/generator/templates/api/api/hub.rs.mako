<%namespace name="lib" file="../lib/lib.mako"/>\
<%namespace name="util" file="../../../lib/util.mako"/>\
<%namespace name="rbuild" file="../lib/rbuild.mako"/>\
<%namespace name="mbuild" file="../lib/mbuild.mako"/>\
<%namespace name="schema" file="../lib/schema.mako"/>\
<%
    from generator.lib.util import (new_context, rust_doc_comment, rb_type, hub_type, mangle_ident, hub_type_params_s,
                      rb_type_params_s, HUB_TYPE_PARAMETERS)

    c = new_context(schemas, resources)
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()

    default_user_agent = "google-api-rust-client/" + cargo.build_version
%>\
use super::*;

/// Central instance to access all ${hub_type} related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
<%block filter="rust_doc_comment">\
${lib.hub_usage_example(c)}\
</%block>
#[derive(Clone)]
pub struct ${hub_type}${ht_params} {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    pub(super) _user_agent: String,
    pub(super) _base_url: String,
    pub(super) _root_url: String,
}

impl<'a, ${', '.join(HUB_TYPE_PARAMETERS)}> client::Hub for ${hub_type}${ht_params} {}

impl<'a, ${', '.join(HUB_TYPE_PARAMETERS)}> ${hub_type}${ht_params} {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> ${hub_type}${ht_params} {
        ${hub_type} {
            client,
            auth: Box::new(auth),
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
