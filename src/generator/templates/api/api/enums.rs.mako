<%namespace name="lib" file="../lib/lib.mako"/>\
<%namespace name="enum" file="../lib/enum.mako"/>\
<%namespace name="util" file="../../../lib/util.mako"/>\
<%namespace name="rbuild" file="../lib/rbuild.mako"/>\
<%namespace name="mbuild" file="../lib/mbuild.mako"/>\
<%namespace name="schema" file="../lib/schema.mako"/>\
<%
    from generator.lib.util import (new_context, hub_type, hub_type_params_s)

    c = new_context(schemas, resources)
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()


    default_user_agent = "google-api-rust-client/" + cargo.build_version
%>\
use super::*;


% for e in c.enums:
${enum.new(e, c)}
% endfor
