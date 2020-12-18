<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="util" file="../lib/util.mako"/>\
<%
    from util import (new_context, rust_comment, rust_module_doc_comment)

    c = new_context(schemas, resources, context.get('methods'))
%>\
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>

<%block filter="rust_module_doc_comment">\
${lib.docs(c)}
</%block>

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

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
<%block filter="rust_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

pub mod api;
pub mod client;

// Re-export the hub type and some basic client structs
pub use api::${hub_type};
pub use client::{Result, Error, Delegate};
