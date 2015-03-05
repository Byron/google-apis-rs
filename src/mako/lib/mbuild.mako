<%!
    from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, mb_type, singular, hub_type, to_fqan, indent_all_but_first_by,
                      method_params, activity_rust_type, mangle_ident, activity_input_type, get_word,
                      split_camelcase_s, property, is_pod_property, TREF, method_io, IO_REQUEST, 
                      schema_to_required_property, rust_copy_value_s, is_required_property,
                      hide_rust_doc_test, build_all_params, REQUEST_VALUE_PROPERTY_NAME, organize_params, 
                      indent_by, to_rust_type, rnd_arg_val_for_type, extract_parts)

    def make_parts_desc(part_prop):
        if not part_prop:
            return None
        parts = extract_parts(part_prop.get('description', ''))
        if not parts:
            return None
        part_desc = "**Settable Parts**\n\n"
        part_desc += ''.join('* *%s*\n' % part for part in parts)
        part_desc = part_desc[:-1]
        return part_desc
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

    params, request_value = build_all_params(schemas, c, m, IO_REQUEST, REQUEST_VALUE_PROPERTY_NAME)
    part_prop = None
    for p in params:
        if p.name == 'part':
            part_prop = p
            break
    # end for each param
    part_desc = make_parts_desc(part_prop)
%>\
% if 'description' in m:
${m.description | rust_doc_comment}
///
% endif
/// A builder for the *${method}* method supported by a *${singular(resource)}* resource.
/// It is not used directly, but through a `${rb_type(resource)}`.
///
% if part_desc:
${part_desc | rust_doc_comment}
% endif
/// # Example
///
/// Instantiate a resource method builder
///
<%block filter="rust_doc_comment">\
${self.usage(resource, method, params, request_value)}\
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
${self._setter(resource, method, m, p, part_prop, ThisType, c)}\
% endfor
}
</%def>


## creates a setter for the call builder
###############################################################################################
###############################################################################################
<%def name="_setter(resource, method, m, p, part_prop, ThisType, c)">\
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

    part_desc = None
    if part_prop is not None and p.name in ('part', REQUEST_VALUE_PROPERTY_NAME):
        part_desc = make_parts_desc(part_prop)
    # end part description
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
    % if part_desc:
    
    ${part_desc | rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    /// 
    % if 'description' in p:
    ${p.description | rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    pub fn ${mangle_ident(p.name)}(mut self, ${value_name}: ${InType}) -> ${ThisType} {
        self.${property(p.name)} = ${new_value_copied};
        return self;
    }
</%def>


## creates a setter for the call builder
###############################################################################################
###############################################################################################
<%def name="usage(resource, method, params, request_value)">\
<%
    hub_type_name = hub_type(canonicalName)
    required_props, optional_props, part_prop = organize_params(params, request_value)
    is_string_value = lambda v: v.endswith('"')

    # to rust value
    trv = lambda spn, sp, sn=None: to_rust_type(sn, spn, sp, allow_optionals=False)
    # rvfrt = random value for rust type
    rvfrt = lambda spn, sp, sn=None: rnd_arg_val_for_type(trv(spn, sp, sn))
    rb_name = 'req'   # name of request binding
    required_args = request_value and ['&' + rb_name] or []
    for p in required_props:
        # could also just skip the first element, but ... let's be safe
        if request_value and request_value.id == p.get(TREF):
            continue
        v = rvfrt(p.name, p)
        # we chose to replace random strings with their meaning, as indicated by the name !
        if is_string_value(v):
            v = '"%s"' % p.name
        required_args.append(v)
    # end for each required property
    required_args = ', '.join(required_args)

    random_value_warning = "Values shown here are random and not representative !"
%>\
<%block filter="rust_doc_test_norun">\
${capture(util.test_prelude) | hide_rust_doc_test}\
% if request_value:
# use ${util.library_name()}::${request_value.id};
% endif
<%block filter="rust_test_fn_invisible">\
${capture(lib.test_hub, hub_type_name, comments=False) | hide_rust_doc_test}
% if request_value:
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure.
// ${random_value_warning}
let mut ${rb_name}: ${request_value.id} = Default::default();
% for spn, sp in request_value.get('properties', dict()).iteritems():
<%
    rtn = trv(spn, sp, request_value.id)
    assignment = rnd_arg_val_for_type(rtn)
    if is_string_value(assignment):
        assignment = assignment + '.to_string()'
    if assignment.endswith('default()'):
        assignment = assignment[1:] # cut & - it's not ok in this case :)!
        assignment += '; // is %s' % rtn
    else:
        assignment = 'Some(%s);' % assignment
%>\
## ${to_rust_type(request_value.id, spn, sp, allow_optionals=False)}
${rb_name}.${mangle_ident(spn)} = ${assignment}
% endfor

% endif
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `${api.terms.action}()`.
% if optional_props:
// ${random_value_warning}
% endif
let result = hub.${mangle_ident(resource)}().${mangle_ident(method)}(${required_args})\
% for p in optional_props:

<%block  filter="indent_by(8)">\
.${mangle_ident(p.name)}(${rvfrt(p.name, p)})\
</%block>\
% endfor
.${api.terms.action}();
// TODO: show how to handle the result !
</%block>
</%block>\
</%def>