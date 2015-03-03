<% 
	import util 
	nested_schemas = list(util.iter_nested_types(schemas))

	sta_map, fqan_map = util.build_activity_mappings(resources)
	c = util.Context(sta_map, fqan_map)
%>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="mutil" file="lib/util.mako"/>\
<%namespace name="schema" file="lib/schema.mako"/>\
<%block filter="util.rust_module_doc_comment">\
<%lib:docs />\
</%block>
#![allow(non_snake_case)]

extern crate cmn;
extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

use std::collections::HashMap;

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