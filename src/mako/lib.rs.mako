<% 
	from util import (iter_nested_types, new_context, rust_comment, rust_module_doc_comment, )
	nested_schemas = list(iter_nested_types(schemas))

	c = new_context(resources)
%>\
<%namespace name="lib" file="lib/lib.mako"/>\
<%namespace name="mutil" file="lib/util.mako"/>\
<%namespace name="schema" file="lib/schema.mako"/>\
<%block filter="rust_comment">\
<%mutil:gen_info source="${self.uri}" />\
</%block>

<%block filter="rust_module_doc_comment">\
${lib.docs(c)}
</%block>
#![feature(core)]
#![allow(non_snake_case)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate "yup-oauth2" as oauth2;

mod cmn;

use std::collections::HashMap;

pub use cmn::{Resource, Part, ResponseResult, RequestResult, NestedType};

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