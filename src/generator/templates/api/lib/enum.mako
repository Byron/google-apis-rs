<%!
    from generator.lib.util import (activity_split, put_and, md_italic, split_camelcase_s, canonical_type_name, hub_type,
                      rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment, markdown_rust_block,
                      unindent_first_by, mangle_ident, mb_type, singular, scope_url_to_variant,
                      PART_MARKER_TRAIT, RESOURCE_MARKER_TRAIT, CALL_BUILDER_MARKERT_TRAIT,
                      find_fattest_resource, build_all_params, pass_through, parts_from_params,
                      REQUEST_MARKER_TRAIT, RESPONSE_MARKER_TRAIT, supports_scopes, to_api_version,
                      to_fqan, METHODS_RESOURCE, ADD_PARAM_MEDIA_EXAMPLE, PROTOCOL_TYPE_INFO, enclose_in,
                      upload_action_fn, METHODS_BUILDER_MARKER_TRAIT, DELEGATE_TYPE,
                      to_extern_crate_name, rust_doc_sanitize)

    from generator.lib.enum_utils import (to_enum_variant_name)

    def pretty_name(name):
        return ' '.join(split_camelcase_s(name).split('.'))
%>\
<%namespace name="util" file="../../../lib/util.mako"/>\
<%namespace name="mbuild" file="mbuild.mako"/>\


## Builds the scope-enum for the API
## It's possible there is no scope enum if there is no auth information
###############################################################################################
###############################################################################################
<%def name="scope_enum()">\
% if not supports_scopes(auth):
<% return '' %>\
% endif
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
% for url, scope in auth.oauth2.scopes.items():
    ${scope.description | rust_doc_sanitize(documentationLink), rust_doc_comment}
    ${scope_url_to_variant(name, url, fully_qualified=False)},
    % if not loop.last:

    % endif
% endfor
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            % for url in auth.oauth2.scopes.keys():
            ${scope_url_to_variant(name, url)} => "${url}",
            % endfor
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
<%
            default_url = None
            shortest_url = None
            for url in auth.oauth2.scopes.keys():
                if not default_url and 'readonly' in url:
                    default_url = url
                if not shortest_url or len(shortest_url) > len(url):
                    shortest_url = url
            # end for each url
            default_url = default_url or shortest_url
%>\
        ${scope_url_to_variant(name, default_url)}
    }
}
</%def>


<%def name="new(enum_type, e, c)">\
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
% if e.get('description'):
/// ${e.description}
% endif
pub enum ${enum_type} {
<%
enum_names = e.get('enum')
enum_descriptions = e.get('enumDescriptions')
if not enum_descriptions:
    enum_descriptions = ['no description found'] * len(enum_names)

%>\
% for (variant_name,description) in zip(enum_names, enum_descriptions):
    <% #print(variant_name, '=>', description)
    %>
    % if description:

    /// ${description}
    ///
    % endif\
    /// value:
    /// "${variant_name}"
    #[serde(rename="${variant_name}")]
    ${to_enum_variant_name(variant_name)},
% endfor
}

impl AsRef<str> for ${enum_type} {
    fn as_ref(&self) -> &str {
        match *self {
            % for variant in e.get('enum'):
            ${enum_type}::${to_enum_variant_name(variant)} => "${variant_name}",
            % endfor
        }
    }
}

impl<'a> Into<std::borrow::Cow<'a, str>> for &'a ${enum_type} {
    fn into(self) -> std::borrow::Cow<'a, str> {
        self.as_ref().into()
    }
}

% if e.get('default') is not None:
impl Default for ${enum_type} {
    fn default() -> ${enum_type} {
        ${enum_type}::${to_enum_variant_name(e.get('default'))}
    }
}
% endif
</%def>
