<%namespace name="lib" file="../lib/lib.mako"/>\
<%namespace name="enum" file="../lib/enum.mako"/>\
<%namespace name="util" file="../../../lib/util.mako"/>\
<%namespace name="rbuild" file="../lib/rbuild.mako"/>\
<%namespace name="mbuild" file="../lib/mbuild.mako"/>\
<%namespace name="schema" file="../lib/schema.mako"/>\
<%
    from generator.lib.util import (new_context, hub_type, hub_type_params_s)
    from generator.lib.enum_utils import (find_enums_in_context)

    c = new_context(schemas, resources)
    hub_type = hub_type(c.schemas, util.canonical_name())
    ht_params = hub_type_params_s()

    enums = find_enums_in_context(c)

    default_user_agent = "google-api-rust-client/" + cargo.build_version
%>\
use super::*;



% for schema_name,property_name,enum_type, e in enums:
${enum.new(enum_type, e, c)}
% endfor
