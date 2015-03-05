<%!
	from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, mb_type, singular, hub_type, to_fqan, indent_all_but_first_by,
                      method_params, activity_rust_type, mangle_ident, activity_input_type, get_word,
                      split_camelcase_s, property, is_pod_property, TREF, method_io, IO_REQUEST, 
                      RESOURCE_MARKER, schema_to_required_property, rust_copy_value_s, is_required_property)
%>\
<%namespace name="util" file="util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a Call builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, method, c)">\
<% 
	hub_type_name = hub_type(canonicalName)
	m = c.fqan_map[to_fqan(name, resource, method)]
	# an identifier for a property. We prefix them to prevent clashes with the setters
	ThisType = mb_type(resource, method) + "<'a, C, NC, A>"

	request_resource = method_io(schemas, c, m, IO_REQUEST, RESOURCE_MARKER)
	params = method_params(m)
	if request_resource:
		params.insert(0, schema_to_required_property(request_resource, 'request'))
%>\
% if 'description' in m:
${m.description | rust_doc_comment}
///
% endif
/// A builder for the *${method}* method supported by a *${singular(resource)}* resource.
/// It is not used directly, but through a `${rb_type(resource)}`.
///
/// # Example
///
/// Instantiate a resource method builder
///
<%block filter="rust_doc_test_norun, rust_doc_comment">\
${util.test_prelude()}\

<%block filter="rust_test_fn_invisible">\
${lib.test_hub(hub_type_name, comments=False)}\

// Usually you wouldn't bind this to a variable, but keep calling methods
// to setup your call.
// TODO: figoure out actual arguments ... 
// let mb = hub.${resource}().${mangle_ident(method)}(...);

// Finally, execute your call and process the result
// TODO: comment in once args are properly setup !
// mb.do()
</%block>
</%block>
pub struct ${ThisType}
    where NC: 'a,
           C: 'a,
           A: 'a, {

    hub: &'a ${hub_type_name}<C, NC, A>,
## PROPERTIES ###############
% for p in params:
	${property(p.name)}:\
	% if is_required_property(p):
	${activity_rust_type(p, allow_optionals=False)},
	% else:
     ${activity_rust_type(p)},
    % endif
% endfor
}

impl<'a, C, NC, A> MethodBuilder for ${ThisType} {}

impl<'a, C, NC, A> ${ThisType} {

	/// Perform the operation you have build so far.
	/// Can only be called once !
	/// TODO: Build actual call
	pub fn ${api.terms.action}(self) {

	}

## SETTERS ###############
% for p in params:
${self._setter(resource, method, m, p, ThisType, c)}\
% endfor
}
</%def>


## creates a setter for the call builder
###############################################################################################
###############################################################################################
<%def name="_setter(resource, method, m, p, ThisType, c)">\
<%
	InType = activity_input_type(p)

	def show_part_info(m, p):
		if p.name != 'part':
			return False
		if not (m.get('request') and m.get('response')):
			return False
		return m.request.get(TREF, 'first') == m.response.get(TREF, 'second')

	value_name = 'new_value'
	new_value_copied = rust_copy_value_s(value_name, InType, p)
	if not is_required_property(p):
		new_value_copied = 'Some(%s)' % new_value_copied
%>\
	/// Sets the *${split_camelcase_s(p.name)}* ${get_word(p, 'location')}property to the given value.
	///
	% if show_part_info(m, p):
	/// Even though the *parts* list is automatically derived from *Resource* passed in 
	/// during instantiation and indicates which values you are passing, the response would contain the very same parts.
	/// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
	/// like statistics that are generated server side. Therefore you should use this method to specify 
	/// the parts you provide in addition to the ones you want in the response.
	% elif is_required_property(p):
	/// Even though the property as already been set when instantiating this call, 
	/// we provide this method for API completeness.
	% endif
	/// 
	% if 'description' in p:
    ${p.description | rust_doc_comment, indent_all_but_first_by(1)}
	% endif
	pub fn ${mangle_ident(p.name)}(&mut self, ${value_name}: ${InType}) -> &mut ${ThisType} {
		self.${property(p.name)} = ${new_value_copied};
		return self;
	}
</%def>