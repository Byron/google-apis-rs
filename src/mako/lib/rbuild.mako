<%!
	from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, singular, hub_type, mangle_ident, mb_type, method_params, property,
                      to_fqan, indent_all_but_first_by, schema_markers, 
                      activity_input_type, TREF, method_io, IO_REQUEST, schema_to_required_property, 
                      rust_copy_value_s, is_required_property)
%>\
<%namespace name="util" file="util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a Resource builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, c)">\
<% 
	hub_type_name = hub_type(canonicalName) 
	ThisType = rb_type(resource) + "<'a, C, NC, A>"
%>\
/// A builder providing access to all methods supported on *${singular(resource)}* resources.
/// It is not used directly, but through the `${hub_type_name}` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
<%block filter="rust_doc_test_norun, rust_doc_comment">\
${util.test_prelude()}\

<%block filter="rust_test_fn_invisible">\
${lib.test_hub(hub_type_name, comments=False)}\

// Usually you wouldn't bind this to a variable, but keep calling *MethodBuilders*
// like ${put_and(sorted('`%s(...)`' % mangle_ident(f) for f in c.rta_map[resource]))}
// to build up your call.
let rb = hub.${mangle_ident(resource)}();
</%block>
</%block>
pub struct ${ThisType}
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a ${hub_type_name}<C, NC, A>
}

impl<'a, C, NC, A> ResourceMethodsBuilder for ${ThisType} {}

## Builder Creators Methods ####################
impl<'a, C, NC, A> ${ThisType} {
	% for a in c.rta_map[resource]:
<%
	m = c.fqan_map[to_fqan(name, resource, a)]
	RType = mb_type(resource, a)

	# skip part if we have a request resource. Only resources can have parts
	# that we can easily deduce
	request_resource = method_io(schemas, c, m, IO_REQUEST)
	params = method_params(m)
	REQUEST_RESOURCE_PROPERTY_NAME = 'request'
	if request_resource:
		# resource into property 
		resprop = schema_to_required_property(request_resource, REQUEST_RESOURCE_PROPERTY_NAME)
		params.insert(0, resprop)

	part_prop = None
	optional_props = list()
	required_props = list()
	for p in params:
		if is_required_property(p):
			if request_resource and p.name == 'part':
				part_prop = p
			else:
				required_props.append(p)
		else:
			optional_props.append(p)
	# end for each property

	method_args = ''
	if required_props:
		method_args = ', ' + ', '.join('%s: %s' % (mangle_ident(p.name), activity_input_type(p)) for p in required_props)
%>\
	
	% if 'description' in m:
	/// Create a builder to help you perform the following task:
	///
	${m.description | rust_doc_comment, indent_all_but_first_by(1)}
	% endif
	pub fn ${mangle_ident(a)}(&self${method_args}) -> ${RType}<'a, C, NC, A> {
		${RType} {
			hub: self.hub,
			% for p in required_props:
			${property(p.name)}: ${rust_copy_value_s(mangle_ident(p.name), activity_input_type(p), p)},
			% endfor
			## auto-generate parts from request resources
			% if part_prop and request_resource:
			${property(part_prop.name)}: ${mangle_ident(REQUEST_RESOURCE_PROPERTY_NAME)}.to_parts(),
			% endif
			% for p in optional_props:
			${property(p.name)}: Default::default(),
			% endfor
		}
	}
	% endfor ## for each activity
}
</%def>