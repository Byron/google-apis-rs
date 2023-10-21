<%namespace name="lib" file="../lib/lib.mako"/>\
<%namespace name="util" file="../../../lib/util.mako"/>\
<%namespace name="rbuild" file="../lib/rbuild.mako"/>\
<%namespace name="mbuild" file="../lib/mbuild.mako"/>\
<%namespace name="schema" file="../lib/schema.mako"/>\
<%
    from generator.lib.util import (new_context, rust_comment, rust_doc_comment, rust_module_doc_comment,
                      rb_type, hub_type, mangle_ident, hub_type_params_s,
                      rb_type_params_s, find_fattest_resource, HUB_TYPE_PARAMETERS, METHODS_RESOURCE,
                      UNUSED_TYPE_MARKER, schema_markers)

    c = new_context(schemas, resources)
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()

    default_user_agent = "google-api-rust-client/" + cargo.build_version
%>\
use super::*;
% for resource in c.rta_map:
${rbuild.new(resource, c)}


% endfor
