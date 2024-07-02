<%!
    from generator.lib.util import (activity_split, put_and, md_italic, split_camelcase_s, canonical_type_name, hub_type,
                      rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment, markdown_rust_block,
                      unindent_first_by, mangle_ident, mb_type, singular, scope_url_to_variant,
                      PART_MARKER_TRAIT, RESOURCE_MARKER_TRAIT, CALL_BUILDER_MARKERT_TRAIT,
                      find_fattest_resource, build_all_params, pass_through, parts_from_params,
                      REQUEST_MARKER_TRAIT, RESPONSE_MARKER_TRAIT, supports_scopes, to_api_version,
                      to_fqan, METHODS_RESOURCE, ADD_PARAM_MEDIA_EXAMPLE, PROTOCOL_TYPE_INFO, enclose_in,
                      upload_action_fn, METHODS_BUILDER_MARKER_TRAIT, DELEGATE_TYPE,
                      to_extern_crate_name, rust_doc_sanitize, escape_rust_string)

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


## Builds any generic enum for the API
###############################################################################################
###############################################################################################
<%def name="new(enum, c)">\
// region ${enum.ty}
% if enum.has_deprecated_variants:
#[allow(non_camel_case_types, deprecated)]
% endif
#[derive(Clone, Copy, Eq, Hash, Debug, PartialEq, Serialize, Deserialize)]
% if enum.description:
${rust_doc_comment(enum.description)}
% endif
pub enum ${enum.ty} {
<%
enum_variants = enum.variants
if not enum_variants:
    print('enum had no variants', enum)
    enum_variants = []
%>\
% for variant in enum_variants:

    % if variant.description:
    ${rust_doc_comment(variant.description)}
    % endif
    /// value:
    /// "${variant.value}"
    #[serde(rename="${variant.value}")]
    % if variant.deprecated:
    #[deprecated(note="${escape_rust_string(variant.deprecation_message)}")]
    % endif
    ${variant.name},
% endfor
}

impl AsRef<str> for ${enum.ty} {
% if enum.has_deprecated_variants:
    #[allow(deprecated)]
% endif
    fn as_ref(&self) -> &str {
        match *self {
            % for variant in enum_variants:
            ${enum.ty}::${variant.name} => "${escape_rust_string(variant.value)}",
            % endfor
        }
    }
}

impl ::std::convert::TryFrom< &str > for ${enum.ty} {
    type Error = ();
% if enum.has_deprecated_variants:
    #[allow(deprecated)]
% endif
    fn try_from(value: &str) -> ::std::result::Result<Self, < ${enum.ty} as ::std::convert::TryFrom < &str > >::Error> {
        match value {
            % for variant in enum_variants:
           "${variant.value}" => ::std::result::Result::Ok(${enum.ty}::${variant.name}),
            % endfor
            _ => ::std::result::Result::Err(()),
        }
    }
}

impl<'a> From < &'a ${enum.ty} > for ::std::borrow::Cow< 'a, str > {
    fn from(val: &'a ${enum.ty}) -> Self {
        val.as_ref().into()
    }
}

% if enum.default is not None:
impl ::core::default::Default for ${enum.ty} {
    fn default() -> ${enum.ty} {
        ${enum.ty}::${enum.default.name}
    }
}
% endif

// endregion

</%def>
