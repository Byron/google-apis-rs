<%!
    from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, singular, hub_type, mangle_ident, mb_type, property,
                      to_fqan, indent_all_but_first_by, is_repeated_property, is_required_property,
                      activity_input_type, TREF, IO_REQUEST, schema_to_required_property, 
                      rust_copy_value_s, organize_params, REQUEST_VALUE_PROPERTY_NAME,
                      build_all_params, rb_type_params_s, hub_type_params_s, mb_type_params_s, mb_additional_type_params, 
                      struct_type_bounds_s, METHODS_RESOURCE, SPACES_PER_TAB, prefix_all_but_first_with,
                      METHODS_BUILDER_MARKER_TRAIT, remove_empty_lines, method_default_scope, rust_doc_sanitize)
%>\
<%namespace name="util" file="../../lib/util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a Resource builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, c)">\
<% 
    hub_type_name = hub_type(schemas, util.canonical_name())
    rb_params = rb_type_params_s(resource, c)
    ThisType = rb_type(resource) + rb_params
%>\
% if resource == METHODS_RESOURCE:
/// A builder providing access to all free methods, which are not associated with a particular resource.
% else:
/// A builder providing access to all methods supported on *${singular(resource)}* resources.
% endif
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

// Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
// like ${put_and(sorted('`%s(...)`' % mangle_ident(f) for f in c.rta_map[resource]))}
// to build up your call.
let rb = hub.${mangle_ident(resource)}();
</%block>
</%block>
pub struct ${ThisType}
    where ${struct_type_bounds_s()} {

    hub: &'a ${hub_type_name}${hub_type_params_s()},
}

impl${rb_params} ${METHODS_BUILDER_MARKER_TRAIT} for ${ThisType} {}

## Builder Creators Methods ####################
impl${rb_params} ${ThisType} {
    % for a in c.rta_map[resource]:
<%
    m = c.fqan_map[to_fqan(c.rtc_map[resource], resource, a)]
    RType = mb_type(resource, a)

    # skip part if we have a request resource. Only resources can have parts
    # that we can easily deduce
    params, request_value = build_all_params(c, m)
    required_props, optional_props, part_prop = organize_params(params, request_value)

    method_args = ''
    if required_props:
        method_args = ', ' + ', '.join('%s: %s' % (mangle_ident(p.name), activity_input_type(schemas, p)) for p in required_props)

    mb_tparams = mb_type_params_s(m)
    # we would could have information about data requirements for each property in it's dict.
    # for now, we just hardcode it, and treat the entries as way to easily change param names
    assert len(api.properties) == 2, "Hardcoded for now, thanks to scope requirements"

    type_params = ''
    if mb_additional_type_params(m):
        type_params = '<%s>' % ', '.join(mb_additional_type_params(m))
%>\
    
    % if 'description' in m:
    /// Create a builder to help you perform the following task:
    ///
    ${m.description | rust_doc_sanitize, rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    % if required_props:
    /// 
    /// # Arguments
    ///
    % for p in required_props:
<%
        arg_prefix = "/// * `" + p.name + "` - "
%>\
    ${arg_prefix}${p.get('description', "No description provided.")
        | remove_empty_lines, prefix_all_but_first_with(' ' * SPACES_PER_TAB + '///'  + ' ' * (len(arg_prefix) - len('///')))}
    % endfor
    % endif
    pub fn ${mangle_ident(a)}${type_params}(&self${method_args}) -> ${RType}${mb_tparams} {
        % if part_prop and request_value:
        use client::ToParts;
        % if is_repeated_property(part_prop):
            let parts = vec![${mangle_ident(REQUEST_VALUE_PROPERTY_NAME)}.to_parts()];
        % else:
        % if not is_required_property(part_prop):
            let parts = Some(${mangle_ident(REQUEST_VALUE_PROPERTY_NAME)}.to_parts());
        % else:
            let parts = ${mangle_ident(REQUEST_VALUE_PROPERTY_NAME)}.to_parts();
        % endif ## not is_required_property(part_prop)
        % endif is_repeated_property(part_prop):
        % endif
        ${RType} {
            hub: self.hub,
            % for p in required_props:
            ${property(p.name)}: ${rust_copy_value_s(mangle_ident(p.name), activity_input_type(schemas, p), p)},
            % endfor
            ## auto-generate parts from request resources
            % if part_prop and request_value:
            ${property(part_prop.name)}: parts,
            % endif
            % for p in optional_props:
            ${property(p.name)}: Default::default(),
            % endfor
% for prop_key, custom_name in api.properties.items():
            % if prop_key == 'scopes' and not method_default_scope(m):
<% continue %>\
            % endif
            ${custom_name}: Default::default(),
            % endfor
        }
    }
    % endfor ## for each activity
}
</%def>
