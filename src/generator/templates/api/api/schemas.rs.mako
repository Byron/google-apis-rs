<%namespace name="lib" file="../lib/lib.mako"/>\
<%namespace name="util" file="../../../lib/util.mako"/>\
<%namespace name="rbuild" file="../lib/rbuild.mako"/>\
<%namespace name="mbuild" file="../lib/mbuild.mako"/>\
<%namespace name="schema" file="../lib/schema.mako"/>\
<%
    from generator.lib.util import (new_context, hub_type, hub_type_params_s, UNUSED_TYPE_MARKER, schema_markers)

    c = new_context(schemas, resources)
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()

    default_user_agent = "google-api-rust-client/" + cargo.build_version
%>\
use super::*;
% for s in c.schemas.values():
% if UNUSED_TYPE_MARKER not in schema_markers(s, c, transitive=True):
${schema.new(s, c)}
% endif
% endfor
