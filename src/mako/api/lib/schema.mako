<%!
    from util import (schema_markers, rust_doc_comment, mangle_ident, to_rust_type, put_and, 
                      IO_TYPES, activity_split, enclose_in, REQUEST_MARKER_TRAIT, mb_type, indent_all_but_first_by,
                      NESTED_TYPE_SUFFIX, RESPONSE_MARKER_TRAIT, split_camelcase_s, METHODS_RESOURCE,
                      PART_MARKER_TRAIT, canonical_type_name, TO_PARTS_MARKER, UNUSED_TYPE_MARKER, is_schema_with_optionals,
                      rust_doc_sanitize, items)
%>\
## Build a schema which must be an object
###################################################################################################################
###################################################################################################################
<%def name="_new_object(s, properties, c, allow_optionals)">\
<% struct = 'pub struct ' + s.id %>\
% if properties:
${struct} {
% for pn, p in items(properties):
    ${p.get('description', 'no description provided') | rust_doc_sanitize, rust_doc_comment, indent_all_but_first_by(1)}
    % if pn != mangle_ident(pn):
    #[serde(rename="${pn}")]
    % endif
    pub ${mangle_ident(pn)}: ${to_rust_type(schemas, s.id, pn, p, allow_optionals=allow_optionals)},
% endfor
}
% elif 'additionalProperties' in s:
${struct}(${to_rust_type(schemas, s.id, NESTED_TYPE_SUFFIX, s, allow_optionals=allow_optionals)});
% elif 'variant' in s:
<% 
    et = s.id
    variant_type = lambda p: canonical_type_name(p.type_value)
%>\
pub enum ${et} {
% for p in s.variant.map:
    ${p.get('description', 'no description provided') | rust_doc_sanitize, rust_doc_comment, indent_all_but_first_by(1)}
    % if variant_type(p) != p.type_value:
    #[serde(rename="${p.type_value}")]
    % endif
    ${variant_type(p)}(${to_rust_type(schemas, s.id, None, p, allow_optionals=allow_optionals)}),
% endfor
}

impl Default for ${et} {
    fn default() -> ${et} {
        ${et}::${variant_type(s.variant.map[0])}(Default::default())
    }
}
% else: ## it's an empty struct, i.e. struct Foo;
        ## However, to enable the empty JSON object to be parsed, we set one unused optional parameter.
${struct} { _never_set: Option<bool> }
% endif ## 'properties' in s
</%def>

## Create new schema with everything.
## 's' contains the schema structure from json to build
###################################################################################################################
###################################################################################################################
<%def name="new(s, c)">\
<% 
    markers = schema_markers(s, c, transitive=True)
    # We always need Serialization support, as others might want to serialize the response, even though we will 
    # only deserialize it.
    # And since we don't know what others want to do, we implement Deserialize as well by default ... 
    traits = ['Clone', 'Debug', 'Serialize', 'Deserialize']

    # default only works for structs, and 'variant' will be an enum
    if 'variant' not in s:
        traits.insert(0, 'Default')
    
    nt_markers = schema_markers(s, c, transitive=False)
    allow_optionals = is_schema_with_optionals(nt_markers)
    
    # waiting for Default: https://github.com/rust-lang/rustc-serialize/issues/71
    if s.type == 'any':
        traits.remove('Default')

    s_type = s.id
%>\
<%block filter="rust_doc_sanitize, rust_doc_comment">\
${doc(s, c)}\
</%block>
#[derive(${', '.join(traits)})]
% if s.type == 'object':
${_new_object(s, s.get('properties'), c, allow_optionals)}\
% elif s.type == 'array':
% if s.items.get('type') != 'object':
pub struct ${s_type}(${to_rust_type(schemas, s.id, NESTED_TYPE_SUFFIX, s, allow_optionals=allow_optionals)});
% else:
${_new_object(s, s.items.get('properties'), c, allow_optionals)}\
% endif ## array item != 'object'
% elif s.type == 'any':
## waiting for Default: https://github.com/rust-lang/rustc-serialize/issues/71
pub struct ${s_type}(json::Value);

impl Default for ${s_type} {
    fn default() -> ${s_type} {
        ${s_type}(json::Value::Null)
    }
}
% else:
<% assert False, "Object not handled: %s" % str(s) %>\
% endif ## type == ?

% for marker_trait in nt_markers:
% if marker_trait not in (TO_PARTS_MARKER, UNUSED_TYPE_MARKER):
impl ${marker_trait} for ${s_type} {}
% endif
% endfor

% if TO_PARTS_MARKER in nt_markers and allow_optionals:
impl ${TO_PARTS_MARKER} for ${s_type} {
    /// Return a comma separated list of members that are currently set, i.e. for which `self.member.is_some()`.
    /// The produced string is suitable for use as a parts list that indicates the parts you are sending, and/or
    /// the parts you want to see in the server response.
    fn to_parts(&self) -> String {
        let mut r = String::new();
        % for pn, p in items(s.properties):
<%
            mn = 'self.' + mangle_ident(pn)
            rt = to_rust_type(schemas, s.id, pn, p, allow_optionals=allow_optionals)
            check = 'is_some()'
            if rt.startswith('Vec') or rt.startswith('HashMap'):
                check = 'len() > 0'
%>\
        if ${mn}.${check} { r = r + "${pn},"; }
        % endfor
        ## remove (possibly non-existing) trailing comma
        r.pop();
        r
    }
}
% endif
</%def>

#########################################################################################################
#########################################################################################################
<%def name="doc(s, c)">\
${s.get('description', 'There is no detailed description.')}
% if s.id in c.sta_map:

# Activities

This type is used in activities, which are methods you may call on this type or where this type is involved in. 
The list links the activity name, along with information about where it is used (one of ${put_and(enclose_in('*', IO_TYPES))}).

% for a, iot in c.sta_map[s.id].items():
<%
    category, name, method = activity_split(a)
    name_suffix = ' ' + split_camelcase_s(name)
    if name == METHODS_RESOURCE:
        name_suffix = ''
    struct_url = mb_type(name, method)
    method_name = ' '.join(split_camelcase_s(method).split('.')) + name_suffix
    value_type = '|'.join(iot) or 'none'
%>\
* [${method_name}](${struct_url}) (${value_type})
% endfor
% else:

This type is not used in any activity, and only used as *part* of another schema.
% endif
% if s.type != 'object':

## for some reason, it's not shown in rustdoc ... 
The contained type is `${to_rust_type(schemas, s.id, s.id, s)}`.
%endif
</%def>
